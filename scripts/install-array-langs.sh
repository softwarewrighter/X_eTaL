#!/usr/bin/env bash
# Install the array-language interpreters the Rosetta stone's cells are
# checked against (scripts/rosetta-run.py), under tools/ in the
# repository (gitignored), so the check is the same on every machine:
#   - Uiua 0.19.1, built from crates.io with cargo (the version whose
#     primitive documentation the cells follow; the website renders its
#     docs from the same source)
#   - CBQN, built from github.com/dzaima/CBQN with make (a C toolchain)
# Nothing is installed outside the repository; delete tools/ to undo.
#   scripts/install-array-langs.sh            # both
#   scripts/install-array-langs.sh uiua|bqn   # one
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
mkdir -p tools
which="${1:-all}"

uiua() {
    if [ -x tools/bin/uiua ]; then echo "uiua: tools/bin/uiua is there ($(tools/bin/uiua --version))"; return; fi
    echo "uiua: building 0.19.1 from crates.io (a few minutes)"
    cargo install --locked uiua@0.19.1 --no-default-features --features binary --root tools
    tools/bin/uiua --version
}

bqn() {
    if [ -x tools/bin/bqn ]; then echo "bqn: tools/bin/bqn is there ($(tools/bin/bqn --version | head -1))"; return; fi
    echo "bqn: building CBQN from GitHub (a few minutes)"
    rm -rf tools/src/CBQN
    mkdir -p tools/src tools/bin
    git clone -q --depth 1 https://github.com/dzaima/CBQN.git tools/src/CBQN
    (cd tools/src/CBQN && make o3 > ../cbqn-build.log 2>&1) || { tail -20 tools/src/cbqn-build.log; exit 1; }
    cp tools/src/CBQN/BQN tools/bin/bqn
    tools/bin/bqn --version | head -1
}

case "$which" in
    all) uiua; bqn ;;
    uiua) uiua ;;
    bqn) bqn ;;
    *) echo "usage: scripts/install-array-langs.sh [uiua|bqn]" >&2; exit 2 ;;
esac
