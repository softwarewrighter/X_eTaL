#!/usr/bin/env python3
"""Run the Rosetta stone's cells in their own languages. For every
idiom that has an input for the language (demos/rosetta/data.toml,
`input.IDIOM.LANG`, the bindings the cell runs with), the input and
the source are run by that language's interpreter and the output is
compared with `output.IDIOM.LANG`, so a cell on the stone is a claim
that has been executed, not remembered.

    scripts/rosetta-run.py bqn            # check every BQN cell with an input
    scripts/rosetta-run.py uiua --bless   # record the outputs as they are now
    scripts/rosetta-run.py bqn uiua       # several languages

The interpreters are the ones scripts/install-array-langs.sh builds
under tools/bin (CBQN as `bqn`, Uiua 0.19.1 as `uiua`); X_eTaL's own
cells are run by scripts/rosetta-check.py. A language whose
interpreter is not there is skipped with a note (the gate runs this
where they are installed), so the check never fails for want of a
tool. Exit status: the number of cells whose output differs.
"""

import re
import subprocess
import sys
import tempfile
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DATA = ROOT / "demos/rosetta/data.toml"
TOOLS = ROOT / "tools/bin"


def run_bqn(prelude, source):
    """CBQN: the bindings and the expression as one line, printed."""
    program = f"{prelude} ⋄ {source}"
    return subprocess.run([TOOLS / "bqn", "-p", program], capture_output=True, text=True, timeout=30)


def run_uiua(prelude, source):
    """Uiua: a file of the bindings then the expression; the stack is
    printed (on standard error, where `uiua run` puts it)."""
    with tempfile.NamedTemporaryFile("w", suffix=".ua", delete=False) as f:
        f.write(prelude + "\n" + source + "\n")
    r = subprocess.run([TOOLS / "uiua", "run", f.name], capture_output=True, text=True, timeout=30)
    if r.returncode == 0:
        r.stdout, r.stderr = r.stderr, ""
    return r


RUNNERS = {"bqn": run_bqn, "uiua": run_uiua}


def clean(text):
    """An interpreter's output as the data records it: no trailing
    blanks on a line, no blank lines at the end."""
    lines = [re.sub(r"[ \t]+$", "", l) for l in text.replace("\r", "").split("\n")]
    while lines and not lines[-1]:
        lines.pop()
    return "\n".join(lines)


def cells(d, lang):
    """(idiom, prelude, source, expected) for the language's runnable cells."""
    for idiom in d["idioms"]:
        prelude = d.get("input", {}).get(idiom, {}).get(lang)
        source = d["source"].get(idiom, {}).get(lang)
        if prelude and source:
            yield idiom, prelude, source, d.get("output", {}).get(idiom, {}).get(lang)


def check(d, lang, bless):
    """Run a language's cells; the outputs to record when blessing, and the failures."""
    run = RUNNERS[lang]
    recorded, failures = {}, []
    for idiom, prelude, source, expected in cells(d, lang):
        r = run(prelude, source)
        got = clean(r.stdout) if r.returncode == 0 else clean(r.stdout + r.stderr) or f"exit {r.returncode}"
        if r.returncode != 0:
            failures.append(f"{lang} {idiom}: the interpreter failed:\n{got}")
        elif bless:
            recorded[idiom] = got
        elif got != expected:
            failures.append(f"{lang} {idiom}: expected {expected!r}, got {got!r}")
    return recorded, failures


def bless_into(text, lang, recorded):
    """The data file's text with the outputs recorded (a line per cell,
    added to or replaced in the idiom's [output.IDIOM] table)."""
    for idiom, got in recorded.items():
        value = '"' + got.replace("\\", "\\\\").replace('"', '\\"').replace("\n", "\\n") + '"'
        head = f"[output.{idiom}]\n"
        if head not in text:
            text = text.rstrip("\n") + f"\n\n{head}{lang} = {value}\n"
            continue
        start = text.index(head) + len(head)
        end = text.find("\n[", start)
        end = len(text) if end < 0 else end
        block = text[start:end]
        line = f"{lang} = {value}\n"
        if re.search(rf"^{lang} = ", block, re.M):
            block = re.sub(rf"^{lang} = .*$", line.rstrip("\n").replace("\\", "\\\\"), block, count=1, flags=re.M)
        else:
            block = block.rstrip("\n") + "\n" + line
        text = text[:start] + block + text[end:]
    return text


def main():
    bless = "--bless" in sys.argv
    langs = [a for a in sys.argv[1:] if a != "--bless"] or list(RUNNERS)
    with DATA.open("rb") as f:
        d = tomllib.load(f)
    text = DATA.read_text()
    failures, ran = [], 0
    for lang in langs:
        if lang not in RUNNERS:
            sys.exit(f"rosetta-run: no runner for {lang} (have {', '.join(RUNNERS)})")
        if not (TOOLS / lang).exists():
            print(f"rosetta-run: {lang}: no tools/bin/{lang} (scripts/install-array-langs.sh {lang}); skipped")
            continue
        recorded, failed = check(d, lang, bless)
        ran += sum(1 for _ in cells(d, lang))
        failures += failed
        if bless:
            text = bless_into(text, lang, recorded)
    if bless:
        DATA.write_text(text)
    for f in failures:
        print(f)
    print(f"rosetta-run: {ran} cells run, {len(failures)} problems" + (" (outputs recorded)" if bless else ""))
    sys.exit(len(failures))


if __name__ == "__main__":
    main()
