# Saga: live-demo perf and UX fixes

After the v1 saga shipped a working live demo on http://localhost:9735,
user review surfaced three issues that need follow-up work:

1. **Slowness** — single-byte-per-tick UART feeding makes the OCaml
   interpreter spend most of its cycles in a busy-poll wait for the
   next source byte. For a 13-byte one-shot program (e.g.
   `print_int 42` + EOT) that's ~13 million cor24 instructions of
   busy-waiting before any real work happens.
2. **Same-line output** — the OCaml REPL (in `ocaml.pas` `lex_init`)
   echoes every input character to UART before evaluating, so the
   browser sees `"> print_int 4242"` (prompt + echoed source + result
   on one line). The CLI's awk pipeline doesn't separate them either,
   but for a web UI the echo is redundant (the user already sees the
   source in the editor).
3. **Errors in some demos** — at least one demo trips. Need to
   triage demo-by-demo.

Fix in three steps; keep each commit independent so any can be
reverted on its own.

## Approach

- **Slowness fix**: in `Session::tick()`, interleave UART feeding
  with small inner batches. Use `emu.read_byte(IO_UARTSTAT) & 0x01`
  to detect when the RX register is empty, push the next byte
  immediately, then continue the batch. Inner batch ~50k cor24
  instructions; outer cap stays at the existing per-tick budget.
- **Echo-strip**: extend `clean_output()` to recognize the REPL's
  prompt-then-echoed-line pattern (`"> ...\n"`) and elide it. Keep
  the result lines.
- **Demo triage**: walk the 13 demos in the running app, note which
  fail, fix the underlying issue (likely in source delivery, the
  awaiting-input heuristic, or the budget escalator UX). Document
  per-demo behavior in `docs/demos.md` if it changed.

Out of scope:
- Restructuring the UI's run loop into a worker thread.
- Replacing the OCaml REPL's echo behavior in the upstream
  Pascal interpreter (would diverge from the CLI).
