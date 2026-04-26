# Rebuild pages with annotated modules demo and push

The previous step (001-annotate-modules-demo) reworked
examples/modules.ml with inline comments + a corrective Math.add
line. The pages/ release artifacts still embed the old bare
modules demo source.

## Tasks

1. Run scripts/build-pages.sh.

2. Commit as `chore(pages): rebuild release artifacts with
   annotated modules demo`.

3. agentrail complete --done.

4. Trailing .agentrail/ commit per convention.

5. Push origin main so GitHub Pages action picks up the deploy.

Do NOT start trunk serve.