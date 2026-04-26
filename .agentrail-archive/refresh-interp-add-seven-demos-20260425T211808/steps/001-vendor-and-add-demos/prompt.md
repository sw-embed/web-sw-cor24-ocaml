# Refresh vendored interpreter and add 7 new demos

Pick up new OCaml language features that landed in the sibling CLI
repo after the last `assets/ocaml.p24m` vendor (top-level let,
variant payloads, string escapes, tuple arity), and surface 7
substantive new demos in the web UI dropdown.

## Tasks

1. Run `scripts/vendor-artifacts.sh` to refresh `asm/pvm.s` and
   `assets/ocaml.p24m` from `../sw-cor24-pcode` and `../sw-cor24-ocaml`.
   This rebuilds the OCaml interpreter image via `just build` in the
   CLI repo and copies the resulting `build/ocaml.p24m` into
   `assets/ocaml.p24m`.

2. Extend the MAPPING table in `scripts/sync-demos.sh` with these
   7 entries (alphabetised by web demo name):

       eval_string_conversion -> string-conversion
       eval_string_eq         -> string-equality
       eval_string_escapes    -> string-escapes
       demo_tco_countdown     -> tco-countdown
       eval_toplevel_let      -> toplevel-let
       eval_tuple_arity       -> tuple-arity
       eval_user_variants     -> variants-with-payload

3. Run `./scripts/sync-demos.sh` to vendor the 7 new
   `examples/*.ml` files into the repo. The script copies each
   CLI test file to `examples/<dst>.ml`. None of the 7 are in
   COLLAPSE_NEWLINES or LOCAL_OVERRIDE.

4. Add 7 new entries to `DEMOS` in `src/demos.rs`. They must slot
   into the existing alphabetised list in the correct positions.
   Each entry needs: `name`, `source: include_str!(...)`,
   `interactive: false`, and a one-line `description`.

5. Run `cargo test --tests demos` (or `cargo test`) to verify the
   alphabetisation, uniqueness, and interactive-demos tests in
   `src/demos.rs` still pass.

6. Commit as a single feat(demos) commit covering:
     - assets/ocaml.p24m
     - asm/pvm.s (if changed)
     - scripts/sync-demos.sh
     - examples/string-conversion.ml
     - examples/string-equality.ml
     - examples/string-escapes.ml
     - examples/tco-countdown.ml
     - examples/toplevel-let.ml
     - examples/tuple-arity.ml
     - examples/variants-with-payload.ml
     - src/demos.rs

Stop after committing. The pages rebuild is the next step
(002-rebuild-pages) and belongs in the next session.

## Vendoring rationale

The web build is fully self-contained: `src/demos.rs` uses
`include_str!("../examples/*.ml")` and `assets/ocaml.p24m` is
loaded by the Yew app. Neither path touches `../sw-cor24-ocaml`
at compile time -- vendoring is the bright line that makes the
web build immune to upstream CLI churn.