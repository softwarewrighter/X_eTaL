#!/usr/bin/env bash
# Programs are executable and libraries are not: every demos/**/*.xtl is
# executable and starts with #!, except a library beside a demo, whose
# name starts with a capital letter as lib/'s do (demos/rosetta/Stone.xtl);
# every lib/*.xtl, userlibs/*.xtl and such a library is not executable.
# Run by the gate.
#   scripts/check-modes.sh
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
status=0
for f in $(find demos -name '*.xtl' | sort); do
    case "$(basename "$f")" in [A-Z]*) continue ;; esac
    [ -x "$f" ] || { echo "check-modes: $f is a program: chmod +x it"; status=1; }
    head -1 "$f" | grep -q '^#!' || { echo "check-modes: $f is a program: start it with #!/usr/bin/env xetal"; status=1; }
done
for f in lib/*.xtl userlibs/*.xtl $(find demos -name '[A-Z]*.xtl' | sort); do
    [ ! -x "$f" ] || { echo "check-modes: $f is a library: chmod -x it"; status=1; }
done
[ "$status" = 0 ] && echo "file modes: programs executable, libraries not"
exit "$status"
