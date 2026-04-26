# Fix text-adventure 'take' bug

The text-adventure demo lets you take the lamp infinitely: each
`take` in the Cave adds another Lamp to inventory and the cave's
description still says "A lamp lies here." Same bug for the Garden
key.

Root cause (line 3 of examples/text-adventure.ml):
  - `pickup = function Cave -> Some Lamp | Garden -> Some Key | _ -> None`
    is a pure function of room with no notion of "already taken".
  - `describe Cave` unconditionally prints "A lamp lies here."
  - Loop state is (room, inventory) -- no flags for what's been
    picked up.

Fix: extend state to a 4-tuple (room, inventory, lamp_taken,
key_taken). Thread the booleans through describe/pickup, branch
descriptions on lt/kt, return None from pickup if the matching
flag is set, set the flag in the take handler.

The CLI source `../sw-cor24-ocaml/tests/demo_adventure.ml` has the
same bug. Web fix is via LOCAL_OVERRIDE; the CLI fix is a separate
follow-up flagged for the sw-cor24-ocaml agent.

## Tasks

1. Add `text-adventure` to `LOCAL_OVERRIDE` in
   `scripts/sync-demos.sh` with a comment block citing this bug
   fix as the divergence rationale.

2. Hand-edit `examples/text-adventure.ml` (line 3) to apply the
   4-tuple state fix. The file is one logical REPL expression,
   so the whole new source must remain one line. Use
   `let (r, inv, lt, kt) = st in` for state destructure (the
   tuple-arity demo proves 4-element tuple patterns work).
   Initial seed becomes `loop (Cave, [], false, false)` and
   the bootstrap describe call becomes
   `describe Cave false false`.

3. cargo test -- `interactive_demos_reach_awaiting_input` must
   still see text-adventure reach awaiting_input.

4. Commit as `fix(demos)`.

5. scripts/build-pages.sh; commit as chore(pages); push.

This is a focused bug fix. Do not expand scope.