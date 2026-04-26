//! Static catalog of OCaml demos shown in the web UI dropdown.
//!
//! Sources live under `examples/` and are pulled into the binary via
//! `include_str!` so the WASM bundle is self-contained. The set is
//! kept in sync with `../sw-cor24-ocaml/tests/` by
//! `scripts/sync-demos.sh`; see that script for the rename mapping.
//!
//! Phase 1 of the multi-file demos plan
//! (`docs/multiple-file-demos-plan.md`): demos can carry zero-to-many
//! `auxiliary_files` alongside the main `source`. The runner pre-
//! concatenates them with `let __module = "<Stem>"` directives
//! between each file, matching what the CLI's `run-ocaml.sh` does
//! when invoked with multiple `.ml` inputs. The main source is
//! always treated as module `Main`. The UI editor still shows only
//! the main source in Phase 1; aux files are baked-in read-only.

/// One auxiliary `.ml` file shipped alongside a demo's main source.
/// `name` should be a bare filename like `"math.ml"`; the runner
/// strips `.ml` and capitalises the first character to derive the
/// module name (`math.ml` -> module `Math`).
pub struct AuxFile {
    pub name: &'static str,
    pub source: &'static str,
}

/// Dropdown grouping. Standard demos are the substantive 36-entry
/// catalog; Minimal demos are the 16 canonical_*.ml one-liners
/// surfaced under a separate <optgroup>.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DemoCategory {
    Standard,
    Minimal,
}

pub struct Demo {
    pub name: &'static str,
    pub source: &'static str,
    pub interactive: bool,
    pub description: &'static str,
    pub auxiliary_files: &'static [AuxFile],
    pub category: DemoCategory,
}

impl Demo {
    /// Concatenate `auxiliary_files` (each preceded by a synthesized
    /// `let __module = "<Stem>"` directive) followed by `main` (the
    /// caller-supplied main source, prefixed with
    /// `let __module = "Main"`). When the demo has no aux files the
    /// directives are skipped and the main source is returned
    /// unchanged -- existing single-file demos keep behaving exactly
    /// as before.
    pub fn concat_main(&self, main: &str) -> String {
        if self.auxiliary_files.is_empty() {
            return main.to_string();
        }
        // Convert baked-in &'static str aux into the borrowed form
        // `concat_with_aux` accepts.
        let pairs: Vec<(&str, &str)> = self
            .auxiliary_files
            .iter()
            .map(|a| (a.name, a.source))
            .collect();
        concat_with_aux(&pairs, main)
    }

    /// `concat_main` using the demo's baked-in main source. Test
    /// sites use this; the live UI uses `concat_main(&edited_source)`
    /// so user edits to the main pane stay live.
    pub fn full_source(&self) -> String {
        self.concat_main(self.source)
    }
}

/// Free-function form of the aux concat. Used by the live UI's Run
/// path to feed user-edited aux source through the same injection
/// rules as the baked-in version. `aux` is `(filename, source)` in
/// dependency order; `main` is the main source (typically
/// `main.ml`).
pub fn concat_with_aux(aux: &[(&str, &str)], main: &str) -> String {
    if aux.is_empty() {
        return main.to_string();
    }
    let mut out = String::new();
    for (name, source) in aux {
        out.push_str(&format!("let __module = \"{}\"\n", capitalize_stem(name)));
        out.push_str(source);
        if !source.ends_with('\n') {
            out.push('\n');
        }
    }
    out.push_str("let __module = \"Main\"\n");
    out.push_str(main);
    out
}

/// Strip a trailing `.ml` and capitalise the first byte. Mirrors the
/// CLI's `run-ocaml.sh` rule: `math.ml` -> `Math`,
/// `game_state.ml` -> `Game_state`.
pub(crate) fn capitalize_stem(filename: &str) -> String {
    let stem = filename.strip_suffix(".ml").unwrap_or(filename);
    let mut chars = stem.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => String::new(),
    }
}

