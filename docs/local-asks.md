# Local asks: what a local agent runs

Work done in a cloud sandbox (the lint lane's branch
`claude/project-thread-v9efz0` and its PRs) is built, tested and
checked against the goldens there, but the sandbox has no agentrail,
sw-checklist, sw-markdown-checker, just or trunk, and Linux limits one
command-line argument to 128 KiB. So some of the gate and all of the
saga record are left for an agent on the user's machine. This file
lists that work. Per-step detail (what each step changed and ran) is
in `lanes/lint/HANDOFF.md`.

Run it on main after merging a lane PR, in the order below. Each
section says how to know it is done.

## 1. Before anything builds

- `git pull` on main; the merge commit of the PR is there.
- `scripts/check-busy.sh`: nothing else is building (no `trunk serve`,
  no other session's cargo). `just stop-serve` stops a live demo
  server.

## 2. The checks the sandbox could not run

| Check | Command | Why it was not run in the cloud |
|-------|---------|---------------------------------|
| Function and module limits | `sw-checklist` | not installed |
| ASCII-only markdown | `sw-markdown-checker -f "docs/*.md"` | not installed |
| The gate for the change | `just gate --affected` | no `just` |
| The just-* goldens | `scripts/reg.sh run` | they run `just` |
| cli-too-deep | `scripts/reg.sh run` | its 200 KB argument exceeds Linux's 128 KiB limit for one argument; fails on main in the sandbox too |
| Live demo (wasm32 with trunk) | part of `just pages` | no trunk |

Done when `just gate --affected` passes and `scripts/reg.sh run`
reports every golden passing. A sw-checklist failure is fixed by
expanding up and out (a new module or crate), never by an `#[allow]`;
the lint lane counted by hand that every module it touched is at or
under seven functions, so none is expected.

## 3. Pages

- `just pages`: rebuilds only the stale parts. Any change to Rust
  sources under `components/` makes the web and rosetta parts stale.
- `just pages --all` instead when a demo or library changed, or how
  programs are drawn or run (the lint lane's step 3a edited demos and
  libraries, so its merge needed `--all`).
- Done when `scripts/check-pages.sh` passes. Then `just publish` from
  the pushed main (pages/ is no longer committed, D164).

## 4. The saga record (agentrail)

Saga 40 (the linter, `xetal lint`; plan in `docs/plan.md`) has no
agentrail record at all: no lane saga under `lanes/lint/` and nothing
in the main `.agentrail/` (the main saga, web-playground, is a
different one and must not be touched). Record it as a lane, as the
other lanes did (CLAUDE.md, "Parallel lanes"):

1. Create the lane saga under `lanes/lint/.agentrail`, run with
   `agentrail --saga lanes/lint ...`; its plan is Saga 40's section of
   `docs/plan.md`. (Check `agentrail --help` for how the installed
   version creates a saga.)
2. Add one step per row below, begin it and complete it with the
   commit named, the decision and a one-line summary taken from
   CHANGES.md. The commit is the step's own commit, not the merge.
3. `git add` the new `lanes/lint/.agentrail` files and commit them
   with the record (`chore`). `agentrail audit` should then find no
   gap for these commits.
4. When the last step (8) is merged: complete with `--done` and
   `agentrail --saga lanes/lint archive --reason "lint lane merged"`.

| Step | Slug | Decision | Commit | PR |
|------|------|----------|--------|----|
| 1 | lint-mvp | D136, S12 | 0d4cc44 | #117 |
| 2 | lint-run-directives | D138 | 52350e7 | #119 |
| 3 | lint-unused | D139, D140 | d0611f5 | #119 |
| 3a | lint-clean-repo | D141 | aab1453 | #119 |
| 4 | lint-effects | D142 | 036f425 | #119 |
| 5 | lint-redundant | D143 | e6e51fe | #120 |
| 7 | lint-fix | D144 | 207f115 | #121 |
| 6 | lint-typed | D145 | c68db43, 27ece1c | #122 |
| 8 | lint-fix-more | D152, D149, D150 | the step 8 commit, once merged | the PR after #123 |

Steps 7 and 6 were done in that order (7 first); record them in the
order done. D137 is the linter's register entry (cd8c726), not a step.

## 5. Tickets to open

- `xetal doc --json demos/tour.xtl` gives `h:s_tarts` the type
  `Char -> Char -> Int` in about half of the runs and
  `(Num a, Truthy a) => Char -> Char -> a` in the others, same binary
  and file: nondeterminism in typing a library item for the doc model,
  probably iteration over a hash map. Found in step 3a; not fixed.
- If `scripts/reg.sh run` fails anything beyond what section 2
  expects, that is new: open a ticket naming the golden and the
  merge.

## 6. Hand back

Push main. Tell the lane (in the project thread) what failed, if
anything, so the next cloud step can fix it; a failure the lane caused
is fixed there, not locally, unless it blocks the gate.
