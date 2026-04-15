# Speed up source delivery via opportunistic UART feeding

The OCaml interpreter (running on `pvm.s` in `cor24-emulator`) reads
its source byte-by-byte through the UART. The current `Session::tick()`
in `src/runner.rs` drains at most one queued RX byte per tick (1M
cor24 instructions). That means every source byte sits ~1M cycles in
the interpreter's UART poll loop before the next byte arrives. For a
13-byte one-shot program (e.g. `print_int 42` + EOT) we waste ~13M
instructions of busy-wait before any real work.

## Fix

Inside `tick()`, replace "drain at most one byte then run a full
batch" with an inner loop:

1. While the UART RX register is empty (`emu.read_byte(0xFF0101) &
   0x01 == 0`) and the queue has bytes: pop and `send_uart_byte`.
2. `emu.run_batch(INNER_BATCH)` (e.g. 50_000 cor24 instrs).
3. Accumulate `instructions_run` into the tick's running total.
4. If a non-`CycleLimit` `StopReason` came back, break out and
   handle it as before.
5. Loop until the per-tick total reaches `DEFAULT_BATCH` or done.

Add a private helper `fn rx_ready(&self) -> bool` that wraps the
status-port read so it's testable.

Constants:
- `INNER_BATCH = 50_000` -- tight enough to keep RX from starving,
  loose enough to amortize batch overhead.
- Keep `DEFAULT_BATCH = 1_000_000` as the per-tick total budget.

The `seeding_source` flag (used by `is_awaiting_input()`) becomes
false as soon as the queue empties; that's still right. After the
queue empties, the inner loop just runs straight `run_batch` calls
without any feed work.

## Validation

- `cargo test --lib` still passes (the existing `print_int_42` test
  exercises the new path; instruction count should drop noticeably
  -- jot the new figure in the commit message).
- `trunk build` clean.
- No new public API surface; just a behavior change inside `tick()`.

## Out of scope

- Worker-thread offload of the run loop.
- Yew re-render throttling (separate concern; address only if user
  still reports slowness after this fix).
