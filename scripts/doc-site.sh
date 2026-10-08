#!/usr/bin/env bash
# Build the documentation site (xetal doc --out) into pages/doc: every
# standard library and macro library in lib/ (System.xtlm among them),
# the built-ins, every bundled demo (the Rosetta stone's among them)
# and userlibs/, so each name shows whose it is: l: a library's export,
# h: a helper, u: a program's own. A demo run --untyped (its #! line
# says so) has no types to document and is left out. Run by
# scripts/build-pages.sh (just pages) and by `just doc`.
#   scripts/doc-site.sh             # into pages/doc
#   XETAL_DOC_OUT=DIR scripts/doc-site.sh
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
xetal="${XETAL:-target/release/xetal}"
out="${XETAL_DOC_OUT:-pages/doc}"
rm -rf "$out"
typed=()
for f in demos/*.xtl demos/rosetta/*.xtl userlibs/*.xtl; do
    head -1 "$f" | grep -q -- --untyped || typed+=("$f")
done
"$xetal" doc --out "$out" lib/*.xtl lib/*.xtlm "${typed[@]}" > /dev/null
echo "doc: $(find "$out" -name '*.html' | wc -l | tr -d ' ') pages in $out"
