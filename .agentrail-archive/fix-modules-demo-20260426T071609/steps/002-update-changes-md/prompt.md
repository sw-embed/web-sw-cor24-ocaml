# Update CHANGES.md to reflect all recent commits

## Goal

CHANGES.md's [Unreleased] section is stale -- it stops at the
2026-04-15 'live-demo-readyup' work and doesn't mention any of the
sagas that landed since. Bring it up to date by summarising every
commit on `main` since the v0.1.0 tag in reverse chronological
order, grouped by saga / theme.

## Tasks

1. Run `git log --oneline v0.1.0..HEAD` (or just `git log --oneline | head -60`)
   to enumerate all commits past the last tagged release.

2. Edit `CHANGES.md` so the [Unreleased] section is a coherent
   summary of every commit since v0.1.0, grouped by saga in
   reverse chronological order. Suggested groupings (newest first):

   - fix-modules-demo (this saga, today): annotate modules demo
     with inline comments + corrective line; document LOCAL_OVERRIDE
     rationale.
   - add-modules-demo-and-multifile-plan: 'modules' demo (single-file
     namespaces); docs/multiple-file-demos-plan.md sketches phase
     1/2/3 multi-file UI modeled on web-sw-cor24-plsw.
   - refresh-interp-add-seven-demos: refreshed assets/ocaml.p24m
     45122->48231 bytes; added 7 demos (string-conversion,
     string-equality, string-escapes, tco-countdown, toplevel-let,
     tuple-arity, variants-with-payload).
   - add-guess-resync-adventure: add guess demo, refresh
     adventure/echo-loop + interp image.
   - earlier work prior to that already in CHANGES.md.

   Skim each commit's subject + body; one bullet per substantive
   change. Skip pure `chore(agentrail): ...` commits in the
   summary -- they are saga housekeeping. Pages-rebuild commits
   can be summarized as "release artifacts rebuilt" once per
   saga rather than line-by-line.

3. Keep style consistent with existing CHANGES.md entries (bullets,
   tone, level of detail).

4. Commit as `docs(changes): summarize recent sagas in [Unreleased]`.

Stop after committing. The pages rebuild + push is the next step
(now 003-rebuild-pages-and-push).