# Rebuild pages and preview with serve.sh

Run ./scripts/build-pages.sh (or trunk build --release) to regenerate pages/ with the new demos included. Vendor artifacts via scripts/vendor-artifacts.sh if that is part of the normal flow.

Start ./scripts/serve.sh in the background, open a browser to confirm 'guess' and 'text-adventure' appear in the dropdown (alphabetical) and that guess runs (enter 42).

Commit regenerated pages/ artifacts alongside .agentrail updates.

Do NOT push -- user will push after preview approval.