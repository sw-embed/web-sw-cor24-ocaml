# Phase 2: ModuleEditor component for multi-file demos

Phase 2 of `docs/multiple-file-demos-plan.md`. Phase 1 (landed in
v0.2.0) baked aux file content in read-only; the editor pane
shows only the main source. This step ships a `ModuleEditor`
component that lets users see and edit aux files in the browser,
modeled 1:1 on `web-sw-cor24-plsw`'s `MacroEditor`.

Read first:
- `../web-sw-cor24-plsw/src/components/macro_editor.rs` end-to-
  end. This is the reference implementation.
- `../web-sw-cor24-plsw/src/components/source_editor.rs` to see
  how the main editor's syntax highlighting works (the
  collapsible cells reuse the same overlay technique).
- `docs/multiple-file-demos-plan.md` for the OCaml-side mapping
  and the explicit Phase 2 description.

## Tasks

1. Create `src/components/mod.rs` if it doesn't exist; create
   `src/components/module_editor.rs` mirroring plsw's
   `macro_editor.rs` structure but trimmed to Phase 2 scope:

   - `ModuleFile { name: String, source: String, collapsed: bool }`
     (state owned by the ModuleEditor, hydrated from the demo's
     `auxiliary_files` on demo selection).
   - `ModuleEditorProps { files: Vec<ModuleFile>, on_change:
     Callback<(usize, String)>, on_toggle_collapse:
     Callback<usize> }`. NO add/remove/upload/rename in Phase 2.
   - One collapsible notebook cell per file with the filename in
     the header.
   - Reuse the OCaml syntax highlighting from the main source
     editor (or copy / adapt as needed; OCaml keywords are
     different from PL/SW).

2. Wire the component into the App in `src/lib.rs`:

   - Add a field `aux_edits: Vec<ModuleFile>` to App state.
   - On demo selection, hydrate `aux_edits` from
     `DEMOS[selected].auxiliary_files`.
   - On Run, build the concatenated source using the EDITED aux
     content, not the baked-in `auxiliary_files`. The simplest
     path is to bypass `Demo::concat_main` and inline the same
     logic (or factor out a free function that takes
     `&[(name, source)]` and a main string).
   - Render the ModuleEditor below the main source editor when
     `auxiliary_files.len() > 0`. Hide it otherwise.

3. CSS: copy / adapt the relevant `.notebook-cell` / cell-header
   styles from plsw's `app.css` so the visual matches the rest
   of the OCaml UI.

4. Update `src/runner.rs` doc comments if any reference the old
   single-source assumption.

5. Tests:
   - `every_non_interactive_demo_halts_cleanly` already exercises
     `modules-multifile` via `demo.full_source()` (baked-in aux
     content). Add a focused test that simulates an aux edit:
     build the concatenated source from a synthetic edited aux
     and confirm Session reaches a halt with the expected output.
   - Optionally add a yew-test for ModuleEditor rendering, if
     the test harness supports it.

6. Update `docs/demos.md`'s `modules-multifile` entry to note
   that the aux file is now editable in the browser (Phase 2
   delivered).

7. Update `docs/multiple-file-demos-plan.md` to reflect Phase 2
   shipped: move the Phase 2 bullets from "Proposed" into a
   "Shipped in v0.x.x" section, and clarify what remains for
   Phase 3 (add/remove/upload/persistence).

8. `cargo test` and `cargo build --release` -- all should pass.

9. Commit as `feat(ui): multi-file Phase 2 -- ModuleEditor
   component`.

10. `scripts/build-pages.sh`, commit as `chore(pages)`, push.

11. `agentrail complete --done` -- this is the saga's final step.

12. Trailing .agentrail commit + push.

## Out of scope (Phase 3)

- Add new aux file (+/upload buttons).
- Rename aux files.
- Remove aux files.
- Persistence across reloads.
- Drag-to-reorder aux files.