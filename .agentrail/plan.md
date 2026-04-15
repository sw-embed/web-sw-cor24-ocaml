# Saga: live-demo-readyup

Goal: finish wiring this repo to ship a public live demo like the
sibling web-sw-cor24-* repos (notably web-sw-cor24-basic). The Trunk
release build, `pages/` directory, `.nojekyll`, `build-pages.sh`, and
`serve.sh` are already in place from earlier sagas. What remains:

1. Demos are in sync with `../sw-cor24-ocaml/tests/` (and any newly
   interesting upstream tests are surfaced).
2. A GitHub Actions workflow publishes `pages/` to GitHub Pages on push
   to `main`, matching the pattern in `web-sw-cor24-basic`.
3. The `README.md` leads with a live-demo link and an inline
   screenshot, using `?ts=<ms-since-epoch>` cache-busting like the
   sibling repos.

## Steps

### 001-sync-demos [production]

Refresh `examples/` from `../sw-cor24-ocaml/tests/` via
`scripts/sync-demos.sh`. Review upstream tests that are NOT currently
mapped; if any are good fits for the public demo dropdown (small,
self-contained, runs cleanly in the browser), extend the MAPPING table
in `sync-demos.sh`, add a matching entry to `src/demos.rs` with a clear
title+description, and document them in `docs/demos.md`. Do not add
demos that need features we know don't work in the browser (infinite
recursion, etc.). Re-run `cargo test --lib` to confirm the catalog
invariants still hold. Commit `examples/`, `scripts/sync-demos.sh`,
`src/demos.rs`, and any doc changes together.

### 002-gh-pages-workflow [production]

Create `.github/workflows/pages.yml` mirroring
`../web-sw-cor24-basic/.github/workflows/pages.yml`: triggers on push
to `main` + manual dispatch, uploads `./pages` as the Pages artifact,
deploys via `actions/deploy-pages@v4`, with the `pages: write` and
`id-token: write` permissions. This is the final piece that lets a
release build in `pages/` become the public live demo at
`https://sw-embed.github.io/web-sw-cor24-ocaml/`. Verify
`pages/.nojekyll` exists (it does). Commit the workflow file alone.

### 003-readme-livedemo-screenshot [production]

Rebuild `pages/` via `./scripts/build-pages.sh` to make sure the live
demo artifacts are current. Start a trunk serve in the background (or
a static server against `pages/`), drive a larger demo end-to-end via
Playwright (e.g. `patterns` or `lists-pairs-demo` — one that produces
visible multi-line output), and save the screenshot to
`images/screenshot-demo.png`. Stop the server.

Update `README.md`:

- Replace the "TODO Screenshot" section with a leading live-demo link
  (`**Live demo**: https://sw-embed.github.io/web-sw-cor24-ocaml/`)
  and an inline screenshot reference: `![demo
  screenshot](images/screenshot-demo.png?ts=<ms>)` where `<ms>` is
  `ep2ms` output at capture time.
- Re-order front matter to match `web-sw-cor24-basic`: title → one-line
  blurb → live-demo link → screenshot → rest.

Commit `README.md`, `images/screenshot-demo.png`, and rebuilt
`pages/` in one commit.
