# Changes

## [Unreleased]

### Annotate modules demo (`fix-modules-demo` saga, 2026-04-25)

- Hand-edited `examples/modules.ml` under `LOCAL_OVERRIDE` so
  `sync-demos.sh` no longer overwrites it. Source now carries
  inline `(* ... *)` comments framing each line plus a corrective
  `Math.add 1 2` before the deliberate-failure `add 1 2`. Output
  reads `5 / 18 / 3 / EVAL ERROR`, contextualising the trailing
  error as the namespace-isolation punchline rather than a bug.
  The interpreter has no try/catch -- the REPL just resets
  `eval_error` per line and continues, which is what makes the
  correct-then-wrong layout work.
- Documented `LOCAL_OVERRIDE` rationale in `scripts/sync-demos.sh`
  for all three overrides (`led-toggle`, `guess`, `modules`).
- Updated `src/demos.rs` description for `modules` to call out the
  educational frame.

### Modules demo + multi-file UI plan (`add-modules-demo-and-multifile-plan` saga, 2026-04-25)

- Added `modules` demo (catalog now 34). Vendored from
  `tests/eval_module_namespace_directive.ml`. Exercises the
  reserved `let __module = "..."` directive: defines `Math.add` /
  `Math.double` in module `Math`, switches to `Main`, dispatches
  by qualified name.
- New `docs/multiple-file-demos-plan.md` sketches a 3-phase upgrade
  for true multi-file demos modeled on `web-sw-cor24-plsw`'s
  main-plus-macros UI: phase 1 data model (`AuxFile` struct +
  runner concat with synthesized `__module` directives), phase 2
  `ModuleEditor` component, phase 3 add / remove / upload
  controls. Includes concrete starting pointers for the next
  saga.
- Pages release artifacts rebuilt and pushed.

### Refresh interpreter, add 7 demos (`refresh-interp-add-seven-demos` saga, 2026-04-25)

- Refreshed `assets/ocaml.p24m` from 45122 to 48231 bytes via
  `scripts/vendor-artifacts.sh` so recently-shipped CLI features
  (top-level `let`, variant payloads, string escapes, tuple arity)
  are available in the browser interpreter.
- 7 new demos vendored and surfaced in alphabetised order:
  `string-conversion`, `string-equality`, `string-escapes`,
  `tco-countdown`, `toplevel-let`, `tuple-arity`,
  `variants-with-payload`. Demo catalog grew from 26 to 33.
- Pages release artifacts rebuilt and pushed.

### Guess demo + resync (`add-guess-resync-adventure` saga, 2026-04-17)

- New `guess` interactive demo (number-guessing game, target 42,
  per-input feedback). Hand-edited under `LOCAL_OVERRIDE` because
  the CLI version's losing branch overflows the browser stack
  with no Ctrl-C to abort.
- PC-sample input readiness wired so `read_line ()` blocks
  cleanly until the user submits. Interactive demos
  (`echo-loop`, `repl-session`, `text-adventure`, `guess`) now
  all reach `awaiting_input` reliably.
- `text-adventure` and `echo-loop` resynced from the CLI;
  interpreter image refreshed alongside.
- Pages release artifacts rebuilt with current build metadata.

### Interactive I/O + UI polish (mid-April 2026)

Between the initial live-demo-readyup work and the guess saga:

- Interactive `text-adventure` demo (room navigation, item
  pickup, n/s/e/w movement, look/inventory/take/quit commands)
  -- with a follow-up fix to keep type declarations on separate
  REPL lines and contextualise the input hint.
- Interactive `echo-loop` demo (type any text, echoed back; type
  `quit` to exit). Originally added to debug `read_line`
  buffering.
- `assets/ocaml.p24m` updated with `getc` / `read_line` support
  plus matching I/O tests, fixing a class of input-buffering
  bugs.
- `fix(repl)`: first Enter after typing input now works
  immediately -- no more "press Enter twice on first input".
- `feat(ui)`: `?` help button opens a modal with User Guide /
  Language Reference tabs.
- `feat(repl)`: ↑ / ↓ recall prior submitted REPL inputs.
- Demo dropdown alphabetised (`refactor(demos)`).
- README gained `Links` and `Copyright` sections.
- `pages/` build artifacts now tracked in git so GitHub Pages can
  actually serve them.
- `assets/ocaml.p24m` revendored so the new demos run in the
  browser.

### Live-demo readiness (`live-demo-readyup` saga, 2026-04-15)

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
