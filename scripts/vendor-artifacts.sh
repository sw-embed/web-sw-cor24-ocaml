#!/usr/bin/env bash
#
# vendor-artifacts.sh -- Build and vendor PVM + OCaml interpreter artifacts
#
# Runs `just build` in the sibling sw-cor24-ocaml CLI repo to produce
# ocaml.p24m, pvm.bin, and code_ptr_addr.txt, then copies them into
# this project's assets/ directory so the WASM runner can include them.
#
# Usage: ./scripts/vendor-artifacts.sh
#   CLI_DIR=/custom/path ./scripts/vendor-artifacts.sh
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
CLI_DIR="${CLI_DIR:-$REPO_DIR/../sw-cor24-ocaml}"

if [ ! -d "$CLI_DIR" ]; then
    echo "error: CLI_DIR not found: $CLI_DIR" >&2
    exit 1
fi

CLI_DIR="$(cd "$CLI_DIR" && pwd)"
echo "CLI_DIR: $CLI_DIR"

echo "Building OCaml interpreter artifacts in $CLI_DIR..."
(cd "$CLI_DIR" && just build)

for name in ocaml.p24m pvm.bin code_ptr_addr.txt; do
    src="$CLI_DIR/build/$name"
    if [ ! -f "$src" ]; then
        echo "error: expected artifact missing: $src" >&2
        exit 1
    fi
done

ASSETS_DIR="$REPO_DIR/assets"
mkdir -p "$ASSETS_DIR"

cp "$CLI_DIR/build/ocaml.p24m"        "$ASSETS_DIR/ocaml.p24m"
cp "$CLI_DIR/build/pvm.bin"           "$ASSETS_DIR/pvm.bin"
cp "$CLI_DIR/build/code_ptr_addr.txt" "$ASSETS_DIR/code_ptr_addr.txt"

CODE_PTR="$(cat "$ASSETS_DIR/code_ptr_addr.txt")"

echo ""
echo "Vendored artifacts into $ASSETS_DIR:"
for name in ocaml.p24m pvm.bin code_ptr_addr.txt; do
    size="$(wc -c < "$ASSETS_DIR/$name" | tr -d ' ')"
    printf "  %-20s %8s bytes\n" "$name" "$size"
done
echo ""
echo "code_ptr: 0x${CODE_PTR}"
