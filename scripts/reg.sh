#!/usr/bin/env bash
# Wrap reg-rs so baselines live in the repo under reg/.
#   scripts/reg.sh run            # build, then run every baseline
#   scripts/reg.sh <reg-rs args>  # any other reg-rs command
# Baseline commands run from the repo root against target/debug/xetal
# (slow demos such as TTTML against target/release/xetal).
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
export REG_RS_DATA_DIR="$root/reg"
# Pictures a golden draws go to work/draw (gitignored), never the repo.
export XETAL_DRAW="${XETAL_DRAW:-work/draw}"
command -v reg-rs >/dev/null || { echo "reg-rs not found on PATH" >&2; exit 127; }
# The gate builds both first (GATE_BUILT) so no two cargos relink the
# binary while a golden or the spec cases run it.
[ "${GATE_BUILT:-}" = 1 ] || (cd components/cli && cargo build -q -p xetal-cli && cargo build -q --release -p xetal-cli)
if [ "${1:-}" = "run" ] && [ "$#" -eq 1 ]; then
    # In parallel: every golden writes only under its own path (CLAUDE.md).
    # reg-rs starts every golden at once (260, several through bash -c
    # and pipes); macOS's default of 256 open files per process is too
    # few ("Too many open files", 2026-10-19), so raise the soft limit.
    ulimit -n 4096 2>/dev/null || true
    exec reg-rs run -p .rgt --parallel
fi
exec reg-rs "$@"
