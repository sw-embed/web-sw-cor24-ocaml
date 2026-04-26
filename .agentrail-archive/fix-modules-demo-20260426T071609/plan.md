# Annotate modules demo + corrective line

## Goal

Make the `modules` demo's intentional namespace-isolation failure
read as an educational climax rather than a bug. The CLI test
source (`tests/eval_module_namespace_directive.ml`) ends on a bare
`add 1 2 -> EVAL ERROR` with no context. Hand-edit the web demo
(via LOCAL_OVERRIDE) to add inline `(* ... *)` comments framing
each line plus a corrective qualified call before the deliberate
failure.

## Steps

### 001-annotate-modules-demo

- Add `modules` to LOCAL_OVERRIDE in `scripts/sync-demos.sh` so
  the script no longer overwrites the hand-edited `examples/modules.ml`.
- Rewrite `examples/modules.ml` with inline trailing
  `(* ... *)` comments, ending on the deliberate-failure pair:
      Math.add 1 2  (* the right way: qualify into Math --> 3 *)
      add 1 2       (* the wrong way: unqualified, EVAL ERROR expected *)
- Update the description in `src/demos.rs` for `modules` to
  call out that the trailing EVAL ERROR is the punchline
  (no language-level try/catch; the REPL just resets per line).
- Run `cargo test` to confirm the cleanly-halts test still passes
  with the augmented source.
- Commit as `fix(demos)`.

### 002-rebuild-pages-and-push

- Run `scripts/build-pages.sh`.
- Commit as chore(pages).
- agentrail complete --done.
- Trailing .agentrail/ commit.
- Push origin main so GitHub Pages picks it up.
