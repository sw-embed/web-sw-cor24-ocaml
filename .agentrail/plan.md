# Saga: live-demo for web-sw-cor24-ocaml

## Goal

Port the CLI demos from `../sw-cor24-ocaml` into a browser-based live
demo, mirroring the pattern used by `../web-sw-cor24-basic`. The end
result is a Yew + Trunk web app that lets visitors pick an OCaml demo
from a dropdown, edit the source, run it in a WASM build of the P-code
VM, and see UART output and (for REPL-style demos) feed input lines.

## Reference projects

- **CLI source of truth** -- `../sw-cor24-ocaml`
  - Pipeline: `.ml` -> Pascal OCaml interpreter -> P-code image
    (`build/ocaml.p24m`), plus pre-assembled PVM (`build/pvm.bin`).
  - Demos live in `tests/` as `.ml` files (eval_*, demo_*, repl_session).
  - Demos are actively being added, so the web UI's demo list must be
    easy to extend.
- **Web UI reference** -- `../web-sw-cor24-basic`
  - Yew SPA, Trunk build, `src/runner.rs` is a WASM-native port of the
    P-code VM (pv24t) that loads a `.p24`-style binary.
  - `src/demos.rs` is a static table of `{ name, source, interactive }`
    entries, with `include_str!` pulling sources from an `examples/`
    directory.
  - `build.rs` copies the compiled `.p24` asset into `OUT_DIR` and
    stamps build metadata (SHA, host, timestamp).
  - Scripts: `scripts/build-pages.sh` (release build for GitHub Pages)
    and `scripts/serve.sh` (dev server on port 9072) with a mkdir lock
    to prevent two Trunk pipelines from racing on `dist/`.

## Architecture (target)

```
web-sw-cor24-ocaml/
  Cargo.toml              # yew, wasm-bindgen, gloo, web-sys, pa24r
  Trunk.toml              # dist/, port 9735 (reserved for this project)
  build.rs                # copy ocaml.p24m + pvm.bin into OUT_DIR;
                          # stamp BUILD_SHA / BUILD_HOST / BUILD_TIMESTAMP
  index.html              # Trunk rust bin, CSS link, favicon
  assets/
    ocaml.p24m            # vendored from sw-cor24-ocaml/build/
    pvm.bin               # vendored from sw-cor24-ocaml/build/
    code_ptr_addr.txt     # resolved code_ptr (needed for patch)
  examples/
    *.ml                  # mirrored subset of sw-cor24-ocaml/tests/*.ml
  src/
    lib.rs                # Yew App (picker, editor, output, input row)
    main.rs               # yew renderer entry
    demos.rs              # static DEMOS table, include_str! from examples/
    runner.rs             # PVM WASM port, adapted for OCaml image layout:
                          #   - load pvm.bin at 0, ocaml.p24m at 0x010000
                          #   - patch code_ptr to 0x010000
                          #   - feed .ml source to UART stdin terminated \x04
                          #   - capture UART output
    config.rs             # default demo, memory size, budget
    ui.css                # ported from web-sw-cor24-basic
  scripts/
    build-pages.sh        # trunk build --release --public-url ...
    serve.sh              # trunk serve --port 9735 with dist lock
    vendor-artifacts.sh   # rebuild ocaml.p24m/pvm.bin from ../sw-cor24-ocaml
  pages/                  # gh-pages output (gitignored contents except .nojekyll)
  docs/
    demos.md              # per-demo documentation
```

## Key design choices

- **Use pa24r as a library dep (path = `../sw-cor24-pcode/assembler`)**
  only if the runner needs assembly; otherwise prefer a pure-Rust port
  of the PVM (same approach as web-sw-cor24-basic `runner.rs`) and drop
  the dep. Decide in the runner-port step.
- **Vendoring vs. live build**: the artifacts (`ocaml.p24m`, `pvm.bin`)
  are produced by the CLI's Pascal-on-P-code toolchain, which is too
  heavyweight to run inside `cargo build`. Vendor the binaries into
  `assets/` via a standalone `scripts/vendor-artifacts.sh` that invokes
  the CLI's `just build` and copies out the artifacts plus the
  `code_ptr_addr.txt`.
- **Demo list extensibility**: `src/demos.rs` uses `include_str!` from
  a local `examples/` directory. A helper script syncs chosen demos
  from `../sw-cor24-ocaml/tests/` into `examples/` so the web demo
  list stays close to the CLI's. Not every CLI test is a demo -- cherry
  pick the user-facing ones (demo_*, eval_fact, eval_pairs, repl_session,
  etc.).
- **Interactive demos**: the `repl_session.ml` flow maps onto the same
  `awaiting_input` + input-row mechanism the BASIC app uses. The OCaml
  interpreter reads expressions from UART delimited by `\x04`, so the
  session wiring needs to convert each submitted input line into an
  appended UART byte stream + terminator.

## Out of scope (future sagas)

- Automated CI to rebuild `assets/` from the CLI on every commit.
- Per-demo documentation pages with walk-throughs.
- Embedding the CLI's regression suite as browser smoke tests.
- A "share link" feature that encodes source in the URL.
