#!/usr/bin/env bash
# A video and its frames, for reading code shown on screen frame by
# frame. Downloads the video (at most 1080p, video only unless
# --audio), then extracts frames with ffmpeg: every N seconds, or at
# each scene change (a new slide or screen of code), named by the
# time they show. Needs yt-dlp (uv tool install yt-dlp) and ffmpeg.
#   scripts/video-get.sh URL                   # frames every 5 s
#   scripts/video-get.sh URL --every 2         # every 2 s
#   scripts/video-get.sh URL --scenes [0.3]    # at scene changes (threshold)
#   scripts/video-get.sh URL --from 10:51 --to 14:00   # only a part
#   scripts/video-get.sh URL --audio           # keep the sound too
# Quote the URL in zsh (its ? and & are special), or give the bare
# video id: scripts/video-get.sh qE5JRyqj7Ck.
# Into work/video/<id>/ (video.mp4, frames/t0651.png = 10:51).
set -euo pipefail
url="${1:?usage: scripts/video-get.sh URL [--every N | --scenes [T]] [--from T] [--to T] [--audio]}"
shift
every=5 scenes="" from="" to="" audio=""
while [ $# -gt 0 ]; do
    case "$1" in
        --every) every="$2"; shift 2 ;;
        --scenes) if [[ "${2:-}" =~ ^[0-9.]+$ ]]; then scenes="$2"; shift 2; else scenes=0.3; shift; fi ;;
        --from) from="$2"; shift 2 ;;
        --to) to="$2"; shift 2 ;;
        --audio) audio=1; shift ;;
        *) echo "video-get: unknown option $1" >&2; exit 2 ;;
    esac
done
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
for tool in yt-dlp ffmpeg; do
    command -v "$tool" >/dev/null || { echo "video-get: $tool not found" >&2; exit 1; }
done
id="$(yt-dlp --no-warnings --print id "$url")"
dir="$root/work/video/$id"
mkdir -p "$dir/frames"
if [ ! -f "$dir/video.mp4" ]; then
    format="bv*[height<=1080][ext=mp4]/bv*[height<=1080]"
    [ -n "$audio" ] && format="bv*[height<=1080]+ba/b[height<=1080]"
    yt-dlp --no-warnings -f "$format" --merge-output-format mp4 --remux-video mp4 \
        -o "$dir/video.%(ext)s" "$url"
fi
secs() { awk -F: '{ s = 0; for (i = 1; i <= NF; i++) s = s * 60 + $i; print s }' <<<"$1"; }
range=()
[ -n "$from" ] && range+=(-ss "$(secs "$from")")
[ -n "$to" ] && range+=(-to "$(secs "$to")")
start="$([ -n "$from" ] && secs "$from" || echo 0)"
tmp="$dir/frames/.new"
rm -rf "$tmp" && mkdir -p "$tmp"
if [ -n "$scenes" ]; then
    ffmpeg -loglevel error "${range[@]}" -i "$dir/video.mp4" -copyts \
        -vf "select='gt(scene,$scenes)',showinfo" -fps_mode vfr "$tmp/%05d.png" 2>"$tmp/info.log"
    times=($(grep -o 'pts_time:[0-9.]*' "$tmp/info.log" | cut -d: -f2))
else
    ffmpeg -loglevel error "${range[@]}" -i "$dir/video.mp4" -vf "fps=1/$every" "$tmp/%05d.png"
    times=()
fi
n=0
for f in "$tmp"/*.png; do
    [ -e "$f" ] || continue
    if [ -n "$scenes" ]; then t="${times[$n]%.*}"; else t=$((start + n * every)); fi
    mv "$f" "$(printf '%s/frames/t%04d.png' "$dir" "$t")"
    n=$((n + 1))
done
rm -rf "$tmp"
echo "video-get: $n frames in $dir/frames (t<seconds>.png)"
