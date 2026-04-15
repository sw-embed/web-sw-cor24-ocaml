//! Browser p-code VM for the OCaml interpreter image.
//!
//! Ported from pv24t (sw-cor24-pcode/tracer/src/lib.rs) and extended to
//! load a multi-unit .p24m image instead of a single-unit .p24, with
//! runtime support for cross-unit XCall / XLoadg / XStoreg via the
//! import resolution tables baked by p24-load.

pub const MASK24: i32 = 0x00FF_FFFF;

#[inline]
fn sign_extend_24(v: i32) -> i32 {
    let v = v & MASK24;
    if v & 0x0080_0000 != 0 { v | !MASK24 } else { v }
}

#[inline]
fn wrap24(v: i32) -> i32 {
    sign_extend_24(v)
}

#[inline]
fn read_le24(b: &[u8], o: usize) -> u32 {
    (b[o] as u32) | ((b[o + 1] as u32) << 8) | ((b[o + 2] as u32) << 16)
}

#[inline]
fn read_le16(b: &[u8], o: usize) -> u16 {
    (b[o] as u16) | ((b[o + 1] as u16) << 8)
}

const WORD: usize = 3;
const CALL_STACK_WORDS: usize = 512;
const EVAL_STACK_WORDS: usize = 512;
const HEAP_WORDS: usize = 4096;
const BATCH_SIZE: u64 = 200_000;

/// Runtime load address for the .p24m image. Matches the CLI script
/// `scripts/run-ocaml.sh` (`ocaml.p24m@0x010000`) so that absolute
/// addresses baked by `p24-load --load-addr 0x010000` resolve correctly.
const LOAD_ADDR: usize = 0x01_0000;

/// Parsed .p24m header information needed to lay out VM memory and
/// service cross-unit operations at runtime.
#[derive(Debug)]
struct P24mLayout {
    entry_code_rel: u32,
    total_code: u32,
    #[allow(dead_code)]
    total_globals: u32,
    code_off: usize,
    globals_off: usize,
    #[allow(dead_code)]
    unit_table_off: usize,
    units: Vec<UnitEntry>,
    irt_abs: Vec<Vec<u32>>, // per-unit IRT target addresses (absolute)
}

#[derive(Debug, Clone, Copy)]
struct UnitEntry {
    base_addr: u32,
    global_base: u32,
    #[allow(dead_code)]
    irt_off: u32,
}

#[derive(Debug)]
pub enum LoadError {
    BadMagic,
    BadVersion(u8),
    Truncated,
}

fn parse_p24m(data: &[u8]) -> Result<P24mLayout, LoadError> {
    const P24M_HEADER_SIZE: usize = 27;
    if data.len() < P24M_HEADER_SIZE {
        return Err(LoadError::Truncated);
    }
    if &data[0..4] != b"P24M" {
        return Err(LoadError::BadMagic);
    }
    if data[4] != 1 {
        return Err(LoadError::BadVersion(data[4]));
    }
    let entry_code_rel = read_le24(data, 5);
    let unit_count = data[8] as usize;
    let total_code = read_le24(data, 9);
    let total_globals = read_le24(data, 12);
    let unit_table_off = read_le24(data, 15) as usize;
    let irt_off = read_le24(data, 18) as usize;
    let code_off = read_le24(data, 21) as usize;
    let globals_off = read_le24(data, 24) as usize;

    let mut units = Vec::with_capacity(unit_count);
    let mut pos = unit_table_off;
    for _ in 0..unit_count {
        if pos + 9 > data.len() {
            return Err(LoadError::Truncated);
        }
        units.push(UnitEntry {
            base_addr: read_le24(data, pos),
            global_base: read_le24(data, pos + 3),
            irt_off: read_le24(data, pos + 6),
        });
        pos += 9;
    }

    let mut irt_abs = Vec::with_capacity(unit_count);
    let mut cur = irt_off;
    for _ in 0..unit_count {
        if cur + 2 > data.len() {
            return Err(LoadError::Truncated);
        }
        let count = read_le16(data, cur) as usize;
        cur += 2;
        let mut entries = Vec::with_capacity(count);
        for _ in 0..count {
            if cur + 3 > data.len() {
                return Err(LoadError::Truncated);
            }
            entries.push(read_le24(data, cur));
            cur += 3;
        }
        irt_abs.push(entries);
    }

    Ok(P24mLayout {
        entry_code_rel,
        total_code,
        total_globals,
        code_off,
        globals_off,
        unit_table_off,
        units,
        irt_abs,
    })
}

