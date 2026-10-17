# lint-xtlm lane: handoff for the local agent

Saga 40 step 12 (lint-xtlm, D162): nine lints on macro libraries, built in
a cloud sandbox on branch `feat/lint-xtlm-gnxaix`. The sandbox had no
agentrail, reg-rs, sw-checklist, sw-markdown-checker, just or trunk,
so this lane has no `lanes/lint-xtlm/.agentrail` yet (agentrail files
are never written by hand). Record it locally after merging.

## What the sandbox ran

- `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D
  warnings` and `cargo test` in components core and lint; `cargo test`
  in cli (spec cases included): clean.
- `scripts/check-locks.sh --fix`: the cli, doc, lint, rosetta and web
  locks gain xetal-macrolint and xetal-doccom; consistent.
- Every golden, by a stand-in for `reg-rs run` comparing stdout, stderr
  and exit status: all pass but the just-* goldens (no `just`) and
  cli-too-deep, which fails the same way on main.
- `lint-list.out` rebased (six new lints, intended); the new golden
  `lint-macro-library` (reg/fixtures/lint/Slips.xtlm) was written from
  a real run and reviewed by eye.
- Doc tests over lib/, reference.py, check-markers, check-modes,
  check-pages, check-spelling: pass. status.py fails on main already
  (ST5 not in docs/status.md); not this lane's.
- sw-checklist limits counted by hand: xetal-macrolint has 4 modules,
  at most 5 functions a module, the longest about 25 lines.

## To do locally, in order

1. `scripts/reg.sh run`: confirm lint-list and lint-macro-library with
   reg-rs itself.
2. `just gate --affected` (sw-checklist, markdown).
3. `just pages`: the web and rosetta parts are stale (Rust sources).
4. agentrail: record step 12 (complete, with this branch's commit).
5. D162 and step 12 may need renumbering if another lint lane merged
   first with the same numbers.

## Contracts (after step 110)

Rebased onto 57cff93; `takes-mismatch`, `ignored-argument` and
`binds-unwritten` added over xetal-contract. Self-checked with fmt,
clippy and tests in core and lint (the repository test included);
`lint-list.out` refreshed from a real run (three more lints). Goldens
and gates are left to the integrator's train (CLAUDE.md, 2026-10-17).
