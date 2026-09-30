#!/usr/bin/env bash
# Screenshot the built live demo (pages/) into images/live-demo.png, as
# the README shows it: pages/ is served under /X_eTaL/, as on GitHub
# Pages, and headless Chrome takes the picture.
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
chrome="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"
[ -x "$chrome" ] || chrome="$(command -v google-chrome || command -v chromium || true)"
if [ -z "$chrome" ]; then echo "live-screenshot: no Chrome; skipped"; exit 0; fi
site="$(mktemp -d)"
ln -s "$root/pages" "$site/X_eTaL"
port=8097
(cd "$site" && python3 -m http.server "$port" --bind 127.0.0.1 > /dev/null 2>&1) &
server=$!
trap 'kill $server 2>/dev/null; rm -rf "$site"' EXIT
sleep 1
"$chrome" --headless=new --disable-gpu --hide-scrollbars --virtual-time-budget=10000 \
    --window-size=1400,900 --screenshot="$root/images/live-demo.png" \
    "http://127.0.0.1:$port/X_eTaL/" > /dev/null 2>&1
echo "images/live-demo.png"
