# Multi-file demos: Phase 1 (data model + runner concat, no UI changes)

Implement Phase 1 of `docs/multiple-file-demos-plan.md`. The goal
is to land the data layer and runner support so a demo can carry
auxiliary `.ml` files alongside its main source, with all aux
content prepended (each preceded by a synthesized
`let __module = "<Stem>"`) before the main source goes through
the line-by-line REPL. No UI changes -- the editor still shows
just the main file. This unblocks shipping multi-file demos as
read-only content.

Read `docs/multiple-file-demos-plan.md` end-to-end before starting,
plus `../web-sw-cor24-plsw/src/demos.rs` and
`../web-sw-cor24-plsw/src/components/macro_editor.rs` for the
reference shape. Also re-read
`../sw-cor24-ocaml/scripts/run-ocaml.sh` for the canonical
`__module` injection mechanism.

## Tasks

1. Extend `src/demos.rs`:

   ```rust
   pub struct AuxFile {
       pub name: &'static str,    // e.g. "math.ml"
       pub source: &'static str,
   }

   pub struct Demo {
       // ... existing fields ...
       pub auxiliary_files: &'static [AuxFile],
   }
   ```

   Default `auxiliary_files: &[]` for all 34 existing demos.
   Add a unit test asserting that for every demo,
   `(demo.name, aux.name)` pairs are unique across the catalog.

2. Vendor a first multi-file demo. Suggested name:
   `modules-multifile`. Source files:
       examples/modules-multifile/main.ml
       examples/modules-multifile/math.ml
   If `../sw-cor24-ocaml/tests/{math.ml, main.ml}` still exist
   and are coherent, use them. Otherwise hand-write a minimal
   pair (e.g. `math.ml` defines `add` and `square`; `main.ml`
   calls `Math.square (Math.add 2 3)` and prints the result).

3. Update `scripts/sync-demos.sh`:
       - Either add a per-demo dir-tree mode (a new MAPPING
         entry shape that copies a directory) and a TREE_MAPPING
         table, or add `modules-multifile` to LOCAL_OVERRIDE if
         the CLI shape doesn't match. Choose whichever yields
         the smaller diff. Document the choice with a comment.

4. Add the demo to DEMOS in `src/demos.rs` with
   `auxiliary_files: &[AuxFile { name: "math.ml", source: include_str!("../examples/modules-multifile/math.ml") }]`
   and main `source: include_str!("../examples/modules-multifile/main.ml")`.
   Slot alphabetically between `modules` and `multi-arg`.

5. Update `src/runner.rs` to pre-concatenate when
   `auxiliary_files.len() > 0`. For each aux file in order,
   prepend `let __module = "<Capitalized stem>"\n` then the
   file's contents. Finally append the main source. Stem
   capitalisation: first char to_uppercase, rest unchanged
   (matches CLI's `run-ocaml.sh`). Strip the `.ml` suffix.
   The result is fed to the REPL line-by-line as today.

6. Tests:
       - `every_non_interactive_demo_halts_cleanly` must execute
         the new demo end-to-end against the vendored interpreter.
         If the test infrastructure currently feeds only
         `demo.source`, extend it to feed the concatenated source
         when aux files are present.
       - Add a unit test for the stem capitalisation helper if
         you factor one out.
       - Add the (demo_name, aux_file_name) uniqueness assertion.

7. Update `docs/demos.md` with an entry for the new demo,
   showing both files' source plus expected output.

8. Run `cargo test` (full suite); all 17+ tests must pass.

9. Commit as `feat(demos): multi-file phase 1 -- AuxFile data
   model + runner concat + first multi-file demo` covering:
       - src/demos.rs
       - src/runner.rs
       - examples/modules-multifile/{main,math}.ml
       - scripts/sync-demos.sh
       - docs/demos.md (new entry)
       - tests/demos.rs (extended assertions)

10. Run `scripts/build-pages.sh` and commit as `chore(pages):
    rebuild release artifacts with multi-file demo`. Push
    origin main so GitHub Pages picks it up.

Stop after the push. The v0.2.0 release tag is the next step
(003-release-v0_2_0).

## Out of scope (Phase 2 / 3)

- Editor UI for aux files (collapsible cells, syntax highlighting
  per file). Phase 2.
- Add / remove / upload aux files at runtime. Phase 3.
- Persistence across reloads.
- Multi-file source positions in error messages.