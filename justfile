# X_eTaL tasks. Recipes call scripts/*.sh, which hold the logic for
# the component workspaces (components/<name>/), so the gate has one
# implementation. `just` alone lists the recipes.

set positional-arguments

xetal := "./target/debug/xetal"

# List the recipes
default:
    @just --list

# Build every component (debug) into the shared target/
build:
    scripts/build-all.sh

# Build every component optimized (target/release/xetal)
build-release:
    scripts/build-all.sh --release

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

# Type-check and run a script: just run demos/factorial.xtl
run file: _quiet-build
    @{{xetal}} run "$1"

# Evaluate an expression: just eval "'+ r_/ 1 2 3"
eval expr: _quiet-build
    @{{xetal}} eval -e "$1"

# The language tour: every feature, commented (pretty-printed, then run)
tour: _quiet-build
    @{{xetal}} render --color demos/tour.xtl
    @echo
    @{{xetal}} run demos/tour.xtl

# Pretty-print a file like cat: decorated and highlighted (errors in red)
pp file: _quiet-build
    @{{xetal}} render --color "$1"

# Edit a file with the live decorated view: just edit demos/life.xtl
edit file: _quiet-build
    @{{xetal}} edit "$1"

# Step the Life blinker (demos/life.xtl)
life: _quiet-build
    @{{xetal}} run demos/life.xtl

# Play the rotate demo (demos/rotate.xtl) as an animation
animate: _quiet-build
    #!/usr/bin/env bash
    {{xetal}} run demos/rotate.xtl | awk -v RS= '{ printf "\033[H\033[2J%s\n", $0; system("sleep 0.4") }'

_quiet-build:
    @scripts/build-all.sh -q > /dev/null
