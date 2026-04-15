# Dev server and pages build scripts

Port the build/serve scripts from `../web-sw-cor24-basic/scripts/` so
this project has the same one-command dev loop and release pipeline.

## Prerequisites

Yew app step complete (the scripts have something useful to serve).

## Work

1. Copy `scripts/serve.sh` from the BASIC project, adjusting:
   - `PORT=9735` (reserved for this project).
   - Preserve the `mkdir` lock under `target/.trunk-dist.lock` — we
     want the same race protection.
2. Copy `scripts/build-pages.sh` from the BASIC project, adjusting:
   - `trunk build --release --public-url /web-sw-cor24-ocaml/`.
   - Same dist lock as serve.sh.
3. Add `pages/.nojekyll` so GitHub Pages does not apply Jekyll
   filtering to the wasm/js assets.
4. Extend `.gitignore` to keep `pages/.nojekyll` tracked while
   ignoring the rest of `pages/` (so we can regenerate without
   committing intermediate builds unless we want to publish).

## Validation

- `./scripts/serve.sh` launches on port 9735 and the app loads.
- Running `./scripts/serve.sh` twice in parallel errors out cleanly
  from the second instance (lock works).
- `./scripts/build-pages.sh` produces a `pages/` directory with
  `index.html`, wasm bundle, CSS, and `.nojekyll`.

## Out of scope

- GitHub Actions for automatic deploy.
- Custom domain configuration.
