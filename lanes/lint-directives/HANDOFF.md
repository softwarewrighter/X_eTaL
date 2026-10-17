# lint-directives lane: handoff for the local agent

The `# lint:` comment scanner (xetal-linter `directive.rs`) assumed
single-line strings with `\"` escapes, which predates raw strings
(ST4). It now finds comments from `xetal_lex::lex`: a comment is the
first `#` in a gap between tokens. Any string form the lexer knows
(plain, raw, and the coming `c"..."` code strings) is a token, so no
second scanner has to learn it. Text that does not lex has no
directives (it does not load either).

Built in a cloud sandbox without agentrail, reg-rs, just or
sw-checklist, so this lane has no `.agentrail` of its own.

## What the sandbox ran

Checked on main at c58204b (binds.rs restored) with this branch
rebased onto it:

- components/lint: `cargo fmt --check`, `cargo clippy --all-targets
  --all-features -- -D warnings`, `cargo test`: clean.
- The probes r2, r2plain, r3 and r4 from the lint plan: r3 now warns
  `mutable-constant` at line 4; r2 and r4 are allowed.
- Every golden in `reg/` by a stand-in for `reg-rs run` (stdout,
  stderr, exit status): the same set fails with and without this
  change (just, timing and sandbox goldens; roll-dice is random), so
  nothing moved.
- `scripts/check-spelling.py`: ok. The cli, doc, lint, rosetta and web locks
  each gained the one new edge (`scripts/check-locks.sh`: consistent).
- sw-checklist by hand: `directive.rs` has 5 functions, each under 25
  lines.

## To do locally

1. `scripts/gate.sh --affected`.
2. Nothing to rebase: no golden output changed.
