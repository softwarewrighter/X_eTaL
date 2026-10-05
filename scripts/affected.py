#!/usr/bin/env python3
"""What a change affects, for the fast gate (scripts/gate.sh).

Reads the files changed since the last gated state (the merge base
with origin/main, or GATE_BASE; committed, uncommitted and untracked
files all count) and prints one line per thing the gate should do:

  check NAME   a component whose own files changed: fmt, clippy, tests
  test NAME    a component whose lock file alone changed, or that builds
               in or tests against changed files outside it (lib/,
               spec/, demos/, userlibs/): tests only
  build NAME   a component that only depends on a changed one: its
               library code compiled (cargo check), its tests neither
               compiled nor run; the spec cases and the goldens, which
               always run, cover its behavior, and the full gate the rest
  flag NAME    a slower check whose inputs changed:
               wasm, literate, emacs, diagrams, smoke, asks

A component with no line is skipped. A change to the list of components
or the cargo configuration checks everything; a change to the gate's
own scripts (gate.sh, this file) turns on every flag and leaves the
components to the plan.

  scripts/affected.py              # the plan for the present change
  scripts/affected.py --self-test  # check the planner on known changes
"""
import os
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
EVERYTHING = ("scripts/components.sh", ".cargo/", "rust-toolchain")
GATE = ("scripts/gate.sh", "scripts/affected.py")
# Files outside components/ that a component builds in or tests against.
READ_BY = {"lib/": "macro", "spec/": "cli", "demos/": "web", "userlibs/": "web"}
FLAGS = {
    "literate": ("docs/literate/", "lib/", "demos/", "userlibs/", "docs/emacs/"),
    "emacs": ("docs/emacs/",),
    "diagrams": ("docs/diagrams/", "demos/", "components/render/"),
    "smoke": ("justfile", "scripts/", "demos/"),
    "asks": ("docs/asks.toml", "docs/asks.md", "scripts/asks.py"),
}


def components():
    line = re.search(r"COMPONENTS=\(([^)]*)\)", (ROOT / "scripts/components.sh").read_text())
    return line.group(1).split()


def depends(names):
    """Each component's direct dependencies: the components owning the
    crates its own crates name (the workspace's list names more)."""
    owner = {}
    for c in names:
        for crate in (ROOT / "components" / c / "crates").iterdir():
            owner[crate.name] = c
    deps = {}
    for c in names:
        used = set()
        for manifest in (ROOT / "components" / c / "crates").glob("*/Cargo.toml"):
            used |= set(re.findall(r"^(xetal-[a-z0-9-]+)\b", manifest.read_text(), re.M))
        deps[c] = {owner[k] for k in used if k in owner} - {c}
    return deps


def plan(files, names, deps):
    """The gate's lines for the changed `files`."""
    if any(f.startswith(EVERYTHING) for f in files):
        return [f"check {c}" for c in names] + [f"flag {n}" for n in ("wasm", *FLAGS)]
    # A component's lock file alone changing (a dependency elsewhere moved)
    # is a reason to test it, not to lint its unchanged code again.
    own = lambda c: [f for f in files if f.startswith(f"components/{c}/")]
    check = {c for c in names if any(f != f"components/{c}/Cargo.lock" for f in own(c))}
    test = {c for prefix, c in READ_BY.items() if any(f.startswith(prefix) for f in files)}
    test |= {c for c in names if own(c)} - check
    touched = check | test
    grew = True
    while grew:
        more = {c for c in names if deps[c] & touched} - touched
        touched |= more
        grew = bool(more)
    tier = lambda c: "check" if c in check else "test" if c in test else "build"
    lines = [f"{tier(c)} {c}" for c in names if c in touched]
    if any(f.startswith(GATE) for f in files):
        return lines + [f"flag {n}" for n in ("wasm", *FLAGS)]
    if "web" in touched:
        lines.append("flag wasm")
    lines += [f"flag {n}" for n, prefixes in FLAGS.items() if any(f.startswith(prefixes) for f in files)]
    return lines


def changed():
    def git(*args):
        return subprocess.run(["git", *args], cwd=ROOT, capture_output=True, text=True).stdout.split("\n")

    base = os.environ.get("GATE_BASE") or git("merge-base", "HEAD", "origin/main")[0]
    files = git("diff", "--name-only", base) + git("ls-files", "--others", "--exclude-standard")
    return sorted({f for f in files if f})


def self_test():
    names = ["base", "syntax", "macro", "eval", "cli", "web"]
    deps = {"base": set(), "syntax": {"base"}, "macro": {"syntax"}, "eval": {"base"}, "cli": {"macro", "eval"}, "web": {"macro", "eval"}}
    assert plan(["docs/plan.md", "CHANGES.md"], names, deps) == []
    assert plan(["components/eval/crates/x/src/a.rs"], names, deps) == ["check eval", "build cli", "build web", "flag wasm"]
    assert plan(["components/base/x"], names, deps)[:2] == ["check base", "build syntax"]
    assert plan(["lib/Stats.xtl"], names, deps) == ["test macro", "build cli", "build web", "flag wasm", "flag literate"]
    assert plan(["spec/eval/a.case"], names, deps) == ["test cli"]
    assert plan(["components/eval/Cargo.lock"], names, deps) == ["test eval", "build cli", "build web", "flag wasm"]
    assert plan(["components/eval/Cargo.lock", "components/eval/crates/x/a.rs"], names, deps)[0] == "check eval"
    assert plan(["docs/emacs/xetal-mode.el"], names, deps) == ["flag literate", "flag emacs"]
    assert plan(["scripts/components.sh"], names, deps)[0] == "check base"
    gate = plan(["scripts/gate.sh", "components/eval/a.rs"], names, deps)
    assert gate[:3] == ["check eval", "build cli", "build web"] and "flag literate" in gate and "flag asks" in gate
    assert "flag asks" in plan(["docs/asks.toml"], names, deps)
    print("affected: self-test ok")


if __name__ == "__main__":
    if "--self-test" in sys.argv:
        self_test()
    else:
        names = components()
        print("\n".join(plan(changed(), names, depends(names))))
