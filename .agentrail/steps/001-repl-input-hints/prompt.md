# REPL input hints + docs for the `in` requirement

See the plan for the full motivation. Do all five items here; the
whole fix is small enough for one commit.

1. `src/demos.rs` -- update the `repl-session` Demo description:
   "Multi-expression REPL session -- type more lines after the seed
   runs. Each input must be a complete expression: `let x = 42 in
   x`, not bare `let x = 42`."

2. `docs/demos.md` -- in the existing `repl-session` section, append
   a "Typing your own input" subsection showing:
   - Works: `42`, `1 + 1`, `let x = 42 in x`, `let f x = x * 2 in
     f 21`, `print_int 99`.
   - Fails (parse error): `let x = 42`, `let f x = x + 1`, any bare
     top-level `let`.
   - One-sentence explanation: the Pascal interpreter parses each
     line as a single expression that reduces to a value; top-level
     bindings without `in` aren't supported.

3. `src/lib.rs` -- when the input row is visible, add a small hint
   caption below the input/button, class `input-hint`. Content:
   "tip: each input must be a complete expression. `let` forms
   require an `in` clause, e.g. `let x = 42 in x`."

4. `src/ui.css` -- add the `.input-hint` rule after `.input-row`.
   Small (0.75em), muted (`var(--text-dim)`), italic, top-margin 4px.

5. `tests/demos.rs` -- delete the temporary `probe_repl_let_forms`
   test that confirmed the limitation (its job is done; it doesn't
   assert anything and now just noises up the suite).

## Validation

- `cargo test --lib` still passes (6 tests).
- `cargo test --test demos` still passes (2 tests, probe removed).
- `trunk build` clean.
- `markdown-checker` passes on README.md, docs/demos.md, CHANGES.md.

## Out of scope

- Changing anything else in repl-session's seed -- the seed already
  uses `let ... in ...` correctly; only user-typed input is at
  risk, which is what the hint addresses.
