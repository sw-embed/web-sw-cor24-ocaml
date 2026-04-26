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
    "eval_list_higher_order:higher-order-lists"
    "eval_when_guards:when-guards"
    "eval_function_keyword:function-keyword"
    "eval_function_pattern_args:function-pattern-args"
    "repl_session:repl-session"
    "demo_adventure:text-adventure"
    "demo_echo_loop:echo-loop"
    "demo_guess:guess"
    "eval_string_conversion:string-conversion"
    "eval_string_eq:string-equality"
    "eval_string_escapes:string-escapes"
    "demo_tco_countdown:tco-countdown"
    "eval_toplevel_let:toplevel-let"
    "eval_tuple_arity:tuple-arity"
    "eval_user_variants:variants-with-payload"
    "eval_module_namespace_directive:modules"
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
# in the browser, where there's no terminal Ctrl-C, or a CLI test
# that ends on a bare EVAL ERROR which reads as broken in the UI).
# The web demo ships a hand-edited examples/<name>.ml that this
# script must NOT overwrite. Document the divergence here and in
# the demo's docs/demos.md section.
#
# - led-toggle: CLI source uses an infinite loop; web variant blocks
#   on switch reads instead.
# - guess: CLI source overflows the browser stack on losing branch.
# - modules: CLI test (eval_module_namespace_directive.ml) ends on
#   `add 1 2 -> EVAL ERROR` with no framing. Web variant adds inline
#   `(* ... *)` comments plus a corrective `Math.add 1 2` before the
#   deliberate-failure line so the trailing EVAL ERROR reads as the
#   educational climax of the namespace-isolation story rather than a
#   bug. The interpreter has no try/catch; the REPL resets eval_error
#   per line and continues.
# - text-adventure: CLI source (demo_adventure.ml) has a take bug --
#   pickup is a pure function of room, so each `take` in the Cave
#   adds another Lamp to inventory and the cave still describes the
#   lamp. Web variant extends loop state to a 4-tuple
#   (room, inventory, lamp_taken, key_taken) so describe and pickup
#   can branch on what's been collected. CLI fix is a separate
#   follow-up flagged for the sw-cor24-ocaml agent.
LOCAL_OVERRIDE=("led-toggle" "guess" "modules" "text-adventure")

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

# "Minimal examples" tier: 16 canonical_*.ml one-liners from the
# CLI's tests/. Each is a single-line demonstration of one feature
# (literal, lambda app, list-map, match, etc.). Surfaced in the
# dropdown under a separate <optgroup label="Minimal examples">
# rather than interleaved with the standard catalog. CLI test name
# `canonical_<feature>` -> web demo `canonical-<feature>`.
MINIMAL_MAPPING=(
    "canonical_arith:canonical-arith"
    "canonical_fact:canonical-fact"
    "canonical_fib:canonical-fib"
    "canonical_int_literal:canonical-int-literal"
    "canonical_lambda_app:canonical-lambda-app"
    "canonical_let_in:canonical-let-in"
    "canonical_list_filter:canonical-list-filter"
    "canonical_list_fold_left:canonical-list-fold-left"
    "canonical_list_map:canonical-list-map"
    "canonical_match_int:canonical-match-int"
    "canonical_print_length:canonical-print-length"
    "canonical_safe_div:canonical-safe-div"
    "canonical_some_42:canonical-some-42"
    "canonical_string_concat:canonical-string-concat"
    "canonical_swap:canonical-swap"
    "canonical_when_guard:canonical-when-guard"
)
echo ""
echo "Syncing minimal examples from $CLI_DIR/tests/ -> $EXAMPLES_DIR/"
for entry in "${MINIMAL_MAPPING[@]}"; do
    src="${entry%%:*}"
    dst="${entry##*:}"
    src_path="$CLI_DIR/tests/$src.ml"
    dst_path="$EXAMPLES_DIR/$dst.ml"
    if [ ! -f "$src_path" ]; then
        echo "  warn: $src_path missing, skipping" >&2
        continue
    fi
    cp "$src_path" "$dst_path"
    printf "  %-30s <- tests/%s.ml\n" "$dst.ml" "$src"
done

# Multi-file demos (Phase 1 of docs/multiple-file-demos-plan.md).
# Each entry is "main_cli:aux_cli,aux_cli,...:web_dir". The script
# copies main_cli to <web_dir>/main.ml and each aux_cli to
# <web_dir>/<aux_cli>. The web demo's name in src/demos.rs is the
# directory basename (web_dir).
MULTIFILE_MAPPING=(
    "main:math:modules-multifile"
)
echo ""
for entry in "${MULTIFILE_MAPPING[@]}"; do
    main_src="${entry%%:*}"
    rest="${entry#*:}"
    aux_list="${rest%%:*}"
    web_dir="${rest##*:}"
    web_path="$EXAMPLES_DIR/$web_dir"
    main_src_path="$CLI_DIR/tests/$main_src.ml"
    if [ ! -f "$main_src_path" ]; then
        echo "  warn: $main_src_path missing, skipping multi-file demo $web_dir" >&2
        continue
    fi
    mkdir -p "$web_path"
    cp "$main_src_path" "$web_path/main.ml"
    printf "  %-30s <- tests/%s.ml\n" "$web_dir/main.ml" "$main_src"
    IFS=',' read -ra aux_files <<< "$aux_list"
    for aux in "${aux_files[@]}"; do
        aux_src_path="$CLI_DIR/tests/$aux.ml"
        if [ ! -f "$aux_src_path" ]; then
            echo "  warn: $aux_src_path missing, skipping aux for $web_dir" >&2
            continue
        fi
        cp "$aux_src_path" "$web_path/$aux.ml"
        printf "  %-30s <- tests/%s.ml\n" "$web_dir/$aux.ml" "$aux"
    done
done

# Hand-written hello.ml: default demo so first-time visitors see
# output immediately. Not synced from the CLI.
cat > "$EXAMPLES_DIR/hello.ml" <<'EOF'
print_int 42
EOF
echo "  hello.ml               <- (hand-written, repo-local)"
