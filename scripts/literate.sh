#!/usr/bin/env bash
# Run the literate documents (docs/literate/*.org) with org-babel in a
# batch Emacs, recording each block's result in the file, after
# scripts/literate-draw.py puts each block's drawn form above it (--check
# fails when either is out of date). Blocks with `:results file :file
# ../../images/NAME.svg` save their pictures there; --check runs each
# document in a copy of that layout and fails when a picture differs. Blocks find
# libraries of your own (userlibs/) through XETAL_PATH, since Emacs runs
# them from the document's directory. Commit what
# changes; scripts/check-literate.sh fails until then.
#   scripts/literate.sh [--check]   # --check: run on copies and compare
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
emacs="${EMACS:-}"
[ -n "$emacs" ] || ! command -v emacs > /dev/null 2>&1 || emacs=emacs
[ -n "$emacs" ] || [ ! -x /Applications/Emacs.app/Contents/MacOS/Emacs ] || emacs=/Applications/Emacs.app/Contents/MacOS/Emacs
if [ -z "$emacs" ]; then echo "literate: no Emacs; skipped"; exit 0; fi
scripts/build-all.sh --release -q > /dev/null
check="${1:-}"
# One document: run it (in a copy, with --check) and, checking, compare.
one() {
    local doc="$1" target="$1" tree=""
    if [ "$check" = "--check" ]; then
        tree="$(mktemp -d "${TMPDIR:-/tmp}/literate.XXXXXX")"
        mkdir -p "$tree/docs/literate" "$tree/images"
        target="$tree/docs/literate/$(basename "$doc")"
        cp "$doc" "$target"
    fi
    scripts/literate-draw.py "$target"
    XETAL_PATH="$root/userlibs:$root/demos/rosetta" XETAL_BIN="$root/target/release/xetal" "$emacs" --batch -Q -L docs/emacs \
        -l docs/emacs/test/literate-run.el "$target" > /dev/null 2>&1 \
        || { echo "literate: $doc failed to run"; return 1; }
    if [ "$check" != "--check" ]; then
        echo "$doc"
        return 0
    fi
    local status=0
    if ! diff -u "$doc" "$target"; then
        echo "literate: $doc results changed (run scripts/literate.sh and commit)"
        status=1
    fi
    # The pictures blocks drew: the links recorded as results.
    for picture in $(grep -A1 '^#+RESULTS:' "$doc" | grep -o 'file:\.\./\.\./images/[A-Za-z0-9_.-]*\.svg' | sed 's#file:\.\./\.\./##' | sort -u); do
        if ! cmp -s "$tree/$picture" "$picture"; then
            echo "literate: $doc draws $picture differently (run scripts/literate.sh and commit)"
            status=1
        fi
    done
    rm -rf "$tree"
    return "$status"
}

# Every document at once (each in its own Emacs), the results in order.
logs="$(mktemp -d "${TMPDIR:-/tmp}/literate-logs.XXXXXX")"
for doc in docs/literate/*.org; do
    ( one "$doc" > "$logs/$(basename "$doc").log" 2>&1; echo $? > "$logs/$(basename "$doc").status" ) &
done
wait
status=0
for doc in docs/literate/*.org; do
    cat "$logs/$(basename "$doc").log"
    [ "$(cat "$logs/$(basename "$doc").status")" = 0 ] || status=1
done
rm -rf "$logs"
exit "$status"
