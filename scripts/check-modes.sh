#!/usr/bin/env bash
# Programs are executable and libraries are not: every demos/**/*.xtl is
# executable and starts with #!; every lib/*.xtl and userlibs/*.xtl is
# not executable. Run by the gate.
#   scripts/check-modes.sh
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
status=0
for f in $(find demos -name '*.xtl' | sort); do
    [ -x "$f" ] || { echo "check-modes: $f is a program: chmod +x it"; status=1; }
    head -1 "$f" | grep -q '^#!' || { echo "check-modes: $f is a program: start it with #!/usr/bin/env xetal"; status=1; }
done
for f in lib/*.xtl userlibs/*.xtl; do
    [ ! -x "$f" ] || { echo "check-modes: $f is a library: chmod -x it"; status=1; }
done
[ "$status" = 0 ] && echo "file modes: programs executable, libraries not"
exit "$status"
