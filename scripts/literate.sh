#!/usr/bin/env bash
# Run the literate documents (docs/literate/*.org) with org-babel in a
# batch Emacs, recording each block's result in the file. Blocks find
# the demos' own libraries (demos/Hello.xtl) through XETAL_PATH. Commit what
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
status=0
for doc in docs/literate/*.org; do
    target="$doc"
    if [ "$check" = "--check" ]; then
        target="$(mktemp -t literate).org"
        cp "$doc" "$target"
    fi
    XETAL_PATH="$root/demos" XETAL_BIN="$root/target/release/xetal" "$emacs" --batch -Q -L docs/emacs \
        -l docs/emacs/test/literate-run.el "$target" > /dev/null 2>&1 \
        || { echo "literate: $doc failed to run"; status=1; continue; }
    if [ "$check" = "--check" ]; then
        if ! diff -u "$doc" "$target"; then
            echo "literate: $doc results changed (run scripts/literate.sh and commit)"
            status=1
        fi
        rm -f "$target"
    else
        echo "$doc"
    fi
done
exit "$status"
