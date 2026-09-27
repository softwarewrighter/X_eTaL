#!/usr/bin/env bash
# Full pre-commit gate: format, lint, tests, reg-rs goldens, markdown.
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
step() { printf '\n==> %s\n' "$*"; }

step "cargo fmt --check"
cargo fmt --all -- --check
step "cargo clippy -D warnings"
cargo clippy --all-targets --all-features -- -D warnings
step "cargo test"
cargo test --workspace
step "reg-rs goldens"
scripts/reg.sh run
step "markdown (ASCII-only)"
sw-markdown-checker -f README.md
sw-markdown-checker -f "docs/*.md"
printf '\ngate: all checks passed\n'
