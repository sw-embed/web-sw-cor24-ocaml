# Changes

## [Unreleased]

Live-demo readiness work (`live-demo-readyup` saga, 2026-04-15):

- Demo catalog grown from 19 to 23: added `higher-order-lists`,
  `function-keyword`, `function-pattern-args`, and `when-guards` by
  extending the `scripts/sync-demos.sh` mapping and
  `src/demos.rs`/`docs/demos.md`. All new demos pass the
  `every_non_interactive_demo_halts_cleanly` integration test.
- `.github/workflows/pages.yml` added: push to `main` or manual
  dispatch uploads `./pages` and deploys via `actions/deploy-pages@v4`,
  matching the sibling `web-sw-cor24-basic` pattern. With
  `pages/.nojekyll` already in place, this is the final wire needed
  for the public live demo at
  <https://sw-embed.github.io/web-sw-cor24-ocaml/>.
- `README.md` front matter restructured to match the sibling style
  (title -> blurb -> Live demo link -> inline screenshot ->
  rest of the content). Added
  `images/screenshot-demo.png` (a `lists-pairs-demo` run) with
  `?ts=<ms>` cache-busting. Dropped the TODO Screenshot section.
- Cleanup: `cargo fmt` + removed two unnecessary `as u64` casts in
  `src/runner.rs`.

## v0.1.0 -- 2026-04-15

Initial release. Browser live demo for the
[sw-cor24-ocaml](https://github.com/sw-embed/sw-cor24-ocaml)
integer-subset OCaml interpreter, delivered via the
`web-live-demo-v1` saga.

### Architecture

- Yew + Trunk SPA, port 9735 reserved for this project.
- Runner delegates to `cor24-emulator` running the canonical `pvm.s`
  (vendored at `asm/pvm.s`, assembled at build time). The OCaml
  interpreter image (`assets/ocaml.p24m`) is loaded at `0x010000`;
  pvm's `init_p24m` path detects the `P24M` magic and walks the
  multi-unit header itself. No hand-ported opcode table; multi-unit
  XCall / IRT are handled by the real VM.
- 270-line `Session` over `EmulatorCore` with UART source injection
  + `0x04` EOT for one-shot runs and an open RX channel for the
  interactive `repl-session` demo.

### Features

- 13 OCaml demos in the dropdown (12 mirrored from
  `../sw-cor24-ocaml/tests/`, 1 hand-written `hello`).
- Source editor, output panel, status bar with budget escalator,
  REPL input row, octocat GitHub corner, footer with build stamp.
- Keyboard shortcuts: Cmd/Ctrl-Enter to Run, Esc to Stop, Enter to
  submit input.
- Violet accent color (`#8945ee`) matching the favicon.

### Tooling

- `scripts/vendor-artifacts.sh` -- refresh `asm/pvm.s` + build and
  copy `assets/ocaml.p24m` from the sibling CLI repo.
- `scripts/sync-demos.sh` -- refresh `examples/*.ml` from the CLI
  repo. Single mapping table is the source of truth for which CLI
  tests surface as web demos.
- `scripts/serve.sh` -- dev server on port 9735 with an exclusive
  `dist/` mkdir lock to prevent build/serve races.
- `scripts/build-pages.sh` -- release build into `pages/` for GitHub
  Pages, same lock.

### Tests

4 unit tests:

- `runner::tests::print_int_42_runs_to_completion_with_42_in_output`
  -- end-to-end across pvm boot + OCaml interp.
- 3 demo catalog invariants (default-is-hello, names-unique,
  only-repl-interactive).

### Saga history

The runner went through one false start: an initial pure-Rust p-code
interpreter (Pattern A, like `web-sw-cor24-basic`) was committed and
then replaced with the Pattern B (cor24-emulator + real `pvm.s`)
approach used by `pascal`, `snobol4`, and the rest of the family.
~800 lines of duplicated VM logic dropped; the
`print_int 42 -> "42"` test held throughout. Saga record preserves
both attempts; see `.agentrail/steps/003-port-pvm-runner/` (Pattern A)
and `.agentrail/steps/009-redo-port-pvm-runner-with-emulator/`
(Pattern B redo) for context.
