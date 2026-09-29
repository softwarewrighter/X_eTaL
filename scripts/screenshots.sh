#!/usr/bin/env bash
# Regenerate the README and docs images from images/tapes/*.tape with
# vhs (https://github.com/charmbracelet/vhs) and ImageMagick: each tape
# records a terminal session; its last frame becomes images/NAME.png.
# Runs are seeded, so the images are reproducible.
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
scripts/build-all.sh -q > /dev/null
mkdir -p target/screens
for tape in images/tapes/*.tape; do
    name="$(basename "$tape" .tape)"
    vhs "$tape" > /dev/null
    magick "target/screens/$name.gif" -coalesce -delete 0--2 "images/$name.png"
    echo "images/$name.png"
done
