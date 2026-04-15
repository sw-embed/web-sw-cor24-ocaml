# Port CSS styling and favicon

Port the visual design from `../web-sw-cor24-basic/src/ui.css` so the
OCaml live demo matches the look of the rest of the web-cor24 family.

## Prerequisites

Yew app step complete (`src/lib.rs` renders the full UI).

## Work

1. Copy `../web-sw-cor24-basic/src/ui.css` to `src/ui.css` (replacing
   the placeholder from the scaffold step).
2. Add rules for the new `.demo-description` caption beneath the demo
   picker (small font, muted color, single line with ellipsis).
3. Pick a distinguishing accent color different from the BASIC app so
   the two are visually separable (e.g. a teal or purple for the run
   button and status strip). Document the chosen accent hex in a
   short comment at the top of `ui.css`.
4. Generate a production favicon:
   `favicon -T -t "O" -b 6666ff -f ffffff` from the repo root (keep
   the placeholder from scaffold; overwrite if needed).

## Validation

- `trunk serve --port 9735` shows a styled page matching the BASIC
  app's general layout (header / controls / split workspace / footer)
  with the chosen accent color.
- Demo description caption renders under the picker and truncates
  gracefully on narrow widths.
- Favicon shows a blue "O" in the browser tab.

## Out of scope

- A full redesign -- this is a port, not a rethink.
- Dark-mode toggle.
