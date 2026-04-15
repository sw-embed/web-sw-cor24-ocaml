# Port the P-code VM runner into WASM

Adapt `../web-sw-cor24-basic/src/runner.rs` (which is a WASM port of
`pv24t` from `sw-cor24-pcode/tracer`) to load the OCaml interpreter
image rather than the BASIC image.

## Prerequisites

- Scaffold step complete.
- Vendor step complete (`assets/ocaml.p24m`, `assets/pvm.bin`,
  `assets/code_ptr_addr.txt` exist; `build.rs` exposes `CODE_PTR` and
  includes the binaries).

## Work

Copy `../web-sw-cor24-basic/src/runner.rs` to `src/runner.rs` and adjust:

1. **Memory layout** — match the CLI's `scripts/run-ocaml.sh`:
   - `pvm.bin` loads at address 0.
   - `ocaml.p24m` loads at `0x010000`.
   - The PVM's `code_ptr` memory location (from `CODE_PTR` env) is
     patched to `0x010000` before execution.
2. **UART input** — the BASIC runner has no stdin; the OCaml
   interpreter reads its source from UART terminated by `\x04` (see
   `scripts/run-ocaml.sh`'s `-u "${ML_INPUT}"$'\x04'`). Extend
   `Session`:
   - `pub fn new(source: &str) -> Self` — seed UART input buffer with
     `source` + `\x04`. One-shot: when EOT is consumed, no more input.
   - `pub fn new_interactive(source: &str) -> Self` — same initial
     seed, but keep the input channel open; track an `awaiting_input`
     state when the program reads and the buffer is empty.
   - `pub fn feed_input(&mut self, line: &str)` — append line + `\n`
     (the REPL eats by expression) and clear `awaiting_input`.
   - `pub fn is_awaiting_input(&self) -> bool`.
3. **UART output** — the CLI strips `PVM OK` and empty lines in
   `run-ocaml.sh` via awk/sed. The runner captures raw UART bytes; let
   the UI worry about formatting (or add a small `clean_output` helper
   mirroring the awk pipeline). Keep the raw output available too.
4. **Instruction counter** and **stop reason** plumbing stay identical
   to the BASIC runner (the UI's budget / status display depends on
   them).

## Validation

- Add a `#[cfg(test)]` unit test that constructs a `Session` with the
  one-liner `print_int 42` and ticks until done, asserting that
  cleaned UART output contains `42`. (Run with `cargo test` — runner
  logic is pure Rust, no WASM required.)
- `cargo check` clean.

## Out of scope

- Yew UI wiring (next step).
- Any behavioral change beyond what the OCaml image needs.
