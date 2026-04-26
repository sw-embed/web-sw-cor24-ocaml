# Surface 16 canonical_* one-liners as a "Minimal examples" tier

The CLI has 16 `canonical_*.ml` files in `tests/` that are tiny
one-line demonstrations of single language features (literal,
arith, lambda app, let-in, list map/filter/fold, match, when
guard, etc.). They were deliberately skipped in earlier sagas
because they overlap with the existing substantive demos. This
step surfaces them as a separate "Minimal examples" tier in the
dropdown using HTML <optgroup> grouping.

## Tasks

1. Read all 16 `../sw-cor24-ocaml/tests/canonical_*.ml` files to
   confirm they are still single-line snippets and capture their
   sources.

2. Extend the data model in `src/demos.rs`:

   ```rust
   #[derive(Clone, Copy, PartialEq)]
   pub enum DemoCategory {
       Standard,
       Minimal,
   }

   pub struct Demo {
       // ... existing fields ...
       pub category: DemoCategory,
   }
   ```

   Default `category: DemoCategory::Standard` for the existing
   35 demos. Tests/UI dropdown can group by category.

3. Vendor the 16 canonical demos. Either:
   - Add a `MINIMAL_MAPPING` table to `scripts/sync-demos.sh`
     (parallel to `MAPPING`) for `canonical_*` files. Names like
     `canonical-arith`, `canonical-fact`, `canonical-fib`, etc.
     (or `min-arith`, `min-fact` for shorter labels -- pick
     whichever reads better in the dropdown).
   - Run the script to copy them to `examples/canonical-*.ml`.

4. Add 16 entries to `DEMOS` in `src/demos.rs` with
   `category: DemoCategory::Minimal`. Keep the alphabetical
   ordering invariant -- they all sort under `c` so they cluster.
   Each gets a one-line description naming the feature.

5. Update the dropdown rendering in `src/lib.rs` (or the
   relevant component) to use Yew's HTML `<optgroup>`:
       <select>
         <optgroup label="Standard demos">
           <option ...>echo-loop</option>
           ...
         </optgroup>
         <optgroup label="Minimal examples">
           <option ...>canonical-arith</option>
           ...
         </optgroup>
       </select>
   The Demo dropdown is currently a flat <select>; introducing
   <optgroup> requires partitioning DEMOS by category at render
   time.

6. Update `docs/demos.md`:
   - Add a "Minimal examples" section near the top (after the
     intro paragraph) listing all 16 with their source one-liners
     and expected output. These are short -- a small table or
     terse format works better than the per-demo full sections.

7. Tests:
   - The `every_non_interactive_demo_halts_cleanly` test should
     pick up the new demos automatically and confirm they all
     halt cleanly.
   - Add a unit test that DEMOS is still alphabetised (already
     exists) and that `category` field works as expected (e.g.
     count Minimal vs Standard).

8. `cargo test` -- all should pass. Capture cleaned outputs via
   `--nocapture` to populate the docs/demos.md expected outputs
   accurately.

9. Commit as `feat(demos): add 16 canonical minimal examples
   under a "Minimal examples" optgroup`.

10. `scripts/build-pages.sh` and commit as `chore(pages)`.

11. `git push origin main`.

Stop after the push. Step 003 (multi-file Phase 2 ModuleEditor)
is the next.

## Naming convention

Pick ONE of these and use it consistently. Don't mix:
   Option A: `canonical-arith`, `canonical-fact`, ...
   Option B: `min-arith`, `min-fact`, ...

Option A is more discoverable; option B is shorter in the
dropdown. Lean toward A unless there's a strong reason
otherwise.