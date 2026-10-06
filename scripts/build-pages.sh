#!/usr/bin/env bash
# Build pages/ (the published live demo and its pages), in parts: only
# the parts whose inputs changed since they were last built are rebuilt
# (scripts/check-pages.sh names the parts and their inputs). The Rust
# sources are not inputs: after a change to how programs are drawn or
# run, rebuild everything; after a change to a page's own crate,
# rebuild that part (`scripts/build-pages.sh web`, `... rosetta`).
#   scripts/build-pages.sh          # rebuild the stale parts
#   scripts/build-pages.sh --all    # rebuild every part
#   scripts/build-pages.sh web doc  # rebuild the parts named
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
case "${1:-}" in
    --all) parts="web rosetta literate doc poster latex" ;;
    "") parts="$(scripts/check-pages.sh --stale | tr '\n' ' ')" ;;
    *) parts="$*" ;;
esac
if [ -z "${parts// /}" ]; then
    echo "pages/ is current: nothing to rebuild (scripts/build-pages.sh --all rebuilds everything)"
    exit 0
fi
scripts/check-busy.sh
mkdir -p pages
touch pages/.nojekyll

# The live demo: trunk's build, copied over everything but the other parts.
web() {
    local dist="$root/target/dist-pages"
    (cd components/web/crates/xetal-web && trunk build --release --public-url /X_eTaL/ --dist "$dist")
    rsync -a --delete --exclude='.nojekyll' --exclude='INPUTS' --exclude='literate/' --exclude='poster/' --exclude='doc/' --exclude='latex/' --exclude='rosetta/' "$dist/" pages/
    scripts/live-screenshot.sh
}

# The Rosetta stone's page (docs/rosetta.md), at /X_eTaL/rosetta/.
rosetta() {
    local dist="$root/target/dist-rosetta"
    (cd components/rosetta/crates/xetal-rosetta && trunk build --release --public-url /X_eTaL/rosetta/ --dist "$dist")
    rsync -a --delete "$dist/" pages/rosetta/
}

for part in $parts; do
    echo "==> pages: $part"
    since=$SECONDS
    case "$part" in
        web) web ;;
        rosetta) rosetta ;;
        literate) scripts/literate-html.sh ;;
        doc) scripts/doc-site.sh ;;
        poster) python3 scripts/poster.py ;;
        latex) scripts/latex-gallery.sh --write ;;
        *) echo "build-pages: no part named $part (web rosetta literate doc poster latex)" >&2; exit 2 ;;
    esac
    scripts/check-pages.sh --write "$part"
    echo "    ($((SECONDS - since))s)"
done
echo "pages/ built ($parts): commit it (git add pages/) and push to publish."
