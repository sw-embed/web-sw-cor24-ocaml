# Scaffold the Cargo + Trunk project

Create the minimal buildable Yew + Trunk project skeleton so later
steps have a working shell to extend. **Do not port demos, runner, or
UI in this step** — those are separate steps.

## Reference

Mirror the top-level layout of `../web-sw-cor24-basic`, adapted for
this project's name and port.

## Files to create

- `Cargo.toml` — name `web-sw-cor24-ocaml`, edition 2024, `[lib]`
  `cdylib, rlib`, deps: `yew = "0.21"` (features `csr`), `wasm-bindgen`,
  `js-sys`, `web-sys` (console, HtmlTextAreaElement, HtmlSelectElement,
  HtmlElement, HtmlInputElement, KeyboardEvent), `gloo = "0.11"`,
  `console_error_panic_hook`. Copy the release profile block from the
  BASIC project (opt-level = "z", lto = true).
- `Trunk.toml` — `dist = "dist"`, `serve.port = 9735` (the port
  reserved for this project).
- `build.rs` — stamp `BUILD_SHA`, `BUILD_HOST`, `BUILD_TIMESTAMP` env
  vars (copy from BASIC project). Don't wire asset copying yet — that
  comes with the runner step.
- `index.html` — same shell as BASIC: trunk rust bin, CSS link to
  `src/ui.css`, favicon.
- `src/main.rs` — `yew::Renderer::<App>::new().render()`.
- `src/lib.rs` — a **placeholder** `App` component that renders a
  single `<h1>{"web-sw-cor24-ocaml"}</h1>`. This is the stub real UI
  replaces later.
- `src/ui.css` — minimal placeholder (one selector is fine). Full port
  happens in the styling step.
- `favicon.ico` — generate via `favicon -T -t "O" -b 6666ff -f ffffff`
  from the repo root (O for OCaml).
- `.gitignore` — `target/`, `dist/`, `pages/*` (but not
  `pages/.nojekyll`), `Cargo.lock` stays **tracked** (this is a binary
  crate).

## Validation

- `cargo check` succeeds.
- `trunk build` succeeds and produces `dist/index.html` that loads the
  placeholder heading.
- `trunk serve --port 9735` serves the page without errors.

## Out of scope for this step

- pa24r path dep (add it only if the runner step actually needs it).
- Asset vendoring (`ocaml.p24m`, `pvm.bin`).
- Real runner, demos table, styling, or build/serve scripts.
