# lint lane: handoff for the local agent

Saga 40 step 1 (lint-mvp, D136) was built in a cloud sandbox on branch
`claude/peaceful-hamilton-g1hdp3`. That sandbox had no agentrail,
reg-rs, sw-checklist, just or trunk, so the following was not run or
recorded there. Do it locally after merging.

## What the sandbox ran

- `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D
  warnings` and `cargo test` in components core, lint and cli: clean
  (the cli build compiles every crate that depends on xetal-lint).
- `cargo test` in macro (all pass) and eval: one failure,
  `lambdas::closures_keep_captured_values` (`error[rebind]`, D135),
  which fails identically on the base commit dfb0e4c; not this lane's.
- `scripts/check-locks.sh`: consistent (cli and core locks updated).
- Every golden, by a stand-in for `reg-rs run` comparing stdout, stderr
  and exit status: all pass except those that fail on the base commit
  too (cli-too-deep, roll-dice, roll-range) and the just-* goldens
  (no `just`), release-binary goldens included.
- The new goldens `reg/lint-*.rgt` were written from real runs and
  reviewed by eye (reg-rs was not available to create them).
- sw-checklist limits counted by hand on the new and changed modules:
  at most 5 functions a module, the longest 43 lines (stages::run).

## To do locally, in order

1. `scripts/reg.sh run`: confirm the lint-* goldens with reg-rs itself.
2. `scripts/gate.sh --affected` (sw-checklist, spelling, markdown).
3. `just pages`: the web and rosetta parts are stale (any Rust source
   under components/ is one of their inputs).
4. agentrail: record Saga 40 and its step 1 (complete, with this
   branch's commit); the plan is in docs/plan.md, Saga 40.
5. (Decided, D139: the naming lint is pedantic; the bird variants stay.)

## Step 2 (lint-run-directives, D138)

Branch `claude/peaceful-hamilton-g1hdp3`, restarted from main at cd8c726.
The sandbox ran fmt, clippy and tests in lint, cli and web (native),
`cargo check` in doc, rosetta and tui (which depend on the changed
crates), the lock check (doc and rosetta locks regenerated), and every
golden by the stand-in runner: all pass but the eight that fail on main
too (cli-too-deep, roll-dice, roll-range, just-*). New goldens
lint-run-allow, lint-run-library-allow, lint-run-echo-allow were
written from reviewed runs. Not run: the wasm32 build of the live demo
(xetal-play now depends on xetal-linter, plain Rust with no new crates
from crates.io), reg-rs itself, sw-checklist, agentrail (record steps 1
and 2), `just pages` (web and rosetta parts are stale again).

## Step 3 (lint-unused, D139, D140)

Same branch, on top of step 2 (not yet merged). The sandbox ran fmt,
clippy and tests in core, lint, cli and web, the lock check and every
golden by the stand-in runner (the same eight fail as on main). Five
lint goldens were rebased on purpose (function-name-case is pedantic
now; unused-binding appears in style.xtl); lint-unused,
lint-explain-unused and lint-pedantic are new, from reviewed runs.
xetal-lint now has seven modules (lib, catalog, naming, registry,
unused, walk, warn): at the sw-checklist limit; the next pass goes in a
new crate. Not run: wasm32, reg-rs, sw-checklist, agentrail (record
steps 1 to 3), `just pages` (web and rosetta stale).

## Step 3a (lint-clean-repo, D141)

Demos and libraries edited (comments and `_ :=` only; every run golden
passes but run-echo-tour, rebased for the new comment line), so `just
pages` must rebuild the literate, doc and latex parts too, besides web
and rosetta. Found while checking that the doc models were unchanged,
not caused by this lane: `xetal doc --json demos/tour.xtl` gives
`h:s_tarts` the type `Char -> Char -> Int` in about half of the runs
and `(Num a, Truthy a) => Char -> Char -> a` in the others (same binary,
same file): nondeterminism in typing a library item for the doc model,
probably iteration over a hash map. Worth a ticket.

## Step 4 (lint-effects, D142)

New crate components/core/crates/xetal-bang; every component lock that
pins xetal-lint gained it (check-locks --fix). `Expr::children` moved
into xetal-ir (additive). Six demos renamed functions to end in `!`
(mastermind-play, mini-apl, life, rosetta/rosetta.xtl, rotate,
tttml-play): run goldens unchanged, but the live demo, the Rosetta
page and the literate pages show these sources, so `just pages --all`.
The sandbox built every component (scripts/build-all.sh) and ran fmt,
clippy and tests in core, lint, cli, web and macro, and every golden.
Not run: wasm32, reg-rs, sw-checklist, agentrail (steps 1 to 4).

## Step 5 (lint-redundant, D143)

Branch `claude/project-thread-v9efz0`, from main at 8002548. New crate
components/core/crates/xetal-complexity; every component lock that pins
xetal-lint gained it (check-locks --fix, additions only). The sandbox
ran fmt, clippy and tests in core, lint, cli, web and macro, the wasm32
checks of web and rosetta (the gate's browser build: clean, which also
covers the wasm32 build steps 2 to 4 could not run), every golden by a
stand-in runner (all pass but the eight that fail on main too:
cli-too-deep, roll-dice, roll-range, just-*), the doc tests of lib/,
reference, status, idioms, rosetta-check and the spelling check.
lib/Maybe.xtl, demos/tour.xtl and demos/classics/wordfreq.xtl changed,
so `just pages --all` (literate, doc and latex parts too). Not run:
reg-rs itself, sw-checklist, sw-markdown-checker, agentrail (record
steps 1 to 5), `just pages`.

## Step 7 (lint-fix, D144)

Branch `claude/project-thread-v9efz0`, restarted from main at 7ce5fd4
(after PR #120 merged). `xetal_base::Diagnostic` gained a boxed `fix`,
so every component rebuilds; no new crates, no lock changes. The
sandbox ran fmt, clippy and tests in base, core, lint, cli, web and
macro, clippy in every other component, the wasm32 checks of web and
rosetta, and every golden with reg-rs itself (built from
sw-cli-tools/reg-rs): 228 of 234 pass, the six failing being
cli-too-deep and the five just-* goldens (no `just` here), as on main.
New goldens lint-json, lint-fix, lint-fix-expr, created with reg-rs.
Not run: sw-checklist (counted by hand: command.rs at seven functions,
fix.rs five, every function under 30 lines), sw-markdown-checker,
agentrail (record steps 1 to 5 and 7), `just pages` (web and rosetta
parts stale: Rust sources changed).
