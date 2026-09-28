# X_eTaL -- Implementation Plan

Development is driven by agentrail sagas (one active saga in
`.agentrail/`, finished or superseded sagas archived to
`.agentrail-archive/`). Each saga delivers one demonstrable milestone
from `docs/PRD.md`. Every step is strict TDD (Red / Green / Refactor)
and ends with the quality gate (`scripts/gate.sh`) and a commit before
`agentrail complete`.

The language specification for all sagas below is
`docs/lang-choices.md` (decisions made one at a time with the user and
consistency-reviewed). Where an older document or step prompt
disagrees with it, `lang-choices.md` governs. Items it lists as
deliberately deferred are not built until a saga below schedules them.

## Saga 1 -- foundations  [ARCHIVED]

Superseded after step 7, when the language decisions changed the
syntax that later steps assumed. Archived in `.agentrail-archive/`.

| #  | Step slug               | Delivered                                                |
| -- | ----------------------- | -------------------------------------------------------- |
| 1  | workspace-scaffold      | cargo workspace, crate stubs, `xetal --version`, spec-case harness with pending support, pending Life case, reg-rs, `scripts/gate.sh` |
| 2  | life-rule-and-checklist | Life one-liner corrected to Conway's rule (verified against sw-apl); sw-checklist conformance; `xetal-spec` crate |
| 3  | lexer-tokens            | lexer for the earlier syntax (`+^r`, `t_12`, `m.f_`), `xetal lex` |
| 4  | decorated-render        | raw <-> Unicode render and LaTeX output for the earlier syntax, `xetal render`, `docs/input.md` |
| 5  | syntax-proposal         | divergence from the research documented; proposal (superseded) |
| 6  | lang-choices            | decisions Q1-Q28 recorded in `docs/lang-choices.md` |
| 7  | lang-choices-2          | decisions Q29-Q50 and consistency review R1-R6 recorded |

Steps 3 and 4 implement the earlier syntax; Saga 2 revises them.

## Saga 2 -- calculus (M0 + M1)  [ACTIVE]

Goal: the implemented syntax matches `lang-choices.md` and renders
decorated and back losslessly (M0 again); the scalar functional
calculus evaluates (M1): `1 + 2` gives 3,
`u:s_quare := { _r * _r }; u:s_quare 7` gives 49,
`u:s_ub := { _l - _r }; 10 u:s_ub 3` gives 7, and a guarded recursive
factorial works.

| #  | Step slug        | Delivers                                                   |
| -- | ---------------- | ---------------------------------------------------------- |
| 1  | lexer-revision   | lexer to the new token set (names with one underline and a trailing mark, `u:` `c:` `l:` prefixes, `_digits` subscripts, `^` exponents on values, `:=` `->` `?` guards, `~` lazy parameters, quotes, strings, `#` comments, symbol digraphs `!=` `<=` `>=`, macros ending `<`, R2 `x!=3` error, newline and `;` tokens); rejection tests first; spec/lex re-written; goldens |
| 2  | render-revision  | Unicode render (underline core functions, prefix superscripts, subscripts, exponents, ligatures) and LaTeX output for the new syntax; round-trip proptest; spec/render and goldens; `docs/design.md` sections 2, 3, 8 and `docs/input.md` rewritten from `lang-choices.md`; README table and Life line; pending Life case in the new syntax |
| 3  | parser           | surface AST with spans: classes from tokens, right-to-left application, strands, exponents, axes, lambdas (`_l`/`_r`, named and `@` and `~` parameters), quotes, operand binding (F8/F9), `(expr)_` application, trains, guards, statements (`:=`, newlines, `;`), SC1 errors; ParseResult alternatives and `AmbiguousExpression`; ambiguity corpus; `xetal parse` |
| 4  | canonical-fmt    | canonical fully parenthesized printer, `xetal fmt`, `parse(fmt(parse x)) == parse x` proptest |
| 5  | core-desugar     | Core IR with NodeId and spans; desugar currying, lambdas, named and lazy parameters, operand binding, trains, guards, statements; normalization-equivalence tests; `xetal core` |
| 6  | scalar-eval      | strict evaluator over Core: Int / Float / Bool scalars with T1-T3 and D-10 rules, symbols, bindings and shadowing (M1), closures, currying, guards, `~` call-by-need (E1-E4), mutable `!` variables (M2), L7 shadowing warnings, printed results (10a) for scalars; `xetal eval`, `xetal run`, `xetal FILE`; M1 demos as reg-rs goldens |
| 7  | m1-docs-release  | README tour for M0/M1 with every command a golden, docs sync, saga retrospective |

