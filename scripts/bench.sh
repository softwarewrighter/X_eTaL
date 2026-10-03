#!/usr/bin/env bash
# Time the benchmarks (bench/*.xtl and a few demos) with the release
# build: each runs three times and the best wall time is printed, as a
# markdown table for docs/speed.md. Timing is not part of the gate (it
# depends on the machine); compare runs on one machine.
#   scripts/bench.sh [RUNS]
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
runs="${1:-3}"
scripts/build-all.sh --release -q > /dev/null
xetal=target/release/xetal
export XETAL_DRAW="${XETAL_DRAW:-work/bench-draw}"   # pictures out of the way
programs=(bench/*.xtl demos/classics/mandelbrot.xtl demos/classics/mastermind.xtl demos/tttml-train.xtl)

# The best of $runs wall times of one program, in seconds.
best() {
    local file="$1" best="" t start end
    for _ in $(seq "$runs"); do
        start=$(date +%s%N)
        "$xetal" run --seed 1 "$file" > /dev/null 2>&1
        end=$(date +%s%N)
        t=$(( (end - start) / 1000000 ))
        if [ -z "$best" ] || [ "$t" -lt "$best" ]; then best=$t; fi
    done
    printf '%d.%03d' $((best / 1000)) $((best % 1000))
}

echo "| Program | Best of $runs (s) |"
echo "| ------- | ---------------- |"
for p in "${programs[@]}"; do
    echo "| \`$p\` | $(best "$p") |"
done
