# Port the Yew App component

Replace the placeholder `src/lib.rs` with the full Yew application,
ported from `../web-sw-cor24-basic/src/lib.rs` and adapted for the
OCaml runner and demos table.

## Prerequisites

- Runner step complete (`Session`, `feed_input`, `is_awaiting_input`).
- Demos step complete (`DEMOS`, `default_demo_index`).

## Work

1. Port `App`, `Msg`, `Component` impl, and `view` from the BASIC
   project. Adjust:
   - `h1` title -> `"web-sw-cor24-ocaml"`.
   - GitHub corner link -> `https://github.com/sw-embed/web-sw-cor24-ocaml`.
   - Source textarea label -> `"source (.ml)"`.
   - `DEFAULT_MAX_INSTRS` -> `500_000_000` (matches the CLI script's
     default -- OCaml interpretation is heavier than BASIC).
   - Render `Demo.description` as a small caption under the picker.
   - Footer links: keep MIT / copyright / MakerLISP / Blog / Discord /
     YouTube. Point "Demo Documentation" at this repo's
     `docs/demos.md` and "Changes" at this repo's `CHANGES.md`.
2. Keep keyboard shortcuts identical: Cmd/Ctrl-Enter = Run, Esc =
   Stop, Enter in input row = submit input.
3. The interactive-demo code path in the BASIC app already handles
   `awaiting_input`; it should work for the OCaml REPL session as-is
   once the runner exposes the same surface.

## Validation

- `trunk build` succeeds.
- `trunk serve --port 9735` loads the app; default demo (`hello`) runs
  and prints `42`.
- `factorial` demo runs and prints `120`.
- `repl-session` demo runs, reaches `awaiting_input`, accepts a line,
  and produces output.
- Cmd-Enter runs, Esc stops while running.

## Out of scope

- CSS polish (next step).
- Per-demo doc pages.
