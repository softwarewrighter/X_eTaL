#!/bin/bash
# Build the syntax poster (pages/poster/index.html and, where Chrome is
# found, images/xetal-syntax-poster.png): scripts/poster.py does it.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/../.."
exec python3 scripts/poster.py
