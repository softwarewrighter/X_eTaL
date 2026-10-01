#!/usr/bin/env bash
# Open a picture (an SVG drawn with []S_HOW) in the default viewer, which
# is usually the browser: open on macOS, xdg-open on Linux.
set -euo pipefail
file="${1:?usage: scripts/open-picture.sh FILE.svg}"
[ -f "$file" ] || { echo "open-picture: no picture at $file (did the program call []S_HOW?)" >&2; exit 1; }
if command -v open >/dev/null && [ "$(uname)" = Darwin ]; then
    exec open "$file"
elif command -v xdg-open >/dev/null; then
    exec xdg-open "$file"
fi
echo "open-picture: no viewer found; the picture is at $file" >&2
