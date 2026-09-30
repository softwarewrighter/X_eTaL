#!/usr/bin/env bash
# Generate the annotated diagrams (docs/diagrams/NAME.notes ->
# images/NAME-annotated.svg) with `xetal diagram`. With --check, fail if
# a committed diagram differs from what its notes generate.
#   scripts/diagrams.sh [--check]
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
scripts/quick-build.sh
status=0
for notes in docs/diagrams/*.notes; do
    name="$(basename "$notes" .notes)"
    svg="images/$name-annotated.svg"
    fresh="$(./target/release/xetal diagram "$notes")"
    if [ "${1:-}" = "--check" ]; then
        if [ "$fresh" != "$(cat "$svg" 2>/dev/null)" ]; then
            echo "diagrams: $svg is not what $notes generates (run scripts/diagrams.sh)"
            status=1
        fi
    else
        printf '%s\n' "$fresh" > "$svg"
        echo "$svg"
    fi
done
exit "$status"
