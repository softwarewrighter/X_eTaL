# X_eTaL -- Implementation Plan

Development is driven by agentrail sagas (one active saga in
`.agentrail/`, finished sagas archived to `.agentrail-archive/`). Each
saga delivers one demonstrable milestone from `docs/PRD.md`. Every step
is strict TDD (Red / Green / Refactor) and ends with the quality gate
(`scripts/gate.sh`) and a commit before `agentrail complete`.

Rule for every saga: resolve the open decisions assigned to it in
`docs/design.md` section 9 by writing the pinning tests first, then
update design.md in the same commit.

## Saga 1 -- foundations (M0 + M1)  [ACTIVE]

Goal: raw keyboard source renders decorated and back (M0); scalar
functional calculus evaluates (M1):
`1 + 2`, `square = { _r * _r }; square_ 7`, `sub = { _l - _r }; 10 sub_ 3`.

| #  | Step slug            | Delivers                                                   |
| -- | -------------------- | ---------------------------------------------------------- |
| 1  | workspace-scaffold   | cargo workspace + crate stubs, `xetal --version`, spec-case harness (with pending support), pending Life case, reg-rs dir + `scripts/reg.sh`, `scripts/gate.sh`, `.gitignore` |
| 2  | life-rule-and-checklist | (inserted) Life one-liner corrected to Conway's rule (D11, verified against sw-apl); sw-checklist conformance: `-V` build info, long help with agent instructions, small modules, `xetal-spec` crate; gate runs sw-checklist |
| 3  | lexer-tokens         | full v0 token set, decoration grammar, negative-literal rule, spans; rejection tests; `xetal lex` |
| 4  | decorated-render     | raw <-> Unicode decorated, lossless round-trip proptest; `xetal render` (M0 demo) |
| 5  | syntax-proposal      | (inserted) divergence from the research documented; `docs/syntax-proposal.md` for user review; no code changes |
| 6  | lang-choices         | (inserted) decisions made one at a time with the user recorded in `docs/lang-choices.md`; no code changes |
| 7  | lang-choices-2       | (inserted) remaining decisions Q29-Q50 and consistency review R1-R6 recorded in `docs/lang-choices.md` |
| 8  | syntax-revision      | (inserted) lexer, renderer, spec cases, goldens, design.md, input.md, README revised to match `docs/lang-choices.md` |
| 9  | parser-surface-ast   | noun/function classes, right-to-left application, strands, lambdas, bindings, `;`; ParseResult alternatives + AmbiguousExpression; ambiguity corpus; decide D1 D2 D4 |
| 10 | canonical-fmt        | canonical printer, `xetal fmt`, parse/fmt round-trip proptest |
| 11 | core-desugar         | Core IR with NodeId/spans, desugar lambdas / dyadic / niladic sugar; normalization-equivalence tests; decide D3; `xetal core` |
| 12 | scalar-eval          | strict evaluator for Int/Float/Bool scalars, `+ - * / =`, bindings, lambdas, closures; `xetal eval`, `xetal run`; M1 demos as reg-rs goldens |
| 13 | m1-docs-release      | README tour for M0/M1, docs sync, all demo commands in reg-rs, saga retrospective |

## Saga 2 -- types-and-unit (M2)

HM inference over Core (Unit, Bool, Int, Float, Char, functions, type
vars, `Num`), let-polymorphism, diagnostics with spans. `@`, `now_ @`,
`now_@` equivalence; `now_ 42` rejected as a type error. `xetal type`.
Decide D5 (Bool as Num). Arrays typed as `Array<T>` stub.

## Saga 3 -- arrays (M3)

`xetal-array`: dense row-major arrays, rank-0 scalars, strands, range
`i_`, shape/reshape `p_`, index/select `x_` with axis subscripts,
scalar extension via one lifting rule, empty arrays, shape errors;
array display format. Decide D6 (index origin). Property tests
(shape of reshape).

## Saga 4 -- derivations (M4)

Superscript derivations `^r` reduce, `^s` scan with axes; derived
functions are first-class (`sum = +^r; sum_ 1 2 3 4`); identities for
empty reduce; long forms `^reduce` / `^scan`. Properties: reduce over
concat, last of scan == reduce.

## Saga 5 -- rotate-and-axes (M5)

`t_` / `rotate_` monadic reverse and dyadic rotate per axis; axis
validation (`t_0`, `t_99` rejected); multi-axis `t_12` Cartesian
lifting. Decide D7 (result shape). Properties: rotate inverse.
Animated 2-D rotate demo via CLI frames.

## Saga 6 -- combinators (M6)

I K S B C W as library definitions with inferred types checked by
tests; `^e` each and `^o` outer; decide D8 (strict vs lazy, Y vs Z).
Combinator notebook demo (`demos/combinators.xtl`).

## Saga 7 -- trains (M7)

`[f g]`, `[f g h]` purely by desugaring; fork law property test;
dyadic trains (D9); type errors for ill-typed trains.

## Saga 8 -- life (M8)

Flip the pending Life spec case to active; block, blinker, glider,
random-board goldens (random boards checked against a reference
implementation in Rust test code). No Life-specific code paths.

## Saga 9 -- trace-and-explain

Trace tree (NodeId, span, value, type, shape, children) from the
evaluator; `xetal explain` CLI prints the right-to-left derivation;
expanded (long-name) printer.

## Saga 10 -- web-playground (M9)

WASM playground: raw editor + decorated overlay, display modes,
semantic highlighting, hover tooltips, "why this parse", right-to-left
explainer with shapes and Life board visuals.

## Cross-cutting (insert as steps when due)

- fuzzing (`cargo-fuzz` lexer/parser/fmt/eval) -- after Saga 1.
- REPL (`xetal repl`) -- after Saga 3.
- namespaces `m.f_` and user modules -- after Saga 6.
