#!/usr/bin/env bash
# Build every component workspace (components/<name>/), in dependency
# order, into the one shared target/ directory (.cargo/config.toml).
#   scripts/build-all.sh             # debug build
#   scripts/build-all.sh --release   # release build (target/release/xetal)
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$root/scripts/components.sh"
for c in "${COMPONENTS[@]}"; do
    echo "==> build components/$c"
    (cd "$root/components/$c" && cargo build "$@")
done
