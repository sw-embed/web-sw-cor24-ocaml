# Refresh help modal Language Reference for v0.2.0 features

The in-UI help modal's "Language Reference" tab predates v0.2.0
and is missing coverage for features shipped since v0.1.0. This
step extends the modal text to cover them. Pages rebuild + push at
the end so the live site picks up the new help content.

## Tasks

1. Find where the help modal text lives. Likely candidates:
   `src/lib.rs`, `src/components/`, or an HTML template in
   `index.html`. Search for distinctive existing strings like
   "Language Reference", "Built-ins", "Module-qualified".

2. Audit the existing reference content against the v0.2.0
   feature surface. Read CHANGES.md's v0.2.0 section for a full
   list. Items that are likely missing:

   - **Top-level `let`** without trailing `in`: `let f x = ...`
     as a top-level declaration (vs `let f x = ... in body`).
     Sugared multi-arg form. `let rec`. Tuple destructuring on
     LHS: `let (a, b) = (1, 2)`. Unit-pattern shorthand:
     `let () = side_effect ()`.
   - **Variant payloads**: `type t = TInt of int | TIdent of
     string | TLArrow | TEOF` -- constructors carry typed
     payloads alongside nullary ones.
   - **Multi-arity tuple patterns**: `match (1, 42, "hi") with
     (1, n, _) -> n | _ -> 0`. The match arm arity must equal
     the subject's. Note: REPL displays multi-element tuples as
     right-nested pairs `(1, 2, 3)` -> `(1, (2, 3))`.
   - **String escapes**: `\n`, `\t`, `\\`, `\"` inside `"..."`.
   - **`string_of_int` / `int_of_string`**: int <-> string
     conversion. `int_of_string` is lenient -- malformed input
     returns 0, no exception.
   - **String equality**: `=` and `<>` on strings (structural).
   - **Modules / namespaces**:
       - Single-file: `let __module = "Math"` directive marks
         subsequent definitions as module Math; later
         `let __module = "Main"` switches modules. Cross-module
         unqualified lookup fails -- use `Math.add 2 3`.
       - Multi-file (Phase 1, in v0.2.0): demos can ship aux
         `.ml` files. The runner concatenates them with
         synthesized `__module` directives. The web UI's main
         editor currently shows only the main file; aux file
         editing is Phase 2.
   - Note explicitly: there is **no** `try ... with` / `raise` /
     exceptions. The REPL just prints `EVAL ERROR` for the
     failing line and continues with the next.

3. Add these as new sections (or extend existing sections) in the
   Language Reference tab. Match the existing style and
   verbosity -- the modal is a quick reference, not a tutorial.
   Cite exact builtin names in backticks; show one-line examples
   where they fit; cross-reference relevant demos by name where
   helpful (e.g. "see `modules` demo").

4. Run `cargo test` and `cargo build --release` to confirm no
   regressions.

5. Commit as `feat(ui)`: a single commit covering the modal text
   changes.

6. `scripts/build-pages.sh` to rebuild release artifacts. Strings-
   check the new wasm contains a distinctive new phrase from the
   added content, e.g. `strings pages/*_bg.wasm | grep "__module"`.

7. Commit pages as `chore(pages): rebuild with refreshed help
   modal`.

8. `git push origin main`. The Pages action will redeploy.

Stop after the push. Step 002 (canonical minimal examples) is the
next.