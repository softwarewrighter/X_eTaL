#!/usr/bin/env bash
# The pre-commit gate.
#   scripts/gate.sh          the fast gate: what the change affects
#   scripts/gate.sh --full   everything, whatever changed
# The fast gate (scripts/affected.py plans it) checks the components
# whose files changed (format, lint, tests), tests the components that
# depend on them, skips the rest, and runs a slower document check only
# when its inputs changed; the cheap checks (locks, goldens, doc tests,
# reference, status, checklist, markdown) always run. "Changed" is
# measured from the merge base with origin/main (GATE_BASE names
# another), so it covers everything not yet pushed. Run the full gate
# between features, after a batch of merges, and before a release.
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
mode=fast
case "${1:-}" in
    --full) mode=full ;;
    "") ;;
    *) echo "usage: scripts/gate.sh [--full]" >&2; exit 2 ;;
esac
started=$SECONDS
since=$SECONDS
# A step's heading; the step before it is timed when it took a while.
step() {
    if [ $((SECONDS - since)) -ge 5 ]; then printf '    (%ss)\n' $((SECONDS - since)); fi
    since=$SECONDS
    printf '\n==> %s\n' "$*"
}

source scripts/components.sh

python3 scripts/affected.py --self-test
plan=""
if [ "$mode" = fast ]; then plan="$(python3 scripts/affected.py)"; fi
# Whether the gate does this (a line of the plan); the full gate does all.
does() { [ "$mode" = full ] || grep -qx "$1" <<< "$plan"; }

step "Cargo.lock consistency"
scripts/check-locks.sh
scripts/check-modes.sh
scripts/check-pages.sh
skipped=0
for c in "${COMPONENTS[@]}"; do
    if does "check $c"; then
        step "components/$c: fmt --check, clippy -D warnings, test"
        (
            cd "components/$c"
            cargo fmt --all -- --check
            cargo clippy -q --all-targets --all-features -- -D warnings
            cargo test -q --workspace
        )
    elif does "test $c"; then
        step "components/$c: test (it depends on what changed)"
        (cd "components/$c" && cargo test -q --workspace)
    else
        skipped=$((skipped + 1))
    fi
done
if does "flag wasm"; then
    step "the live demo's engine and the Rosetta page build for the browser (wasm32)"
    (cd components/web && cargo check -q --target wasm32-unknown-unknown)
    (cd components/rosetta && cargo check -q --target wasm32-unknown-unknown)
fi
# The goldens run target/debug/xetal: current even when cli was skipped.
(cd components/cli && cargo build -q -p xetal-cli)
step "reg-rs goldens"
scripts/reg.sh run
# Every ## >> example in lib/ prints what its doc shows (S10).
step "doc tests (xetal doc --test over lib/)"
for f in lib/*.xtl lib/*.xtlm; do
    report="$(target/debug/xetal doc --test "$f" 2>&1)" || { echo "$report"; exit 1; }
done
echo "doc tests: lib/ passes"
if does "flag smoke"; then
    step "every just recipe runs (scripts/just-smoke.sh)"
    smoke="$(scripts/just-smoke.sh 2>&1)" || { echo "$smoke"; exit 1; }
    echo "$smoke" | tail -1
fi
if does "flag emacs"; then
    step "Emacs mode and org-babel (ERT; skipped without Emacs)"
    emacs_out="$(just test-emacs 2>&1)" || { echo "$emacs_out"; exit 1; }
    echo "$emacs_out" | grep -E "Ran [0-9]+ tests|skipped" || true
fi
step "built-in reference (docs/reference.md)"
python3 scripts/reference.py --check
step "status table (docs/status.md)"
python3 scripts/status.py --check
step "the Rosetta stone's data (demos/rosetta/data.toml)"
python3 scripts/rosetta-check.py | tail -1
if does "flag asks"; then
    step "asks ledger (docs/asks.md, each repro run)"
    python3 scripts/asks.py --check
fi
if does "flag diagrams"; then
    step "annotated diagrams (docs/diagrams)"
    scripts/diagrams.sh --check
    scripts/latex-gallery.sh
fi
if does "flag literate"; then
    step "literate documents (docs/literate)"
    scripts/literate.sh --check
fi
step "sw-checklist"
checklist="$(sw-checklist . 2>&1)" || { echo "$checklist"; exit 1; }
echo "$checklist" | tail -1
if ! echo "$checklist" | grep -q ' 0 failed'; then
    sw-checklist -v . 2>&1 | grep FAIL
    exit 1
fi
# README.md stays ASCII (images for any glyph). Other docs may hold
# Unicode (APL glyphs render on GitHub; HTML character references in
# code blocks do not), so only the README is checked while the rule
# and the checker are settled with the user.
step "markdown (README ASCII-only)"
sw-markdown-checker -f README.md
step "spelling (American only)"
python3 scripts/check-spelling.py --self-test
python3 scripts/check-spelling.py
step "done"
if [ "$mode" = fast ]; then
    printf 'fast gate: %s of %s components skipped (unchanged); run scripts/gate.sh --full between features\n' "$skipped" "${#COMPONENTS[@]}"
fi
printf 'gate: all checks passed (%s, %ss)\n' "$mode" $((SECONDS - started))
