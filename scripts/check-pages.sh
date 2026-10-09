#!/usr/bin/env bash
# pages/ (the published live demo and its pages) is built locally and
# committed, so it can fall behind what it shows. It is built in parts,
# each from its own inputs:
#   web       the live demo: the demos and libraries built into it
#   rosetta   the Rosetta stone's page: its programs, libraries and data
#   literate  the literate documents' HTML: the documents, their style,
#             and the libraries and demos they include
#   doc       the documentation site: the libraries, demos and userlibs,
#             and the doc tool's own sources
#   poster    the syntax poster: its template
#   latex     the LaTeX gallery: every line of code shipped or documented
# pages/INPUTS holds one line per part, the part's name and a hash of
# its inputs as they were when the part was last built. This check
# recomputes the hashes and fails, naming the parts, when inputs have
# changed since. Run by the gate; scripts/build-pages.sh rebuilds the
# stale parts and writes their lines.
#   scripts/check-pages.sh            # check every part
#   scripts/check-pages.sh --stale    # print the stale parts, one per line
#   scripts/check-pages.sh --write PART...   # record these parts as built
set -euo pipefail
export LC_ALL=C
cd "$(dirname "${BASH_SOURCE[0]}")/.."
PARTS="web rosetta literate doc poster latex"

# The files a part is built from, one per line.
files() {
    case "$1" in
        # The live demo and the Rosetta page compile the language in: any
        # Rust source can change what they show.
        web) find demos lib userlibs -type f \( -name '*.xtl' -o -name '*.xtlm' -o -name '*.toml' \); find components -path '*/target' -prune -o -path '*/src/*' -type f \( -name '*.rs' -o -name '*.css' -o -name '*.html' -o -name '*.js' -o -name '*.toml' \) -print ;;
        rosetta) find demos/rosetta lib -type f; echo components/rosetta/crates/xetal-rosetta/index.html; find components -path '*/target' -prune -o -path '*/src/*' -type f -name '*.rs' -print ;;
        literate) find docs/literate lib userlibs demos -type f \( -name '*.org' -o -name '*.css' -o -name '*.xtl' -o -name '*.xtlm' \) ;;
        doc) find lib demos userlibs -type f \( -name '*.xtl' -o -name '*.xtlm' \); find components/doc components/docsearch -path '*/target' -prune -o -type f \( -name '*.rs' -o -name '*.css' -o -name '*.js' \) -print; echo scripts/doc-site.sh ;;
        poster) find scripts/poster -type f -name '*.html' ;;
        # The gallery reads the session lines (six spaces in) of the markdown:
        # only the files that have any count.
        latex) find demos lib userlibs docs/literate spec -type f \( -name '*.xtl' -o -name '*.org' -o -name '*.case' \); grep -l '^      [^ ]' README.md docs/*.md || true ;;
    esac
}

# A part's hash: the contents of its files, by name.
hash_of() {
    files "$1" | sort | xargs shasum -a 256 | shasum -a 256 | cut -d' ' -f1
}

# The hash recorded for a part when it was last built.
recorded() {
    [ -f pages/INPUTS ] && awk -v p="$1" '$1 == p { print $2 }' pages/INPUTS || true
}

stale() {
    for part in $PARTS; do
        [ "$(recorded "$part")" = "$(hash_of "$part")" ] || echo "$part"
    done
}

case "${1:-}" in
    --write)
        shift
        touch pages/INPUTS
        for part in "$@"; do
            { grep -v "^$part " pages/INPUTS || true; echo "$part $(hash_of "$part")"; } > pages/INPUTS.new
            sort pages/INPUTS.new > pages/INPUTS
            rm pages/INPUTS.new
        done
        exit 0 ;;
    --stale) stale; exit 0 ;;
esac
# Where pages/ cannot be built (no trunk: the cloud sandbox), the check
# is skipped; whoever merges with trunk rebuilds pages/.
if ! command -v trunk > /dev/null; then
    echo "check-pages: no trunk to rebuild pages/ with; skipped"
    exit 0
fi
behind="$(stale | tr '\n' ' ')"
if [ -n "$behind" ]; then
    echo "check-pages: pages/ is stale in: ${behind}(their inputs changed): run just pages"
    exit 1
fi
echo "pages: current with the demos, libraries and literate documents"
