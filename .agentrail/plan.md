# combinators

Saga 9 of X_eTaL (docs/plan.md), milestone M6: the combinators as the
standard library Combinators, function power, and the Maybe monad.

Decided with the user when the saga was planned (record in
lang-choices and the design.md register in step 1):
- Combinators.xtl holds every bird Smullyan names (To Mock a
  Mockingbird) that type-checks, under his letters as single-capital
  function names (`l:K_`, `l:B_1`-style spellings chosen in step 1),
  each with its inferred type pinned by a test.
- Y is in the library as a recursive definition,
  `l:Y_ := { f_ -> f_ l:Y_ 'f_ }`, typed `(a -> a) -> a`, working
  because the functional's parameter is lazy. The self-applying birds
  (M, L, U, the textbook Y and Z) are in an untyped demo.
- Function power (D-7): a superscript on any function name, `f_^3 x`,
  including a quoted operand (`'u:l_ife^4`); a symbol stays an error;
  `n 'f_ p_ower x` for computed counts; `^-1` stays reserved.
- A second standard library, Maybe (Church-encoded: n_othing, j_ust,
  b_ind and helpers), with demos/monads.xtl (safe division by bind).
- The tour gains a Combinators section (the web demo's tour uses it).

Rules: strict TDD, tests are the spec, no panics, stricter gates and
design for them up front, docs ASCII-only, gate before commit, commit
.agentrail with the work, push.

## Steps

1. aviary -- the list of Smullyan's birds with their definitions,
   which type-check and which need self-application, and their
   spellings; `xetal type` of a library file on its own (showing its
   exports), to pin the types; record the saga decisions.
2. power -- function power: `f_^n` on any function name (lexer,
   parser, Core as `p_ower`), the built-in `p_ower`, typed; rejection
   tests (a symbol, a computed exponent, `^-1`).
3. combinators-library -- lib/Combinators.xtl with every typable
   bird and the recursive Y; types pinned; the untyped birds in
   demos/birds-untyped.xtl (golden, --untyped).
4. maybe -- lib/Maybe.xtl and demos/monads.xtl; spec and goldens.
5. notebook -- demos/combinators.xtl as a notebook (just show), a
   literate docs/literate/birds.org, the tour's Combinators section;
   demos/life.xtl uses `u:l_ife^4`.
6. m6-docs-release -- M6 tour page, docs sync, retrospective.
