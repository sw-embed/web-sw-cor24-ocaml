# Fix text-adventure prompt + add Hello, World demo

## Tasks

1. In src/lib.rs Tick handler, change
   `const PC_POLL_WINDOW: u32 = 64;` to
   `const PC_POLL_WINDOW: u32 = 16;`.
   Update the surrounding comment to note that 16 bytes is
   tight enough to require the actual read_line UART RX poll
   (a few-byte fetch+branch) and exclude parser warm-up.

2. Hand-write `examples/hello-world.ml`:
       print_endline "Hello, World!"

3. Add a Demo entry in src/demos.rs slotted alphabetically
   between `hello` and `higher-order-lists`:
       Demo {
           name: "hello-world",
           source: include_str!("../examples/hello-world.ml"),
           interactive: false,
           description: "Print the string \"Hello, World!\" via print_endline.",
           auxiliary_files: &[],
           category: DemoCategory::Standard,
       }

4. Update docs/demos.md by adding a `## hello-world` section
   right after the `## hello` section (matching the topical
   adjacent-to-related-demo flow):
       ## hello-world
       Variant of hello that prints a string instead of an integer.
       ```ocaml
       print_endline "Hello, World!"
       ```
       **Expected output:**
       ```
       Hello, World!
       ```

5. Update the existing test
   `text_adventure_seed_prints_cave_description_before_input_prompt`
   in tests/demos.rs to use PC_POLL_WINDOW = 16 (matching the
   App). Verify it now passes.

6. Run cargo test full suite. Critical to verify:
   - text-adventure seed test passes (cave description visible)
   - text_adventure_take_lamp_only_once still passes
   - interactive_demos_reach_awaiting_input still passes (all 4
     interactive demos: echo-loop, repl-session, text-adventure,
     guess)
   - every_non_interactive_demo_halts_cleanly still passes
   - hello-world is in the new test output as a non-interactive
     demo that halts cleanly

7. Commit as a single feat covering lib.rs window tightening +
   hello-world addition + docs + test:
       feat(ui,demos): tighten PC stability window; add hello-world demo

8. scripts/build-pages.sh; commit chore(pages); push origin main.

9. agentrail complete --done.

10. Trailing .agentrail commit + push.

## Important

If tightening the window to 16 breaks any of the other
interactive demos (echo-loop, repl-session, guess), fall back
to 24 or 32 -- whatever's tight enough to reject the parser
warm-up but loose enough to accept the read_line poll for all
4 demos. Document the chosen value in the comment.