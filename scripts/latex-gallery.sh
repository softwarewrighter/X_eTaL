#!/usr/bin/env bash
# Check that KaTeX accepts xetal render --latex for every line of code we
# ship or document (tools/katex/gallery.mjs); with --write, also draw the
# gallery, pages/latex/index.html. Needs node and tools/katex installed
# (cd tools/katex && npm ci); skipped without them. Run by the gate.
#   scripts/latex-gallery.sh [--write]
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
if ! command -v node > /dev/null || [ ! -d tools/katex/node_modules/katex ]; then
    echo "latex-gallery: no node or KaTeX (cd tools/katex && npm ci); skipped"
    exit 0
fi
scripts/build-all.sh --release -q > /dev/null
node tools/katex/gallery.mjs "$@"
