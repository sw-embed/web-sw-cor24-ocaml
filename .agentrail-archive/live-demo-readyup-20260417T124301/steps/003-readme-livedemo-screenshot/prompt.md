# Live-demo link, screenshot, README polish

Rebuild pages/ via ./scripts/build-pages.sh. Start a local server (trunk serve, or a static server on ./pages/) and use Playwright to: open the app, pick a larger demo (lists-pairs-demo or patterns), click Run, wait for output, screenshot the whole app to images/screenshot-demo.png. Stop the server.

Update README.md: restructure the front matter to match web-sw-cor24-basic (title -> one-line blurb -> **Live demo**: https://sw-embed.github.io/web-sw-cor24-ocaml/ -> inline screenshot with ?ts=<ms> cache-bust from ep2ms -> rest of content). Replace the TODO 'Screenshot' section at the bottom.

Commit README.md, images/screenshot-demo.png, and the rebuilt pages/ artifacts together. Mark saga --done.