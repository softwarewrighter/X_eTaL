#!/usr/bin/env bash
# Build the live demo (components/web/crates/xetal-web) into pages/, which
# is committed: the Pages workflow publishes that folder as it is. Then
# export the literate documents into pages/literate/, build the
# documentation site into pages/doc/ (xetal doc) and screenshot the
# built page into images/live-demo.png for the README.
#   scripts/build-pages.sh
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
dist="$root/target/dist-pages"
cd "$root/components/web/crates/xetal-web"
trunk build --release --public-url /X_eTaL/ --dist "$dist"
# The Rosetta stone's page (docs/rosetta.md), at /X_eTaL/rosetta/.
cd "$root/components/rosetta/crates/xetal-rosetta"
trunk build --release --public-url /X_eTaL/rosetta/ --dist "$dist/rosetta"
cd "$root"
mkdir -p "$root/pages"
touch "$root/pages/.nojekyll"
# pages/literate/ is written by scripts/literate-html.sh; keep it.
rsync -a --delete --exclude='.nojekyll' --exclude='INPUTS' --exclude='literate/' --exclude='poster/' --exclude='doc/' "$dist/" "$root/pages/"
"$root/scripts/literate-html.sh"
"$root/scripts/doc-site.sh"
python3 "$root/scripts/poster.py"
"$root/scripts/latex-gallery.sh" --write
"$root/scripts/live-screenshot.sh"
# What pages/ now shows, for the gate's staleness check.
"$root/scripts/check-pages.sh" --write
echo "pages/ built; commit it (git add pages/) and push to publish."
