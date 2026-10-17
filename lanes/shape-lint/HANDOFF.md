# shape-lint lane: handoff for the local agent

Built in a cloud sandbox on branch `feat/lint-shape-huno3l` (lint plan
steps 13 and 14: /mnt/project-files/lint-plans/macro-and-string-lints.md
sections 4.4 and 4.5). The sandbox had no agentrail, reg-rs,
sw-checklist, sw-markdown-checker, just or trunk.

## What the sandbox ran

- `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D
  warnings` and `cargo test` in base, eval, core, lint and cli: clean
  (spec cases included); `cargo check --all-targets` in web, doc,
  rosetta and tui.
- `scripts/check-locks.sh`, `scripts/check-spelling.py`,
  `scripts/status.py --check`, `scripts/reference.py --check`: clean.
- Every golden, by a stand-in for `reg-rs run` (stdout, stderr, exit):
  all pass but the eight that fail on main in this sandbox too
  (cli-too-deep, roll-dice, roll-range, just-*). New goldens
  lint-shape, lint-shape-pedantic, lint-strings, lint-fix-strings were
  written from reviewed runs; lint-list.out rebased on purpose.
- sw-checklist limits counted by hand: at most 6 functions a module
  (xetal-shape infer.rs), the longest 40 lines (infer::infer).

## To do locally, in order

1. `scripts/reg.sh run`: confirm the new goldens with reg-rs itself.
2. `just gate --affected` (sw-checklist, markdown).
3. `just pages`: the live demo embeds the catalog (new built-in, six
   new lints), so the web parts are stale.
4. agentrail: record the lane (`--saga lanes/shape-lint`) or the main
   saga's lint steps, with this branch's commits.
5. Decision register: D170 (shape lint, s_hapeIs) and D171 (string
   lints) were numbered past the parallel lanes; renumber if needed.
   lang-choices B19 records s_hapeIs as proposed: the name and form
   wait on the user's pick.
