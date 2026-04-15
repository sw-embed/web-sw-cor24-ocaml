# README and demo documentation

Write user-facing docs so a first-time visitor knows what this repo is
and how to run it locally.

## Prerequisites

Scripts step complete (we have working `serve.sh` / `build-pages.sh`
to document).

## Work

1. Write `README.md` at the repo root covering:
   - One-paragraph summary: browser-based OCaml interpreter running on
     the COR24 p-code VM, ported from the CLI in
     `../sw-cor24-ocaml`.
   - Quickstart: `./scripts/vendor-artifacts.sh`, `./scripts/serve.sh`,
     open `http://localhost:9735`.
   - List of demos with one-line descriptions.
   - Link to `../sw-cor24-ocaml` for language reference and REPL.
   - Build/deploy section referencing `scripts/build-pages.sh`.
   - License + copyright.
   - Screenshot placeholder (capture later).
2. Write `docs/demos.md` with one section per demo: name, what it
   shows, annotated source, expected output. Start from the demo
   sources in `examples/` and the descriptions in `src/demos.rs`.
3. Write `CHANGES.md` with a single initial entry for v0.1.0
   (scaffold + port of demos from CLI).
4. Run `markdown-checker -p . -f "**/*.md"` and fix any ASCII
   violations.

## Validation

- `markdown-checker` passes clean across all markdown files.
- README renders on GitHub without Unicode warnings.
- `docs/demos.md` has one section for every entry in `DEMOS`.

## Out of scope

- A contribution guide (future if the project grows).
- Tutorial content beyond the per-demo explanations.
