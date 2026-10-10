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
5. Decide with the user: lib/Combinators.xtl's bird variants (`C_s`,
   `E_h`, `C_ss`, ...) are `function-name-case` findings (11), as is
   demos/classics/fibonacci.xtl's `u:F_ib`. Either the style guide
   names them (a capital and a lowercase modifier), or the files say
   `# lint-file: allow function-name-case` / are renamed.

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
