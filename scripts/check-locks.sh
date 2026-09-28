#!/usr/bin/env bash
# Every component is its own workspace with its own Cargo.lock, and a
# lock also pins path dependencies owned by other components, so a
# manifest change in one component can leave another's lock stale.
#   scripts/check-locks.sh          # verify only
#   scripts/check-locks.sh --fix    # regenerate stale locks
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$root/scripts/components.sh"
stale=0
for c in "${COMPONENTS[@]}"; do
    ws="$root/components/$c"
    if ! (cd "$ws" && cargo metadata --locked --format-version 1 >/dev/null 2>&1); then
        if [ "${1:-}" = "--fix" ]; then
            echo "regenerating stale lock: components/$c"
            (cd "$ws" && cargo metadata --format-version 1 >/dev/null)
        else
            echo "STALE LOCK: components/$c (run scripts/check-locks.sh --fix)"
            stale=1
        fi
    fi
done
[ "$stale" -eq 0 ] && echo "all component locks consistent"
exit "$stale"
