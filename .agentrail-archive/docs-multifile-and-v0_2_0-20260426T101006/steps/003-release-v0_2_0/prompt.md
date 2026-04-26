# Cut v0.2.0 release

Collapse the giant `[Unreleased]` section in CHANGES.md into a
v0.2.0 release boundary, tag the commit, push the tag, and create
a GitHub release. v0.1.0 was tagged 2026-04-15. v0.2.0 covers
everything since: live-demo polish, guess + interactive I/O,
refresh-interp + 7 demos, modules demo + multi-file plan,
modules-demo annotation fix, docs/demos.md refresh, and the
multi-file Phase 1 data layer.

## Tasks

1. Sanity check the current state:
       git tag -l                     # confirm v0.1.0 exists, v0.2.0 does not
       git log --oneline v0.1.0..HEAD # full set of commits being released

2. Edit `CHANGES.md`:
       - Replace `## [Unreleased]` with `## v0.2.0 -- 2026-04-26`.
       - Above it, insert a new empty `## [Unreleased]` heading
         with a single line under it (e.g. `_No unreleased
         changes._` or just an empty line).
       - Optional: extend the v0.2.0 notes with the two items
         landed by this saga's earlier steps (docs/demos.md
         refresh, multi-file Phase 1 -- AuxFile + runner concat
         + modules-multifile demo).

3. Commit as `release: v0.2.0`.

4. Annotated tag:
       git tag -a v0.2.0 -m "v0.2.0 -- 2026-04-26 -- $(brief one-liner)"

5. Push:
       git push origin main
       git push origin v0.2.0

6. Create the GitHub release. Pull the body from CHANGES.md's
   v0.2.0 section:
       gh release create v0.2.0 \
         --title "v0.2.0 -- 2026-04-26" \
         --notes "$(<v0.2.0 section body>)"

   Strip leading-section markdown to make the release body read
   well on GitHub: the section heading itself becomes the
   release title, so the body should start with the per-saga
   subheadings.

7. agentrail complete --done.

8. Trailing .agentrail/ commit per project convention.

## Notes

- Do NOT push --force or skip hooks under any circumstances.
- The release is reproducible: anyone checking out v0.2.0 should
  see the same pages/ artifacts that the deploy serves. The
  trailing .agentrail commit lands AFTER the tag, which is fine
  -- the tag points at the release commit, not the housekeeping.