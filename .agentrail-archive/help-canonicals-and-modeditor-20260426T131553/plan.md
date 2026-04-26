# Help modal refresh + canonical examples + ModuleEditor

Three lower-leverage but user-visible items the user picked off the
"next possible steps" list. Each step bundles its feature change
with a pages rebuild + push so it ships independently.

## Steps

### 001-refresh-help-modal-language-ref

Update the in-UI help modal's Language Reference to cover features
shipped in v0.2.0 that are not yet documented in the modal:
top-level `let`, variant constructors with payloads, multi-arity
tuple matching, string escape sequences (`\n` `\t` `\\` `\"`), the
`modules` namespace mechanism (single-file via `__module`
directive AND multi-file via `auxiliary_files`), `string_of_int` /
`int_of_string`, `=`/`<>` on strings. Keep additions tight and
example-led; the modal is reference, not a tutorial.

Pages rebuild + push at the end.

### 002-canonical-minimal-examples

Surface the 16 `canonical_*.ml` one-liner snippets from
`../sw-cor24-ocaml/tests/` as a "minimal examples" tier in the
dropdown -- ideal for users who just want to see one-line
demonstrations of individual features. Use HTML `<optgroup>` (or
equivalent Yew construct) so the canonical snippets group
visually under a "Minimal examples" label rather than
interleaving with the existing 35-entry catalog.

Vendoring path: extend `scripts/sync-demos.sh` with a `MINIMAL_MAPPING`
(or similar) and add a new field to `Demo` (e.g.
`category: Category` enum with `Standard` / `Minimal` variants,
default `Standard`).

Pages rebuild + push at the end.

### 003-multifile-phase-2-module-editor

Phase 2 of `docs/multiple-file-demos-plan.md`: ship a
`ModuleEditor` component patterned 1:1 on
`../web-sw-cor24-plsw/src/components/macro_editor.rs`. One
collapsible notebook cell per `AuxFile` with a filename header
and a textarea (reuse the OCaml syntax highlighting from the main
editor). Wire it as a notebook cell that appears below the main
source editor when the active demo has `auxiliary_files.len() > 0`.
State: aux file edits live in component state per session (no
persistence, no add/remove yet -- those are Phase 3). Re-running
the demo concatenates current edits, not the baked-in source.

Pages rebuild + push at the end.
