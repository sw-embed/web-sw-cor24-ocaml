#!/usr/bin/env bash
#
# sync-demos.sh -- Refresh examples/ from the sibling CLI repo.
#
# The mapping table is the single source of truth for which CLI test
# files surface as web demos and what they're renamed to. Add new
# entries here as the CLI grows new demos worth showing.
#
# Usage: ./scripts/sync-demos.sh
#   CLI_DIR=/custom/path ./scripts/sync-demos.sh
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
CLI_DIR="${CLI_DIR:-$REPO_DIR/../sw-cor24-ocaml}"

if [ ! -d "$CLI_DIR/tests" ]; then
    echo "error: $CLI_DIR/tests not found" >&2
    exit 1
fi

EXAMPLES_DIR="$REPO_DIR/examples"
mkdir -p "$EXAMPLES_DIR"

# CLI test file (without .ml) -> web demo file (without .ml)
MAPPING=(
    "eval_fact:factorial"
    "eval_pairs:pairs"
    "eval_list_basics:lists"
    "eval_list_module:list-module"
    "demo_lists_pairs:lists-pairs-demo"
    "demo_led_blink:led-blink"
    "demo_led_toggle:led-toggle"
    "eval_fun:functions"
    "eval_multi_arg:multi-arg"
    "eval_seq:sequencing"
    "eval_print:print"
    "eval_function_form_let:function-form-let"
    "eval_strings:strings"
    "eval_named_adts:named-adts"
    "eval_options:options"
    "demo_patterns:patterns"
    "eval_let_destructure:let-destructure"
    "repl_session:repl-session"
)

# Demos whose source is one logical expression that spans multiple
# physical lines via semicolon-sequencing or `let ... in` chaining.
# The OCaml REPL reads line-by-line and parses each line as a
# standalone top-level expression, so a trailing `;` on a line is a
# parse error in the web context. Collapsing newlines to spaces
# preserves the program's semantics while making it a single REPL
# input. Demos NOT in this list (pairs.ml, lists.ml, list-module.ml,
# repl-session.ml) genuinely have one independent expression per
# line and must be left as-is.
COLLAPSE_NEWLINES=("led-blink")

# Demos whose CLI source is unsuitable for the web demo (e.g. an
# infinite recursion that overflows the OCaml interp's call stack
# in the browser, where there's no terminal Ctrl-C). The web demo
# ships a hand-edited examples/<name>.ml that this script must NOT
# overwrite. Document the divergence here and in the demo's
# docs/demos.md section.
LOCAL_OVERRIDE=("led-toggle")

echo "Syncing demos from $CLI_DIR/tests/ -> $EXAMPLES_DIR/"
for entry in "${MAPPING[@]}"; do
    src="${entry%%:*}"
    dst="${entry##*:}"
    src_path="$CLI_DIR/tests/$src.ml"
    dst_path="$EXAMPLES_DIR/$dst.ml"
    if [ ! -f "$src_path" ]; then
        echo "  warn: $src_path missing, skipping" >&2
        continue
    fi
    if [[ " ${LOCAL_OVERRIDE[*]} " =~ " $dst " ]]; then
        printf "  %-22s [skipped: hand-edited local override]\n" "$dst.ml"
        continue
    fi
    cp "$src_path" "$dst_path"
    if [[ " ${COLLAPSE_NEWLINES[*]} " =~ " $dst " ]]; then
        # Collapse newlines to spaces so the multi-line source is one
        # REPL input. Trim trailing whitespace.
        tr '\n' ' ' < "$dst_path" | sed -e 's/[[:space:]]*$//' > "$dst_path.tmp"
        mv "$dst_path.tmp" "$dst_path"
        printf "  %-22s <- tests/%s.ml [newlines collapsed]\n" "$dst.ml" "$src"
    else
        printf "  %-22s <- tests/%s.ml\n" "$dst.ml" "$src"
    fi
done

# Hand-written hello.ml: default demo so first-time visitors see
# output immediately. Not synced from the CLI.
cat > "$EXAMPLES_DIR/hello.ml" <<'EOF'
print_int 42
EOF
echo "  hello.ml               <- (hand-written, repo-local)"
