# Multi-file demos: plan

How to upgrade the web UI so a single demo can carry more than one
`.ml` source file, mirroring the multi-file module workflow the CLI
already supports. This document is a starting line for a future saga,
not a full design.

## Where we are today

The web build is single-file per demo. `src/demos.rs` defines:

```rust
pub struct Demo {
    pub name: &'static str,
    pub source: &'static str,         // one .ml file, baked in via include_str!
    pub interactive: bool,
    pub description: &'static str,
}
```

`src/runner.rs` feeds `source` to the OCaml interpreter line by line.
The `modules` demo proves namespaces work in this single-file model
by using the reserved `let __module = "..."` directive directly:

```ocaml
let __module = "Math"
let add x y = x + y
let __module = "Main"
Math.add 2 3
```

That's the same directive the CLI's multi-file driver synthesises.

## What the CLI exposes

`../sw-cor24-ocaml/scripts/run-ocaml.sh` accepts multiple `.ml`
inputs in dependency order. The driver injects, before each file's
contents:

```
let __module = "<Capitalized stem>"
```

So `math.ml` becomes module `Math`, `game_state.ml` becomes module
`Game_state`. The interpreter treats `__module` as a reserved compile-
unit marker. Cross-file unqualified lookup is rejected; every
reference into another file must be `Module.name`.

See `../sw-cor24-ocaml/docs/module-system.md` for the full spec.

## Reference UI: web-sw-cor24-plsw

The plsw web UI already solves the analogous problem (one main
`.plsw` plus zero-to-many `.msw` macro files per demo). Worth
imitating verbatim:

- `src/demos.rs` — `Demo` carries `macros: &'static [DemoMacro]`
  alongside the main `source`. `DemoMacro` is `{ name, source }`.
- `src/components/macro_editor.rs` — one collapsible notebook cell
  per macro file: filename header, syntax-highlighted textarea, and
  add / remove / rename / upload controls. Mirrors the main source
  editor's overlay highlighting technique.
- `src/components/wizard.rs` — wizard step `Macros` sits between
  `Source` and `Preprocess`, so multi-file editing is a first-class
  pipeline stage rather than bolted onto the main editor.

## Proposed OCaml mapping

### Phase 1: data model + runner concatenation (no UI changes)

1. Add to `src/demos.rs`:

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

   Default `auxiliary_files: &[]` for the existing 34 demos so they
   compile unchanged.

2. New multi-file demos vendor a directory tree:
   `examples/<demo-name>/main.ml` + auxiliary `<module>.ml` files.
   Extend `scripts/sync-demos.sh` with a per-demo dir-tree mode
   that copies a subdirectory from `../sw-cor24-ocaml/tests/` (or
   from a new `../sw-cor24-ocaml/demos/modules/` location) and
   surfaces each non-`main.ml` file as an `AuxFile` entry.

3. `src/runner.rs` pre-concatenates before feeding the REPL: for
   each `AuxFile` in order, emit `let __module = "<Stem>"` then the
   file's contents; finally append the main `source`. Capitalisation
   matches the CLI's rule (first character upper-cased, rest left
   alone, underscores preserved).

   This matches what `run-ocaml.sh` does, so demos behave identically
   in CLI and browser.

4. Tests: extend `tests/demos.rs::every_non_interactive_demo_halts_cleanly`
   to cover the multi-file path. Add a new `names_are_unique`-style
   invariant for `(demo_name, aux_file_name)` uniqueness.

This phase ships multi-file demos as **read-only** content. Users
see one demo in the dropdown; the editor still shows just the main
file. The auxiliary module sources are bundled but not exposed for
editing.

### Phase 2: ModuleEditor component (read + edit aux files) — SHIPPED

Patterned after plsw's `MacroEditor`, trimmed to the read+edit slice.

- `src/components/module_editor.rs` ships `ModuleEditor` (function
  component) plus `ModuleFile { name, source, collapsed }`. Props:
  `files`, `on_change(idx, source)`, `on_toggle_collapse(idx)`,
  `disabled`. No add/remove/upload (those are Phase 3).
- Renders below the main source editor inside the existing
  `panel-src` section, only when `aux_edits.len() > 0`. CSS lives
  in `src/ui.css` under `/* Auxiliary module files */`.
- State: `App.aux_edits: Vec<ModuleFile>` is hydrated from
  `DEMOS[selected].auxiliary_files` on `load_demo` and on
  `Component::create`. Edits flow through `Msg::AuxFileChanged(idx,
  source)` and `Msg::AuxFileToggleCollapse(idx)`. No persistence
  across reloads.
- Run path: `start_run()` builds the source via
  `demos::concat_with_aux(&aux_pairs, &self.source)` where
  `aux_pairs` borrows from the live `aux_edits`, NOT from the
  demo's baked-in `auxiliary_files`. So edits to either pane are
  honoured.
- Test: `tests/demos.rs::concat_with_aux_edited_runs_through_session`
  feeds Session an edited math.ml (where `Math.add x y = x * y`) and
  asserts the output reflects the edit, locking in the
  edit-honouring contract.

### Phase 3: add / remove / upload aux files

Match plsw's MacroEditor controls 1:1: `+` to add a blank file with
a placeholder name, `×` to remove, click-to-rename on the filename,
and a `📂` upload button that calls `gloo::file::callbacks::read_as_text`.

Users can build a fresh multi-file program from scratch in the
browser. This unlocks the same workflow as `run-ocaml.sh a.ml b.ml`
without leaving the page.

## Out of scope for the first pass

- Persistence across reloads (localStorage) — defer to a later
  pass once the in-session model proves out.
- Drag-and-drop reordering of aux files — not in plsw either.
- Visual indicator of inter-module references (`Math.add` link in
  `main.ml` jumping to `math.ml`) — nice-to-have; revisit if users
  ask.
- Compile-error mapping back to a specific aux file's line — needs
  interpreter-side line offset tracking; treat as a follow-up.

## Concrete next-saga starting line

Phases 1 and 2 are shipped (v0.2.0 + post-release). Phase 3
remains:

1. Re-read `../web-sw-cor24-plsw/src/components/macro_editor.rs`
   for the add/remove/upload pattern (the trimmed Phase 2 version
   is in `src/components/module_editor.rs`).
2. Wire `Msg::AuxFileAdd`, `Msg::AuxFileRemove(idx)`,
   `Msg::AuxFileRename(idx, String)`,
   `Msg::AuxFileUpload(name, source)` -- the matching
   `MacroEditor` props are already named symmetrically.
3. Decide on persistence story (probably localStorage keyed by
   demo name) and whether reordering aux files is in scope.
