#!/usr/bin/env bash
# Export the literate documents (docs/literate/*.org) to HTML under
# pages/literate/, with an index page and the images they draw, for the
# live demo's "Literate docs" link. Each xetal block is shown drawn (by
# xetal render --html), with the lines as typed after it as comments. The recorded results are exported
# as they are (scripts/literate.sh records them).
#   scripts/literate-html.sh
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
emacs="${EMACS:-}"
[ -n "$emacs" ] || ! command -v emacs > /dev/null 2>&1 || emacs=emacs
[ -n "$emacs" ] || [ ! -x /Applications/Emacs.app/Contents/MacOS/Emacs ] || emacs=/Applications/Emacs.app/Contents/MacOS/Emacs
if [ -z "$emacs" ]; then echo "literate-html: no Emacs; skipped"; exit 0; fi
scripts/build-all.sh --release -q > /dev/null
export XETAL_BIN="$root/target/release/xetal"
out="$root/pages/literate"
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
mkdir -p "$work/docs/literate" "$out/images"
cp docs/literate/*.org "$work/docs/literate/"
mkdir -p "$work/lib" && cp lib/*.xtl "$work/lib/"
"$emacs" --batch -Q -l docs/emacs/literate-export.el "$work"/docs/literate/*.org > "$work/emacs.log" 2>&1 \
    || { cat "$work/emacs.log"; exit 1; }
rm -f "$out"/*.html
for page in "$work"/docs/literate/*.html; do
    sed 's#\.\./\.\./images/#images/#g' "$page" > "$out/$(basename "$page")"
done
for image in $(grep -oh 'images/[A-Za-z0-9_.-]*' docs/literate/*.org | sort -u); do
    cp "$image" "$out/images/"
done
cp docs/literate/style.css "$out/style.css"
scripts/literate-index.sh > "$out/index.html"
ls "$out"
