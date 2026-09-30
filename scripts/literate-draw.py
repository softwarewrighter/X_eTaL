#!/usr/bin/env python3
"""Put the drawn form above every xetal block of a literate document.

Before each `#+begin_src xetal` block, write what `xetal render` draws
for it (underlines, raised letters, arrows) as an example block, marked
by an Org comment line so it can be found and refreshed. The source
block stays ASCII (it is what runs); the drawn form comes first, so it
is what a reader sees first on GitHub and in Emacs. The HTML export
drops these copies, since it draws the blocks themselves.

    scripts/literate-draw.py FILE.org...   # rewrite in place

Uses target/release/xetal (XETAL_BIN overrides).
"""

import os
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
XETAL = os.environ.get("XETAL_BIN", str(ROOT / "target/release/xetal"))
MARK = "# drawn by xetal render (scripts/literate-draw.py); edit the block below"
DRAWN = re.compile(
    re.escape(MARK) + r"\n#\+begin_example\n.*?#\+end_example\n", re.DOTALL
)
BLOCK = re.compile(r"^#\+begin_src xetal\b[^\n]*\n(.*?)^#\+end_src", re.DOTALL | re.MULTILINE)


def drawn(code):
    """The block's code as xetal render draws it."""
    out = subprocess.run(
        [XETAL, "render", "-e", code], capture_output=True, text=True, check=False
    )
    if out.returncode != 0:
        sys.exit(f"literate-draw: xetal render failed:\n{code}\n{out.stderr}")
    return out.stdout.rstrip("\n")


def draw(text):
    """TEXT with every drawn copy removed, then one put above each block."""
    text = DRAWN.sub("", text)

    def above(m):
        body = drawn(m.group(1).rstrip("\n"))
        lines = "\n".join(("," + l if l.startswith(("*", "#+")) else l) for l in body.split("\n"))
        return f"{MARK}\n#+begin_example\n{lines}\n#+end_example\n{m.group(0)}"

    return BLOCK.sub(above, text)


def main():
    for name in sys.argv[1:]:
        path = Path(name)
        old = path.read_text()
        new = draw(old)
        if new != old:
            path.write_text(new)


if __name__ == "__main__":
    main()
