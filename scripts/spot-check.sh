#!/usr/bin/env bash
# A spot check after a push (CLAUDE.md, "Checks"): a random subset of
# the goldens and every spec case, against the debug CLI, about a
# minute. It does not gate: run it in the background and fix forward
# what it finds. The full suite is the nightly's.
#   scripts/spot-check.sh          # the next fifth of the goldens
#   SPOT=40 scripts/spot-check.sh  # the next that many
#   SPOT=all scripts/spot-check.sh # every golden (the sample gate's set)
# The subset cycles: each run takes the goldens after the last run's
# (work/spot.cursor, starting at random), so five runs cover them all.
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
ulimit -n 4096 2>/dev/null || true
(cd components/cli && cargo build -q -p xetal-cli)
export GATE_BUILT=1
names=(reg/*.rgt)
names=("${names[@]##*/}")
names=("${names[@]%.rgt}")
total=${#names[@]}
want="${SPOT:-$((total / 5))}"
failed=0
if [ "$want" = all ]; then
    scripts/reg.sh run || failed=1
else
    mkdir -p work
    cursor="work/spot.cursor"
    start=$(( $(cat "$cursor" 2>/dev/null || echo $((RANDOM % total))) % total ))
    picked="$(printf '%s\n' "${names[@]}" "${names[@]}" | sed -n "$((start + 1)),$((start + want))p")"
    echo $(( (start + want) % total )) > "$cursor"
    echo "spot: $want of $total goldens, from number $((start + 1))"
    while read -r name; do
        [ -n "$name" ] || continue
        scripts/reg.sh run -p "$name" > /dev/null 2>&1 || { failed=1; echo "  FAIL: $name"; }
    done <<< "$picked"
fi
(cd components/cli && cargo test -q -p xetal-cli --test spec 2>&1 | grep -E "^test result|cases failed|\.case:") || failed=1
if [ "$failed" -ne 0 ]; then echo "spot: a check failed; fix forward"; exit 1; fi
echo "spot: all picked checks pass"
