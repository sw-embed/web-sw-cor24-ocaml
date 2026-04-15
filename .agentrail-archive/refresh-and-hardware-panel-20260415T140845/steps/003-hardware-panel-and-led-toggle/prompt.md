# Hardware panel (S2 switch + D2 LED) and led-toggle fix

User: "The UI is missing the hardware panel that has Switch S2 and
LED D2 like other ../web-sw-cor24-* repos do; led-toggle demo fails
with error."

## Work

### 1. New `src/hardware.rs`

Port `../web-sw-cor24-apl/src/hardware.rs` -- a function component
`HardwarePanel` with props `{ led_on: bool, s2_on: bool,
on_s2_toggle: Callback<()> }`. Drop the TX/RX byte indicators APL
shows -- not useful for OCaml's REPL UX. Keep the markup classes
(`hw-panel`, `hw-title`, `hw-row`, `hw-label`, `hw-led{on,off}`,
`hw-switch{on,off}`) compatible with APL's CSS so the port is
mechanical.

### 2. CSS additions to `src/ui.css`

Append rules ported from `web-sw-cor24-apl/src/app.css` lines
137-227 (hw-panel + child rules), adapted to our color palette
(`--accent` for the LED-on glow / S2-on background instead of
APL's `--green` / `--blue`). Drop the TX/RX rules. Adjust
positioning so the panel doesn't collide with the octocat corner
(top-right is taken by the corner; place at `bottom: 16px;
right: 16px;`).

### 3. `src/runner.rs` API additions

Two thin wrappers on `Session`:

```rust
pub fn set_switch(&mut self, on: bool) {
    self.emu.set_button_pressed(on);
}
pub fn led_on(&self) -> bool {
    self.emu.is_led_on()
}
```

Forward to `cor24-emulator`'s already-public methods. No state to
duplicate. Call `set_switch` once at the top of each `tick()` so
the cor24 switch register reflects the current UI state going into
the batch.

### 4. `src/lib.rs` App wiring

- New `App` fields: `s2_on: bool`, `led_on: bool` (both initialised
  to `false`).
- New `Msg::ToggleS2`.
- `Msg::Tick` handler: after `session.tick()`, sample
  `session.led_on()`; if it differs from `self.led_on`, update and
  return `true` to re-render.
- `Msg::Reset` resets both flags.
- `view()` renders `<HardwarePanel led_on=self.led_on
  s2_on=self.s2_on on_s2_toggle=ctx.link().callback(|_|
  Msg::ToggleS2) />` once, after the existing `<main>` block but
  before the `<footer>` (so the panel floats over the workspace).

Add `pub mod hardware;` to `lib.rs`.

### 5. Fix `led-toggle`

The current source is a non-tail-recursive infinite loop:
```ocaml
let rec loop = fun u -> let s = switch () in set_led s; loop () in loop ()
```
With the new image it traps as `TRAP 4` instead of `EVAL ERROR`,
but either way the demo is "broken-looking". Two paths:

a. Rewrite as a bounded loop -- read switch N times so it doesn't
   overflow the interp's call stack. With the hardware panel the
   user can toggle S2 and re-run to see different LED behavior.
b. Keep infinite, document the trap.

Pick (a). The replacement source (one-line so the existing
collapse rule doesn't apply):

```ocaml
let rec loop = fun n -> if n = 0 then 0 else (let s = switch () in set_led s; print_int (if s then 1 else 0); loop (n - 1)) in loop 8
```

8 iterations: each reads S2, drives the LED, prints 0 or 1. Run
with S2 off -> `00000000`; toggle S2 mid-run for a mix. Persist
this by hand-editing `examples/led-toggle.ml` directly (not synced
from CLI -- it would diverge if the CLI demo changes; document the
divergence in `scripts/sync-demos.sh`).

To prevent the next `sync-demos.sh` run from clobbering this:
either remove `demo_led_toggle:led-toggle` from the MAPPING, or
add a `LOCAL_OVERRIDE` allowlist that skips the copy. Pick the
allowlist (preserves the doc trail of where the demo originated).

### 6. Update docs

- `docs/demos.md`: rewrite the led-toggle section -- new source,
  expected output `0\n0\n0\n0\n0\n0\n0\n0` (S2 off baseline);
  note that toggling S2 in the panel between runs changes the
  output.
- `README.md`: no change needed (description still accurate).

## Validation

- `cargo test --lib` passes (existing tests + any added).
- `cargo test --test demos` passes; `led-toggle` cleaned output is
  `"0\n0\n0\n0\n0\n0\n0\n0"`.
- `trunk build` clean.
- markdown-checker passes on user-facing docs.
- `./scripts/serve.sh` brings up http://localhost:9735 with the
  hardware panel visible bottom-right; clicking S2 toggles the
  switch; running `led-blink` lights D2 momentarily; running
  `led-toggle` prints `0 0 0 0 0 0 0 0` with S2 off, prints `1`s
  with S2 on (verify by toggling S2 then Run).

## Out of scope

- TX/RX byte indicators in the panel.
- Animation of the LED on each transition.
- Per-tick s2_on push optimization (just push every tick;
  cor24-emulator's set_button_pressed is cheap).
