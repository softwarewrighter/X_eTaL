#!/usr/bin/env python3
"""Check the Rosetta stone's data (demos/rosetta/data.toml, RS3): every
idiom and language has a name, every table is keyed by known idioms and
languages, and every X_eTaL cell with an input runs and prints its
output. Exit status: the number of problems."""

import subprocess
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DATA = ROOT / "demos/rosetta/data.toml"
XETAL = ROOT / "target/debug/xetal"


def main():
    with DATA.open("rb") as f:
        d = tomllib.load(f)
    idioms, languages = d["idioms"], d["languages"]
    problems = []
    for key in idioms + languages:
        if key not in d["names"]:
            problems.append(f"{key}: no name")
    for table in ("source", "output", "notes"):
        for idiom, row in d.get(table, {}).items():
            if idiom not in idioms:
                problems.append(f"{table}.{idiom}: not an idiom")
            for lang in row:
                if lang not in languages:
                    problems.append(f"{table}.{idiom}.{lang}: not a language")
    for idiom in idioms:
        source = d["source"].get(idiom, {}).get("xetal")
        prelude = d.get("input", {}).get(idiom)
        expected = d.get("output", {}).get(idiom, {}).get("xetal")
        if not (source and prelude and expected):
            continue
        p = subprocess.run([str(XETAL), "eval", "-e", f"{prelude}; {source}"],
                           capture_output=True, text=True, cwd=ROOT)
        got = p.stdout.rstrip("\n")
        if p.returncode != 0 or got != expected:
            problems.append(f"{idiom}: xetal cell gave {got!r} ({p.stderr.strip()}), expected {expected!r}")
    cells = sum(len(r) for r in d["source"].values())
    for p in problems:
        print(f"rosetta-check: {p}")
    print(f"rosetta-check: {len(idioms)} idioms, {len(languages)} languages, {cells} cells, {len(problems)} problems")
    sys.exit(len(problems))


if __name__ == "__main__":
    main()
