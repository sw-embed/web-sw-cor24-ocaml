# Saga: refresh upstream, expand demos, add hardware panel

User-driven follow-up after saga 2 review. Three buckets, three steps.

## Why

1. **Stale upstream**. `assets/ocaml.p24m` was vendored at 30961 B
   from a build that predates three new language features upstream:
   strings (`String.length`, `print_endline`, concat), function-form
   `let f x y = body`, and named ADTs (`type T = C1 | C2`). Latest
   image is 39870 B. `asm/pvm.s` is unchanged vs vendored copy --
   the VM didn't move.
2. **Missing demos**. Upstream `tests/` has 21 `.ml` files not in
   `scripts/sync-demos.sh`'s mapping. Most are basic eval cases
   redundant with what we already ship; six are
   user-interesting demos exercising the new language features:
   `eval_strings`, `eval_named_adts`, `eval_function_form_let`,
   `eval_options`, `demo_patterns`, `eval_let_destructure`. Skip the
   `lex_*` regression tests entirely.
3. **No hardware panel**. Every other `web-sw-cor24-*` project that
   uses `cor24-emulator` exposes a small floating panel with the S2
   switch (clickable to toggle) and the D2 LED indicator (driven by
   `emu.is_led_on()`). The `led-toggle` demo is unusable without it
   -- and even with it, the demo currently traps with `EVAL ERROR`
   because the OCaml interp's `loop ()` recursion isn't tail-call
   optimised, so the call stack overflows after a few thousand
   iterations. Need to either rewrite the demo to a finite loop or
   document the boundedness explicitly.

## Architecture for the hardware panel

Mirror `../web-sw-cor24-apl/src/hardware.rs`:

- New `src/hardware.rs` defines a `HardwarePanel` function component
  with props `{ led_on, s2_on, on_s2_toggle }`. Initial scope drops
  TX/RX byte display (we don't need it for OCaml's REPL UX; can
  add later if useful).
- `App` gains two state fields `led_on: bool`, `s2_on: bool` and a
  `Msg::ToggleS2` variant.
- `Session` gains:
  - `set_switch(&mut self, on: bool)` -- forwards to
    `emu.set_button_pressed(on)`. Pushed once per tick before
    `run_batch`.
  - `led_on(&self) -> bool` -- returns `emu.is_led_on()`.
- Tick loop: before each inner batch, push s2; after each inner
  batch, sample led; if changed, dispatch a render.
- CSS: port `.hw-panel`, `.hw-led*`, `.hw-switch*`, `.hw-row`,
  `.hw-label`, `.hw-title` from APL's `app.css`. Floating panel,
  fixed top-right under the octocat (or bottom-right -- pick one
  that doesn't collide with the corner SVG; APL has it floating).

## Out of scope

- TX/RX byte indicators in the hardware panel (APL has them; defer
  for OCaml since UART is the source channel and a single byte
  display is misleading for line-buffered REPL input).
- Auto-refresh upstream artifacts in CI.
- Rewriting `pvm.s` or the OCaml interpreter.
- Tail-call optimisation in the interp.
