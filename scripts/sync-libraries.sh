#!/usr/bin/env bash
# Vendor the X_eTaL-libraries collection for the live demo: its
# libraries (libs/NAME/src/NAME.xtl and .xtlm) and their demos
# (libs/NAME/demos/*.xtl) and their data files, at the commit in
# vendor/libraries/PIN, from
# the sibling repository (../X_eTaL-libraries, or XETAL_LIBRARIES).
#   scripts/sync-libraries.sh            # the pinned commit
#   scripts/sync-libraries.sh COMMIT     # re-pin to COMMIT, then copy
# The live demo embeds what is here (xetal-collection); the command
# line does not use it (it finds libraries on XETAL_PATH).
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
repo="${XETAL_LIBRARIES:-$root/../X_eTaL-libraries}"
dest="$root/vendor/libraries"
[ -d "$repo/.git" ] || { echo "sync-libraries: no repository at $repo (set XETAL_LIBRARIES)" >&2; exit 1; }
if [ "$#" -ge 1 ]; then
    git -C "$repo" rev-parse --verify "$1^{commit}" > "$dest/PIN"
fi
commit="$(tr -d '[:space:]' < "$dest/PIN")"
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
git -C "$repo" archive "$commit" libs | tar -x -C "$work"
rm -rf "$dest/libs" "$dest/demos" "$dest/data"
mkdir -p "$dest/libs" "$dest/demos" "$dest/data"
for lib in "$work"/libs/*/; do
    name="$(basename "$lib")"
    for f in "$lib"src/"$name".xtl "$lib"src/"$name".xtlm; do
        [ -f "$f" ] && cp "$f" "$dest/libs/"
    done
    if compgen -G "$lib"'demos/*.xtl' > /dev/null; then
        mkdir -p "$dest/demos/$name"
        cp "$lib"demos/*.xtl "$dest/demos/$name/"
    fi
    # A demo's data (cities.csv), which the live demo puts among your files.
    for f in "$lib"demos/*; do
        case "$f" in *.xtl) ;; *) [ -f "$f" ] && cp "$f" "$dest/data/" ;; esac
    done
done
echo "sync-libraries: $(ls "$dest/libs" | wc -l | tr -d ' ') library files, $(find "$dest/demos" -name '*.xtl' | wc -l | tr -d ' ') demos, at ${commit:0:12}"
