#!/usr/bin/env bash
# Full pre-commit gate: format, lint, tests, reg-rs goldens,
# sw-checklist conformance, markdown.
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
step "sw-checklist"
checklist="$(sw-checklist . 2>&1)" || { echo "$checklist"; exit 1; }
echo "$checklist" | tail -1
if ! echo "$checklist" | grep -q ' 0 failed'; then
    sw-checklist -v . 2>&1 | grep FAIL
    exit 1
fi
step "markdown (ASCII-only)"
sw-markdown-checker -f README.md
sw-markdown-checker -f "docs/*.md"
printf '\ngate: all checks passed\n'
