#!/usr/bin/env bash
# Every just recipe runs without error: the file recipes on every demo,
# the others on a sample. A recipe this script does not know fails it,
# so a new recipe gets a smoke test (or a reason to skip) when added.
# Run by scripts/gate.sh.
set -uo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
failed=0
check() {
    local out
    if ! out="$("$@" 2>&1 </dev/null)"; then
        printf 'FAIL: %s\n%s\n' "$*" "$out" | head -20
        failed=1
    fi
}
skip() { printf 'skip %-15s %s\n' "$1" "$2"; }
# A program starts with #!; a library (lib/, userlibs/) does not.
program() { head -1 "$1" | grep -q '^#!'; }
for recipe in $(just --summary); do
    case "$recipe" in
    default | build | tour | life | animate | tttml | diagrams | reference) check just "$recipe" ;;
    # demos/tttml-play.xtl reads typed moves: the tttml-play check below
    # pipes some in.
    show) for f in demos/*.xtl; do program "$f" && [ "$f" != demos/tttml-play.xtl ] && check just show "$f"; done ;;
    pp) for f in demos/*.xtl; do check just pp "$f"; done ;;
    slow-show) check just slow-show demos/square.xtl 1 ;;
    run)
        for f in demos/*.xtl; do
            { [ "$f" = demos/tttml-play.xtl ] || ! program "$f"; } && continue
            flags=$(head -1 "$f" | grep -o -- '--untyped' || true)
            check just run $flags "$f"
        done
        ;;
    eval) check just eval "'+ r_/ 1 2 3" && check just eval --echo "r_ange 3" ;;
    repl) check bash -c "printf '1 + 2\n' | just repl" ;;
    tttml-train) check just tttml-train ;;
    tttml-play) check bash -c "just tttml-train && printf '5\n3\n4\n8\n9\n7\n2\n6\n1\n' | just tttml-play" ;;
    locks) check just locks ;;
    build-release) skip "$recipe" "the release profile of build (slow)" ;;
    install) skip "$recipe" "writes outside the repository" ;;
    test | fmt | clippy | reg | gate | test-emacs | check-literate)
        skip "$recipe" "run by the gate itself" ;;
    literate) skip "$recipe" "rewrites docs/literate; check-literate checks it" ;;
    screenshots | videos) skip "$recipe" "regenerates media (vhs)" ;;
    edit) skip "$recipe" "needs a terminal; the editor has its own tests" ;;
    web) skip "$recipe" "a server that runs until stopped; the gate checks the wasm32 build" ;;
    name-image) skip "$recipe" "regenerates images/name-forms.png (headless Chrome)" ;;
    literate-html) skip "$recipe" "rewrites pages/literate; run by pages" ;;
    pages) skip "$recipe" "rewrites pages/ and the screenshot; run before publishing" ;;
    *)
        printf 'FAIL: no smoke test for recipe %s (add one to %s)\n' "$recipe" "$0"
        failed=1
        ;;
    esac
done
[ "$failed" = 0 ] && echo "just-smoke: every recipe ran"
exit "$failed"
