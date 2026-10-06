#!/usr/bin/env python3
"""Check the Rosetta stone's data (demos/rosetta/data.toml, RS3): the
name lists match the axes, each axis is in alphabetical order of its
display names (the page's lists show that order; case aside), every
table is keyed by known idioms and languages, and every X_eTaL cell
with an input runs and prints its output; then run the doc examples of the libraries beside the demo
(Stone, Comparison), which import each other from that directory.
Exit status: the number of problems."""

import os
import subprocess
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DATA = ROOT / "demos/rosetta/data.toml"
XETAL = ROOT / "target/debug/xetal"


def out_of_order(axis, names):
    """The places where an axis's display names are not alphabetical
    (case aside): the page's lists show the data's order, so the data
    keeps them sorted, whatever is added later."""
    return [
        f"{axis}: {a!r} comes before {b!r}; keep the axis alphabetical (case aside)"
        for a, b in zip(names, names[1:])
        if a.casefold() > b.casefold()
    ]


def self_test():
    assert out_of_order("x", ["Apple", "bee", "Cat"]) == []
    assert len(out_of_order("x", ["bee", "Apple"])) == 1
    print("rosetta-check: self-test ok")


def main():
    if "--self-test" in sys.argv:
        return self_test()
    with DATA.open("rb") as f:
        d = tomllib.load(f)
    idioms, languages = d["idioms"], d["languages"]
    problems = []
    for axis, names in (("idioms", "idiom_names"), ("languages", "language_names")):
        if len(d.get(names, [])) != len(d[axis]):
            problems.append(f"{names}: {len(d.get(names, []))} names for {len(d[axis])} {axis}")
        problems += out_of_order(names, d.get(names, []))
    for table in ("input", "source", "output", "notes"):
        for idiom, row in d.get(table, {}).items():
            if idiom not in idioms:
                problems.append(f"{table}.{idiom}: not an idiom")
            for lang in row:
                if lang not in languages:
                    problems.append(f"{table}.{idiom}.{lang}: not a language")
    for idiom in idioms:
        source = d["source"].get(idiom, {}).get("xetal")
        prelude = d.get("input", {}).get(idiom, {}).get("xetal")
        expected = d.get("output", {}).get(idiom, {}).get("xetal")
        if not (source and prelude and expected):
            continue
        p = subprocess.run([str(XETAL), "eval", "-e", f"{prelude}; {source}"],
                           capture_output=True, text=True, cwd=ROOT)
        got = p.stdout.rstrip("\n")
        if p.returncode != 0 or got != expected:
            problems.append(f"{idiom}: xetal cell gave {got!r} ({p.stderr.strip()}), expected {expected!r}")
    for lib in sorted(ROOT.glob("demos/rosetta/[A-Z]*.xtl")):
        env = dict(os.environ, XETAL_PATH=str(ROOT / "demos/rosetta"))
        p = subprocess.run([str(XETAL), "doc", "--test", str(lib)], capture_output=True, text=True, cwd=ROOT, env=env)
        if p.returncode != 0:
            problems.append(f"{lib.name}: doc examples: {p.stdout.strip().splitlines()[-1] if p.stdout.strip() else p.stderr.strip()}")
    cells = sum(len(r) for r in d["source"].values())
    for p in problems:
        print(f"rosetta-check: {p}")
    print(f"rosetta-check: {len(idioms)} idioms, {len(languages)} languages, {cells} cells, {len(problems)} problems")
    sys.exit(len(problems))


if __name__ == "__main__":
    main()
