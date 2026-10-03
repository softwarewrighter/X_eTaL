# transpose lane: Saga 24, transpose

A lane (branch per PR, from main) whose saga lives in
lanes/transpose/.agentrail; run agentrail with `--saga lanes/transpose`.

Goal: transpose, asked for by X_eTaL-demos (ask D9: attention, PCA),
ADVANCEDEX (BIN, FC, INV) and the classics (Mastermind's codes as
rows). Decided with the user on 2026-10-02 (to be recorded as
lang-choices B17):

- `o_\ A` (reserved in A2, B2) reverses the order of all axes, as in
  APL and J: a 2 3 4 array becomes 4 3 2; a vector or scalar is
  unchanged. Monadic only (a name has one arity, B9).
- `p t_ranspose A` permutes the axes: p is a permutation of 1..rank
  A, and axis i of A becomes axis p[i] of the result (APL's dyadic
  transpose without diagonals); a repeated or out-of-range axis is
  error[domain], a p of another length error[length].
- `o_\_jk A` swaps axes j and k (its own axis rule, like rotate and
  catenate); one digit or three are error[axis].
- `[]P_ATH` keeps taking 2 rows, x over y; programs write `o_\ pts`.

Rules: TDD; the full gate; one PR per step from the newest main.

Steps
1. transpose: B17 recorded; `o_\` and `t_ranspose` in the catalog, the
   kernel in the axes component, spec cases first (accept and reject),
   property tests (transposing twice is the identity; t_ranspose by
   the reversed identity is o_\), reference and design entries.
2. transpose-axes: `o_\_jk`, its axis rule and errors, spec cases.
3. transpose-docs: tour, README, idioms (APL `⍉`), reference, poster.
4. transpose-retrofit: programs that worked around transpose
   (dogfooding.md: Mastermind's codes, Turtle, TTTML's symmetries,
   and the rest the audit names), goldens rebased on purpose.
5. transpose-release: plan.md retrospective, the lane archived.
