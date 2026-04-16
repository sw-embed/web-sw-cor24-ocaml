//! Static catalog of OCaml demos shown in the web UI dropdown.
//!
//! Sources live under `examples/` and are pulled into the binary via
//! `include_str!` so the WASM bundle is self-contained. The set is
//! kept in sync with `../sw-cor24-ocaml/tests/` by
//! `scripts/sync-demos.sh`; see that script for the rename mapping.

pub struct Demo {
    pub name: &'static str,
    pub source: &'static str,
    pub interactive: bool,
    pub description: &'static str,
}

// Kept in alphabetical order by `name` so the UI dropdown is
// alphabetised. `default_demo_index` finds `hello` by name, so this
// ordering is independent of which demo is the default.
pub static DEMOS: &[Demo] = &[
    Demo {
        name: "factorial",
        source: include_str!("../examples/factorial.ml"),
        interactive: false,
        description: "Recursive factorial via `let rec`; computes 5!.",
    },
    Demo {
        name: "function-form-let",
        source: include_str!("../examples/function-form-let.ml"),
        interactive: false,
        description: "Sugared `let f x y = body` form (curried function definitions).",
    },
    Demo {
        name: "function-keyword",
        source: include_str!("../examples/function-keyword.ml"),
        interactive: false,
        description: "The `function` keyword: shorthand for `fun x -> match x with ...`.",
    },
    Demo {
        name: "function-pattern-args",
        source: include_str!("../examples/function-pattern-args.ml"),
        interactive: false,
        description:
            "Destructuring patterns directly in function arguments: `let swap (x, y) = ...`.",
    },
    Demo {
        name: "functions",
        source: include_str!("../examples/functions.ml"),
        interactive: false,
        description: "First-class functions and `let` bindings.",
    },
    Demo {
        name: "hello",
        source: include_str!("../examples/hello.ml"),
        interactive: false,
        description: "Smallest possible program: print the integer 42.",
    },
    Demo {
        name: "higher-order-lists",
        source: include_str!("../examples/higher-order-lists.ml"),
        interactive: false,
        description:
            "`List.map`, `List.filter`, `List.fold_left`, `List.iter` with inline lambdas.",
    },
    Demo {
        name: "led-blink",
        source: include_str!("../examples/led-blink.ml"),
        interactive: false,
        description: "Drive the COR24 LED via `led_on` / `led_off`. Browser stubs the LED.",
    },
    Demo {
        name: "led-toggle",
        source: include_str!("../examples/led-toggle.ml"),
        interactive: false,
        description: "Read the COR24 switch and reflect it on the LED.",
    },
    Demo {
        name: "let-destructure",
        source: include_str!("../examples/let-destructure.ml"),
        interactive: false,
        description: "Destructuring `let (a, b) = ...`, `let h :: t = ...`, and friends.",
    },
    Demo {
        name: "list-module",
        source: include_str!("../examples/list-module.ml"),
        interactive: false,
        description: "`List.length`, `List.rev`, qualified-name lookups.",
    },
    Demo {
        name: "lists",
        source: include_str!("../examples/lists.ml"),
        interactive: false,
        description: "List literals, cons, head/tail, is_empty.",
    },
    Demo {
        name: "lists-pairs-demo",
        source: include_str!("../examples/lists-pairs-demo.ml"),
        interactive: false,
        description: "Sum, length, map, swap, countdown — lists + pairs in one program.",
    },
    Demo {
        name: "multi-arg",
        source: include_str!("../examples/multi-arg.ml"),
        interactive: false,
        description: "Multi-argument curried function via `fun x y -> ...`.",
    },
    Demo {
        name: "named-adts",
        source: include_str!("../examples/named-adts.ml"),
        interactive: false,
        description: "Sum types via `type T = C1 | C2 | ...` and `match` expressions.",
    },
    Demo {
        name: "options",
        source: include_str!("../examples/options.ml"),
        interactive: false,
        description: "The built-in `option` type: `None` and `Some x`.",
    },
    Demo {
        name: "pairs",
        source: include_str!("../examples/pairs.ml"),
        interactive: false,
        description: "Tuple construction with `fst` / `snd` accessors.",
    },
    Demo {
        name: "patterns",
        source: include_str!("../examples/patterns.ml"),
        interactive: false,
        description: "Pattern matching across lists, tuples, options, and literals.",
    },
    Demo {
        name: "print",
        source: include_str!("../examples/print.ml"),
        interactive: false,
        description: "`print_int` and `putc` writing through the UART.",
    },
    Demo {
        name: "repl-session",
        source: include_str!("../examples/repl-session.ml"),
        interactive: true,
        description: "Multi-expression REPL session -- type more lines after the seed runs. \
                      Each input must be a complete expression: `let x = 42 in x`, not bare \
                      `let x = 42`.",
    },
    Demo {
        name: "sequencing",
        source: include_str!("../examples/sequencing.ml"),
        interactive: false,
        description: "Semicolon-sequenced expressions evaluated left-to-right.",
    },
    Demo {
        name: "strings",
        source: include_str!("../examples/strings.ml"),
        interactive: false,
        description: "String literals, `^` concatenation, `print_endline`, `String.length`.",
    },
    Demo {
        name: "text-adventure",
        source: include_str!("../examples/text-adventure.ml"),
        interactive: true,
        description: "Interactive text adventure: navigate rooms, pick up items. \
                      Commands: look, inventory, take, n/s/e/w, quit.",
    },
    Demo {
        name: "when-guards",
        source: include_str!("../examples/when-guards.ml"),
        interactive: false,
        description: "`match ... when <guard>` clauses for conditional pattern arms (abs, sign).",
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
            interactive.contains(&"repl-session"),
            "repl-session must be interactive"
        );
        assert!(
            interactive.contains(&"text-adventure"),
            "text-adventure must be interactive"
        );
    }
}