/// A WASM-friendly execution session for one OCaml program.
///
/// The caller drives the VM by repeatedly invoking `tick()` until
/// `is_done()` returns true or `is_awaiting_input()` is true (in
/// interactive mode). Output is accumulated in a UART byte buffer and
/// is available via `output()` / `clean_output()`.
pub struct Session {
    mem: Vec<u8>,
    pc: usize,
    esp: usize,
    csp: usize,
    fp_vm: usize,
    gp: usize,
    hp: usize,
    code_abs_base: usize,
    code_end: usize,
    eval_stack_base: usize,
    heap_base: usize,

    // multi-unit bookkeeping
    units: Vec<UnitEntry>,
    unit_abs_code_ranges: Vec<(usize, usize)>, // (start, end) in absolute memory for each unit
    irt_abs_addrs: Vec<Vec<u32>>,              // per-unit IRT target addresses (abs)

    // I/O
    stdin_buf: Vec<u8>,
    stdin_pos: usize,
    stdout_buf: Vec<u8>,
    interactive: bool,
    awaiting_input: bool,

    // status
    status: u8,
    trap_code: u8,
    instruction_count: u64,
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
        let p24m = crate::config::OCAML_P24M;
        let layout = match parse_p24m(p24m) {
            Ok(l) => l,
            Err(e) => {
                web_sys::console::error_1(&format!("p24m parse error: {:?}", e).into());
                return Self::dummy();
            }
        };

        let code_abs_base = LOAD_ADDR + layout.code_off;
        let code_end = code_abs_base + layout.total_code as usize;
        let file_end_abs = LOAD_ADDR + p24m.len();

        // Globals section is written zeroed inside the .p24m file by
        // p24-load (after data), so its VM location is LOAD_ADDR +
        // globals_off and its contents come from the file bytes.
        let gp = LOAD_ADDR + layout.globals_off;

        // Stacks and heap live beyond the end of the file image.
        let call_stack_base = file_end_abs;
        let call_stack_size = CALL_STACK_WORDS * WORD;
        let eval_stack_base = call_stack_base + call_stack_size;
        let eval_stack_size = EVAL_STACK_WORDS * WORD;
        let heap_base = eval_stack_base + eval_stack_size;
        let heap_size = HEAP_WORDS * WORD;

        let total = heap_base + heap_size;
        let mut mem = vec![0u8; total];
        mem[LOAD_ADDR..LOAD_ADDR + p24m.len()].copy_from_slice(p24m);

        let unit_abs_code_ranges: Vec<(usize, usize)> = {
            let mut v = Vec::with_capacity(layout.units.len());
            for i in 0..layout.units.len() {
                let start = code_abs_base + layout.units[i].base_addr as usize;
                let end = if i + 1 < layout.units.len() {
                    code_abs_base + layout.units[i + 1].base_addr as usize
                } else {
                    code_end
                };
                v.push((start, end));
            }
            v
        };

        let pc = code_abs_base + layout.entry_code_rel as usize;

        // Seed the UART with the source. For one-shot runs, terminate
        // with EOT so the interpreter's REPL sees end-of-input and
        // halts. For interactive runs, leave the stream open for
        // feed_input() to append more bytes as the user types.
        let mut stdin_buf: Vec<u8> = source.bytes().collect();
        if !interactive {
            stdin_buf.push(0x04);
        }

