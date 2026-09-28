# higher-order

Saga 5 of X_eTaL (see docs/plan.md). Milestone M4 from docs/PRD.md:
higher-order built-ins over quoted functions and lambdas. F4, F5, F8
and F9 (quoting, applying function values, operand binding and
chaining) already work for user functions; this saga adds the B6
higher-order built-ins and the B7 search and random built-ins.

Specification: docs/lang-choices.md (governs). Decisions made with the
user when the saga was planned (record them in lang-choices B6/B7 and
the design.md register in step 1):

- `r_/` is a right fold along the leading axis (J/BQN/APL):
  `'- r_/ 1 2 3` is 2. Fold from the end in one pass, constant extra
  space (also over virtual ranges).
- `s_\` gives prefix reductions: item k is `r_/` of the first k items,
  so the last item of a scan equals the reduce (`'- s_\ 1 2 3` is
  1 -1 2). A linear path for associative built-ins is an implementation
  detail with identical results.
- Empty reduce identities: `+ -` 0, `* /` 1 (Float for `/`), `&` 1,
  `|` 0, `=` 1, `!=` 0; `m_ax`, `m_in`, user functions and lambdas give
  `error[no-identity]`.
- `e_ach : (a -> b) -> a -> b` is one built-in; dyadic use is by
  currying: when f applied to an item returns a function, `e_ach f A`
  is a pending item-wise application that is zipped with the next
  argument (`A '= e_ach B`). Results are scalars (A7 later).
- `i_nner` pairs the last axis of A with the first axis of B (APL/J):
  shape is (shape A without last), (shape B without first); vector
  with vector is a scalar.
- `r_oll!` is truly random by default; a seed (`--seed N` /
  `XETAL_SEED`) is only for tests that need it. Goldens use reg-rs's
  handling of nondeterministic output; tests assert ranges and that
  values (usually) differ.
- `s_ort` / `g_rade` are stable ascending on Ord types (numbers,
  Char), sorting major cells lexicographically; `g_rade` gives
  1-origin indices.
- Multi-axis reduce/scan (`'+ r_/_12`, R1) moves to Saga 6 with axis
  subscripts (A6).

Rules for every step: strict TDD (failing test first, rejection tests
for every accepted rule), tests are the spec (spec/**/*.case, crate
tests, proptest, reg-rs goldens), no panics on any input, built-ins
added to builtins.toml first, design new code to the stricter gates
(25 LOC/fn, 5 fns/module, 5 modules/crate, 5 crates/component; lib.rs
a facade) and expand up and out (eval is at 5 crates: higher-order
built-ins go in a new component). Run the gate before committing,
commit (including .agentrail/ and Cargo.lock files) before agentrail
complete, push to origin/main.

## Steps

1. reduce-scan -- a way for built-ins to apply function values (the
   machine implements an apply callback; a new component for the
   higher-order built-ins); `r_/` and `s_\` on the leading axis with
   the identities above; types `(a -> a -> a) -> a -> a`; record the
   saga decisions in lang-choices and design.md.
2. each-table -- `e_ach` (monadic, and dyadic by currying) and
   `t_able` (outer product), typed; shape errors.
3. inner-compose-swap -- `i_nner` (last axis with first axis),
   `c_ompose`, `s_wap`.
4. search-builtins -- `i_ndexOf`, `m_ember?`, `u_nique`, `s_ort`,
   `g_rade`, `w_here`.
5. roll -- `r_oll!` (random 1..n, item-wise), seed flag and env var
   for tests, reg-rs goldens tolerant of random output, range and
   difference tests.
6. hof-properties -- proptests: reduce over concat, last of scan is
   the reduce, each of identity, table shape, sort is sorted and a
   permutation, grade selects the sort.
7. m4-docs-release -- README M4 tour (every command a golden), docs
   sync, Saga 5 retrospective in docs/plan.md.
