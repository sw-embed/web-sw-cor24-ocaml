#!/usr/bin/env bash
#
# vendor-artifacts.sh -- Refresh vendored sources and artifacts
#
# - Re-builds ocaml.p24m via the sibling sw-cor24-ocaml CLI repo
#   (`just build`) and copies it into assets/.
# - Refreshes asm/pvm.s from sw-cor24-pcode so build.rs assembles
#   the canonical p-code VM into pvm.bin at compile time.
#
# Note: pvm.bin is NOT vendored. It is assembled from asm/pvm.s
# inside build.rs using the cor24-emulator assembler so it always
# matches the in-tree source.
#
# Usage: ./scripts/vendor-artifacts.sh
#   CLI_DIR=/custom/path PCODE_DIR=/custom/path ./scripts/vendor-artifacts.sh
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
CLI_DIR="${CLI_DIR:-$REPO_DIR/../sw-cor24-ocaml}"
PCODE_DIR="${PCODE_DIR:-$REPO_DIR/../sw-cor24-pcode}"

if [ ! -d "$CLI_DIR" ]; then
    echo "error: CLI_DIR not found: $CLI_DIR" >&2
    exit 1
fi
if [ ! -f "$PCODE_DIR/vm/pvm.s" ]; then
    echo "error: $PCODE_DIR/vm/pvm.s not found" >&2
    exit 1
fi

CLI_DIR="$(cd "$CLI_DIR" && pwd)"
PCODE_DIR="$(cd "$PCODE_DIR" && pwd)"

echo "CLI_DIR:   $CLI_DIR"
echo "PCODE_DIR: $PCODE_DIR"

echo ""
echo "Refreshing asm/pvm.s from $PCODE_DIR/vm/pvm.s..."
mkdir -p "$REPO_DIR/asm"
cp "$PCODE_DIR/vm/pvm.s" "$REPO_DIR/asm/pvm.s"

echo ""
echo "Building OCaml interpreter image in $CLI_DIR..."
(cd "$CLI_DIR" && just build)

ASSETS_DIR="$REPO_DIR/assets"
mkdir -p "$ASSETS_DIR"
cp "$CLI_DIR/build/ocaml.p24m" "$ASSETS_DIR/ocaml.p24m"

echo ""
echo "Vendored:"
pvm_lines="$(wc -l < "$REPO_DIR/asm/pvm.s" | tr -d ' ')"
p24m_size="$(wc -c < "$ASSETS_DIR/ocaml.p24m" | tr -d ' ')"
printf "  %-25s %8s lines\n" "asm/pvm.s"           "$pvm_lines"
printf "  %-25s %8s bytes\n" "assets/ocaml.p24m"   "$p24m_size"
echo ""
echo "build.rs assembles asm/pvm.s into pvm.bin via cor24-emulator."
