//! OCaml interpreter session for the browser.
//!
//! This is the same architecture used by `../web-sw-cor24-pascal`,
//! `../web-sw-cor24-snobol4`, and the rest of the family: load
//! `pvm.bin` (assembled from the canonical `asm/pvm.s`) at address 0
//! of a `cor24-emulator::EmulatorCore`, place the linked OCaml
//! interpreter image (`ocaml.p24m`) at `OCAML_LOAD_ADDR`, patch the
//! pvm `code_ptr` slot, and let pvm boot. pvm sees the `P24M` magic at
//! the load address and walks the multi-unit header itself, so we
//! avoid every multi-unit / IRT / opcode-table edge case the previous
//! Rust-native interpreter had to handle.
//!
//! The OCaml REPL on top of pvm reads its source from UART terminated
//! by `0x04` (EOT). One-shot sessions queue source + EOT at startup;
//! interactive sessions leave the stream open so `feed_input()` can
//! append more bytes as the user types.

use std::collections::VecDeque;

use cor24_emulator::{EmulatorCore, StopReason};

use crate::config::{OCAML_LOAD_ADDR, OCAML_P24M, PVM_BIN, label_addr};

/// Per-tick instruction budget. Snobol4 and friends use ~200k; OCaml
/// interpretation of even small expressions is heavier (we measured
/// ~6 million pvm instructions for `print_int 42` end-to-end), so a
/// larger batch keeps the frame budget reasonable.
pub const DEFAULT_BATCH: u64 = 1_000_000;

pub struct Session {
    emu: EmulatorCore,
    pub instructions: u64,
    pub done: bool,
    pub halted: bool,
    pub stop_reason: String,
    interactive: bool,
    /// UART RX has FIFO depth 1; we drain at most one byte per tick.
    rx_queue: VecDeque<u8>,
    /// If true, every byte we drain is the program's source. Once
    /// drained, we set this to false and the queue becomes the channel
    /// for `feed_input`.
    seeding_source: bool,
}

pub struct TickResult {
    pub done: bool,
}

impl Session {
    pub fn new(source: &str) -> Self {
        Self::new_with_mode(source, false)
    }

    pub fn new_interactive(source: &str) -> Self {
        Self::new_with_mode(source, true)
    }

    fn new_with_mode(source: &str, interactive: bool) -> Self {
        let mut emu = EmulatorCore::new();
        // Skip the UART TX busy-cycle simulation — we want bytes
        // through immediately rather than throttled at hardware speed.
        emu.set_uart_tx_busy_cycles(0);

        // Place pvm.bin at address 0.
        for (i, &b) in PVM_BIN.iter().enumerate() {
            emu.write_byte(i as u32, b);
        }

        // Place ocaml.p24m at the address its absolute data refs were
        // baked for (`p24-load --load-addr 0x010000`).
        for (i, &b) in OCAML_P24M.iter().enumerate() {
            emu.write_byte(OCAML_LOAD_ADDR + i as u32, b);
        }

        // Patch pvm's `code_ptr` slot to point at the loaded p24m. At
        // boot, pvm reads code_ptr, sees the "P24M" magic, and runs
        // its `init_p24m` path which sets up code_base, unit_table,
        // irt_base, and entry_point from the header.
        let code_ptr_addr = label_addr("code_ptr");
        emu.write_byte(code_ptr_addr, OCAML_LOAD_ADDR as u8);
        emu.write_byte(code_ptr_addr + 1, (OCAML_LOAD_ADDR >> 8) as u8);
        emu.write_byte(code_ptr_addr + 2, (OCAML_LOAD_ADDR >> 16) as u8);

        // Start at pvm's reset entry (address 0).
        emu.set_pc(0);
        emu.resume();

        let mut rx_queue: VecDeque<u8> = source.bytes().collect();
        if !interactive {
            rx_queue.push_back(0x04);
        }

        Self {
            emu,
            instructions: 0,
            done: false,
            halted: false,
            stop_reason: String::new(),
            interactive,
            rx_queue,
            seeding_source: true,
        }
    }

    pub fn tick(&mut self) -> TickResult {
        self.tick_with_budget(DEFAULT_BATCH)
    }

