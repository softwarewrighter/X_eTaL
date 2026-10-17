# lint-capture lane: handoff for the local agent

Saga 40 steps 10 (lint-origins) and 11 (lint-capture), D162, built in a
cloud sandbox on branch `feat/lint-capture-s0r4mz`. The sandbox had no
agentrail, reg-rs, just, trunk or sw-markdown-checker, so the saga
record for this lane (`lanes/lint-capture/.agentrail`) could not be
written; the steps are recorded in docs/plan.md, Saga 40.

## What the sandbox ran

- `cargo fmt`, `cargo clippy --all-targets --all-features -- -D
  warnings` and `cargo test` in components macro, lint and core: clean.
  `cargo test` in cli (spec cases included): clean. `cargo clippy` in
  doc, web, rosetta, tui and expand (they depend on what changed): clean.
- `scripts/check-locks.sh`: consistent after `--fix` (cli, doc, lint,
  rosetta and web locks updated for the new crate xetal-origin).
- sw-checklist (built from github.com/softwarewrighter/sw-checklist):
  0 failed over the repository.
- Every golden, by a stand-in for `reg-rs run` comparing stdout, stderr
  and exit status: all pass except those that fail on the base commit
  in this sandbox too (cli-too-deep, roll-dice, roll-range) and the
  just-* goldens (no `just`). lint-list was rebased on purpose (three
  new lints listed).
- The new goldens (lint-capture, lint-capture-pedantic,
  lint-explain-capture) were written from real runs and reviewed by
  eye; reg-rs did not create them.
- `xetal doc --test` over lib/: passes. `scripts/check-spelling.py`:
  ok. `scripts/reference.py --check`: ok. `scripts/status.py --check`
  fails on the base commit too (ST5 from 5313094 is not in
  docs/status.md); not regenerated here, to leave it to its lane.

## Also in this branch

- Commit "fix: restore xetal-expand binds.rs": main at 0c525d7 did not
  build (5313094 deleted `components/expand/crates/xetal-expand/src/binds.rs`
  while lib.rs still declares it). If step 110 replaces that file, keep
  step 110's version when merging.

## To do locally, in order

1. `scripts/reg.sh run`: confirm the lint-* goldens with reg-rs itself.
2. `just gate --affected` (markdown checker included).
3. `just pages`: the parts with Rust sources as inputs are stale.
4. agentrail: record steps 10 and 11 for this lane (complete, with this
   branch's commits), or fold them into the main saga's Saga 40.
5. D162 is the next free number on main at 0c525d7; if another lane
   took it first, renumber here (docs/design.md, docs/plan.md,
   CHANGES.md, xetal-origin's crate docs do not cite it).
