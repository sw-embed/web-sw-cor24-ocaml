# Add modules demo + document multi-file UI plan

## Goal

Surface OCaml's new module-namespace feature in the web UI as a
single-file demo (`modules`), and capture in
`docs/multiple-file-demos-plan.md` the design for upgrading the UI to
the multi-file workflow modeled on `web-sw-cor24-plsw` (one main +
zero-to-many auxiliary files per demo).

## Steps

### 001-add-modules-demo-and-write-plan

- Extend MAPPING in `scripts/sync-demos.sh` with one entry:
    eval_module_namespace_directive -> modules
- Run `./scripts/sync-demos.sh` to vendor `examples/modules.ml`.
- Add an alphabetised entry to `DEMOS` in `src/demos.rs`. Slots
  between `list-module` and `multi-arg`. Description should
  highlight the `let __module = "..."` directive and qualified
  dispatch.
- Write `docs/multiple-file-demos-plan.md` describing:
    - Current single-file model (Demo.source, include_str!).
    - The CLI's multi-file driver model (run-ocaml.sh injects
      `let __module = "..."` between files).
    - The plsw reference UI: Demo.macros: &[DemoMacro], one
      collapsible notebook cell per .msw file, add/remove/upload.
    - Concrete OCaml-side mapping: Demo gains
      `auxiliary_files: &'static [AuxFile]`; runner concatenates
      with synthesized `__module` directives; new ModuleEditor
      component mirroring plsw's MacroEditor.
    - Phasing / scope so a future saga has a clear starting line.
- Run `cargo test` to confirm the demo entry passes alphabetisation,
  uniqueness, and the cleanly-halts integration test.
- Commit as feat(demos) covering sync mapping, examples/modules.ml,
  src/demos.rs, and docs/multiple-file-demos-plan.md.

### 002-rebuild-pages

- Run `scripts/build-pages.sh` to regenerate pages/ with the
  modules demo in the bundle.
- Commit as chore(pages).
- After agentrail complete --done, do the trailing .agentrail/
  commit per project convention.
