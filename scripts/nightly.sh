#!/usr/bin/env bash
# The nightly full gate: bring main up to date, run everything, keep
# the log, and open a GitHub issue when it fails (closing the open one
# when it passes again). Scheduled by scripts/nightly.plist (launchd);
# run it by hand with `just nightly`.
#   scripts/nightly.sh
set -uo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
export PATH="$HOME/.cargo/bin:/opt/homebrew/bin:/usr/local/bin:$PATH"
mkdir -p work/nightly
stamp="$(date +%Y-%m-%d)"
log="work/nightly/$stamp.log"
{
    echo "nightly gate, $(date), $(git rev-parse --short HEAD)"
    git fetch -q origin && git merge -q --ff-only origin/main 2>&1
    scripts/gate.sh --full
} > "$log" 2>&1
status=$?
title="nightly gate failed"
open="$(gh issue list --state open --search "$title in:title" --json number -q '.[0].number' 2>/dev/null)"
if [ "$status" -ne 0 ]; then
    body="$(printf 'The full gate failed on %s at %s.\n\nLast lines of %s:\n\n```\n%s\n```\n' "$stamp" "$(git rev-parse --short HEAD)" "$log" "$(tail -40 "$log")")"
    if [ -n "$open" ]; then gh issue comment "$open" --body "$body" > /dev/null 2>&1
    else gh issue create --title "$title" --body "$body" > /dev/null 2>&1; fi
    echo "nightly: FAILED ($log)"
else
    [ -n "$open" ] && gh issue close "$open" --comment "The full gate passed on $stamp at $(git rev-parse --short HEAD)." > /dev/null 2>&1
    echo "nightly: passed ($log)"
fi
exit "$status"
