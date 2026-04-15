# Vendor PVM + OCaml interpreter artifacts

Produce the binary artifacts the browser runtime will load, and vendor
them into `assets/`. The browser runtime cannot run the CLI's
Pascal-on-P-code build pipeline, so we bake the artifacts at build
time and ship them as static assets.

## Prerequisite

The scaffold step is complete (Cargo.toml, Trunk.toml, src/ stub).

## Work

1. Write `scripts/vendor-artifacts.sh`:
   - `set -euo pipefail`.
   - Accept `CLI_DIR` env var (default `../sw-cor24-ocaml`).
   - `cd "$CLI_DIR" && just build` (uses their vendored toolchain).
   - Copy `$CLI_DIR/build/ocaml.p24m` -> `assets/ocaml.p24m`.
   - Copy `$CLI_DIR/build/pvm.bin` -> `assets/pvm.bin`.
   - Copy `$CLI_DIR/build/code_ptr_addr.txt` -> `assets/code_ptr_addr.txt`.
   - Print sizes + the code_ptr value to stdout.
2. Run it once from the repo root. Commit the three files under
   `assets/`. These are binary / tiny text — track them, do not
   `.gitignore` them.
3. Extend `build.rs` to `include_bytes!` the two binaries into
   `OUT_DIR` (mirror the pattern in `../web-sw-cor24-basic/build.rs`
   for `basic.p24`), and emit a `CODE_PTR` rustc env var read from
   `assets/code_ptr_addr.txt`. Add `cargo:rerun-if-changed=` lines for
   all three asset files.

## Validation

- `scripts/vendor-artifacts.sh` runs clean from the repo root.
- `cargo check` succeeds after the build.rs changes (no callers yet —
  that's the runner step).
- `ls -la assets/` shows the three files with non-zero size.

## Out of scope

- Using the artifacts (that's the runner step).
- CI automation for refreshing assets when the CLI changes.
