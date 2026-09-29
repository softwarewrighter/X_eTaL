#!/usr/bin/env bash
# Regenerate videos/NAME.webm (VP9, smallest) and videos/NAME.webp
# (animated, plays inline in markdown) from videos/tapes/*.tape, with
# vhs, ffmpeg and gif2webp. Formats compared for the tour (60 s of
# scrolling text): VP9 webm 0.2 MB, AV1 0.3 MB, H.264 0.5 MB, animated
# webp 1.6 MB, optimized gif 2.6 MB; scrolling defeats the frame
# differencing of image formats, so the inline copy is small and slow.
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
scripts/build-all.sh -q > /dev/null
mkdir -p target/screens videos
for tape in videos/tapes/*.tape; do
    name="$(basename "$tape" .tape)"
    vhs "$tape" > /dev/null
    src="target/screens/$name.mp4"
    ffmpeg -loglevel error -y -i "$src" -vf "fps=8,scale=880:-1" \
        -c:v libvpx-vp9 -crf 42 -b:v 0 -row-mt 1 -an "videos/$name.webm"
    ffmpeg -loglevel error -y -i "$src" -vf "fps=4,scale=760:-1:flags=lanczos,split[a][b];[a]palettegen=max_colors=24[p];[b][p]paletteuse=dither=none" \
        "target/screens/$name-760.gif"
    gif2webp -lossy -q 35 -m 4 "target/screens/$name-760.gif" -o "videos/$name.webp" > /dev/null
    ls -l "videos/$name.webm" "videos/$name.webp" | awk '{ print $9, $5 }'
done
