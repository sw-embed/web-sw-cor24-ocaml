# web-sw-cor24-ocaml

Browser-based live demo for the
[sw-cor24-ocaml](https://github.com/sw-embed/sw-cor24-ocaml)
integer-subset OCaml interpreter. The interpreter is written in Pascal,
compiled to p-code, and runs on the canonical COR24 p-code VM
(`pvm.s`); this project assembles `pvm.s` with the COR24 emulator at
build time and runs it inside the browser via WebAssembly. No backend.

Same architecture as the rest of the `web-sw-cor24-*` family
(`pascal`, `snobol4`, `forth`, ...): one Yew SPA, one Trunk build,
one `cor24-emulator` running real COR24 native code under Rust.

## Quickstart

```bash
# 1. Refresh vendored sources (asm/pvm.s + assets/ocaml.p24m) from
#    sibling repos. Required first time, then any time the upstream
#    interpreter or VM changes.
./scripts/vendor-artifacts.sh

# 2. Run the dev server (port 9735, exclusive dist/ lock).
./scripts/serve.sh

# 3. Open http://localhost:9735/
```

Pick a demo from the dropdown and hit **Run** (or Cmd/Ctrl-Enter).
**Esc** stops a running session. **Reset** reloads the demo source.
**Clear** wipes the output panel.

## Demos

23 OCaml programs ship with the build (22 mirrored from
`../sw-cor24-ocaml/tests/` via `scripts/sync-demos.sh`, 1 hand-written):

| Demo | Description |
| --- | --- |
| `hello` | Smallest possible program: print the integer 42. |
| `factorial` | Recursive factorial via `let rec`; computes 5!. |
| `functions` | First-class functions and `let` bindings. |
| `multi-arg` | Multi-argument curried function via `fun x y -> ...`. |
| `pairs` | Tuple construction with `fst` / `snd` accessors. |
| `lists` | List literals, cons, head/tail, is_empty. |
| `list-module` | `List.length`, `List.rev`, qualified-name lookups. |
| `higher-order-lists` | `List.map`, `List.filter`, `List.fold_left`, `List.iter` with inline lambdas. |
| `lists-pairs-demo` | Sum, length, map, swap, countdown -- lists + pairs in one program. |
| `sequencing` | Semicolon-sequenced expressions evaluated left-to-right. |
| `print` | `print_int` and `putc` writing through the UART. |
| `led-blink` | Drive the COR24 LED via `led_on` / `led_off`. Browser stubs the LED. |
| `led-toggle` | Read the COR24 switch and reflect it on the LED. |
| `function-form-let` | Sugared `let f x y = body` form (curried function definitions). |
| `function-keyword` | `function` keyword: shorthand for `fun x -> match x with ...`. |
| `function-pattern-args` | Destructuring patterns directly in function arguments. |
| `strings` | String literals, `^` concatenation, `print_endline`, `String.length`. |
| `named-adts` | Sum types via `type T = C1 \| C2 \| ...` and `match` expressions. |
| `options` | The built-in `option` type: `None` and `Some x`. |
| `patterns` | Pattern matching across lists, tuples, options, and literals. |
| `when-guards` | `match ... when <guard>` clauses for conditional pattern arms. |
| `let-destructure` | Destructuring `let (a, b) = ...`, `let h :: t = ...`, and friends. |
| `repl-session` | Multi-expression REPL session -- type more lines after the seed runs. |

See [`docs/demos.md`](docs/demos.md) for annotated source and expected
output for each.

To add a new demo: drop the `.ml` file in
`../sw-cor24-ocaml/tests/`, add a mapping line to
`scripts/sync-demos.sh`, append an entry to `src/demos.rs`, and rerun
the sync script.

## Architecture

- `asm/pvm.s` -- vendored copy of the canonical p-code VM from
  `../sw-cor24-pcode/vm/pvm.s`.
- `build.rs` -- assembles `pvm.s` via `cor24_emulator::Assembler`,
  captures the `code_ptr` label address, and bakes
  `assets/ocaml.p24m` into `OUT_DIR`.
- `src/runner.rs` -- 270-line `Session` over `EmulatorCore`. Loads
  `pvm.bin` at address 0, places the OCaml interpreter image at
  `0x010000`, patches `code_ptr` so pvm's `init_p24m` path picks it up
  on boot, and ticks via `emu.run_batch()`. Source bytes flow into
  the UART RX queue terminated by EOT (`0x04`).
- `src/lib.rs` -- Yew `App`: demo dropdown, source editor, output
  pane, status bar with a budget escalator, REPL input row,
  octocat corner, footer with build stamp.
- `src/demos.rs` -- static catalog (`include_str!` from `examples/`).
- `examples/` -- demo `.ml` sources synced from the CLI repo.
- `scripts/serve.sh` -- dev server on port 9735 with an exclusive
  `dist/` lock.
- `scripts/build-pages.sh` -- release build into `pages/` for GitHub
  Pages, same lock.

## Build for GitHub Pages

```bash
./scripts/build-pages.sh
git add pages/
git commit -m "Release: deploy <date>"
git push
```

The `--public-url` is `/web-sw-cor24-ocaml/` so paths resolve under
`https://sw-embed.github.io/web-sw-cor24-ocaml/`.

## Tests

```bash
cargo test --lib
```

4 unit tests:

- `runner::tests::print_int_42_runs_to_completion_with_42_in_output`
  -- end-to-end: source bytes -> UART -> pvm boot -> OCaml interp ->
  `42` in cleaned output. Confirms the whole stack including
  multi-unit XCall / IRT through the real `pvm.s`.
- `demos::tests::default_is_hello`,
  `demos::tests::names_are_unique`,
  `demos::tests::only_repl_session_is_interactive` --
  catalog invariants.

## Screenshot

(TODO: capture and add `images/screenshot.png` after the first
public deploy.)

## Related projects

- [sw-cor24-ocaml](https://github.com/sw-embed/sw-cor24-ocaml) --
  the OCaml interpreter (Pascal -> p-code) that this demo runs.
- [sw-cor24-pcode](https://github.com/sw-embed/sw-cor24-pcode) --
  canonical p-code VM (`pvm.s`), assembler (`pa24r`), linker
  (`p24-load`), tracer (`pv24t`).
- [sw-cor24-emulator](https://github.com/sw-embed/sw-cor24-emulator)
  -- COR24 native CPU emulator. Hosts `pvm.s` here.
- [web-sw-cor24-pascal](https://github.com/sw-embed/web-sw-cor24-pascal),
  [web-sw-cor24-snobol4](https://github.com/sw-embed/web-sw-cor24-snobol4),
  [web-sw-cor24-basic](https://github.com/sw-embed/web-sw-cor24-basic)
  -- sibling browser demos in the same family.

## License

MIT. (C) 2026 Michael A Wright. See [LICENSE](LICENSE).
