#!/usr/bin/env bash
#
# build-pages.sh -- Release build into pages/ for GitHub Pages.
#
# Wraps `trunk build --release --public-url /web-sw-cor24-ocaml/` with
# the same exclusive dist/ lock the dev server uses, so a build
# cannot race a running serve (which would corrupt dist/.stage
# mid-pipeline and produce empty/SRI-blocked artifacts).
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

BUILD_HOST="$(hostname -s 2>/dev/null || echo unknown)"
BUILD_TS="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
BUILD_SHA="$(git -C "$PROJECT_DIR" rev-parse --short HEAD 2>/dev/null || echo unknown)"

echo "=== Building pages/ ==="
echo "  host:  $BUILD_HOST"
echo "  time:  $BUILD_TS"
echo "  sha:   $BUILD_SHA"
cd "$PROJECT_DIR"

mkdir -p target
LOCK="$PROJECT_DIR/target/.trunk-dist.lock"
if ! mkdir "$LOCK" 2>/dev/null; then
  HOLDER="$(cat "$LOCK/pid" 2>/dev/null || echo unknown)"
  if [ "$HOLDER" != "unknown" ] && ! kill -0 "$HOLDER" 2>/dev/null; then
    echo "build-pages.sh: removing stale lock from pid $HOLDER" >&2
    rm -rf "$LOCK"
    mkdir "$LOCK"
  else
    echo "build-pages.sh: another trunk process (pid $HOLDER) holds $LOCK -- refusing to share dist/" >&2
    exit 1
  fi
fi
echo $$ > "$LOCK/pid"
trap 'rm -rf "$LOCK"' EXIT INT TERM

mkdir -p pages
touch pages/.nojekyll
trunk build --release --public-url /web-sw-cor24-ocaml/
rsync -a --delete --exclude='.nojekyll' dist/ pages/

echo "=== Done ==="
echo "Pages built in: $PROJECT_DIR/pages/"
echo "To deploy: git add pages/ && git commit && git push"