    pub fn tick_with_budget(&mut self, batch: u64) -> TickResult {
        if self.done {
            return TickResult { done: true };
        }

        // Drain at most one queued RX byte per tick — UART RX is
        // depth 1 so the previous batch needs to have given the
        // interpreter time to consume the prior byte. A million
        // pvm instructions is more than enough for a single GETC
        // round trip.
        if let Some(b) = self.rx_queue.pop_front() {
            self.emu.send_uart_byte(b);
            if self.rx_queue.is_empty() {
                self.seeding_source = false;
            }
        }

        let result = self.emu.run_batch(batch);
        self.instructions += result.instructions_run as u64;

        match result.reason {
            StopReason::Halted => {
                self.done = true;
                self.halted = true;
                self.stop_reason = "halted".into();
            }
            StopReason::InvalidInstruction(byte) => {
                self.done = true;
                self.stop_reason = format!(
                    "invalid instruction 0x{:02X} at PC=0x{:06X}",
                    byte,
                    self.emu.pc()
                );
            }
            StopReason::Breakpoint(addr) => {
                self.done = true;
                self.stop_reason = format!("breakpoint at 0x{:06X}", addr);
            }
            StopReason::Paused => {
                self.done = true;
                self.stop_reason = "paused".into();
            }
            StopReason::StackOverflow(addr) => {
                self.done = true;
                self.stop_reason = format!("stack overflow at SP=0x{:06X}", addr);
            }
            StopReason::StackUnderflow(addr) => {
                self.done = true;
                self.stop_reason = format!("stack underflow at SP=0x{:06X}", addr);
            }
            StopReason::CycleLimit => {
                if result.instructions_run == 0 {
                    self.done = true;
                    self.stop_reason = "stalled".into();
                }
            }
        }
        TickResult { done: self.done }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn is_halted(&self) -> bool {
        self.halted
    }

    pub fn stop_reason(&self) -> String {
        self.stop_reason.clone()
    }

    pub fn instructions(&self) -> u64 {
        self.instructions
    }

    /// Raw UART output: pvm boot banner ("PVM OK\n") plus everything
    /// the OCaml REPL wrote.
    pub fn output(&self) -> String {
        self.emu.get_uart_output().to_string()
    }

    /// UART output with the pvm boot banner and OCaml REPL prompt
    /// markers stripped, mirroring the awk/sed pipeline in
    /// `../sw-cor24-ocaml/scripts/run-ocaml.sh`.
    pub fn clean_output(&self) -> String {
        let raw = self.output();
        let mut out = String::with_capacity(raw.len());
        for line in raw.lines() {
            let t = line.trim();
            if t.is_empty() || t == "PVM OK" || t == "HALT" {
                continue;
            }
            if !out.is_empty() {
                out.push('\n');
            }
            out.push_str(line);
        }
        out
    }

    /// Append a line of user input for the interactive REPL. The
    /// OCaml interpreter reads expression-by-expression terminated by
    /// `\n`, so we ensure each line ends with a newline.
    pub fn feed_input(&mut self, line: &str) {
        if !self.interactive {
            return;
        }
        self.rx_queue.extend(line.bytes());
        if !line.ends_with('\n') {
            self.rx_queue.push_back(b'\n');
        }
    }

    /// True when the interpreter is waiting on user input (interactive
    /// mode only). For now this is a conservative approximation:
    /// "interactive, no queued bytes, and we already finished seeding
    /// the source" — same heuristic snobol4 uses.
    pub fn is_awaiting_input(&self) -> bool {
        self.interactive && !self.seeding_source && self.rx_queue.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_to_halt(s: &mut Session, max_ticks: u64) {
        for _ in 0..max_ticks {
            let r = s.tick();
            if r.done {
                break;
            }
        }
    }

    #[test]
    fn print_int_42_runs_to_completion_with_42_in_output() {
        let mut s = Session::new("print_int 42");
        // OCaml interpretation of even tiny expressions is multi-million
        // cor24 instructions; allow a generous tick budget.
        run_to_halt(&mut s, 200);
        assert!(
            s.is_done(),
            "session did not finish: instrs={}",
            s.instructions
        );
        assert!(
            s.is_halted(),
            "session stopped abnormally: {} (instrs={}, out={:?})",
            s.stop_reason(),
            s.instructions,
            s.output()
        );
        assert!(
            s.clean_output().contains("42"),
            "expected '42' in cleaned output, got: {:?}",
            s.output()
        );
    }
}
