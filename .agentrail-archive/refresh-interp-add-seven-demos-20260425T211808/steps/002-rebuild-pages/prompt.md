# Rebuild pages with refreshed interpreter and 7 new demos

The previous step (001-vendor-and-add-demos) refreshed
assets/ocaml.p24m and added 7 new demos to src/demos.rs. The
pages/ release artifacts are stale -- they still embed the old
interpreter and only the original 26 demos.

## Tasks

1. Run scripts/build-pages.sh. It takes the trunk dist/ lock,
   runs `trunk build --release --public-url /web-sw-cor24-ocaml/`,
   then rsyncs dist/ -> pages/ (preserving .nojekyll).

2. Verify pages/ contains the rebuilt artifacts (check `ls pages/`
   for fresh timestamps on index.html and the *.wasm/.js/.bin
   bundle).

3. Commit as `chore(pages): rebuild release artifacts with 7 new
   demos and refreshed interpreter` covering everything under
   pages/.

4. After agentrail complete --done, do the trailing
   .agentrail/ commit per the project's convention.

Do NOT start `trunk serve` -- per the user's stored preference,
dev server only runs after the saga is fully complete and only at
the user's request.