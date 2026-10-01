#!/usr/bin/env bash
# Build the live demo (components/web/crates/xetal-web) into pages/, which
# is committed: the Pages workflow publishes that folder as it is. Then
# export the literate documents into pages/literate/ and screenshot the
# built page into images/live-demo.png for the README.
#   scripts/build-pages.sh
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
dist="$root/target/dist-pages"
cd "$root/components/web/crates/xetal-web"
trunk build --release --public-url /X_eTaL/ --dist "$dist"
mkdir -p "$root/pages"
touch "$root/pages/.nojekyll"
# pages/literate/ is written by scripts/literate-html.sh; keep it.
rsync -a --delete --exclude='.nojekyll' --exclude='literate/' "$dist/" "$root/pages/"
"$root/scripts/literate-html.sh"
"$root/scripts/latex-gallery.sh" --write
"$root/scripts/live-screenshot.sh"
echo "pages/ built; commit it (git add pages/) and push to publish."
