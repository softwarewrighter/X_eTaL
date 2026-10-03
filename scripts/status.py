#!/usr/bin/env python3
"""Generate docs/status.md, "what works today", from the sources that
define the language, so the table cannot drift from them:

- the built-in catalog (components/base/crates/xetal-catalog/builtins.toml):
  every built-in, its type, and the spec cases that use it;
- the language decisions (docs/lang-choices.md): every rule, planned
  when its row says "not yet implemented", partial when it says
  "partly implemented", otherwise works, with the spec cases citing it;
- the spec cases (spec/**/*.case): a case with STATUS pending is
  planned work;
- the standard libraries (lib/*.xtl), from their first comment line.

    scripts/status.py           # write docs/status.md
    scripts/status.py --check   # fail if docs/status.md is not current
"""

import re
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "docs/status.md"
CATALOG = ROOT / "components/base/crates/xetal-catalog/builtins.toml"
CHOICES = ROOT / "docs/lang-choices.md"
REFERENCE = ROOT / "docs/reference/builtins.ref"
ROW = re.compile(r"^\| ([A-Z]+[0-9]+[a-z]?) \| (.*) \|\s*$")


def cases():
    """Each spec case: its path (from spec/), source text, comment text
    and whether it is pending."""
    found = []
    for path in sorted((ROOT / "spec").rglob("*.case")):
        text = path.read_text()
        comment = "\n".join(l for l in text.splitlines() if l.startswith("#"))
        source = section(text, "SOURCE")
        pending = section(text, "STATUS").strip() == "pending"
        found.append((str(path.relative_to(ROOT / "spec")), source, comment, pending))
    return found


def section(text, name):
    """The body of a `== NAME` section of a case file."""
    match = re.search(rf"^== {name}\n(.*?)(?=^== |\Z)", text, re.M | re.S)
    return match.group(1) if match else ""


def uses(name, source):
    """Whether `name` appears in `source` as a whole token."""
    if re.fullmatch(r"[A-Za-z_\[\]][\w\[\]?!<]*", name):
        return re.search(rf"(?<![\w\[\]]){re.escape(name)}(?![\w])", source) is not None
    return re.search(rf"(?:^|\s){re.escape(name)}(?:\s|$)", source, re.M) is not None


def examples():
    """Each built-in's run examples in the reference (`> code` lines
    under its `== name` heading), counted by name."""
    counts, name = {}, None
    for line in REFERENCE.read_text().splitlines():
        if line.startswith("== "):
            name = None if line.startswith("== SECTION") else line[3:].strip()
        elif name and (line.startswith("> ") or line.startswith("!> ")):
            counts[name] = counts.get(name, 0) + 1
    return counts


def builtins(all_cases):
    """(group rule, [(name, type, state, case count, example count)])
    per catalog group; a built-in no spec case or reference example
    runs is partial."""
    catalog = tomllib.loads(CATALOG.read_text())
    runs = examples()
    groups = []
    for kind in ("group", "later"):
        for group in catalog.get(kind, []):
            rows = []
            for entry in group.get("entries", group.get("names", [])):
                name = entry[0] if isinstance(entry, list) else entry
                sig = entry[2] if isinstance(entry, list) and len(entry) > 2 else ""
                count = sum(uses(name, src) for _, src, _, _ in all_cases)
                shown = runs.get(name, 0)
                state = ("planned" if kind != "group"
                         else "works" if count + shown else "partial")
                rows.append((name, sig, state, count, shown))
            groups.append((group["rule"], rows))
    return groups


def decisions(all_cases):
    """(id, summary, state, case count) for every decision row."""
    found = []
    for line in CHOICES.read_text().splitlines():
        match = ROW.match(line)
        if not match:
            continue
        rule, text = match.group(1), match.group(2)
        lower = text.lower()
        state = ("planned" if "not yet implemented" in lower
                 else "partial" if "partly implemented" in lower else "works")
        cited = re.compile(rf"\b{re.escape(rule)}\b")
        count = sum(bool(cited.search(c)) for _, _, c, _ in all_cases)
        found.append((rule, summary(text), state, count))
    return found


def summary(text):
    """The first sentence of a decision, short enough for a table."""
    first = re.split(r"(?<=[a-z0-9`)])\. ", text, maxsplit=1)[0].rstrip(".")
    first = first.replace("|", "/")
    return first if len(first) <= 110 else first[:107].rsplit(" ", 1)[0] + " (...)"


