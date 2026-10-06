#!/usr/bin/env python3
"""docs/idioms.md from demos/rosetta/data.toml: the prose lives in
docs/idioms.template.md, the array-language table is generated from the
Rosetta stone's data, so the document and the stone can never disagree.

    scripts/idioms.py               # rewrite docs/idioms.md (just idioms)
    scripts/idioms.py --check       # fail if docs/idioms.md is not current
    scripts/idioms.py --self-test   # check the cell rendering on samples

The template is copied through; its one marker line, `<!-- table: array -->`,
becomes the table: a header of the data's language names, then a row per
idiom in the data's order. A cell is the language's source in <code>
(HTML-escaped, every non-ASCII character an entity, so the document
stays ASCII), followed by the cell's note in parentheses when it has
one; a cell with a note and no source is the note; a cell with neither
is "none". The mainstream table is prose to the generator and stays in
the template until those languages join the data (Saga 35).
"""

import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DATA = ROOT / "demos" / "rosetta" / "data.toml"
TEMPLATE = ROOT / "docs" / "idioms.template.md"
DOC = ROOT / "docs" / "idioms.md"
MARKER = "<!-- table: array -->"


def escape(text):
    """`text` for a markdown table cell: HTML-escaped, ASCII-only."""
    out = []
    for ch in text:
        if ch == "&":
            out.append("&amp;")
        elif ch == "<":
            out.append("&lt;")
        elif ch == ">":
            out.append("&gt;")
        elif ch == "|":
            out.append("&#124;")
        elif ord(ch) > 126:
            out.append(f"&#{ord(ch)};")
        else:
            out.append(ch)
    return "".join(out).replace("\n", " ")


def cell(source, note):
    """One table cell from a language's source and note, either optional."""
    if source and note:
        return f"<code>{escape(source)}</code> ({escape(note)})"
    if source:
        return f"<code>{escape(source)}</code>"
    return escape(note) if note else "none"


def table(data):
    """The array-language table's lines, from the data's axes and tables."""
    langs, names = data["languages"], data["language_names"]
    lines = ["| Idiom | " + " | ".join(names) + " |", "| ----- | " + " | ".join("-" * max(3, len(n)) for n in names) + " |"]
    for idiom, name in zip(data["idioms"], data["idiom_names"]):
        src, note = data["source"].get(idiom, {}), data["notes"].get(idiom, {})
        lines.append(f"| {name} | " + " | ".join(cell(src.get(l), note.get(l)) for l in langs) + " |")
    return lines


def render(template, data):
    """The document: the template with its marker replaced by the table."""
    out = []
    for line in template.splitlines():
        out.extend(table(data) if line.strip() == MARKER else [line])
    return "\n".join(out) + "\n"


def self_test():
    assert escape("a <b> & |c|") == "a &lt;b&gt; &amp; &#124;c&#124;"
    assert escape("⍳n") == "&#9075;n"
    assert escape("x\ny") == "x y"
    assert cell("+/v", None) == "<code>+/v</code>"
    assert cell("⍉a", "a matrix") == "<code>&#9033;a</code> (a matrix)"
    assert cell(None, "the function's name") == "the function's name"
    assert cell(None, None) == "none"
    data = {"languages": ["j"], "language_names": ["J"], "idioms": ["sum"], "idiom_names": ["Sum"], "source": {"sum": {"j": "+/ v"}}, "notes": {}}
    assert render(f"x\n{MARKER}\ny\n", data) == "x\n| Idiom | J |\n| ----- | --- |\n| Sum | <code>+/ v</code> |\ny\n"
    print("idioms: self-test ok")


def main():
    if "--self-test" in sys.argv:
        return self_test()
    data = tomllib.loads(DATA.read_text())
    text = render(TEMPLATE.read_text(), data)
    if "--check" in sys.argv:
        if DOC.read_text() != text:
            sys.exit(f"idioms: {DOC.relative_to(ROOT)} is not current with {DATA.relative_to(ROOT)}; run: just idioms")
        print("idioms: docs/idioms.md is current")
        return
    DOC.write_text(text)
    print(f"idioms: wrote {DOC.relative_to(ROOT)} ({len(data['idioms'])} idioms x {len(data['languages'])} languages)")


if __name__ == "__main__":
    main()
