# Port the Yew App component (re-instates step 005)

Re-instates the original step 005 against the new emulator-backed
`Session` API. Original prompt at
`.agentrail/steps/005-port-yew-app/prompt.md` — same intent, same
validation; only the runner the App talks to has changed (still
exposes `new`, `new_interactive`, `tick`, `is_done`, `is_halted`,
`stop_reason`, `instructions`, `output`, `clean_output`,
`feed_input`, `is_awaiting_input` — the BASIC App's wiring drops in).

## Work

Per the original step 005 (verbatim):

1. Port `App`, `Msg`, `Component` impl, and `view` from
   `../web-sw-cor24-basic/src/lib.rs`. Adjust:
   - `h1` title -> `"web-sw-cor24-ocaml"`.
   - GitHub corner link ->
     `https://github.com/sw-embed/web-sw-cor24-ocaml`.
   - Source textarea label -> `"source (.ml)"`.
   - `DEFAULT_MAX_INSTRS` -> `500_000_000` to match the CLI script.
   - Render `Demo.description` as a small caption under the picker
     (the OCaml `demos.rs` carries `description`; BASIC's doesn't).
   - Footer links: MIT / copyright / MakerLISP / Blog / Discord /
     YouTube; "Demo Documentation" -> this repo's `docs/demos.md`;
     "Changes" -> this repo's `CHANGES.md`.
2. Keyboard: Cmd/Ctrl-Enter = Run; Esc = Stop; Enter in the input
   row = submit input.
3. Adapt the run-loop's instruction budget for the heavier OCaml
   workload but keep the BASIC project's "Increase budget 4x" UX.
4. **Output cleaning**: use `Session::clean_output()` for display
   (strips pvm boot banner / blank lines) so users see just the
   program's output, not pvm chatter.

## Validation

- `cargo check` clean.
- `trunk build` succeeds.
- `cargo test --lib` still passes (4 tests from runner + demos).
- (Defer browser smoke test to the final saga step per the
  no-mid-saga-server rule.)

## Out of scope

- CSS polish (separate step).
- Per-demo doc pages.
