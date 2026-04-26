# Refresh docs/demos.md with the 8 missing demo entries

`docs/demos.md` has one section per demo entry in `src/demos.rs`,
showing source + expected output. The doc has fallen behind: the
8 demos added in the two recent sagas
(refresh-interp-add-seven-demos + add-modules-demo-and-multifile-plan)
have no entries yet.

## Tasks

1. Read the existing `docs/demos.md` end-to-end so new entries
   match the established format exactly:
       ## <name>
       <one-paragraph description, what it's exercising>
       ```ocaml
       <source from examples/<name>.ml>
       ```
       **Expected output:**
       ```
       <what the live UI prints>
       ```

2. Add per-demo entries (alphabetised in the same order as
   src/demos.rs) for:

   - modules
   - string-conversion
   - string-equality
   - string-escapes
   - tco-countdown
   - toplevel-let
   - tuple-arity
   - variants-with-payload

   Each entry must slot into the existing alphabetised section
   ordering (modules between list-module/lists-pairs-demo and
   multi-arg; string-* between sequencing and strings; tco-*
   between strings and text-adventure; etc.). Match the order in
   src/demos.rs.

3. Source comes from `examples/<demo>.ml` verbatim. For the
   `modules` entry, include the inline `(* ... *)` comments --
   they are part of the demo's pedagogy and should appear in the
   doc.

4. Expected output: derive from the demo's logic. For numeric
   results (e.g. tco-countdown counting from 100, string-conversion
   round-trips), trace through manually. For demos ending in a
   deliberate EVAL ERROR (modules), include that line in the
   expected output and explain in the surrounding paragraph that
   the error is the namespace-isolation punchline.

5. If genuinely uncertain about exact output for any demo, add it
   to a list at the bottom of this step's commit message rather
   than guessing -- correctness matters here.

6. Commit as `docs(demos): add entries for 8 recent demos`.

Stop after committing. No pages rebuild needed -- docs/ is not
served via GitHub Pages.