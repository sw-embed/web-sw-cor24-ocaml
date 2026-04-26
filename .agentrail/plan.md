# Fix text-adventure prompt + add Hello, World demo

Two small post-deploy issues the user reported:

1. text-adventure doesn't print its initial cave description before
   the input prompt appears. Root cause: the App's PC-stability
   detector (lib.rs Tick handler, PC_POLL_WINDOW = 64 bytes,
   SAMPLES_REQUIRED = 3) false-positives during parser warm-up of
   the longer text-adventure source. Parser PC range observed at
   31 bytes (e.g. 518..549) which is within 64 -- detector reports
   "settled", App stops ticking, seed never finishes printing.

   Fix: tighten PC_POLL_WINDOW to 16 so only the truly tight
   read_line UART RX poll loop (a 4-byte fetch+branch) matches.
   Verify all 4 interactive demos still reach awaiting_input
   correctly.

2. The current `hello` demo prints the integer 42, not "Hello,
   World!" the user requested a string variant. Add a new
   `hello-world` demo showing `print_endline "Hello, World!"`.

## Step

### 001-fix-prompt-add-hello-world

- Change `PC_POLL_WINDOW: u32 = 64` -> `16` in src/lib.rs Tick
  handler.
- Add `hello-world` demo: hand-written examples/hello-world.ml
  containing `print_endline "Hello, World!"`. Slot alphabetically
  between `hello` and `higher-order-lists` in src/demos.rs.
  category: Standard.
- Update docs/demos.md with the new entry near the `hello` entry.
- Add an integration test that runs text-adventure through the
  same App-style PC-stability loop (this test was prototyped as
  `text_adventure_seed_prints_cave_description_before_input_prompt`
  during diagnosis and is currently in tests/demos.rs but
  failing). Confirm it now passes with the tightened window.
- Run all tests including the slow text-adventure regression.
- Commit feat(ui) for the lib.rs change + feat(demos) for hello-
  world. Could be one combined commit since they ship together.
- Build pages, commit chore(pages), push.
