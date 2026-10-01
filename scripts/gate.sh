#!/usr/bin/env bash
# Full pre-commit gate: lock consistency, then format, lint and tests
# in every component workspace, then reg-rs goldens, sw-checklist
# conformance and markdown.
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
step() { printf '\n==> %s\n' "$*"; }

source scripts/components.sh

step "Cargo.lock consistency"
scripts/check-locks.sh
scripts/check-modes.sh
for c in "${COMPONENTS[@]}"; do
    step "components/$c: fmt --check, clippy -D warnings, test"
    (
        cd "components/$c"
        cargo fmt --all -- --check
        cargo clippy -q --all-targets --all-features -- -D warnings
        cargo test -q --workspace
    )
done
step "the live demo's engine builds for the browser (wasm32)"
(cd components/web && cargo check -q --target wasm32-unknown-unknown)
step "reg-rs goldens"
scripts/reg.sh run
step "every just recipe runs (scripts/just-smoke.sh)"
smoke="$(scripts/just-smoke.sh 2>&1)" || { echo "$smoke"; exit 1; }
echo "$smoke" | tail -1
step "Emacs mode and org-babel (ERT; skipped without Emacs)"
emacs_out="$(just test-emacs 2>&1)" || { echo "$emacs_out"; exit 1; }
echo "$emacs_out" | grep -E "Ran [0-9]+ tests|skipped" || true
step "built-in reference (docs/reference.md)"
python3 scripts/reference.py --check
step "annotated diagrams (docs/diagrams)"
scripts/diagrams.sh --check
step "literate documents (docs/literate)"
scripts/literate.sh --check
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
