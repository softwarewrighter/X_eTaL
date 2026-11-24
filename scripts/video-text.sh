#!/usr/bin/env bash
# A video's text, without the video: its description, chapters and
# details (info.json), and its English captions (manual or automatic)
# turned into plain text with timestamps, for reading what a talk
# covers and where. Needs yt-dlp (uv tool install yt-dlp).
#   scripts/video-text.sh URL            # into work/video/<id>/
#   scripts/video-text.sh URL DIR        # into DIR
# Quote the URL in zsh (its ? and & are special), or give the bare
# video id: scripts/video-text.sh qE5JRyqj7Ck.
# Writes description.txt, info.json, chapters.txt, captions.vtt and
# captions.txt (one "mm:ss text" line per caption, repeats dropped).
set -euo pipefail
url="${1:?usage: scripts/video-text.sh URL [DIR]}"
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
command -v yt-dlp >/dev/null || { echo "video-text: yt-dlp not found (uv tool install yt-dlp)" >&2; exit 1; }
id="$(yt-dlp --no-warnings --print id "$url")"
dir="${2:-$root/work/video/$id}"
mkdir -p "$dir"
yt-dlp --no-warnings --skip-download --write-description --write-info-json \
    -o "$dir/video.%(ext)s" "$url"
# Captions apart, so a refusal (HTTP 429, too many requests) keeps the rest;
# run again later for them.
yt-dlp --no-warnings --skip-download --write-subs --write-auto-subs --sub-langs en \
    --sub-format vtt --sleep-subtitles 3 -o "$dir/video.%(ext)s" "$url" ||
    echo "video-text: captions refused for now (run again later)" >&2
[ -f "$dir/video.description" ] && mv "$dir/video.description" "$dir/description.txt"
[ -f "$dir/video.info.json" ] && mv "$dir/video.info.json" "$dir/info.json"
vtt="$(ls "$dir"/video.*.vtt 2>/dev/null | head -1 || true)"
[ -n "$vtt" ] && mv "$vtt" "$dir/captions.vtt" && rm -f "$dir"/video.*.vtt
"$root/scripts/py" - "$dir" <<'EOF'
import json, re, sys, pathlib
d = pathlib.Path(sys.argv[1])
info = json.loads((d / "info.json").read_text())
def mmss(s):
    s = int(float(s)); return f"{s // 3600}:{s % 3600 // 60:02}:{s % 60:02}" if s >= 3600 else f"{s // 60}:{s % 60:02}"
chapters = info.get("chapters") or []
(d / "chapters.txt").write_text("".join(f"{mmss(c['start_time'])} {c['title']}\n" for c in chapters))
vtt = d / "captions.vtt"
if vtt.exists():
    out, last = [], ""
    for block in vtt.read_text().split("\n\n"):
        m = re.match(r"(\d+):(\d+):(\d+)\.\d+ -->", block.strip())
        if not m: continue
        text = " ".join(l for l in block.strip().splitlines()[1:] if "-->" not in l)
        text = re.sub(r"<[^>]+>", "", text).strip()
        if text and text != last and not last.endswith(text):
            h, mi, s = map(int, m.groups())
            out.append(f"{mmss(h * 3600 + mi * 60 + s)} {text}")
            last = text
    (d / "captions.txt").write_text("\n".join(out) + "\n")
print(f"video-text: {info.get('title')} ({mmss(info.get('duration', 0))}, {info.get('channel')}, {info.get('upload_date')})")
print(f"video-text: {len(chapters)} chapters; files in {d}")
EOF
