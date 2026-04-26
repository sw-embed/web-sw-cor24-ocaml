# Retroactively tag v0.1.0

CHANGES.md has carried a `## v0.1.0 -- 2026-04-15` heading since
commit 4406834 (CHANGES.md was introduced there with the v0.1.0
entry). v0.2.0 was tagged on 2026-04-26 from main HEAD, but
v0.1.0 was never an actual git tag -- only a CHANGES.md heading.
This saga creates the missing tag retroactively, pointing at
4406834, so `git tag -l` includes both releases and history is
self-consistent.

## Step

### 001-tag-v0_1_0

- Annotated tag `v0.1.0` at `4406834` with a message summarising
  the v0.1.0 release (initial v1 saga delivery).
- Push the tag to origin.
- Create a GitHub release for v0.1.0 with notes pulled from the
  v0.1.0 section of CHANGES.md. Mark it as historical
  (--target 4406834 ensures the release points at the right
  commit).
