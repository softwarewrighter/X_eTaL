#!/usr/bin/env bash
# Draw images/name-forms.png: the language's name every way, side by
# side, for docs/name.md and the live demo's Help (the logo, the
# favicon, prose, ASCII as typed, the drawn form in the live demo's
# colors, LaTeX, the slug, the file type, and how to say it). The drawn
# and LaTeX forms come from xetal itself; headless Chrome takes the
# picture.
#   scripts/name-image.sh
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
chrome="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"
[ -x "$chrome" ] || chrome="$(command -v google-chrome || command -v chromium || true)"
if [ -z "$chrome" ]; then echo "name-image: no Chrome; skipped"; exit 0; fi
scripts/build-all.sh --release -q > /dev/null
x=./target/release/xetal
page="$(mktemp -t name-forms).html"
trap 'rm -f "$page"' EXIT
drawn_short="$($x render --html -e 'X_eTaL')"
drawn_logo="$($x render --html -e 'X_ e:T a:L')"
latex="$($x render --latex -e 'X_ e:T a:L' | sed 's/&/\&amp;/g; s/</\&lt;/g')"
cat > "$page" <<HTML
<!doctype html><meta charset="utf-8">
<style>
body { margin: 0; background: #1d2330; color: #d8dbe2; font: 17px/1.4 Georgia, serif; }
.grid { display: grid; grid-template-columns: 150px max-content 1fr; gap: 12px 28px; padding: 26px 30px; }
h1 { grid-column: 1 / -1; margin: 0 0 6px; font-size: 24px; }
.k { color: #8a90a0; font-size: 14px; text-transform: uppercase; letter-spacing: .06em; align-self: center; }
.v { align-self: center; font-size: 22px; }
.n { color: #8a90a0; font-size: 14px; align-self: center; }
code, .mono { font-family: JuliaMono, "PT Mono", Menlo, monospace; }
.mono { font-size: 22px; background: #262c3a; padding: 2px 10px; border-radius: 6px; }
img.logo { height: 64px; border-radius: 8px; background: #fff; }
img.fav { height: 48px; image-rendering: auto; } img.fav16 { height: 16px; }
.say { font-size: 26px; color: #f5c518; }
.c-builtin { color: #5b9cff; } .c-userfunc { color: #3fbf5f; } .c-libfunc { color: #22b8c8; }
.c-lambdaarg { color: #d25fd2; } .c-number { color: #e0b400; } .c-comment { color: #8a90a0; }
</style>
<div class="grid">
<h1>One name, many spellings</h1>
<div class="k">say it</div><div class="v say">Ecks-e-tal</div><div class="n">as .xtl is said eks-tee-ell</div>
<div class="k">logo</div><div class="v"><img class="logo" src="file://$root/images/modern-xetal-logo.jpg"></div><div class="n">underlined X, raised e, T, raised a, L</div>
<div class="k">favicon</div><div class="v"><img class="fav" src="file://$root/components/web/crates/xetal-web/favicon.ico"> <img class="fav16" src="file://$root/components/web/crates/xetal-web/favicon.ico"></div><div class="n">italic underlined X, raised ellipsis: the browser tab</div>
<div class="k">in prose</div><div class="v">XeTaL</div><div class="n">no marks, in a sentence</div>
<div class="k">typed (ASCII)</div><div class="v"><span class="mono">X_eTaL</span> &nbsp; <span class="mono">X_ e:T a:L</span></div><div class="n">what you type; _ underlines, e: raises</div>
<div class="k">drawn</div><div class="v"><span class="mono">$drawn_short</span> &nbsp; <span class="mono">$drawn_logo</span></div><div class="n">as xetal render and the live demo draw them</div>
<div class="k">LaTeX</div><div class="v"><code style="font-size:12px; white-space:nowrap">$latex</code></div><div class="n">xetal render --latex</div>
<div class="k">binary, code</div><div class="v"><span class="mono">xetal</span></div><div class="n">the slug: crates xetal-*, the command</div>
<div class="k">files</div><div class="v"><span class="mono">.xtl</span></div><div class="n">demos/life.xtl, lib/Stats.xtl</div>
<div class="k">repository</div><div class="v"><span class="mono" style="font-size:17px">softwarewrighter/X_eTaL</span></div><div class="n">GitHub, and the live demo's URL</div>
</div>
HTML
"$chrome" --headless=new --disable-gpu --hide-scrollbars --allow-file-access-from-files \
    --window-size=1200,640 --screenshot="$root/images/name-forms.png" "file://$page" > /dev/null 2>&1
echo images/name-forms.png
