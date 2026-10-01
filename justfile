# X_eTaL tasks. Recipes call scripts/*.sh, which hold the logic for
# the component workspaces (components/<name>/), so the gate has one
# implementation. `just` alone lists the recipes.

set positional-arguments

# The recipes run the optimized build (programs run about ten times
# faster than in the debug build); scripts/quick-build.sh rebuilds it
# only when a source changed.
xetal := "./target/release/xetal"

# Pictures shown with []S_HOW land in work/draw (gitignored), named after
# the program: demos/classics/life-drawn.xtl draws life-drawn-1.svg, ...
export XETAL_DRAW := env("XETAL_DRAW", "work/draw")

# List the recipes
default:
    @just --list

# Build every component (debug) into the shared target/
build:
    scripts/build-all.sh

# Build every component optimized (target/release/xetal: the run recipes use it)
build-release:
    scripts/build-all.sh --release

# Build optimized and install xetal and x_etal into a PATH directory (for #! scripts)
install dir="~/.local/bin": build-release
    #!/usr/bin/env bash
    set -euo pipefail
    dest="${1:-$HOME/.local/bin}"
    dest="${dest/#\~/$HOME}"
    mkdir -p "$dest"
    cp target/release/xetal "$dest/xetal"
    ln -sf xetal "$dest/x_etal"
    echo "installed $dest/xetal ($(target/release/xetal --version | head -1))"

# Run every component's tests
test:
    for c in $(scripts/components.sh); do (cd components/$c && cargo test -q --workspace) || exit 1; done

# Format every component
fmt:
    for c in $(scripts/components.sh); do (cd components/$c && cargo fmt --all); done

# Lint every component (warnings are errors)
clippy:
    for c in $(scripts/components.sh); do (cd components/$c && cargo clippy -q --all-targets --all-features -- -D warnings) || exit 1; done

# Run the reg-rs CLI goldens in reg/
reg:
    scripts/reg.sh run

# Check (or with --fix, regenerate) the component Cargo.lock files
locks *args:
    scripts/check-locks.sh "$@"

# The full pre-commit gate: locks, fmt, clippy, tests, goldens, checklist, markdown
gate:
    scripts/gate.sh

# Start the interactive session
repl: _quiet-build
    @{{xetal}} repl

# Type-check and run a script, flags first: just run --echo demos/tour.xtl
run +args: _quiet-build
    @{{xetal}} run "$@"

# Evaluate an expression, flags first: just eval --echo "'+ r_/ 1 2 3"
eval +args: _quiet-build
    #!/usr/bin/env bash
    {{xetal}} eval "${@:1:$#-1}" -e "${@: -1}"

# The language tour: every feature, commented, each line with its output
tour: _quiet-build
    @{{xetal}} run --echo demos/tour.xtl

# Run a file as a notebook: each statement pretty-printed, then its output (#! flags such as --untyped apply)
show file: _quiet-build
    @scripts/show.sh "$1"

# A notebook run that pauses after each statement (ms, default 500): for watching or recording
slow-show file delay="500": _quiet-build
    @scripts/show.sh "$1" "$2"

# The Emacs mode and Org Babel language, with ERT in a batch Emacs (skipped without Emacs)
test-emacs: build
    #!/usr/bin/env bash
    set -euo pipefail
    emacs=${EMACS:-}
    [ -n "$emacs" ] || ! command -v emacs >/dev/null 2>&1 || emacs=emacs
    [ -n "$emacs" ] || [ ! -x /Applications/Emacs.app/Contents/MacOS/Emacs ] || emacs=/Applications/Emacs.app/Contents/MacOS/Emacs
    if [ -z "$emacs" ]; then echo "test-emacs: no Emacs; skipped"; exit 0; fi
    "$emacs" --batch -Q -L docs/emacs -l docs/emacs/test/xetal-tests.el -f ert-run-tests-batch-and-exit

# Run the literate documents (docs/literate/*.org), recording each block's result
literate:
    scripts/literate.sh

# The literate documents' recorded results are current
check-literate:
    scripts/literate.sh --check

# Draw images/name-forms.png, the name every way (docs/name.md, the live demo's Help)
name-image:
    scripts/name-image.sh

# Export the literate documents to HTML under pages/literate/ (with an index page)
literate-html:
    scripts/literate-html.sh

# Build the live demo into pages/ (committed; the Pages workflow publishes it) and screenshot it
pages:
    scripts/build-pages.sh

alias serve := web

# Serve the live demo locally (http://127.0.0.1:8095/), rebuilt when a source changes
web:
    cd components/web/crates/xetal-web && trunk serve --release --port 8095 --address 127.0.0.1

# Regenerate docs/reference.md (every built-in, its examples run) from docs/reference/builtins.ref
reference: _quiet-build
    python3 scripts/reference.py

# Regenerate the annotated diagrams (images/*-annotated.svg) from docs/diagrams/*.notes
diagrams:
    scripts/diagrams.sh

# Regenerate the README / docs images (vhs, ImageMagick)
screenshots:
    scripts/screenshots.sh

# Regenerate the videos (vhs, ffmpeg, gif2webp)
videos:
    scripts/videos.sh

# Pretty-print a file like cat: decorated and highlighted (errors in red)
pp file: _quiet-build
    @{{xetal}} render --color "$1"

# Edit a file with the live decorated view: just edit demos/life.xtl
edit file: _quiet-build
    @{{xetal}} edit "$1"

# Life: a blinker and a glider, stepped (demos/life.xtl)
life: _quiet-build
    @{{xetal}} run demos/life.xtl

# TTTML: a machine that learns tic-tac-toe by playing itself, as a notebook (optimized build)
tttml: _quiet-build
    @{{xetal}} run --echo demos/tttml.xtl

# TTTML part 1: learn tic-tac-toe by playing itself and save the model in work/
tttml-train: _quiet-build
    @{{xetal}} run demos/tttml-train.xtl

# TTTML part 2: play the saved model (run tttml-train first); you are X
tttml-play: _quiet-build
    @{{xetal}} run demos/tttml-play.xtl

# Play the rotate demo (demos/rotate.xtl) as an animation
animate: _quiet-build
    #!/usr/bin/env bash
    {{xetal}} run demos/rotate.xtl | awk -v RS= '{ printf "\033[H\033[2J%s\n", $0; system("sleep 0.4") }'

_quiet-build:
    @scripts/quick-build.sh

# Run a program that draws ([]S_HOW), then open its first picture in the browser
draw file: _quiet-build
    @{{xetal}} run "$1"
    @scripts/open-picture.sh "$XETAL_DRAW/$(basename "$1" .xtl)-1.svg"

