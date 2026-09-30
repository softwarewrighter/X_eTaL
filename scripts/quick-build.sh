#!/usr/bin/env bash
# Build target/release/xetal for the just recipes, quickly: nothing runs
# when no source is newer than the binary; otherwise one cargo build of
# the CLI (which builds the components it depends on), announced on
# stderr so a long rebuild does not look like a hang.
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
bin="$root/target/release/xetal"
if [ -x "$bin" ] && [ -z "$(find "$root/components" "$root/lib" -newer "$bin" \
    \( -name '*.rs' -o -name '*.toml' -o -name 'Cargo.lock' -o -name '*.xtl' -o -name '*.txt' \) \
    -not -path '*/target/*' -print -quit)" ]; then
    exit 0
fi
echo "building xetal..." >&2
(cd "$root/components/cli" && cargo build -q --release -p xetal-cli)
touch "$bin"
