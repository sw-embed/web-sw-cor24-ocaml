# Refresh interpreter and add 7 new demos

## Goal

Pick up new OCaml language features that landed in the sibling CLI
repo after the last interpreter vendor (top-level let, variant
payloads, string escapes, tuple arity), and surface 7 substantive new
demos in the web UI dropdown. Vendoring means: copy CLI test files
into examples/ and refresh assets/ocaml.p24m -- the build never
reaches into ../sw-cor24-ocaml at compile time.

## Steps

### 001-vendor-and-add-demos

- Run scripts/vendor-artifacts.sh to refresh asm/pvm.s and
  assets/ocaml.p24m so the new language features are available in
  the browser-side interpreter.
- Extend MAPPING in scripts/sync-demos.sh with 7 entries:
    eval_string_conversion -> string-conversion
    eval_string_eq         -> string-equality
    eval_string_escapes    -> string-escapes
    demo_tco_countdown     -> tco-countdown
    eval_toplevel_let      -> toplevel-let
    eval_tuple_arity       -> tuple-arity
    eval_user_variants     -> variants-with-payload
- Run scripts/sync-demos.sh to vendor the 7 new examples/*.ml files.
- Add 7 alphabetised entries to DEMOS in src/demos.rs.
- Verify with `cargo test` (the alphabetisation and uniqueness tests
  in src/demos.rs guard the dropdown invariants).
- Commit as feat(demos) covering the vendored artifacts, sync mapping,
  examples, and src/demos.rs.

### 002-rebuild-pages

- Run scripts/build-pages.sh to regenerate pages/ release artifacts
  with the new interpreter and demos baked into the WASM bundle.
- Commit as chore(pages).
