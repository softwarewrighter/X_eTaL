#!/usr/bin/env bash
# Render the live demo's app icons from images/app-icon.svg (headless
# Chrome, sips): 192 and 512 pixels, a maskable 512 (the design inside the
# safe zone, on the same yellow), and the 180-pixel Apple touch icon.
#   scripts/app-icons.sh
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
chrome="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"
[ -x "$chrome" ] || chrome="$(command -v google-chrome || command -v chromium || true)"
if [ -z "$chrome" ]; then echo "app-icons: no Chrome; skipped"; exit 0; fi
out="$root/components/web/crates/xetal-web/icons"
mkdir -p "$out"
page="$(mktemp -t app-icon).html"
trap 'rm -f "$page"' EXIT
# Chrome will not make a window smaller than about 500 pixels, so each
# icon is drawn at 512 and the small ones scaled down (sips, on a Mac).
render() { # padding-percent file
    cat > "$page" <<HTML
<!doctype html><body style="margin:0;background:#fcc419">
<img src="file://$root/images/app-icon.svg" style="display:block;width:$((100 - 2 * $1))vw;margin:$1vw">
HTML
    "$chrome" --headless=new --disable-gpu --hide-scrollbars --allow-file-access-from-files \
        --window-size=512,512 --screenshot="$out/$2" "file://$page" > /dev/null 2>&1
    echo "icons/$2"
}
scale() { # size from to
    sips -z "$1" "$1" "$out/$2" --out "$out/$3" > /dev/null && echo "icons/$3"
}
render 0 icon-512.png
render 10 icon-maskable-512.png
scale 192 icon-512.png icon-192.png
scale 180 icon-512.png apple-touch-icon.png
