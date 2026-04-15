# Add 6 new demos exercising upstream language features

The newly-vendored `assets/ocaml.p24m` (from saga 3 step 1) supports
strings, function-form `let`, named ADTs, and other features that we
don't currently exercise. Surface 6 high-signal upstream tests as
new web demos.

## Mapping additions

Append to `scripts/sync-demos.sh` MAPPING (in this order so they
land grouped by feature in the dropdown):

```
"eval_function_form_let:function-form-let"
"eval_strings:strings"
"eval_named_adts:named-adts"
"eval_options:options"
"demo_patterns:patterns"
"eval_let_destructure:let-destructure"
```

None of these need newline-collapsing -- each line is a complete
top-level REPL expression.

## src/demos.rs additions

Add six entries with `interactive: false` and short descriptions:

- `function-form-let` -- "Sugared `let f x y = body` form (curried
  function definitions)."
- `strings` -- "String literals, `^` concatenation,
  `print_endline`, `String.length`."
- `named-adts` -- "Sum types via `type T = C1 | C2 | ...` and
  `match` expressions."
- `options` -- "The built-in `option` type: `None` and `Some x`."
- `patterns` -- "Pattern matching across lists, tuples, options,
  and literals."
- `let-destructure` -- "Destructuring `let (a, b) = ...`,
  `let h :: t = ...`, and friends."

## Work

1. Edit `scripts/sync-demos.sh`. Run it; verify the 6 new files
   appear under `examples/`.
2. Edit `src/demos.rs` to add the entries. Order: keep `hello`
   first, group the new feature demos together, keep
   `repl-session` last as the only interactive one.
3. Run `cargo test --lib` (3 demos catalog tests should still
   pass; `names_are_unique` will catch typos).
4. Run `cargo test --test demos` -- the integration test will
   itemise per-demo output. Investigate any new failures (likely
   none -- these are upstream regression tests).
5. Update `README.md` demo table with the six additions.
6. Add one section per new demo to `docs/demos.md` with the source
   verbatim and expected output (capture from the integration
   test's `cleaned=` output).
7. Run `markdown-checker -f README.md && markdown-checker -f
   docs/demos.md && markdown-checker -f CHANGES.md`.
8. `trunk build` clean.

## Validation

- `examples/` has 19 `.ml` files (was 13: 12 + hello + 6 new).
- `cargo test`: all suites green; integration test prints all 18
  non-interactive demos halting cleanly with sensible cleaned output.
- markdown-checker passes on user-facing docs.
- `trunk build` clean.

## Out of scope

- Hardware panel (next step).
- The 14 other upstream tests (small `eval_*` basics + `lex_*`
  regression tests) -- they're either redundant with what we
  already ship or aren't user-facing demos.
