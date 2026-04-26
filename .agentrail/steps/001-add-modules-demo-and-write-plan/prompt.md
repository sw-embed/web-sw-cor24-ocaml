# Add `modules` demo + author docs/multiple-file-demos-plan.md

## Tasks

1. Extend MAPPING in `scripts/sync-demos.sh` with:
       eval_module_namespace_directive -> modules

2. Run `./scripts/sync-demos.sh` to vendor `examples/modules.ml`.

3. Add an alphabetised entry to `DEMOS` in `src/demos.rs` between
   `list-module` and `multi-arg`. The demo is non-interactive.
   Description should call out the `let __module = "..."` directive
   and qualified dispatch (`Math.add 2 3`).

4. Author `docs/multiple-file-demos-plan.md` covering:

   - Where we are today: single-file model -- `Demo` has only
     `source: &'static str`, baked in via `include_str!`. The
     `modules` demo proves namespaces work in one buffer using the
     reserved `let __module = "..."` directive.

   - What the CLI exposes: multi-file driver in
     `../sw-cor24-ocaml/scripts/run-ocaml.sh`. Files are passed in
     dependency order; the driver injects `let __module = "Math"`
     before each file's contents. Each filename's stem becomes the
     module (file `math.ml` -> module `Math`).

   - Reference UI: `../web-sw-cor24-plsw`. Demo struct carries
     `macros: &'static [DemoMacro]` (zero-to-many .msw files in
     addition to the main .plsw). UI uses a wizard with one
     collapsible notebook cell per macro file plus
     add/remove/upload controls. See
     `src/components/macro_editor.rs` and
     `src/components/wizard.rs`.

   - Proposed OCaml mapping:
       * Add `AuxFile { name: &'static str, source: &'static str }`
         and extend `Demo` with
         `auxiliary_files: &'static [AuxFile]`.
       * Default to `&[]` so existing 33 demos compile unchanged.
       * New module-style demos vendor a tree of files
         (e.g. `examples/modules-multifile/{math.ml,main.ml}`).
       * Runner pre-concatenates: for each aux file in order, emit
         `let __module = "<Stem>"` then the file's source; finally
         emit the main file. Feed the joined source to the REPL
         line by line as today.
       * UI: introduce a ModuleEditor component patterned after
         plsw's MacroEditor -- one collapsible cell per aux file,
         filename header, syntax highlighting reused from the main
         editor.

   - Phasing: phase 1 (data model + runner concat, no UI changes)
     unblocks shipping multi-file demos as read-only. Phase 2
     adds the editor cells and add/remove. Phase 3 adds upload.

   Keep the doc focused on what a future saga needs to know to
   start work -- not exhaustive design. Cite specific plsw files
   and the CLI's run-ocaml.sh injection mechanism so the future
   agent can read primary sources.

5. Run `cargo test` (full suite). The integration test
   `every_non_interactive_demo_halts_cleanly` will execute the
   modules demo end-to-end against the vendored interpreter.

6. Commit as a single feat(demos) commit covering:
     - scripts/sync-demos.sh
     - examples/modules.ml
     - src/demos.rs
     - docs/multiple-file-demos-plan.md

Stop after committing. The pages rebuild is the next step
(002-rebuild-pages) and belongs in the next session.