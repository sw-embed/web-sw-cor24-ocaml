# Retroactively tag v0.1.0 at commit 4406834

## Tasks

1. Verify the target commit:
       git show --stat 4406834 | head
   Expected: "Add README, demo docs, and CHANGES; fix Unicode tick",
   author Mike Wright, date 2026-04-15. CHANGES.md is added in
   this commit with the v0.1.0 entry.

2. Annotated tag at that commit:
       git tag -a v0.1.0 4406834 -m "v0.1.0 -- 2026-04-15 -- Initial release. 13 demos, Yew+Trunk SPA, cor24-emulator+pvm.s runner. Tagged retroactively on 2026-04-26."

3. Push the tag:
       git push origin v0.1.0

4. Extract the v0.1.0 section of CHANGES.md as release notes:
       awk '/^## v0\.1\.0/{flag=1; next} /^## /{flag=0} flag' \
           CHANGES.md > /tmp/v0.1.0-release-notes.md

5. Create the GitHub release pointing at the historical commit:
       gh release create v0.1.0 \
         --title "v0.1.0 -- 2026-04-15" \
         --target 4406834 \
         --notes-file /tmp/v0.1.0-release-notes.md

6. agentrail complete --done.

7. Trailing .agentrail/ commit per project convention.

8. git push origin main (the trailing commit only; no source
   changes in this step).

Do NOT push --force or skip hooks. The tag is non-destructive --
it adds a ref pointing at an existing commit; it does not modify
history.