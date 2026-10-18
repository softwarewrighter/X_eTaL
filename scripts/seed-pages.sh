#!/usr/bin/env bash
# pages/ is the local build area of the published site, not tracked on
# main (the site is the gh-pages branch, scripts/publish-pages.sh). A
# fresh clone has no pages/, so every part would rebuild; this seeds it
# from what was last published, so only the stale parts do:
#   1. origin/gh-pages, the published site (its INPUTS stamps too);
#   2. else the last main commit that still tracked pages/ (before the
#      site moved to gh-pages).
# Does nothing when pages/INPUTS already exists.
#   scripts/seed-pages.sh
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
[ -f pages/INPUTS ] && exit 0
mkdir -p pages
if git rev-parse -q --verify origin/gh-pages > /dev/null; then
    git archive origin/gh-pages | tar -x -C pages
    echo "seed-pages: pages/ seeded from origin/gh-pages"
    exit 0
fi
# The commit that removed pages/INPUTS from main; its parent still had it.
gone="$(git rev-list -1 HEAD -- pages/INPUTS)"
if [ -n "$gone" ] && git cat-file -e "$gone^:pages/INPUTS" 2> /dev/null; then
    git archive "$gone^" pages | tar -x
    echo "seed-pages: pages/ seeded from ${gone:0:7}^ (the last main commit that tracked it)"
    exit 0
fi
echo "seed-pages: nothing to seed pages/ from; every part will rebuild"
