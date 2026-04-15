# Populate demos table from CLI tests

Mirror `../web-sw-cor24-basic/src/demos.rs` — a static table of demos,
each loaded via `include_str!` from a local `examples/` directory —
for the OCaml interpreter.

## Prerequisites

Scaffold step complete. (Runner can be in-flight; demos.rs only
depends on the file layout, not the runner.)

## Work

1. Create `examples/` at the repo root and copy these files from
   `../sw-cor24-ocaml/tests/` verbatim:
   - `eval_fact.ml` -> `factorial.ml`
   - `eval_pairs.ml` -> `pairs.ml`
   - `eval_list_basics.ml` -> `lists.ml`
   - `eval_list_module.ml` -> `list-module.ml`
   - `demo_lists_pairs.ml` -> `lists-pairs-demo.ml`
   - `demo_led_blink.ml` -> `led-blink.ml`
   - `demo_led_toggle.ml` -> `led-toggle.ml`
   - `eval_fun.ml` -> `functions.ml`
   - `eval_multi_arg.ml` -> `multi-arg.ml`
   - `eval_seq.ml` -> `sequencing.ml`
   - `eval_print.ml` -> `print.ml`
   - `repl_session.ml` -> `repl-session.ml` (mark interactive)
   Add a short hand-written `hello.ml` that prints 42 -- this will be
   the default selected demo so first-time visitors see something
   immediately.
2. Write `scripts/sync-demos.sh` that re-copies the above files from
   the CLI repo. This is the cheap path to pulling in new demos the
   CLI adds later; the mapping table lives in the script.
3. Create `src/demos.rs`:
   - `pub struct Demo { name, source, interactive, description }` —
     add a short `description` field beyond the BASIC version so the
     UI can explain each demo.
   - `DEMOS: &[Demo]` with one entry per file in `examples/`.
   - Only `repl-session` has `interactive: true`.
   - `pub fn default_demo_index() -> usize` -> `hello`.
4. Wire `pub mod demos;` into `src/lib.rs` (the stub lib) so it
   compiles as part of the crate.

## Validation

- `cargo check` clean.
- `cargo test` (runner tests, if present) still passes.
- `scripts/sync-demos.sh` is idempotent (re-running changes nothing
  when CLI tests haven't changed).

## Out of scope

- Using the demos in the UI (next step).
- Auto-discovery via build script glob (explicit mapping is clearer
  and lets us cherry-pick).