def libraries():
    """(name, what) for each standard library."""
    found = []
    for path in sorted((ROOT / "lib").glob("*.xtl")):
        head = path.read_text().splitlines()[0].lstrip("# ").strip()
        what = head.split(":", 1)[1].strip() if ":" in head else head
        found.append((path.stem, what.rstrip(".")))
    return found


def render():
    """The whole of docs/status.md."""
    all_cases = cases()
    groups, rules, libs = builtins(all_cases), decisions(all_cases), libraries()
    pending = [path for path, _, _, p in all_cases if p]
    out = [*header(), *totals(groups, rules, libs, pending, len(all_cases))]
    out += planned_section(rules, pending)
    out += builtin_section(groups) + decision_section(rules) + library_section(libs)
    return "\n".join(out) + "\n"


def header():
    return [
        "# Status: what works today",
        "",
        "Generated by `scripts/status.py` from the built-in catalog",
        "(`components/base/crates/xetal-catalog/builtins.toml`), the language",
        "decisions (`docs/lang-choices.md`), the spec cases (`spec/`) and the",
        "standard libraries (`lib/`); the gate checks that it is current, so",
        "edit those sources, never this file, and run `just status`.",
        "",
        "States: **works** (implemented, and for a built-in run by at least",
        "one spec case or reference example), **partial** (a built-in nothing",
        "runs, or a decision that says it is partly implemented), **planned**",
        "(decided and not yet implemented, or a spec case marked pending). The",
        "counts are the spec cases that use a built-in or cite a decision, and",
        "the reference examples a built-in runs. Decisions that are",
        "conventions (naming, layout) work without a spec case citing them.",
        "",
    ]


def totals(groups, rules, libs, pending, case_count):
    builtin_rows = [r for _, rows in groups for r in rows]
    count = lambda xs, s: sum(1 for x in xs if x[2] == s)
    return [
        "## Summary",
        "",
        "| What | Works | Partial | Planned |",
        "| ---- | ----- | ------- | ------- |",
        f"| Built-in functions | {count(builtin_rows, 'works')} | {count(builtin_rows, 'partial')} | {count(builtin_rows, 'planned')} |",
        f"| Language decisions | {count(rules, 'works')} | {count(rules, 'partial')} | {count(rules, 'planned')} |",
        f"| Standard libraries | {len(libs)} | 0 | 0 |",
        f"| Spec cases | {case_count - len(pending)} | 0 | {len(pending)} |",
        "",
    ]


def planned_section(rules, pending):
    out = ["## Planned (decided, not yet implemented)", ""]
    for rule, text, state, _ in rules:
        if state != "works":
            out.append(f"- **{rule}** ({state}): {text}.")
    out += [f"- spec case `{path}` (pending)" for path in pending]
    return out + [""]


def builtin_section(groups):
    out = ["## Built-in functions", ""]
    for rule, rows in groups:
        out += [f"### {rule}", "", "| Name | Type | State | Spec cases | Reference examples |",
                "| ---- | ---- | ----- | ---------- | ------------------ |"]
        out += [f"| `{n}` | `{t}` | {s} | {c} | {e} |" for n, t, s, c, e in rows]
        out.append("")
    return out


def decision_section(rules):
    out = ["## Language decisions", "", "| Rule | Decision | State | Spec cases |",
           "| ---- | -------- | ----- | ---------- |"]
    out += [f"| {r} | {t} | {s} | {c} |" for r, t, s, c in rules]
    return out + [""]


def library_section(libs):
    out = ["## Standard libraries", "", "Built into `xetal`; import one with",
           "`\"s:\" u_se< \"Stats\"` (the alias is yours).", "",
           "| Library | What | State |", "| ------- | ---- | ----- |"]
    return out + [f"| `{n}` | {w} | works |" for n, w in libs]


def main():
    text = render()
    if "--check" in sys.argv[1:]:
        if not OUT.exists() or OUT.read_text() != text:
            print("status: docs/status.md is not current (run scripts/status.py, or just status)")
            sys.exit(1)
        print("status: docs/status.md is current")
        return
    OUT.write_text(text)
    print(OUT.relative_to(ROOT))


if __name__ == "__main__":
    main()
