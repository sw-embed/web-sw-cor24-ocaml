# Rebuild pages with modules demo

The previous step (001-add-modules-demo-and-write-plan) added the
'modules' demo and authored docs/multiple-file-demos-plan.md. The
pages/ release artifacts are stale -- they ship the 33-demo catalog
without 'modules'.

## Tasks

1. Run scripts/build-pages.sh.

2. Verify pages/ index.html and *.wasm/*.js have fresh timestamps
   and that strings(1) on the new wasm shows the modules demo
   source baked in (sanity check; optional).

3. Commit as `chore(pages): rebuild release artifacts with modules
   demo` covering everything under pages/.

4. agentrail complete --done.

5. Trailing .agentrail/ commit per project convention.

6. Push origin main so the GitHub Pages action picks up the deploy.

Do NOT start trunk serve.