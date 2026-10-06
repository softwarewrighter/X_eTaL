#!/usr/bin/env bash
# The machine is free to build: nothing else is compiling in this
# repository's target/. A `trunk serve` (just web) watches the source
# tree and rebuilds the live demo on every change, taking the cargo
# lock and the CPU from every build and gate; a cargo of another
# session does the same. Run first by the gate and by just pages.
#   scripts/check-busy.sh          # fail when something else is building
#   scripts/check-busy.sh --stop   # stop a trunk serve of this repository
set -uo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
serving="$(pgrep -f "trunk serve" || true)"
if [ "${1:-}" = "--stop" ]; then
    [ -n "$serving" ] && kill $serving && echo "check-busy: stopped trunk serve ($serving)" || echo "check-busy: no trunk serve running"
    exit 0
fi
if [ -n "$serving" ]; then
    echo "check-busy: a trunk serve is running (pid $serving); it rebuilds on every change and holds the cargo lock: just stop-serve"
    exit 1
fi
# Another cargo holding this target's lock: not one of this process's own.
lock="$root/target/debug/.cargo-lock"
holder="$(lsof -t "$lock" 2>/dev/null | grep -v "^$$\$" | head -1 || true)"
if [ -n "$holder" ] && ! pstree -p $$ 2>/dev/null | grep -q "($holder)"; then
    echo "check-busy: another cargo (pid $holder, $(ps -o etime= -p "$holder" | tr -d ' ') old) holds target/debug: wait for it or stop it"
    exit 1
fi
echo "machine: free to build"