## Saga 3 -- types-and-unit (M2)

Hindley-Milner inference over Core (Unit, Bool, Int, Float, Char,
functions, type variables, `Num`), let-polymorphism, diagnostics with
spans. Bool <-> Int coercions (T1). Niladic functions `{ @ -> ... }`:
`u:n_ow! @` works, `u:n_ow! 42` is a type error. `xetal type`. `::`
stays reserved (T4).

## Saga 4 -- arrays (M3)

`xetal-array`: dense row-major arrays, rank-0 scalars, strands, scalar
extension via one lifting rule, empty arrays, shape errors, 1-origin
(A5). Structural built-ins (B4, B5): `s_hape`, `r_eshape`, `r_ange`,
`o_ffsets`, `t_ally`, `f_irst`, `t_ake`, `d_rop`, `s_elect`, `r_avel`,
`c_at`. Division and equality rules (T2, T3), power (D-10). Strings as
Char vectors (section 13). Printed arrays (10a). Virtual ranges where
cheap. Property tests (shape of reshape). REPL (`xetal repl`).

## Saga 5 -- higher-order (M4)

Quoted functions and lambdas as values (F4), operand binding and
chaining (F8, F9), applying function values (F5). `r_/` and `s_\`
(leading axis, multi-axis R1, empty-reduce identities), `e_ach`,
`t_able`, `i_nner`, `c_ompose`, `s_wap`; the B7 arithmetic, search
and effect built-ins. Properties: reduce over concat, last of scan
equals reduce.

## Saga 6 -- rotate-and-axes (M5)

`o_-` rotate (leading axis), `r_ev`, axis subscripts on any function
by the move-to-front rule (A6), multi-axis rotate giving every
combination (A4). Axis validation. Properties: rotate inverse.
Animated 2-D rotate demo via CLI frames.

## Saga 7 -- combinators (M6)

The birds I K S B C W V T and more, written with named parameters
(L4), with inferred types checked by tests; Y working through a lazy
self parameter (E1-E3) and Z; user-written control structures with
lazy parameters. Combinator notebook demo (`demos/combinators.xtl`).

## Saga 8 -- libraries (M6b)

The macro phase (MC1-MC9): `u_se<` with a required alias, libraries
defining under `l:`, per-file aliases with private imports, one shared
instance per library, the macro-phase error table. The birds become
the first library, `Combinators.xtl`, used as `"c:" u_se<
"Combinators"`.

## Saga 9 -- trains (M7)

`[F G H]` forks and `[F G]` atop (TR1-TR3), purely by desugaring;
fork-law property test; type errors for ill-typed trains.

## Saga 10 -- life (M8)

Flip the pending Life case to active; block, blinker, glider and
random-board goldens (random boards checked against a reference
implementation in Rust test code). No Life-specific code paths.

## Saga 11 -- trace-and-explain

Trace tree (NodeId, span, value, type, shape, children) following the
evaluation order (E4); `xetal explain` prints the right-to-left
derivation; expanded (long-name) printer.

## Saga 12 -- web-playground (M9)

WASM playground: raw editor with decorated overlay (Unicode and a
LaTeX subset), display modes, semantic highlighting, hover tooltips,
"why this parse", right-to-left explainer with shapes and Life board
visuals.

## Deferred (from `lang-choices.md` section 15)

Nested arrays (A7), raw strings `r"..."`, Unicode text and complex
numbers via a type-extension mechanism, checked `::` signatures, axes
above 9, count-from-the-end axes, function power, the `_` wildcard
parameter. Each gets a saga (or steps) when scheduled.

## Cross-cutting (insert as steps when due)

- fuzzing (`cargo-fuzz` for lexer, parser, fmt, eval) -- after Saga 2.
- install tooling: `x_etal` alias next to `xetal` (S7) -- with Saga 2's
  release step.