        Session {
            mem,
            pc,
            esp: eval_stack_base,
            csp: call_stack_base,
            fp_vm: call_stack_base,
            gp,
            hp: heap_base,
            code_abs_base,
            code_end,
            eval_stack_base,
            heap_base,
            units: layout.units,
            unit_abs_code_ranges,
            irt_abs_addrs: layout.irt_abs,
            stdin_buf,
            stdin_pos: 0,
            stdout_buf: Vec::new(),
            interactive,
            awaiting_input: false,
            status: 0,
            trap_code: 0,
            instruction_count: 0,
        }
    }

    fn dummy() -> Self {
        Session {
            mem: vec![0u8; 1],
            pc: 0,
            esp: 0,
            csp: 0,
            fp_vm: 0,
            gp: 0,
            hp: 0,
            code_abs_base: 0,
            code_end: 0,
            eval_stack_base: 0,
            heap_base: 0,
            units: Vec::new(),
            unit_abs_code_ranges: Vec::new(),
            irt_abs_addrs: Vec::new(),
            stdin_buf: Vec::new(),
            stdin_pos: 0,
            stdout_buf: Vec::new(),
            interactive: false,
            awaiting_input: false,
            status: 1,
            trap_code: 0,
            instruction_count: 0,
        }
    }

    pub fn is_awaiting_input(&self) -> bool {
        self.awaiting_input
    }

    pub fn feed_input(&mut self, line: &str) {
        if !self.interactive {
            return;
        }
        self.stdin_buf.extend_from_slice(line.as_bytes());
        if !line.ends_with('\n') {
            self.stdin_buf.push(b'\n');
        }
        self.awaiting_input = false;
    }

    pub fn is_done(&self) -> bool {
        self.status != 0
    }

    pub fn is_halted(&self) -> bool {
        self.status == 1
    }

    pub fn stop_reason(&self) -> String {
        if self.status == 1 {
            "halted".into()
        } else {
            format!("trap {}", self.trap_code)
        }
    }

    pub fn instructions(&self) -> u64 {
        self.instruction_count
    }

    /// Raw UART output bytes as a lossy UTF-8 string.
    pub fn output(&self) -> String {
        String::from_utf8_lossy(&self.stdout_buf).into_owned()
    }

    /// UART output with the interpreter's REPL banner / prompts
    /// stripped, matching the awk/sed pipeline in
    /// `../sw-cor24-ocaml/scripts/run-ocaml.sh`.
    pub fn clean_output(&self) -> String {
        let raw = self.output();
        let stripped: Vec<&str> = raw
            .lines()
            .filter(|l| !l.trim().is_empty() && l.trim() != "PVM OK" && l.trim() != "HALT")
            .collect();
        stripped.join("\n")
    }

    pub fn tick(&mut self) -> TickResult {
        if self.status != 0 {
            return TickResult { done: true };
        }
        if self.awaiting_input {
            return TickResult { done: false };
        }
        let mut ran = 0u64;
        while self.status == 0 && !self.awaiting_input && ran < BATCH_SIZE {
            let op = self.fetch_u8();
            self.execute(op);
            if self.awaiting_input {
                break;
            }
            self.instruction_count += 1;
            ran += 1;
        }
        TickResult {
            done: self.status != 0,
        }
    }

    // --- memory helpers ---

    fn read_word(&self, addr: usize) -> i32 {
        if addr + 2 >= self.mem.len() {
            return 0;
        }
        let lo = self.mem[addr] as i32;
        let mid = self.mem[addr + 1] as i32;
        let hi = self.mem[addr + 2] as i32;
        sign_extend_24(lo | (mid << 8) | (hi << 16))
    }

    fn write_word(&mut self, addr: usize, val: i32) {
        if addr + 2 >= self.mem.len() {
            self.trap(2);
            return;
        }
        let v = val & MASK24;
        self.mem[addr] = v as u8;
        self.mem[addr + 1] = (v >> 8) as u8;
        self.mem[addr + 2] = (v >> 16) as u8;
    }

    fn read_byte(&self, addr: usize) -> i32 {
        if addr >= self.mem.len() {
            0
        } else {
            self.mem[addr] as i32
        }
    }

    fn write_byte(&mut self, addr: usize, val: i32) {
        if addr >= self.mem.len() {
            self.trap(2);
            return;
        }
        self.mem[addr] = val as u8;
    }

    fn fetch_u8(&mut self) -> u8 {
        let v = if self.pc < self.code_end && self.pc >= self.code_abs_base {
            self.mem[self.pc]
        } else {
            0
        };
        self.pc += 1;
        v
    }

    fn fetch_i8(&mut self) -> i32 {
        self.fetch_u8() as i8 as i32
    }

    fn fetch_u24(&mut self) -> u32 {
        let lo = self.fetch_u8() as u32;
        let mid = self.fetch_u8() as u32;
        let hi = self.fetch_u8() as u32;
        lo | (mid << 8) | (hi << 16)
    }

    fn push_eval(&mut self, val: i32) {
        if self.esp >= self.heap_base {
            self.trap(2);
            return;
        }
        self.write_word(self.esp, val);
        self.esp += WORD;
    }

    fn pop_eval(&mut self) -> i32 {
        if self.esp <= self.eval_stack_base {
            self.trap(3);
            return 0;
        }
        self.esp -= WORD;
        self.read_word(self.esp)
    }

    fn peek_eval(&self) -> i32 {
        if self.esp <= self.eval_stack_base {
            0
        } else {
            self.read_word(self.esp - WORD)
        }
    }

    fn trap(&mut self, code: u8) {
        self.status = 2;
        self.trap_code = code;
    }

    fn follow_static_links(&self, depth: usize) -> usize {
        let mut frame = self.fp_vm;
        for _ in 0..depth {
            frame = self.read_word(frame + 2 * WORD) as usize;
        }
        frame
    }

    /// Which unit (0..units.len()) does the current PC fall into.
    fn current_unit(&self) -> usize {
        for (i, (start, end)) in self.unit_abs_code_ranges.iter().enumerate() {
            if self.pc >= *start && self.pc < *end {
                return i;
            }
        }
        0
    }

    fn jump_code_rel(&mut self, operand: u32) {
        self.pc = self.code_abs_base + operand as usize;
    }

    // --- execute ---

    fn execute(&mut self, op: u8) {
        match op {
            // Stack
            0x00 => self.status = 1,
            0x01 => {
                let v = self.fetch_u24() as i32;
                self.push_eval(sign_extend_24(v));
            }
            0x02 => {
                let v = self.fetch_i8();
                self.push_eval(wrap24(v));
            }
            0x03 => {
                let v = self.peek_eval();
                self.push_eval(v);
            }
            0x04 => {
                self.pop_eval();
            }
            0x05 => {
                let b = self.pop_eval();
                let a = self.pop_eval();
                self.push_eval(b);
                self.push_eval(a);
            }
            0x06 => {
                let b = self.pop_eval();
                let a = self.pop_eval();
                self.push_eval(a);
                self.push_eval(b);
                self.push_eval(a);
            }

            // Arithmetic
            0x10 => {
                let b = self.pop_eval();
                let a = self.pop_eval();
                self.push_eval(wrap24(a.wrapping_add(b)));
            }
            0x11 => {
                let b = self.pop_eval();
                let a = self.pop_eval();
                self.push_eval(wrap24(a.wrapping_sub(b)));
            }
            0x12 => {
                let b = self.pop_eval();
                let a = self.pop_eval();
                self.push_eval(wrap24(a.wrapping_mul(b)));
            }
            0x13 => {
                let b = self.pop_eval();
                let a = self.pop_eval();
                if b == 0 {
                    self.trap(1);
                    return;
                }
                self.push_eval(wrap24(a / b));
            }
            0x14 => {
                let b = self.pop_eval();
                let a = self.pop_eval();
                if b == 0 {
                    self.trap(1);
                    return;
                }
                self.push_eval(wrap24(a % b));
            }
            0x15 => {
                let a = self.pop_eval();
                self.push_eval(wrap24(-a));
            }

            // Logic
            0x16 => {
                let b = self.pop_eval();
                let a = self.pop_eval();
                self.push_eval(wrap24(a & b));
            }
            0x17 => {
                let b = self.pop_eval();
                let a = self.pop_eval();
                self.push_eval(wrap24(a | b));
            }
            0x18 => {
                let b = self.pop_eval();
                let a = self.pop_eval();
                self.push_eval(wrap24(a ^ b));
            }
            0x19 => {
                let a = self.pop_eval();
                self.push_eval(wrap24(!a));
            }
            0x1A => {
                let n = self.pop_eval();
                let a = self.pop_eval();
                self.push_eval(wrap24(a << (n & 0x1F)));
            }
            0x1B => {
                let n = self.pop_eval();
                let a = self.pop_eval();
                self.push_eval(wrap24(a >> (n & 0x1F)));
            }

            // Comparison
            0x20 => {
                let b = self.pop_eval();
                let a = self.pop_eval();
                self.push_eval(if a == b { 1 } else { 0 });
            }
            0x21 => {
                let b = self.pop_eval();
                let a = self.pop_eval();
                self.push_eval(if a != b { 1 } else { 0 });
            }
            0x22 => {
                let b = self.pop_eval();
                let a = self.pop_eval();
                self.push_eval(if a < b { 1 } else { 0 });
            }
            0x23 => {
                let b = self.pop_eval();
                let a = self.pop_eval();
                self.push_eval(if a <= b { 1 } else { 0 });
            }
            0x24 => {
                let b = self.pop_eval();
                let a = self.pop_eval();
                self.push_eval(if a > b { 1 } else { 0 });
            }
            0x25 => {
                let b = self.pop_eval();
                let a = self.pop_eval();
                self.push_eval(if a >= b { 1 } else { 0 });
            }

            // Control flow — operands are code-section-relative, so
            // add code_abs_base to reach the absolute PC.
            0x30 => {
                let addr = self.fetch_u24();
                self.jump_code_rel(addr);
            }
            0x31 => {
                let addr = self.fetch_u24();
                let flag = self.pop_eval();
                if flag == 0 {
                    self.jump_code_rel(addr);
                }
            }
            0x32 => {
                let addr = self.fetch_u24();
                let flag = self.pop_eval();
                if flag != 0 {
                    self.jump_code_rel(addr);
                }
            }
            0x33 => {
                let addr = self.fetch_u24();
                self.write_word(self.csp, self.pc as i32);
                self.write_word(self.csp + WORD, self.fp_vm as i32);
                self.write_word(self.csp + 2 * WORD, self.fp_vm as i32);
                self.write_word(self.csp + 3 * WORD, self.esp as i32);
                self.csp += 4 * WORD;
                self.jump_code_rel(addr);
            }
            0x34 => {
                let nargs = self.fetch_u8() as usize;
                let return_pc = self.read_word(self.fp_vm) as usize;
                let dynamic_link = self.read_word(self.fp_vm + WORD) as usize;
                let saved_esp = self.read_word(self.fp_vm + 3 * WORD) as usize;
                let has_return = self.esp > saved_esp;
                let return_val = if has_return {
                    Some(self.pop_eval())
                } else {
                    None
                };
                self.csp = self.fp_vm;
                self.fp_vm = dynamic_link;
                self.esp = saved_esp - nargs * WORD;
                if let Some(rv) = return_val {
                    self.push_eval(rv);
                }
                self.pc = return_pc;
            }
            0x35 => {
                let depth = self.fetch_u8();
                let addr = self.fetch_u24();
                let mut sl = self.fp_vm;
                for _ in 0..depth {
                    sl = self.read_word(sl + 2 * WORD) as usize;
                }
                self.write_word(self.csp, self.pc as i32);
                self.write_word(self.csp + WORD, self.fp_vm as i32);
                self.write_word(self.csp + 2 * WORD, sl as i32);
                self.write_word(self.csp + 3 * WORD, self.esp as i32);
                self.csp += 4 * WORD;
                self.jump_code_rel(addr);
            }
            0x36 => {
                let code = self.fetch_u8();
                self.trap(code);
            }

            // Frame management
            0x40 => {
                let nlocals = self.fetch_u8() as usize;
                self.fp_vm = self.csp - 4 * WORD;
                for _ in 0..nlocals {
                    self.write_word(self.csp, 0);
                    self.csp += WORD;
                }
            }
            0x41 => {
                self.csp = self.fp_vm + 4 * WORD;
            }

            // Local, global, arg, nonlocal access
            0x42 => {
                let off = self.fetch_u8() as usize;
                let addr = self.fp_vm + 4 * WORD + off * WORD;
                self.push_eval(self.read_word(addr));
            }
            0x43 => {
                let off = self.fetch_u8() as usize;
                let val = self.pop_eval();
                let addr = self.fp_vm + 4 * WORD + off * WORD;
                self.write_word(addr, val);
            }
            0x44 => {
                let off = self.fetch_u24() as usize;
                self.push_eval(self.read_word(self.gp + off * WORD));
            }
            0x45 => {
                let off = self.fetch_u24() as usize;
                let val = self.pop_eval();
                self.write_word(self.gp + off * WORD, val);
            }
            0x46 => {
                let off = self.fetch_u8() as usize;
                let addr = self.fp_vm + 4 * WORD + off * WORD;
                self.push_eval(addr as i32);
            }
            0x47 => {
                let off = self.fetch_u24() as usize;
                self.push_eval((self.gp + off * WORD) as i32);
            }
            0x48 => {
                let idx = self.fetch_u8() as usize;
                let saved_esp = self.read_word(self.fp_vm + 3 * WORD) as usize;
                let addr = saved_esp - (idx + 1) * WORD;
                self.push_eval(self.read_word(addr));
            }
            0x49 => {
                let idx = self.fetch_u8() as usize;
                let val = self.pop_eval();
                let saved_esp = self.read_word(self.fp_vm + 3 * WORD) as usize;
                let addr = saved_esp - (idx + 1) * WORD;
                self.write_word(addr, val);
            }
            0x4A => {
                let depth = self.fetch_u8() as usize;
                let off = self.fetch_u8() as usize;
                let frame = self.follow_static_links(depth);
                let addr = frame + 4 * WORD + off * WORD;
                self.push_eval(self.read_word(addr));
            }
            0x4B => {
                let depth = self.fetch_u8() as usize;
                let off = self.fetch_u8() as usize;
                let val = self.pop_eval();
                let frame = self.follow_static_links(depth);
                let addr = frame + 4 * WORD + off * WORD;
                self.write_word(addr, val);
            }

            // Indirect memory access — operands are absolute VM
            // addresses (data refs were baked at link time with
            // load_addr = 0x010000).
            0x50 => {
                let addr = self.pop_eval();
                if addr == 0 {
                    self.trap(6);
                    return;
                }
                self.push_eval(self.read_word(addr as usize));
            }
            0x51 => {
                let addr = self.pop_eval();
                let val = self.pop_eval();
                if addr == 0 {
                    self.trap(6);
                    return;
                }
                self.write_word(addr as usize, val);
            }
            0x52 => {
                let addr = self.pop_eval();
                if addr == 0 {
                    self.trap(6);
                    return;
                }
                self.push_eval(self.read_byte(addr as usize));
            }
            0x53 => {
                let addr = self.pop_eval();
                let val = self.pop_eval();
                if addr == 0 {
                    self.trap(6);
                    return;
                }
                self.write_byte(addr as usize, val);
            }

            // SYS
            0x60 => {
                let id = self.fetch_u8();
                self.sys_call(id);
            }

            // Block memory ops
            0x70 => {
                let len = self.pop_eval() as usize;
                let dst = self.pop_eval() as usize;
                let src = self.pop_eval() as usize;
                if len > 0 {
                    if src < dst {
                        for i in (0..len).rev() {
                            let b = self.read_byte(src + i);
                            self.write_byte(dst + i, b);
                        }
                    } else {
                        for i in 0..len {
                            let b = self.read_byte(src + i);
                            self.write_byte(dst + i, b);
                        }
                    }
                }
            }
            0x71 => {
                let len = self.pop_eval() as usize;
                let val = self.pop_eval();
                let dst = self.pop_eval() as usize;
                for i in 0..len {
                    self.write_byte(dst + i, val);
                }
            }
            0x72 => {
                let len = self.pop_eval() as usize;
                let b = self.pop_eval() as usize;
                let a = self.pop_eval() as usize;
                let mut result: i32 = 0;
                for i in 0..len {
                    let ba = self.read_byte(a + i) & 0xFF;
                    let bb = self.read_byte(b + i) & 0xFF;
                    if ba != bb {
                        result = if ba < bb { -1 } else { 1 };
                        break;
                    }
                }
                self.push_eval(result);
            }

            // 0x73 — JMPIND: pop absolute PC (no code-rel adjustment;
            // callers push absolute addresses e.g. from ADDRG).
            0x73 => {
                let addr = self.pop_eval() as usize;
                self.pc = addr;
            }

            // 0x74 — XCALL slot16: cross-unit call via the current
            // unit's IRT. The baked IRT entries are code-section-
            // relative (p24-load stores `target_unit.code_base +
            // export.offset` there); pvm.s dispatches via code_ptr +
            // target, so we add code_abs_base to reach the absolute PC.
            0x74 => {
                let slot = (self.fetch_u8() as u16) | ((self.fetch_u8() as u16) << 8);
                let unit = self.current_unit();
                let target_rel =
                    match self.irt_abs_addrs.get(unit).and_then(|t| t.get(slot as usize)) {
                        Some(a) => *a as usize,
                        None => {
                            self.trap(4);
                            return;
                        }
                    };
                let target = self.code_abs_base + target_rel;
                self.write_word(self.csp, self.pc as i32);
                self.write_word(self.csp + WORD, self.fp_vm as i32);
                self.write_word(self.csp + 2 * WORD, self.fp_vm as i32);
                self.write_word(self.csp + 3 * WORD, self.esp as i32);
                self.csp += 4 * WORD;
                self.pc = target;
            }

            // 0x75 — XLOADG unit_id offset: load from another unit's
            // globals via the unit table.
            0x75 => {
                let uid = self.fetch_u8() as usize;
                let off = self.fetch_u8() as usize;
                match self.units.get(uid).copied() {
                    Some(u) => {
                        let addr = self.gp + (u.global_base as usize + off) * WORD;
                        self.push_eval(self.read_word(addr));
                    }
                    None => self.trap(4),
                }
            }

            // 0x76 — XSTOREG unit_id offset
            0x76 => {
                let uid = self.fetch_u8() as usize;
                let off = self.fetch_u8() as usize;
                let val = self.pop_eval();
                match self.units.get(uid).copied() {
                    Some(u) => {
                        let addr = self.gp + (u.global_base as usize + off) * WORD;
                        self.write_word(addr, val);
                    }
                    None => self.trap(4),
                }
            }

            _ => self.trap(4),
        }
    }

    fn sys_call(&mut self, id: u8) {
        match id {
            0 => self.status = 1,                       // HALT
            1 => {                                       // PUTC
                let ch = self.pop_eval() as u8;
                self.stdout_buf.push(ch);
            }
            2 => {                                       // GETC
                if self.stdin_pos < self.stdin_buf.len() {
                    let c = self.stdin_buf[self.stdin_pos];
                    self.stdin_pos += 1;
                    self.push_eval(c as i32);
                } else if self.interactive {
                    // Rewind past SYS (0x60) + id so the syscall
                    // replays once feed_input() provides more data.
                    self.pc -= 2;
                    self.awaiting_input = true;
                } else {
                    self.push_eval(-1);
                }
            }
            3 => {                                       // LED (no-op)
                self.pop_eval();
            }
            4 => {                                       // ALLOC (bump)
                let size = self.pop_eval() as usize;
                let ptr = self.hp;
                self.hp += size;
                if self.hp > self.mem.len() {
                    self.mem.resize(self.hp, 0);
                }
                self.push_eval(ptr as i32);
            }
            5 => {                                       // FREE (no-op)
                self.pop_eval();
            }
            6 => self.push_eval(0),                      // READ_SWITCH: always off in browser
            7 => {                                       // SET_IRT_BASE (no-op; handled per-call)
                self.pop_eval();
            }
            8 => {}                                      // DUMP_STATE (no-op in browser)
            _ => self.trap(4),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
pub(crate) mod console_stub {
    // Non-wasm builds don't link web_sys::console; provide a stub so
    // native `cargo test` builds still compile.
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_to_done(session: &mut Session, max_ticks: u64) {
        for _ in 0..max_ticks {
            let r = session.tick();
            if r.done || session.is_awaiting_input() {
                break;
            }
        }
    }

    #[test]
    fn print_int_42_runs_to_completion_with_42_in_output() {
        let mut s = Session::new("print_int 42");
        run_to_done(&mut s, 500);
        assert!(s.is_done(), "session did not finish: instrs={}", s.instruction_count);
        assert!(
            s.is_halted(),
            "session trapped: {} (out={:?})",
            s.stop_reason(),
            s.output()
        );
        assert!(
            s.clean_output().contains("42"),
            "expected '42' in cleaned output, got: {:?}",
            s.output()
        );
    }
}
