# Redo runner: use cor24-emulator running real pvm.s

Supersedes step 003's pure-Rust p-code interpreter. The current
`src/runner.rs` (~1000 lines) reimplements `pvm.s` in Rust and
replicates `p24-load` multi-unit loading. This step replaces it with
the pattern used by `../web-sw-cor24-pascal` and the nine other
`web-sw-cor24-*` projects: run the real `pvm.s` on the real
`cor24-emulator`, load `ocaml.p24m` as data, and drive execution via
`emu.run_batch()`.

## Why redo

- `../web-sw-cor24-pascal` is the direct precedent: both Pascal and
  OCaml compile to p-code that runs on `pvm.s`. Everything Pascal
  does at load time applies here, only simpler because `.p24m`
  addresses are already baked absolute.
- Removes ~800 lines of duplicated VM logic that will drift from
  `pvm.s` over time.
- Fixes multi-unit XCall / IRT semantics automatically (the real
  `pvm.s` handles them; no more subtle address-space bugs).
- `cargo test runner::tests::print_int_42_runs_to_completion_with_42_in_output`
  must still pass.

## Prerequisites

Step 003 is **completed in the saga record** (kept as history of the
Pattern A attempt). The code it produced will be replaced in this
step; the test it wrote should pass against the new runner too.

## Work

### 1. Dependencies

Update `Cargo.toml`:

```toml
[dependencies]
cor24-emulator = { path = "../sw-cor24-emulator", default-features = false }
# pa24r can stay if useful for build.rs; remove if no build-time need
```

`pa24r` is no longer needed at runtime (we do not interpret p-code in
Rust). Drop it from `[dependencies]` unless `build.rs` uses it.

Add `[build-dependencies] cor24-emulator = { path = "…", default-features = false }`
so `build.rs` can call `cor24_emulator::Assembler`.

### 2. Vendor `asm/pvm.s`

Copy `../sw-cor24-pcode/vm/pvm.s` to `asm/pvm.s` in this repo. Track
it in git. Add an entry to `scripts/vendor-artifacts.sh` that refreshes
`asm/pvm.s` alongside the other artifacts so upstream changes are
easy to pick up.

(Pascal follows this convention: `web-sw-cor24-pascal/asm/pvm.s` is
the canonical on-disk copy.)

### 3. `build.rs` changes

Mirror Pascal's `build.rs`:

- Read `asm/pvm.s`.
- `cor24_emulator::Assembler::new().assemble(&src)`.
- On errors: `panic!("pvm.s assembly failed")` with each error printed.
- Write `pvm.bin` to `OUT_DIR/pvm.bin`.
- Capture and emit label addresses to `OUT_DIR/pvm_labels.rs`:
  - `vm_state`, `vm_loop`, `code_seg` (or `code_buf` fallback),
    `eval_stack`, `call_stack`.
- Keep the existing `BUILD_SHA` / `BUILD_HOST` / `BUILD_TIMESTAMP`
  stamps.
- Keep `cargo:rerun-if-changed=asm/pvm.s` and
  `cargo:rerun-if-changed=assets/ocaml.p24m`.
- `assets/pvm.bin` is no longer needed (we assemble from `asm/pvm.s`
  freshly each build) — delete the vendored `assets/pvm.bin` in this
  step and update `scripts/vendor-artifacts.sh` to stop copying it.
  Keep `assets/ocaml.p24m` and `assets/code_ptr_addr.txt` —
  `code_ptr_addr.txt` is informational, not needed at runtime
  (pascal resolves addresses via label capture instead).

### 4. `src/config.rs` changes

```rust
pub const PVM_BINARY: &[u8]     = include_bytes!(concat!(env!("OUT_DIR"), "/pvm.bin"));
pub const OCAML_P24M: &[u8]     = include_bytes!(concat!(env!("OUT_DIR"), "/ocaml.p24m"));
include!(concat!(env!("OUT_DIR"), "/pvm_labels.rs"));
pub fn label_addr(name: &str) -> u32 { /* as in pascal */ }
```

