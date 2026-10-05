#!/usr/bin/env bash
# Build the documentation site (xetal doc --out) into pages/doc: every
# standard library and macro library in lib/ (System.xtlm among them),
# the built-ins, and two programs to read from the top down: Life and
# the TTTML game. Run by scripts/build-pages.sh (just pages) and by
# `just doc`.
#   scripts/doc-site.sh             # into pages/doc
#   XETAL_DOC_OUT=DIR scripts/doc-site.sh
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
xetal="${XETAL:-target/release/xetal}"
out="${XETAL_DOC_OUT:-pages/doc}"
rm -rf "$out"
"$xetal" doc --out "$out" lib/*.xtl lib/*.xtlm demos/life.xtl demos/tttml-play.xtl > /dev/null
echo "doc: $(find "$out" -name '*.html' | wc -l | tr -d ' ') pages in $out"
