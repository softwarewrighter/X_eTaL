#!/usr/bin/env bash
# The performance regression check (Saga 30): every program in bench/ is
# timed with the release build (the best of RUNS wall times) and
# compared with this machine's baseline, bench/baseline/HOST.tsv. A
# program more than LIMIT percent slower than its baseline, and slower
# by more than a noise floor of FLOOR ms, fails the check. --bless
# records the new times as the baseline; blessing a slowdown needs the
# user's approval (say so in the commit message).
#   scripts/bench-check.sh [--bless] [RUNS]
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
bless=false
if [ "${1:-}" = "--bless" ]; then bless=true; shift; fi
runs="${1:-5}"
limit="${BENCH_LIMIT:-15}"
floor="${BENCH_FLOOR:-15}"
host="$(hostname -s)"
baseline="bench/baseline/$host.tsv"
scripts/build-all.sh --release -q > /dev/null
xetal=target/release/xetal
export XETAL_DRAW="${XETAL_DRAW:-work/bench-draw}"

# The best of $runs wall times of one program, in milliseconds.
best() {
    local best="" t start end
    for _ in $(seq "$runs"); do
        start=$(date +%s%N)
        "$xetal" run --seed 1 "$1" > /dev/null 2>&1
        end=$(date +%s%N)
        t=$(( (end - start) / 1000000 ))
        if [ -z "$best" ] || [ "$t" -lt "$best" ]; then best=$t; fi
    done
    echo "$best"
}

# The baseline time of one program, or nothing.
was() {
    if [ -f "$baseline" ]; then awk -F '\t' -v p="$1" '$1 == p { print $2 }' "$baseline"; fi
}

times="$(mktemp)"
failed=0
printf '%-26s %10s %8s %8s\n' "program" "baseline" "now" "change"
for p in bench/*.xtl; do
    now="$(best "$p")"
    printf '%s\t%s\n' "$p" "$now" >> "$times"
    base="$(was "$p")"
    if [ -z "$base" ]; then
        printf '%-26s %10s %6sms %8s\n' "$p" "-" "$now" "new"
        continue
    fi
    change=$(( (now - base) * 100 / (base > 0 ? base : 1) ))
    mark=""
    if [ $(( now * 100 )) -gt $(( base * (100 + limit) )) ] && [ $(( now - base )) -gt "$floor" ]; then
        mark="  SLOWER"
        failed=1
    fi
    printf '%-26s %8sms %6sms %7s%%%s\n' "$p" "$base" "$now" "$change" "$mark"
done

if $bless; then
    mkdir -p bench/baseline
    mv "$times" "$baseline"
    echo "bench-check: baseline recorded in $baseline"
    exit 0
fi
rm -f "$times"
if [ ! -f "$baseline" ]; then
    echo "bench-check: no baseline for $host; record one with just bench-bless"
    exit 2
fi
if [ "$failed" -ne 0 ]; then
    echo "bench-check: slower than the baseline by more than $limit% (see SLOWER);"
    echo "fix it, or with the user's approval record a new baseline (just bench-bless)"
    exit 1
fi
echo "bench-check: within $limit% of the baseline"