### 5. Rewrite `src/runner.rs`

Replace entirely. New `Session`:

```rust
pub struct Session {
    emu: EmulatorCore,
    pub instructions: u64,
    pub done: bool,
    pub halted: bool,
    pub stop_reason: String,
    // UART input plumbing
    rx_queue: std::collections::VecDeque<u8>,
    interactive: bool,
    source_sent: bool,     // one-shot: we still push EOT after the source
    awaiting_input: bool,
}
```

Construction — mirror Pascal's `load_vm_binary` + `load_p24_image` +
`apply_pending_code_base` flow, specialized for `.p24m` at 0x010000:

1. `EmulatorCore::new()`, `set_uart_tx_busy_cycles(0)`.
2. Write `PVM_BINARY` at address 0.
3. Write `OCAML_P24M` at address 0x010000. No relocation pass needed
   (p24-load already baked absolute addresses for this load addr).
4. Write a `sys halt` (`0x60 0x00`) at `label_addr("code_seg")` so
   pvm's init returns cleanly.
5. `resume()`, `run_batch(10_000)` to complete pvm boot, then
   `clear_uart_output()` (discard the boot banner).
6. `reset()` (soft), `set_uart_tx_busy_cycles(0)`.
7. `set_pc(vm_loop)`, `set_reg(3, vm_state)` (fp).
8. Patch `vm_state`:
   - byte offset 18..21 = `code_base` = `0x010000`.
   - byte offset 0..3   = `pc` = `0` (pvm's internal p-code PC,
     code-section-relative — pvm will add `code_base` on every
     fetch).
   - byte offset 21..24 = `status` = `0`.
9. Queue the source bytes followed by `0x04` (EOT) into `rx_queue`
   for UART delivery (one per tick).

Actually — the p-code entry point isn't 0; it's in the `.p24m`
header. Read `entry_point` from the header (little-endian u24 at
offset 5) and patch `vm_state.pc` to that value.

`tick(batch)`: drain at most one `rx_queue` byte via
`emu.send_uart_byte`, then `emu.run_batch(batch)`. Handle every
`StopReason` variant (see `snobol4` runner). On Halted set
`halted=true, done=true`.

`output()`: `emu.get_uart_output()` — strip any leading banner bytes
that slipped past boot.

`feed_input(line)`: append each byte to `rx_queue`; if `line` didn't
already end with `\n` push one. Clear `awaiting_input`.

`is_awaiting_input()`: for now, always `false`. Detecting that pvm
is blocked on GETC from the outside is non-trivial; a future step
can plumb it through (snobol4 dodges this by treating "no input
file" as interactive without detecting stalls). Leave the method so
the UI can still query.

### 6. Delete obsolete code

- Remove `fn parse_p24m`, `struct P24mLayout`, `struct UnitEntry`,
  the inline opcode `execute` match, XCall/XLoadg/XStoreg handlers
  — all gone.
- Keep the `Session` public API shape compatible enough that the
  upcoming Yew UI step (005) can consume it without rewrite: at
  minimum `new`, `new_interactive`, `tick`, `is_done`, `is_halted`,
  `stop_reason`, `instructions`, `output`, `clean_output` (if used),
  `feed_input`, `is_awaiting_input`.

### 7. Test

Keep the existing `print_int_42_runs_to_completion_with_42_in_output`
test. It should pass against the new runner. Update instruction
budgets if needed (the real pvm + emulator may need more cycles than
the direct interpreter did).

## Validation

- `cargo check` clean.
- `cargo test` passes — specifically
  `runner::tests::print_int_42_runs_to_completion_with_42_in_output`.
- `trunk build` succeeds (WASM builds link `cor24-emulator`).
- Line count of `src/runner.rs` is substantially lower than current
  (~150-300 lines target).

## Out of scope

- Detecting pvm-blocked-on-GETC (future step or leave TODO).
- Yew UI changes (step 005's work).
- `pa24r` dep removal cleanup — do minimally, don't over-clean.
