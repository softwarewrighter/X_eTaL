#!/usr/bin/env bash
# Fail when a tracked or staged file holds a merge conflict marker
# (a line starting "<<<<<<< " or ">>>>>>> "), so a botched merge cannot
# be committed or pushed. Goldens (.rgt, .out, .err) may print anything
# and are skipped. Run by every gate.
#   scripts/check-markers.sh              # check this repository
#   scripts/check-markers.sh --self-test  # prove it catches a marker
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Both what is staged and the tracked files as they are now: a file in
# the middle of a merge holds its markers in the working tree only.
markers() {
    local found=1
    for where in --cached ""; do
        # shellcheck disable=SC2086
        git -C "$1" grep -nI $where -E '^(<<<<<<< |>>>>>>> )' -- . ':!*.rgt' ':!*.out' ':!*.err' && found=0
    done
    return $found
}

if [ "${1:-}" = --self-test ]; then
    t="$(mktemp -d)"
    trap 'rm -rf "$t"' EXIT
    git -C "$t" init -q
    printf 'clean\n' > "$t/a.md"
    git -C "$t" add a.md
    if markers "$t" > /dev/null; then echo "check-markers: self-test: a clean file was flagged"; exit 1; fi
    printf '<<<<<<< HEAD\nours\n=======\ntheirs\n>>>>>>> other\n' > "$t/a.md"
    git -C "$t" add a.md
    if ! markers "$t" > /dev/null; then echo "check-markers: self-test: a staged marker was missed"; exit 1; fi
    git -C "$t" commit -qm clean --allow-empty
    printf 'clean\n' > "$t/a.md"
    git -C "$t" add a.md
    printf '<<<<<<< HEAD\nours\n=======\ntheirs\n>>>>>>> other\n' > "$t/a.md"
    if ! markers "$t" > /dev/null; then echo "check-markers: self-test: a working-tree marker was missed"; exit 1; fi
    echo "check-markers: self-test ok"
    exit 0
fi

if markers "$root"; then
    echo "check-markers: conflict markers in the files above: resolve them"
    exit 1
fi
echo "check-markers: no conflict markers"
