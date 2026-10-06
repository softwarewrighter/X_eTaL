#!/usr/bin/env bash
# The pre-commit gate, in three sizes.
#   scripts/gate.sh              sample: the end-to-end checks only, in
#                                parallel (goldens, spec cases, doc tests,
#                                reference, status, data tables, spelling,
#                                markdown); about a minute. Before every
#                                commit, merge and push.
#   scripts/gate.sh --affected   the sample plus the components the change
#                                touches (scripts/affected.py plans it:
#                                check, test, compile), the browser build and
#                                the document checks whose inputs changed.
#                                Before agentrail complete of a code step.
#   scripts/gate.sh --full       everything. Nightly (scripts/nightly.sh),
#                                before a release.
# Every step of 5 seconds or more prints its time; the end lists the
# slowest three. "Changed" is measured from the merge base with
# origin/main (GATE_BASE names another).
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
mode=sample
case "${1:-}" in
    --full) mode=full ;;
    --affected) mode=affected ;;
    "") ;;
    *) echo "usage: scripts/gate.sh [--affected|--full]" >&2; exit 2 ;;
esac
started=$SECONDS
since=$SECONDS
times=""
last="start"
# A step's heading; the step before it is timed.
step() {
    if [ $((SECONDS - since)) -ge 5 ]; then printf '    (%ss)\n' $((SECONDS - since)); fi
    times="$times$((SECONDS - since)) $last\n"
    since=$SECONDS
    last="$*"
    printf '\n==> %s\n' "$*"
}

source scripts/components.sh

plan=""
if [ "$mode" = affected ]; then
    python3 scripts/affected.py --self-test
    plan="$(python3 scripts/affected.py)"
fi
# Whether the gate does this (a line of the plan): the full gate does
# all, the sample gate none of the planned steps.
does() { case "$mode" in full) true ;; sample) false ;; *) grep -qx "$1" <<< "$plan" ;; esac; }

step "nothing else is building here"
scripts/check-busy.sh
step "Cargo.lock consistency"
scripts/check-locks.sh
scripts/check-modes.sh
scripts/check-pages.sh
skipped=0
for c in "${COMPONENTS[@]}"; do
    if does "check $c"; then
        step "components/$c: fmt --check, clippy -D warnings, test"
        (
            cd "components/$c"
            cargo fmt --all -- --check
            cargo clippy -q --all-targets --all-features -- -D warnings
            cargo test -q --workspace
        )
    elif does "test $c"; then
        step "components/$c: test (files it builds in or tests against changed)"
        (cd "components/$c" && cargo test -q --workspace)
    elif does "build $c"; then
        step "components/$c: compiles (it depends on what changed)"
        (cd "components/$c" && cargo check -q --workspace)
    else
        skipped=$((skipped + 1))
    fi
done
if does "flag wasm"; then
    step "the live demo's engine and the Rosetta page build for the browser (wasm32)"
    (cd components/web && cargo check -q --target wasm32-unknown-unknown)
    (cd components/rosetta && cargo check -q --target wasm32-unknown-unknown)
fi
# The goldens and the rest run target/debug/xetal: built here, so it is
# current whatever the component plan did.
step "the CLI (debug) for the end-to-end checks"
(cd components/cli && cargo build -q -p xetal-cli)
# The end-to-end checks share nothing once the CLI is built: all at once,
# each to its own log, reported in order when all are done.
step "end-to-end checks, in parallel: goldens, spec cases, doc tests, reference, status, data tables, spelling, markdown"
logs="$(mktemp -d "${TMPDIR:-/tmp}/gate.XXXXXX")"
run() { local name="$1"; shift; ( set +e; started_at=$SECONDS; "$@" > "$logs/$name" 2>&1; echo "$? $((SECONDS - started_at))" > "$logs/$name.status" ) & }
doctests() { for f in lib/*.xtl lib/*.xtlm; do target/debug/xetal doc --test "$f" || return 1; done; echo "doc tests: lib/ passes"; }
run goldens scripts/reg.sh run
run spec bash -c 'cd components/cli && cargo test -q -p xetal-cli --test spec'
run doctests doctests
run reference python3 scripts/reference.py --check
run status python3 scripts/status.py --check
run rosetta python3 scripts/rosetta-check.py
run rosetta-run python3 scripts/rosetta-run.py
run idioms bash -c 'python3 scripts/idioms.py --self-test && python3 scripts/idioms.py --check'
run spelling bash -c 'python3 scripts/check-spelling.py --self-test && python3 scripts/check-spelling.py'
run markdown sw-markdown-checker -f README.md
wait
failed=0
for name in goldens spec doctests reference status rosetta rosetta-run idioms spelling markdown; do
    read -r code took < "$logs/$name.status"
    printf '    %-10s %3ss  %s\n' "$name" "$took" "$(tail -1 "$logs/$name" | cut -c1-100)"
    if [ "$code" -ne 0 ]; then failed=1; printf '\n--- %s failed:\n' "$name"; tail -40 "$logs/$name"; fi
done
rm -rf "$logs"
[ "$failed" -eq 0 ] || { echo "gate: an end-to-end check failed"; exit 1; }
if does "flag smoke"; then
    step "every just recipe runs (scripts/just-smoke.sh)"
    smoke="$(scripts/just-smoke.sh 2>&1)" || { echo "$smoke"; exit 1; }
    echo "$smoke" | tail -1
fi
if does "flag emacs"; then
    step "Emacs mode and org-babel (ERT; skipped without Emacs)"
    emacs_out="$(just test-emacs 2>&1)" || { echo "$emacs_out"; exit 1; }
    echo "$emacs_out" | grep -E "Ran [0-9]+ tests|skipped" || true
fi
if does "flag asks"; then
    step "asks ledger (docs/asks.md, each repro run)"
    python3 scripts/asks.py --check
fi
if does "flag diagrams"; then
    step "annotated diagrams (docs/diagrams)"
    scripts/diagrams.sh --check
    scripts/latex-gallery.sh
fi
if does "flag literate"; then
    step "literate documents (docs/literate)"
    docs=""
    if [ "$mode" = fast ]; then docs="$(python3 scripts/affected.py --literate)"; fi
    # shellcheck disable=SC2086
    scripts/literate.sh --check $docs
fi
if [ "$mode" != sample ]; then
step "sw-checklist (structure; the affected and full gates)"
checklist="$(sw-checklist . 2>&1)" || { echo "$checklist"; exit 1; }
echo "$checklist" | tail -1
if ! echo "$checklist" | grep -q ' 0 failed'; then
    sw-checklist -v . 2>&1 | grep FAIL
    exit 1
fi
fi
# README.md stays ASCII (images for any glyph). Other docs may hold
# Unicode (APL glyphs render on GitHub; HTML character references in
# code blocks do not), so only the README is checked while the rule
# and the checker are settled with the user.
step "done"
printf 'slowest steps:\n'; printf "$times" | sort -n -r | head -3 | sed 's/^/    /'
if [ "$mode" = affected ]; then
    printf 'affected gate: %s of %s components skipped (unaffected)\n' "$skipped" "${#COMPONENTS[@]}"
fi
printf 'gate: all checks passed (%s, %ss)\n' "$mode" $((SECONDS - started))
