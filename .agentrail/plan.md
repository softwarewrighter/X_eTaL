# rotate-and-axes

Saga 6 of X_eTaL (see docs/plan.md). Milestone M5 from docs/PRD.md:
rotate `o_-` and reverse `r_ev` on the leading axis, axis subscripts
on any function by the move-to-front rule (A6), multi-axis rotate
giving every combination (A4) and multi-axis reduce and scan (R1).
With these the Life one-liner runs by general rules alone.

Specification: docs/lang-choices.md (governs): A1-A6, D-8, D-9, B6
(R1). Decisions made with the user when the saga was planned (record
them in lang-choices and the design.md register in step 1):

- Rotate direction as APL/J/BQN: a positive amount moves items toward
  the front: `1 o_- 1 2 3` is `2 3 1`.
- A list of amounts means every combination, whatever the subscript:
  one leading result axis per rotated axis, so `-1 0 1 o_- V` has
  shape 3 n (A4 applied uniformly).
- On a dyadic function `_k` moves axis k of the right (data)
  argument only (control arguments are on the left, F6/B10);
  `A c_at_2 B` is an error for now.
- After f runs on the moved array: a result of the same rank has
  axis 1 moved back to k; one rank less (f consumed the leading axis,
  as reduce does) is left as is; any other rank change is
  error[axis].

Rules for every step: strict TDD (failing test first, rejection tests
for every accepted rule, PLACEHOLDER then review blessed output rather
than guessing spans), tests are the spec, no panics, built-ins added
to builtins.toml first, stricter gates (25 LOC/fn, 5 fns/module, 5
modules/crate, 5 crates/component) and expand up and out. No
Life-specific code: Life must pass only through general rules. Gate
before commit; commit .agentrail with the work; push to origin/main.

## Steps

1. rotate-reverse -- `o_-` (amount on the left, leading axis, APL
   direction, a list of amounts gives every combination) and `r_ev`,
   typed, in a new component; record the saga decisions.
2. axis-subscripts -- A6 for any function, built-ins and user
   functions: move axis k of the right argument to the front, apply,
   move back by the result-rank rule; the function's argument count
   comes from its type (elaborated) or, untyped, its visible arity;
   axis validation (0 is a lex error; beyond the rank is error[axis];
   multi-digit subscripts only where a function defines them).
3. multi-axis -- `o_-_12` every combination (A4), `r_/_12` and
   `s_\_12` over each listed axis in turn (R1).
4. life-runs -- the pending Life acceptance case passes by general
   rules: flip it to active, confirm the docs-life-line golden, tell
   the user.
5. rotate-properties -- proptests: rotate by n then -n is identity,
   r_ev twice is identity, `f_1` equals f, rotating every combination
   then selecting one equals one rotate, axis reduce matches a model.
6. rotate-demo -- an animated 2-D rotate demo printing frames from the
   CLI (a demo script and golden).
7. m5-docs-release -- README M5 tour (every command a golden), docs
   sync, Saga 6 retrospective in docs/plan.md.
