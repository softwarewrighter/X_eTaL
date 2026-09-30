#!/usr/bin/env python3
"""Build docs/reference.md, the built-in functions, from
docs/reference/builtins.ref (descriptions and examples) and the catalog
(components/base/crates/xetal-catalog/builtins.toml: names, argument
counts, types). Every example is run by target/release/xetal and its
result recorded; an example marked "!>" must fail. With --check, fail
when docs/reference.md is not what this would write.

    scripts/reference.py [--check]
"""
import re
import subprocess
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DATA = ROOT / "docs/reference/builtins.ref"
CATALOG = ROOT / "components/base/crates/xetal-catalog/builtins.toml"
OUT = ROOT / "docs/reference.md"
XETAL = ROOT / "target/release/xetal"
PRELUDE = ["v := 3 1 2", "M := 2 3 r_eshape r_ange 6", "N := 2 3 r_eshape 3 1 2 6 4 5"]
ARGS = {1: "one argument", 2: "two arguments", 3: "three arguments", 4: "four arguments"}


def catalog():
    data = tomllib.loads(CATALOG.read_text())
    return {name: (arity, sig) for g in data["group"] for name, arity, sig in g["entries"]}


def sections():
    """[(title, intro, [(name, description, [(must_fail, code)])])]"""
    out = []
    for line in DATA.read_text().splitlines():
        if line.startswith(";;") or (not line.strip() and not out):
            continue
        if line.startswith("== SECTION "):
            out.append([line[11:].strip(), [], []])
        elif line.startswith("== "):
            out[-1][2].append([line[3:].strip(), [], []])
        elif line.startswith("> ") or line.startswith("!> "):
            fail = line.startswith("!")
            out[-1][2][-1][2].append((fail, line.split("> ", 1)[1]))
        elif out[-1][2]:
            out[-1][2][-1][1].append(line)
        else:
            out[-1][1].append(line)
    return out


def run(code, must_fail):
    source = "\n".join(PRELUDE + [code])
    p = subprocess.run([str(XETAL), "eval", "--seed", "1", "-e", source], cwd=ROOT,
                       input="a typed line\n", capture_output=True, text=True)
    if must_fail != (p.returncode != 0):
        sys.exit(f"reference: {code!r} {'should fail' if must_fail else 'failed'}: {p.stderr.strip()}")
    if must_fail:
        return re.sub(r" at \d+\.\.\d+$", "", p.stderr.strip().splitlines()[0])
    return p.stdout.rstrip("\n")


def text(lines):
    return "\n".join(lines).strip()


def document(parts, known):
    names = {n for _, _, entries in parts for n, _, _ in entries}
    missing, unknown = sorted(set(known) - names), sorted(names - set(known))
    if missing or unknown:
        sys.exit(f"reference: not described: {missing}; not built-ins: {unknown}")
    doc = ["# Built-in functions", "",
           "Every built-in function, what it does and how it is used. Each",
           "example is shown as in a session, the code indented six spaces",
           "and its result under it; every one was run by `xetal`",
           "(`scripts/reference.py` writes this page from",
           "`docs/reference/builtins.ref` and the built-in catalog, and the",
           "gate checks it is current). The examples use three arrays:", "", "```"]
    doc += ["      " + p for p in PRELUDE] + ["```", "",
            "`M` has two rows, 1 2 3 and 4 5 6: axis 1 (down the rows) has",
            "length 2, axis 2 (along a row) length 3. A subscript after a",
            "function's name picks the axis it works along: `'+ r_/ M` works",
            "along axis 1, the default; `'+ r_/_1 M` writes that axis out; and",
            "`'+ r_/_2 M` works along axis 2 instead.", ""]
    for title, intro, entries in parts:
        doc += [f"## {title}", ""] + ([text(intro), ""] if text(intro) else [])
        for name, desc, examples in entries:
            arity, sig = known[name]
            doc += [f"### `{name}`", "", f"`{sig}`, {ARGS[arity]}.", "", text(desc), "", "```"]
            for fail, code in examples:
                doc += ["      " + code, run(code, fail)]
            doc += ["```", ""]
    return "\n".join(doc).rstrip("\n") + "\n"


def main():
    new = document(sections(), catalog())
    if "--check" in sys.argv[1:]:
        if OUT.read_text() != new:
            sys.exit("reference: docs/reference.md is not current (run scripts/reference.py)")
        return
    OUT.write_text(new)
    print(OUT.relative_to(ROOT))


main()
