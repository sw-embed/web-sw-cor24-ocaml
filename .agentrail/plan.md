# Demo docs refresh + multi-file phase 1 + v0.2.0 release

## Goal

Three independent items the user picked off the next-steps list:

1. Bring `docs/demos.md` back in sync with the 34-entry catalog --
   the doc stops well before the 8 demos added in the recent two
   sagas (refresh-interp-add-seven-demos + add-modules-demo-and-multifile-plan).
2. Land Phase 1 of the multi-file demos plan from
   `docs/multiple-file-demos-plan.md`: data model
   (`AuxFile { name, source }`, `Demo.auxiliary_files`), runner
   concatenation with `let __module = "..."` injection, and one
   read-only multi-file demo to prove the path. No UI changes
   yet -- the editor still shows just the main file.
3. Cut a v0.2.0 release: collapse the giant `[Unreleased]`
   section to a `## v0.2.0 -- 2026-04-26` heading, push the
   v0.2.0 tag, and create a GitHub release.

The user is collaborating with the sibling sw-cor24-ocaml repo's
agent on language features in parallel, so this saga is
intentionally orthogonal to feature work. Do not attempt to add
new demos from CLI tests in this saga -- those are CLI-side
deliverables.

## Steps

### 001-document-recent-demos

Add per-demo entries in `docs/demos.md` for the 8 demos missing
coverage: `string-conversion`, `string-equality`, `string-escapes`,
`tco-countdown`, `toplevel-let`, `tuple-arity`,
`variants-with-payload`, `modules`. Match the existing entry
format (## name + paragraph + ```ocaml source + Expected output).
Source comes from `examples/<demo>.ml`; expected output should
match what the live UI prints. Commit as `docs(demos)`.

### 002-multifile-phase-1

Implement Phase 1 from `docs/multiple-file-demos-plan.md`:

- Add `AuxFile { name: &'static str, source: &'static str }`
  and extend `Demo` with `auxiliary_files: &'static [AuxFile]`.
  Default to `&[]` for the existing 34 demos.
- Vendor a first multi-file demo, e.g. `modules-multifile`, with
  `examples/modules-multifile/{math.ml, main.ml}`. Source comes
  from `../sw-cor24-ocaml/tests/{math.ml, main.ml}` if those
  files still exist; otherwise hand-write a minimal pair.
- Extend `scripts/sync-demos.sh` with a tree-mode entry that
  copies the dir contents (or hand-edit and add to
  `LOCAL_OVERRIDE` if the CLI shape doesn't match what we want).
- Update `src/runner.rs` to pre-concatenate when a demo has
  `auxiliary_files.len() > 0`: for each aux file in order, emit
  `let __module = "<Capitalized stem>"` then the file's source;
  finally append the main `source`. Capitalisation matches
  `run-ocaml.sh`: first character upper-cased, rest left alone,
  underscores preserved.
- Extend tests: `every_non_interactive_demo_halts_cleanly` must
  cover the multi-file path. Add a `(demo_name, aux_file_name)`
  uniqueness invariant.
- Rebuild pages and push so the live site exposes the new demo.

Commit as `feat(demos)` for the data model + first multi-file
demo, separate `chore(pages)` for the release artifact rebuild.

### 003-release-v0.2.0

- Edit `CHANGES.md`: replace the `## [Unreleased]` heading with
  `## v0.2.0 -- 2026-04-26`. Add a new empty `## [Unreleased]`
  above it.
- Commit as `release: v0.2.0`.
- `git tag -a v0.2.0 -m "v0.2.0 -- ..."`.
- `git push origin main && git push origin v0.2.0`.
- `gh release create v0.2.0 --notes "$(...)"` with the release
  notes pulled from the v0.2.0 section of CHANGES.md.
- Final `agentrail complete --done` plus trailing .agentrail commit.
