#!/usr/bin/env bash
# Run a file as a notebook (xetal run --echo), with the flags its #!
# line gives xetal run (--untyped), so a file that runs untyped as a
# script shows untyped too.
#   scripts/show.sh FILE [DELAY_MS]
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
file="$1"
flags=()
if head -1 "$file" | grep -q -- '--untyped'; then flags+=(--untyped); fi
if [ -n "${2:-}" ]; then flags+=(--delay "$2"); fi
exec "$root/target/release/xetal" run --echo ${flags[@]+"${flags[@]}"} "$file"
