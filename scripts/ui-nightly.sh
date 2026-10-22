#!/usr/bin/env bash
# The nightly UI tests of the live demos (docs/ui-nightly.md): bring
# main up to date, run scripts/ui-check.py against the published sites
# in a headless Chromium, keep the log, and open a GitHub issue when it
# fails (closing the open one when it passes again). Scheduled by the
# systemd user timer scripts/ui-nightly.timer on an always-on Linux
# machine; run it by hand with `just ui-nightly`.
#   scripts/ui-nightly.sh
set -uo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
export PATH="$HOME/.local/bin:$HOME/.cargo/bin:/usr/local/bin:$PATH"
mkdir -p work/ui
stamp="$(date +%Y-%m-%d)"
log="work/ui/$stamp.log"
{
    echo "nightly UI tests, $(date), $(git rev-parse --short HEAD)"
    git fetch -q origin && git merge -q --ff-only origin/main 2>&1
    uv run --no-project --with playwright python scripts/ui-check.py --quiet
} > "$log" 2>&1
status=$?
title="nightly UI tests failed"
open="$(gh issue list --state open --search "$title in:title" --json number -q '.[0].number' 2>/dev/null)"
if [ "$status" -ne 0 ]; then
    body="$(printf 'The UI tests of the live demos failed on %s (%s failure(s)).\n\nLast lines of %s:\n\n```\n%s\n```\n\nScreenshots are under work/ui/ on the machine that ran them.\n' "$stamp" "$status" "$log" "$(tail -40 "$log")")"
    if [ -n "$open" ]; then gh issue comment "$open" --body "$body" > /dev/null 2>&1
    else gh issue create --title "$title" --body "$body" > /dev/null 2>&1; fi
    echo "ui-nightly: FAILED ($log)"
else
    [ -n "$open" ] && gh issue close "$open" --comment "The UI tests passed on $stamp." > /dev/null 2>&1
    echo "ui-nightly: passed ($log)"
fi
exit "$status"
