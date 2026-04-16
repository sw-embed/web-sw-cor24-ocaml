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

/// Per-tick total instruction budget across all inner sub-batches.
/// Snobol4 and friends use ~200k as a single batch; OCaml is heavier
/// so we run a larger total budget broken into smaller inner batches
/// so we can feed UART bytes between them (see `INNER_BATCH`).
pub const DEFAULT_BATCH: u64 = 1_000_000;

/// Inner sub-batch size between UART feeds. Small enough that the
/// next source byte is delivered before the interpreter spends much
/// time busy-polling the UART RX register; large enough to amortize
/// the per-call overhead of `EmulatorCore::run_batch`. Tuned by
/// inspection of the OCaml REPL's read loop -- a few hundred cor24
/// instructions consume a byte and resume polling.
const INNER_BATCH: u64 = 50_000;

/// Memory-mapped UART status port. Bit 0 set means a byte is sitting
/// in the RX register waiting to be read; clear means the register
/// is empty and we can push a fresh byte without overflow.
const IO_UARTSTAT: u32 = 0xFF_0101;
const UARTSTAT_RX_READY: u8 = 0x01;

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

/// Remove REPL prompt segments from a raw UART transcript. Anywhere a
/// line starts with `"> "` (the OCaml REPL's prompt), drop the prompt
/// and the echoed source bytes that follow it up to and including the
/// next newline. Result text on subsequent lines passes through.
///
/// Handles both shapes:
/// - One-shot, no trailing newline:
///   `"> print_int 4242"` -> `"42"` (echo runs to end of buffer for
///   12 bytes then result begins; we anchor on the source we sent
///   only when echo ends at a newline, so the no-newline case still
///   conflates echo with result -- callers wanting clean separation
///   should ensure their source ends with `\n`).
/// - Multi-line REPL session: each `"> <line>\n"` is dropped.
fn strip_prompt_echoes(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let bytes = raw.as_bytes();
    let mut i = 0;
    let mut at_line_start = true;
    while i < bytes.len() {
        if at_line_start && i + 1 < bytes.len() && bytes[i] == b'>' && bytes[i + 1] == b' ' {
            // Skip "> " then echoed source up to and including the next newline.
            i += 2;
            while i < bytes.len() {
                let b = bytes[i];
                i += 1;
                if b == b'\n' {
                    break;
                }
            }
            at_line_start = true;
            continue;
        }
        let b = bytes[i];
        out.push(b as char);
        at_line_start = b == b'\n';
        i += 1;
    }
    out
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
        // Ensure each source ends with a newline so the REPL's read
        // loop terminates the echoed line cleanly. Without this, a
        // one-shot like `print_int 42` (no trailing newline) emits
        // `"> print_int 4242"` with no separator between echoed
        // source and the computed result, and clean_output() can't
        // tell them apart.
        if !source.ends_with('\n') {
            rx_queue.push_back(b'\n');
        }
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

    pub fn tick_with_budget(&mut self, budget: u64) -> TickResult {
        if self.done {
            return TickResult { done: true };
        }

        let mut total: u64 = 0;
        while total < budget {
            // Feed bytes opportunistically: every time the UART RX
            // register is empty, push the next queued byte. This
            // collapses what used to be N ticks of single-byte feed
            // into one tick that delivers bytes as fast as the
            // interpreter consumes them.
            while !self.rx_ready()
                && let Some(b) = self.rx_queue.pop_front()
            {
                self.emu.send_uart_byte(b);
                if self.rx_queue.is_empty() {
                    self.seeding_source = false;
                }
            }

            let chunk = INNER_BATCH.min(budget - total);
            let result = self.emu.run_batch(chunk);
            self.instructions += result.instructions_run;
            total += result.instructions_run;

            match result.reason {
                StopReason::Halted => {
                    self.done = true;
                    self.halted = true;
                    self.stop_reason = "halted".into();
                    break;
                }
                StopReason::InvalidInstruction(byte) => {
                    self.done = true;
                    self.stop_reason = format!(
                        "invalid instruction 0x{:02X} at PC=0x{:06X}",
                        byte,
                        self.emu.pc()
                    );
                    break;
                }
                StopReason::Breakpoint(addr) => {
                    self.done = true;
                    self.stop_reason = format!("breakpoint at 0x{:06X}", addr);
                    break;
                }
                StopReason::Paused => {
                    self.done = true;
                    self.stop_reason = "paused".into();
                    break;
                }
                StopReason::StackOverflow(addr) => {
                    self.done = true;
                    self.stop_reason = format!("stack overflow at SP=0x{:06X}", addr);
                    break;
                }
                StopReason::StackUnderflow(addr) => {
                    self.done = true;
                    self.stop_reason = format!("stack underflow at SP=0x{:06X}", addr);
                    break;
                }
                StopReason::CycleLimit => {
                    if result.instructions_run == 0 {
                        self.done = true;
                        self.stop_reason = "stalled".into();
                        break;
                    }
                    // Loop and run another inner batch.
                }
            }
        }
        TickResult { done: self.done }
    }

    fn rx_ready(&self) -> bool {
        (self.emu.read_byte(IO_UARTSTAT) & UARTSTAT_RX_READY) != 0
    }

    /// Push the COR24 S2 switch state into the emulator. Called from
    /// the App on every tick before run_batch so the cor24 switch
    /// register reflects the user's current toggle state. The
    /// emulator's set_button_pressed is cheap (one byte write).
    pub fn set_switch(&mut self, on: bool) {
        self.emu.set_button_pressed(on);
    }

    /// Read the current COR24 D2 LED state. Call after each tick to
    /// drive the hardware panel indicator.
    pub fn led_on(&self) -> bool {
        self.emu.is_led_on()
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

    /// UART output with the pvm boot banner, REPL prompts, echoed
    /// source, and trailing CRs stripped -- only the program's actual
    /// output remains. The OCaml interpreter (`ocaml.pas`'s
    /// `lex_init`) echoes every input character before evaluating;
    /// the user already sees the source in the editor pane, so we
    /// suppress it in the output panel here.
    pub fn clean_output(&self) -> String {
        let raw = self.output();
        let stripped = strip_prompt_echoes(&raw);
        let mut out = String::with_capacity(stripped.len());
        for line in stripped.lines() {
            let t = line.trim_end_matches('\r').trim();
            if t.is_empty() || t == "PVM OK" || t == "HALT" {
                continue;
            }
            if !out.is_empty() {
                out.push('\n');
            }
            out.push_str(line.trim_end_matches('\r'));
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
    /// mode only). This is a conservative approximation: "interactive,
    /// no queued bytes, and we already finished seeding the source."
    ///
    /// Note: this can return true before the interp has finished
    /// evaluating a line we just fed it (rx_queue empties as soon as
    /// all bytes are pushed to UART, which may precede the interp
    /// actually emitting its result). The App layer compensates by
    /// tracking output changes — see `output_len_at_feed` in lib.rs.
    pub fn is_awaiting_input(&self) -> bool {
        self.interactive && !self.seeding_source && self.rx_queue.is_empty()
    }

    pub fn raw_output_len(&self) -> usize {
        self.emu.get_uart_output().len()
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
        eprintln!(
            "print_int 42: {} cor24 instructions to halt",
            s.instructions
        );
        let cleaned = s.clean_output();
        assert_eq!(
            cleaned,
            "42",
            "expected exactly '42' after stripping echo, got: {cleaned:?} (raw: {:?})",
            s.output()
        );
    }

    #[test]
    fn strip_prompt_echoes_one_shot() {
        // Mirrors the raw UART transcript an OCaml REPL produces for
        // a single-line source ending in newline: prompt + echoed
        // line + crlf + result.
        let raw = "PVM OK\r\n> print_int 42\r\n42";
        let cleaned = strip_prompt_echoes(raw);
        // Both prompt-and-echo lines stripped; result remains.
        assert!(
            !cleaned.contains("print_int"),
            "echo not stripped: {cleaned:?}"
        );
        assert!(cleaned.contains("42"), "result missing: {cleaned:?}");
    }

    #[test]
    fn strip_prompt_echoes_multi_line() {
        let raw = "PVM OK\r\n> 42\r\n42\r\n> let x = 1 + 1 in x\r\n2\r\n";
        let cleaned = strip_prompt_echoes(raw);
        assert!(!cleaned.contains("> 42"), "echo line not stripped");
        assert!(!cleaned.contains("let x"), "second echo not stripped");
        assert!(cleaned.contains("42"), "first result missing");
        assert!(cleaned.contains("2"), "second result missing");
    }
}
