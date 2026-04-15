# Strip REPL prompt + echoed source from displayed output

The OCaml interpreter (`ocaml.pas`) explicitly echoes every input
character before evaluating, and prints `> ` as a prompt before each
read. So the raw UART stream for `print_int 42` (one-shot) is:

```
PVM OK
> print_int 4242
```

That is: pvm boot banner, then prompt, then the echoed 12 source
bytes, then immediately the `42` result with no separator. The user
already sees the source in the editor pane; the echo is redundant
in the browser and -- worse -- visually fuses with the result on
the same line.

## Fix

Extend `Session::clean_output()` to recognize and elide the
prompt-then-echo segments, while keeping result text intact.

The shape we strip:
- `PVM OK` line(s) at the top (already done).
- Lines or line fragments matching `^> <text>` where `<text>` is
  bytes from the source we sent. The line ends at `\n` (or end of
  stream, for the no-newline one-shot case).

Algorithm: scan the raw UART output; whenever we are at a line
boundary and see `> ` (prompt + space), enter "echo mode" -- skip
characters until either a newline (consumed and dropped) or end of
stream. Resume normal mode after each echo segment. Anything not
inside an echo segment passes through.

This handles:
- One-shot `print_int 42` -> raw `> print_int 4242` -> cleaned `42`.
- Multi-line `repl-session` -> each `> <line>` echo segment is
  dropped, leaving only the result lines.

After the echo strip, also drop:
- `PVM OK` lines (already in current implementation).
- `HALT` lines.
- Trailing CR (`\r`) chars that pvm/Pascal write as part of CRLF.
- Empty lines after stripping.

## Validation

- New unit test: `Session::new("print_int 42")`, run to halt, assert
  `clean_output() == "42"` (not `"> print_int 4242"`).
- Existing `print_int_42` test still passes.
- `cargo test --lib` clean.
- `trunk build` clean.

## Out of scope

- Changing the upstream OCaml interpreter to suppress its own echo.
- Stripping in `output()` (raw stays raw -- only `clean_output()`
  changes).