// Kept in alphabetical order by `name` so the UI dropdown is
// alphabetised. `default_demo_index` finds `hello` by name, so this
// ordering is independent of which demo is the default.
pub static DEMOS: &[Demo] = &[
    Demo {
        name: "canonical-arith",
        source: include_str!("../examples/canonical-arith.ml"),
        interactive: false,
        description: "Operator precedence: `1 + 2 * 3` evaluates `*` before `+` -> 7.",
        auxiliary_files: &[],
        category: DemoCategory::Minimal,
    },
    Demo {
        name: "canonical-fact",
        source: include_str!("../examples/canonical-fact.ml"),
        interactive: false,
        description: "Recursive factorial via `let rec` (5! = 120).",
        auxiliary_files: &[],
        category: DemoCategory::Minimal,
    },
    Demo {
        name: "canonical-fib",
        source: include_str!("../examples/canonical-fib.ml"),
        interactive: false,
        description: "Recursive Fibonacci, prints `fib 10` = 55.",
        auxiliary_files: &[],
        category: DemoCategory::Minimal,
    },
    Demo {
        name: "canonical-int-literal",
        source: include_str!("../examples/canonical-int-literal.ml"),
        interactive: false,
        description: "The smallest possible expression: an integer literal `42`.",
        auxiliary_files: &[],
        category: DemoCategory::Minimal,
    },
    Demo {
        name: "canonical-lambda-app",
        source: include_str!("../examples/canonical-lambda-app.ml"),
        interactive: false,
        description: "Apply an inline lambda: `(fun x -> x * 2) 5` -> 10.",
        auxiliary_files: &[],
        category: DemoCategory::Minimal,
    },
    Demo {
        name: "canonical-let-in",
        source: include_str!("../examples/canonical-let-in.ml"),
        interactive: false,
        description: "Local binding via `let ... in`: `let x = 10 in x * x` -> 100.",
        auxiliary_files: &[],
        category: DemoCategory::Minimal,
    },
    Demo {
        name: "canonical-list-filter",
        source: include_str!("../examples/canonical-list-filter.ml"),
        interactive: false,
        description: "`List.filter` with an inline predicate (keep evens).",
        auxiliary_files: &[],
        category: DemoCategory::Minimal,
    },
    Demo {
        name: "canonical-list-fold-left",
        source: include_str!("../examples/canonical-list-fold-left.ml"),
        interactive: false,
        description: "`List.fold_left` summing `[1;2;3;4]` -> 10.",
        auxiliary_files: &[],
        category: DemoCategory::Minimal,
    },
    Demo {
        name: "canonical-list-map",
        source: include_str!("../examples/canonical-list-map.ml"),
        interactive: false,
        description: "`List.map` doubling `[1;2;3]` -> `[2; 4; 6]`.",
        auxiliary_files: &[],
        category: DemoCategory::Minimal,
    },
    Demo {
        name: "canonical-match-int",
        source: include_str!("../examples/canonical-match-int.ml"),
        interactive: false,
        description: "Integer `match` with literal arms and `_` fallthrough.",
        auxiliary_files: &[],
        category: DemoCategory::Minimal,
    },
    Demo {
        name: "canonical-print-length",
        source: include_str!("../examples/canonical-print-length.ml"),
        interactive: false,
        description: "Compose `print_endline`, `string_of_int`, `List.length` to print `3`.",
        auxiliary_files: &[],
        category: DemoCategory::Minimal,
    },
    Demo {
        name: "canonical-safe-div",
        source: include_str!("../examples/canonical-safe-div.ml"),
        interactive: false,
        description: "Option-returning division: `safe_div 10 2` -> `Some 5`.",
        auxiliary_files: &[],
        category: DemoCategory::Minimal,
    },
    Demo {
        name: "canonical-some-42",
        source: include_str!("../examples/canonical-some-42.ml"),
        interactive: false,
        description: "Constructing `Some 42` -- a one-line option intro.",
        auxiliary_files: &[],
        category: DemoCategory::Minimal,
    },
    Demo {
        name: "canonical-string-concat",
        source: include_str!("../examples/canonical-string-concat.ml"),
        interactive: false,
        description: "String concatenation with `^`: `\"OCaml\" ^ \" rocks\"`.",
        auxiliary_files: &[],
        category: DemoCategory::Minimal,
    },
    Demo {
        name: "canonical-swap",
        source: include_str!("../examples/canonical-swap.ml"),
        interactive: false,
        description: "Tuple-destructuring function arg: `let swap (x, y) = (y, x)`.",
        auxiliary_files: &[],
        category: DemoCategory::Minimal,
    },
    Demo {
        name: "canonical-when-guard",
        source: include_str!("../examples/canonical-when-guard.ml"),
        interactive: false,
        description: "`match ... when <guard>` arm: minimal `abs` implementation.",
        auxiliary_files: &[],
        category: DemoCategory::Minimal,
    },
    Demo {
        name: "echo-loop",
        source: include_str!("../examples/echo-loop.ml"),
        interactive: true,
        description: "Type any text and it's echoed back. Type `quit` to exit.",
        auxiliary_files: &[],
        category: DemoCategory::Standard,
    },
    Demo {
        name: "factorial",
        source: include_str!("../examples/factorial.ml"),
        interactive: false,
        description: "Recursive factorial via `let rec`; computes 5!.",
        auxiliary_files: &[],
        category: DemoCategory::Standard,
    },
    Demo {
        name: "function-form-let",
        source: include_str!("../examples/function-form-let.ml"),
        interactive: false,
        description: "Sugared `let f x y = body` form (curried function definitions).",
        auxiliary_files: &[],
        category: DemoCategory::Standard,
    },
    Demo {
        name: "function-keyword",
        source: include_str!("../examples/function-keyword.ml"),
        interactive: false,
        description: "The `function` keyword: shorthand for `fun x -> match x with ...`.",
        auxiliary_files: &[],
        category: DemoCategory::Standard,
    },
    Demo {
        name: "function-pattern-args",
        source: include_str!("../examples/function-pattern-args.ml"),
        interactive: false,
        description:
            "Destructuring patterns directly in function arguments: `let swap (x, y) = ...`.",
        auxiliary_files: &[],
        category: DemoCategory::Standard,
    },
    Demo {
        name: "functions",
        source: include_str!("../examples/functions.ml"),
        interactive: false,
        description: "First-class functions and `let` bindings.",
        auxiliary_files: &[],
        category: DemoCategory::Standard,
    },
    Demo {
        name: "guess",
        source: include_str!("../examples/guess.ml"),
        interactive: true,
        description: "Number-guessing game: the target is 42. Enter an integer; \
                      the demo replies `too low`, `too high`, or `correct!`.",
        auxiliary_files: &[],
        category: DemoCategory::Standard,
    },
    Demo {
        name: "hello",
        source: include_str!("../examples/hello.ml"),
        interactive: false,
        description: "Smallest possible program: print the integer 42.",
        auxiliary_files: &[],
        category: DemoCategory::Standard,
    },
    Demo {
        name: "hello-world",
        source: include_str!("../examples/hello-world.ml"),
        interactive: false,
        description: "Print the string `\"Hello, World!\"` via `print_endline`.",
        auxiliary_files: &[],
        category: DemoCategory::Standard,
    },
    Demo {
        name: "higher-order-lists",
        source: include_str!("../examples/higher-order-lists.ml"),
        interactive: false,
        description:
            "`List.map`, `List.filter`, `List.fold_left`, `List.iter` with inline lambdas.",
        auxiliary_files: &[],
        category: DemoCategory::Standard,
    },
    Demo {
        name: "led-blink",
        source: include_str!("../examples/led-blink.ml"),
        interactive: false,
        description: "Drive the COR24 LED via `led_on` / `led_off`. Browser stubs the LED.",
        auxiliary_files: &[],
        category: DemoCategory::Standard,
    },
    Demo {
        name: "led-toggle",
        source: include_str!("../examples/led-toggle.ml"),
        interactive: false,
        description: "Read the COR24 switch and reflect it on the LED.",
        auxiliary_files: &[],
        category: DemoCategory::Standard,
    },
    Demo {
        name: "let-destructure",
        source: include_str!("../examples/let-destructure.ml"),
        interactive: false,
        description: "Destructuring `let (a, b) = ...`, `let h :: t = ...`, and friends.",
        auxiliary_files: &[],
        category: DemoCategory::Standard,
    },
    Demo {
        name: "list-module",
        source: include_str!("../examples/list-module.ml"),
        interactive: false,
        description: "`List.length`, `List.rev`, qualified-name lookups.",
        auxiliary_files: &[],
        category: DemoCategory::Standard,
    },
    Demo {
        name: "lists",
        source: include_str!("../examples/lists.ml"),
        interactive: false,
        description: "List literals, cons, head/tail, is_empty.",
        auxiliary_files: &[],
        category: DemoCategory::Standard,
    },
    Demo {
        name: "lists-pairs-demo",
        source: include_str!("../examples/lists-pairs-demo.ml"),
        interactive: false,
        description: "Sum, length, map, swap, countdown — lists + pairs in one program.",
        auxiliary_files: &[],
        category: DemoCategory::Standard,
    },
    Demo {
        name: "modules",
        source: include_str!("../examples/modules.ml"),
        interactive: false,
        description: "User-defined module namespaces via the `let __module = \"...\"` \
                      directive. Defines `Math.add`/`Math.double`, switches to `Main`, \
                      shows three qualified calls succeeding -- then a deliberate \
                      unqualified `add 1 2` to demonstrate namespace isolation. The \
                      trailing `EVAL ERROR` is the punchline; the language has no \
                      try/catch, the REPL just resets per line.",
        auxiliary_files: &[],
        category: DemoCategory::Standard,
    },
    Demo {
        name: "modules-multifile",
        source: include_str!("../examples/modules-multifile/main.ml"),
        interactive: false,
        description: "Multi-file module demo (Phase 1): `math.ml` defines `Math.add` / \
                      `Math.square` / `Math.double`; `main.ml` calls them by qualified \
                      name. The runner concatenates the files with synthesized \
                      `let __module = \"...\"` directives, matching the CLI's \
                      `run-ocaml.sh` behaviour. The editor shows only `main.ml`; the \
                      aux file is baked in read-only.",
        auxiliary_files: &[AuxFile {
            name: "math.ml",
            source: include_str!("../examples/modules-multifile/math.ml"),
        }],
        category: DemoCategory::Standard,
    },
    Demo {
        name: "multi-arg",
        source: include_str!("../examples/multi-arg.ml"),
        interactive: false,
        description: "Multi-argument curried function via `fun x y -> ...`.",
        auxiliary_files: &[],
        category: DemoCategory::Standard,
    },
    Demo {
        name: "named-adts",
        source: include_str!("../examples/named-adts.ml"),
        interactive: false,
        description: "Sum types via `type T = C1 | C2 | ...` and `match` expressions.",
        auxiliary_files: &[],
        category: DemoCategory::Standard,
    },
    Demo {
        name: "options",
        source: include_str!("../examples/options.ml"),
        interactive: false,
        description: "The built-in `option` type: `None` and `Some x`.",
        auxiliary_files: &[],
        category: DemoCategory::Standard,
    },
    Demo {
        name: "pairs",
        source: include_str!("../examples/pairs.ml"),
        interactive: false,
        description: "Tuple construction with `fst` / `snd` accessors.",
        auxiliary_files: &[],
        category: DemoCategory::Standard,
    },
    Demo {
        name: "patterns",
        source: include_str!("../examples/patterns.ml"),
        interactive: false,
        description: "Pattern matching across lists, tuples, options, and literals.",
        auxiliary_files: &[],
        category: DemoCategory::Standard,
    },
    Demo {
        name: "print",
        source: include_str!("../examples/print.ml"),
        interactive: false,
        description: "`print_int` and `putc` writing through the UART.",
        auxiliary_files: &[],
        category: DemoCategory::Standard,
    },
    Demo {
        name: "repl-session",
        source: include_str!("../examples/repl-session.ml"),
        interactive: true,
        description: "Multi-expression REPL session -- type more lines after the seed runs. \
                      Each input must be a complete expression: `let x = 42 in x`, not bare \
                      `let x = 42`.",
        auxiliary_files: &[],
        category: DemoCategory::Standard,
    },
    Demo {
        name: "sequencing",
        source: include_str!("../examples/sequencing.ml"),
        interactive: false,
        description: "Semicolon-sequenced expressions evaluated left-to-right.",
        auxiliary_files: &[],
        category: DemoCategory::Standard,
    },
    Demo {
        name: "string-conversion",
        source: include_str!("../examples/string-conversion.ml"),
        interactive: false,
        description: "`string_of_int` / `int_of_string` round-trips, including malformed inputs.",
        auxiliary_files: &[],
        category: DemoCategory::Standard,
    },
    Demo {
        name: "string-equality",
        source: include_str!("../examples/string-equality.ml"),
        interactive: false,
        description: "Structural `=` and `<>` on strings, including in `if` conditions.",
        auxiliary_files: &[],
        category: DemoCategory::Standard,
    },
    Demo {
        name: "string-escapes",
        source: include_str!("../examples/string-escapes.ml"),
        interactive: false,
        description: "Escape sequences in string literals: `\\n`, `\\t`, `\\\\`, `\\\"`.",
        auxiliary_files: &[],
        category: DemoCategory::Standard,
    },
    Demo {
        name: "strings",
        source: include_str!("../examples/strings.ml"),
        interactive: false,
        description: "String literals, `^` concatenation, `print_endline`, `String.length`.",
        auxiliary_files: &[],
        category: DemoCategory::Standard,
    },
    Demo {
        name: "tco-countdown",
        source: include_str!("../examples/tco-countdown.ml"),
        interactive: false,
        description: "Tail-call optimisation: count down from 100 without growing the stack.",
        auxiliary_files: &[],
        category: DemoCategory::Standard,
    },
    Demo {
        name: "text-adventure",
        source: include_str!("../examples/text-adventure.ml"),
        interactive: true,
        description: "Interactive text adventure: navigate rooms, pick up items. \
                      Commands: look, inventory, take, n/s/e/w, quit.",
        auxiliary_files: &[],
        category: DemoCategory::Standard,
    },
    Demo {
        name: "toplevel-let",
        source: include_str!("../examples/toplevel-let.ml"),
        interactive: false,
        description: "Top-level `let` bindings, recursive `let rec`, tuple destructuring, `let () = ...`.",
        auxiliary_files: &[],
        category: DemoCategory::Standard,
    },
    Demo {
        name: "tuple-arity",
        source: include_str!("../examples/tuple-arity.ml"),
        interactive: false,
        description: "Pattern matching against 3- and 4-tuples with wildcards.",
        auxiliary_files: &[],
        category: DemoCategory::Standard,
    },
    Demo {
        name: "variants-with-payload",
        source: include_str!("../examples/variants-with-payload.ml"),
        interactive: false,
        description: "Variant constructors carrying payloads (`TInt of int`, `TIdent of string`) dispatched via `match`.",
        auxiliary_files: &[],
        category: DemoCategory::Standard,
    },
    Demo {
        name: "when-guards",
        source: include_str!("../examples/when-guards.ml"),
        interactive: false,
        description: "`match ... when <guard>` clauses for conditional pattern arms (abs, sign).",
        auxiliary_files: &[],
        category: DemoCategory::Standard,
    },
];

