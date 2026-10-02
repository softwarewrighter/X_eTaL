#!/usr/bin/env python3
"""Build the syntax poster: scripts/poster/xetal-syntax-poster/index.html
with every sample (an element of class xetal-placeholder with a
data-source attribute) filled with what `xetal render --html -e SOURCE`
draws, so each one is drawn by xetal itself. Writes
pages/poster/index.html, and captures it as images/xetal-syntax-poster.png
for the README. scripts/build-pages.sh runs it.

    scripts/poster.py
"""

import html
import re
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
TEMPLATE = ROOT / "scripts/poster/xetal-syntax-poster/index.html"
OUT = ROOT / "pages/poster/index.html"
XETAL = ROOT / "target/release/xetal"
PICTURE = ROOT / "images/xetal-syntax-poster.png"
PLACEHOLDER = re.compile(
    r"<(span|div)([^>]*class=\"[^\"]*xetal-placeholder[^\"]*\"[^>]*data-source='([^']*)'[^>]*)>(.*?)</\1>",
    re.DOTALL,
)


def drawn(source):
    """SOURCE as `xetal render --html` draws it; an error is fatal."""
    out = subprocess.run(
        [str(XETAL), "render", "--html", "-e", source],
        capture_output=True, text=True, check=False,
    )
    if out.returncode != 0 or "c-error" in out.stdout:
        raise SystemExit(f"poster: xetal cannot draw {source!r}: {out.stderr or out.stdout}")
    return out.stdout.rstrip("\n")


def fill(match):
    tag, attrs, source = match.group(1), match.group(2), html.unescape(match.group(3))
    return f"<{tag}{attrs}>{drawn(source)}</{tag}>"


def capture():
    """The poster as a picture for the README (headless Chrome)."""
    chrome = Path("/Applications/Google Chrome.app/Contents/MacOS/Google Chrome")
    if not chrome.exists():
        print("poster: no Chrome; images/xetal-syntax-poster.png not refreshed")
        return
    subprocess.run(
        [str(chrome), "--headless=new", "--disable-gpu", "--hide-scrollbars",
         "--window-size=1600,1185", f"--screenshot={PICTURE}", OUT.as_uri()],
        capture_output=True, check=False,
    )
    print(PICTURE.relative_to(ROOT))


def main():
    text, count = PLACEHOLDER.subn(fill, TEMPLATE.read_text())
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(text)
    print(f"{OUT.relative_to(ROOT)}: {count} samples drawn by xetal")
    capture()


if __name__ == "__main__":
    main()
