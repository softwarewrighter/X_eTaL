#!/usr/bin/env bash
# pages/ (the published live demo and literate pages) is built locally
# and committed, so it can fall behind what it shows. pages/INPUTS holds
# a hash of the contents of the demos, the libraries, the literate
# documents and the syntax poster's template as they were at the last
# `just pages`; this check recomputes
# it and fails when they have changed since. Run by the gate;
# scripts/build-pages.sh writes the stamp (--write).
#   scripts/check-pages.sh [--write]
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
inputs() {
    find demos lib userlibs docs/literate scripts/poster -type f \( -name '*.xtl' -o -name '*.org' -o -name '*.html' \) \
        | LC_ALL=C sort | xargs shasum -a 256 | shasum -a 256 | cut -d' ' -f1
}
if [ "${1:-}" = "--write" ]; then
    inputs > pages/INPUTS
    exit 0
fi
# Where pages/ cannot be built (no trunk: the cloud sandbox), the check
# is skipped; whoever merges with trunk rebuilds pages/.
if ! command -v trunk > /dev/null; then
    echo "check-pages: no trunk to rebuild pages/ with; skipped"
    exit 0
fi
if [ "$(cat pages/INPUTS 2>/dev/null)" != "$(inputs)" ]; then
    echo "check-pages: pages/ is stale (a demo, library or literate document changed): run just pages"
    exit 1
fi
echo "pages: current with the demos, libraries and literate documents"
