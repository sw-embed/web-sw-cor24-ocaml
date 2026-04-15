# Saga: REPL input hints

User reported: typing `let add x y = x + y` into the repl-session's
interactive input row produces `PARSE ERROR`. Confirmed by probing
the runner directly -- the Pascal OCaml interpreter's REPL requires
every `let` form to include `in <expr>`:

- `let add x y = x + y`           -> PARSE ERROR
- `let add x y = x + y in add 20 22` -> `42`
- `let x = 42`                     -> PARSE ERROR
- `let x = 42 in x`                -> `42`

This is an upstream interpreter constraint (top-level definitions
aren't supported; every expression must reduce to a value). Surface
it in the UI and docs so users don't get stuck.

## Single step

1. Update the `repl-session` demo description to mention the `in`
   requirement.
2. Expand the `repl-session` section in `docs/demos.md` with a
   "Typing your own input" block that shows the working / failing
   shapes side-by-side.
3. In `src/lib.rs`, when the input row is visible, render a small
   caption (class `input-hint`) below it: something like "tip: each
   input must be a complete expression, e.g. `let x = 42 in x`".
4. Add the `.input-hint` CSS rule in `src/ui.css` (small, muted,
   italic -- reuse `--text-dim`).
5. Remove the temporary `probe_repl_let_forms` test from
   `tests/demos.rs` (it served its diagnostic purpose and now
   clutters the suite).

## Validation

- `cargo test` passes.
- `trunk build` clean.
- markdown-checker passes on user-facing docs.

## Out of scope

- Changing the upstream OCaml interpreter to accept bare top-level
  `let`.
- Auto-rewriting user input (too magical; would confuse when it
  isn't applicable).
