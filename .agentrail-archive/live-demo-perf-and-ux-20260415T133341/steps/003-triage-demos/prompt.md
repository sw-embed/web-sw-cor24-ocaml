# Triage demos: per-demo end-to-end tests

User reported "some demos produce errors". Without a screenshot I
need to run each demo myself. Cheapest reliable path: a Rust
integration test that iterates `DEMOS`, runs every non-interactive
entry through the same `Session` the browser uses, and asserts each
one halts cleanly. Anything that traps or stalls becomes a concrete
debuggable failure.

## Work

1. Add `tests/demos.rs` (cargo integration test). For each
   non-interactive `Demo` in the public catalog:
   - Construct `Session::new(d.source)`.
   - Tick to halt with a generous max-tick budget (each demo bounded
     by `DEFAULT_BATCH * max_ticks` cor24 instructions).
   - Assert `s.is_done() && s.is_halted()`. On failure, print:
     - The demo name, source, raw output, cleaned output, instr count,
       stop_reason. (Whatever helps diagnose.)
2. Run the test. For each failing demo:
   - Diagnose. Likely candidates: budget too low (loops), the new
     trailing-`\n` injection conflicts with multi-line source,
     interpreter trapping on a primitive we expected (e.g.
     `set_led ()`).
   - Fix at the right level (runner, demos catalog, source clean-up,
     or document-as-budget-bound-by-design like `led-toggle`).
3. Update `docs/demos.md` if any demo's expected output / behavior
   changes.
4. Mark `led-toggle` as a known infinite loop in the catalog if it
   genuinely is (already documented in docs/demos.md). The test
   should skip it OR assert it hits the per-demo budget cap rather
   than halting -- whichever is more honest.

## Validation

- `cargo test` exits zero. Per-demo failures are itemized.
- For demos that legitimately don't halt (`led-toggle`), document
  the expected behavior in the test (skip, or `is_done()` via cycle
  cap) and in `docs/demos.md`.
- `trunk build` clean.

## Out of scope

- Interactive demo (`repl-session`) -- needs feed_input plumbing,
  separate from the static demos. Cover with a smoke test that goes
  far enough to reach `is_awaiting_input()`.
- Performance regressions: the prior step capped budget; if a demo
  needs more, bump per-demo `max_ticks` rather than DEFAULT_BATCH.