pub fn default_demo_index() -> usize {
    DEMOS.iter().position(|d| d.name == "hello").unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_hello() {
        assert_eq!(DEMOS[default_demo_index()].name, "hello");
    }

    #[test]
    fn names_are_unique() {
        let mut names: Vec<_> = DEMOS.iter().map(|d| d.name).collect();
        names.sort();
        let len = names.len();
        names.dedup();
        assert_eq!(names.len(), len, "duplicate demo names");
    }

    #[test]
    fn minimal_examples_present_and_well_formed() {
        // The "Minimal examples" tier should have all 16 canonical
        // demos and they should all start with the `canonical-`
        // prefix so the dropdown grouping stays predictable.
        let minimals: Vec<_> = DEMOS
            .iter()
            .filter(|d| d.category == DemoCategory::Minimal)
            .collect();
        assert_eq!(
            minimals.len(),
            16,
            "expected 16 minimal-category demos, got {}",
            minimals.len()
        );
        for demo in &minimals {
            assert!(
                demo.name.starts_with("canonical-"),
                "minimal demo '{}' should have canonical- prefix",
                demo.name
            );
            assert!(
                !demo.interactive,
                "minimal demo '{}' should be non-interactive",
                demo.name
            );
            assert!(
                demo.auxiliary_files.is_empty(),
                "minimal demo '{}' should not carry auxiliary files",
                demo.name
            );
        }
    }

    #[test]
    fn aux_filenames_are_unique_within_each_demo() {
        for demo in DEMOS {
            let mut names: Vec<_> = demo.auxiliary_files.iter().map(|a| a.name).collect();
            names.sort();
            let len = names.len();
            names.dedup();
            assert_eq!(
                names.len(),
                len,
                "duplicate aux filenames in demo {}",
                demo.name
            );
        }
    }

    #[test]
    fn capitalize_stem_matches_cli_rule() {
        assert_eq!(capitalize_stem("math.ml"), "Math");
        assert_eq!(capitalize_stem("main.ml"), "Main");
        assert_eq!(capitalize_stem("game_state.ml"), "Game_state");
        assert_eq!(capitalize_stem("a.ml"), "A");
        assert_eq!(capitalize_stem("noext"), "Noext");
        assert_eq!(capitalize_stem(""), "");
    }

    #[test]
    fn full_source_is_passthrough_without_aux() {
        // Demos with no aux files must produce concatenated source
        // identical to their main source -- preserves backward compat
        // for the 34 single-file demos.
        for demo in DEMOS {
            if demo.auxiliary_files.is_empty() {
                assert_eq!(
                    demo.full_source(),
                    demo.source,
                    "full_source diverges from source for single-file demo {}",
                    demo.name
                );
            }
        }
    }

    #[test]
    fn full_source_injects_module_directives_with_aux() {
        // Find the modules-multifile demo and verify the concat shape.
        let demo = DEMOS
            .iter()
            .find(|d| d.name == "modules-multifile")
            .expect("modules-multifile demo not found");
        let full = demo.full_source();
        assert!(
            full.starts_with("let __module = \"Math\"\n"),
            "missing leading Math directive: {full:?}"
        );
        assert!(
            full.contains("\nlet __module = \"Main\"\n"),
            "missing Main directive between aux and main: {full:?}"
        );
        // Both files' bodies must appear in the concatenation.
        assert!(full.contains("let add x y = x + y"), "aux body missing");
        assert!(full.contains("Math.add 2 3"), "main body missing");
    }

    #[test]
    fn demos_are_alphabetised() {
        let names: Vec<_> = DEMOS.iter().map(|d| d.name).collect();
        let mut sorted = names.clone();
        sorted.sort();
        assert_eq!(
            names, sorted,
            "DEMOS must stay alphabetised so the UI dropdown is alphabetised"
        );
    }

    #[test]
    fn interactive_demos_exist() {
        let interactive: Vec<_> = DEMOS
            .iter()
            .filter(|d| d.interactive)
            .map(|d| d.name)
            .collect();
        assert!(
            !interactive.is_empty(),
            "at least one demo must be interactive"
        );
        assert!(
            interactive.contains(&"echo-loop"),
            "echo-loop must be interactive"
        );
        assert!(
            interactive.contains(&"repl-session"),
            "repl-session must be interactive"
        );
        assert!(
            interactive.contains(&"text-adventure"),
            "text-adventure must be interactive"
        );
    }
}
