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

## Saga 2 -- calculus (M0 + M1)  [DONE]

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
| 3  | parser           | surface AST with spans: classes from tokens, right-to-left application, strands, exponents, axes, lambdas (`_l`/`_r`, named and `@` and `~` parameters), quotes, operand binding (F8/F9), `(expr)_` application, trains, guards, statements (`:=`, newlines, `;`), SC1 errors; deterministic grammar (at most one parse; boundary shapes rejected with specific errors, PRD F3); ambiguity corpus; `xetal parse` |
| 4  | canonical-fmt    | canonical fully parenthesized printer, `xetal fmt`, `parse(fmt(parse x)) == parse x` proptest |
| 5  | core-desugar     | Core IR with NodeId and spans; desugar currying, lambdas, named and lazy parameters, operand binding, trains, guards, statements; normalization-equivalence tests; `xetal core` |
| 6  | scalar-eval      | strict evaluator over Core: Int / Float / Bool scalars with T1-T3 and D-10 rules, symbols, bindings and shadowing (M1), closures, currying, guards, `~` call-by-need (E1-E4), mutable `!` variables (M2), L7 shadowing warnings, printed results (10a) for scalars; `xetal eval`, `xetal run`, `xetal FILE`; M1 demos as reg-rs goldens |
| 7  | m1-docs-release  | README tour for M0/M1 with every command a golden, docs sync, saga retrospective |

### Saga 2 retrospective

Delivered: the lexer and renderer rewritten to `lang-choices.md`, the
parser (`xetal parse`), the canonical formatter (`xetal fmt`), Core
desugaring (`xetal core`) and a strict scalar evaluator (`xetal eval`,
`xetal run`, `xetal FILE`), with the M1 demos in `demos/` and every
stage pinned by spec cases (lex, render, syntax, ambiguity, eval) and
reg-rs goldens.

What went well:

- Deciding the language one question at a time before coding paid off:
  steps 3-6 needed only two new decisions (train arity, TR4; the
  deterministic-grammar reading of PRD F3), both put to the user.
- Property tests caught real bugs early: touching underline runs in the
  inverse renderer (saga 1), and the formatter is checked to preserve
  both the tree and the Core.
- The spec harness with blessing plus review kept expectations honest
  and cheap to update when output changed on purpose.

What to do differently:

- A README rewrite in step 1 silently dropped the Status and
  Documentation sections; restored in step 3. Review whole-section
  replacements with a heading diff.
- A step commit missed `Cargo.lock`; stage it explicitly.
- A decision example was wrong (`0.1 + 0.2 = 0.3` needs parentheses
  under right-to-left evaluation); examples in `lang-choices.md`
  should be executed by tests, as user docs already are.
- sw-checklist limits (7 functions per module, 50 lines per function)
  forced several refactors after the fact; design modules to the limit
  up front.
- Deep recursion first crashed the test thread; the evaluator now runs
  on a large stack with a depth limit. Keep the no-panic rule in mind
  for every recursive component (parser and desugarer next).

## Saga 3 -- types-and-unit (M2)  [DONE, ARCHIVED]

Goal: static types (M2). `u:a_nswer := { @ -> 42 }; u:a_nswer @`
works and `u:a_nswer 42` is refused by the type checker before
anything runs.

| #  | Step slug        | Delivered                                                  |
| -- | ---------------- | ---------------------------------------------------------- |
| 1  | nesting-limit    | bracket nesting limit (64) and AST depth limit (256) with a `too-deep` error; deep-nesting property tests |
| 2  | type-core        | `xetal-types`: types, unifier with occurs check, `Num` and `Truthy` classes, schemes |
| 3  | infer            | Algorithm W over Core with Haskell-style numeric typing (T5), value restriction, late-bound module definitions, `f_loat` (B8) |
| 4  | type-cli         | `xetal type`; type-checked `eval`/`run` with `--untyped` (T6); binding groups; Float-literal elaboration; TYPE sections in the spec corpus; `demos/unit.xtl` and goldens |
| 5  | m2-docs-release  | README M2 tour (every command a golden), docs sync, this retrospective |

### Saga 3 retrospective

Delivered: Hindley-Milner inference with let-polymorphism and the
`Num`/`Truthy` classes, printed per item by `xetal type`; `xetal eval`
and `xetal run` refuse ill-typed programs, with `--untyped` for
experiments such as Y; the spec corpus carries TYPE sections; the
parser has explicit nesting limits.

What went well:

- Asking before resolving open typing rules (Haskell-style numbers,
  typed-by-default with `--untyped`) kept the implementation aligned
  with the user's intent; both went into `lang-choices.md` as T5/T6.
- Blessing TYPE sections across the whole corpus and reviewing the
  diff exposed two real bugs that unit tests had missed: mutually
  recursive definitions closing with unquantified variables, and a
  value printed as Int where its type was Float.

What to do differently:

- Type soundness needs whole-corpus checks, not only targeted unit
  tests: compare each case's TYPE with its RESULT (a Float type with
  an integral-looking result is a smell).
- Elaboration covers literals at monomorphic sites. A literal inside a
  polymorphic function returned unchanged into a Float context (for
  example `u:k_ := { @ -> 1 }` used where a Float is expected) still
  evaluates as Int; runtime arithmetic promotes, so only printing can
  differ. Closing this needs dictionary passing or specialization;
  revisit with the arrays saga's printer.
- The module-count limit (7 per crate) is full in `xetal-types`.
  Limits are met by expanding up and out, never by merging: a full
  module gets a sibling module, a full crate a sibling crate, a full
  component a sibling component. Saga 4 starts by moving to the
  components layout.

## Saga 4 -- arrays (M3)  [DONE, ARCHIVED]

`xetal-array`: dense row-major arrays, rank-0 scalars, strands, scalar
extension via one lifting rule, empty arrays, shape errors, 1-origin
(A5). Structural built-ins (B4, B5): `s_hape`, `r_eshape`, `r_ange`,
`o_ffsets`, `t_ally`, `f_irst`, `t_ake`, `d_rop`, `s_elect`, `r_avel`,
`c_at`. Division and equality rules (T2, T3), power (D-10). Strings as
Char vectors (section 13). Printed arrays (10a). Virtual ranges where
cheap. Property tests (shape of reshape). REPL (`xetal repl`).

| #  | Step slug           | Delivers                                                |
| -- | ------------------- | ------------------------------------------------------- |
| 1  | components-layout   | `components/<name>/` multi-crate workspaces (as in sw-mlpl), shared target dir, `scripts/build-all.sh`, per-component gate, `xetal-types` split into crates; no behavior change |
| 2  | num-dictionaries    | the Saga 3 polymorphic-literal gap closed by passing a hidden number type to `Num`-quantified functions |
| 3  | array-core          | dense arrays, strands, scalar extension, shape errors, printed arrays, Array types |
| 4  | identity-tacks      | `i_d`, `l_eft`, `r_ight` (B9); S and S' as hook, fork and lambdas |
| 5  | structural-builtins | B4/B5/B10 structural built-ins, 1-origin, property tests |
| 6  | concise-refactor    | built-in catalog (TOML + build.rs codegen), facade-only lib.rs, tests in files, reg-rs over test code; no behavior change |
| 7  | life-docs           | docs show only the tested Life line (golden `docs-life-line`) |
| 8  | strings             | Char comparisons (T8, `Eq`/`Ord`), string structure, Float printing per 10a |
| 9  | repl                | `xetal repl` (replaying session) |
| 10 | m3-docs-release     | README M3 tour, docs sync, this retrospective |

### Saga 4 retrospective

Delivered: dense 1-origin arrays with rank-erased types (T7), scalar
extension, strings as Char vectors with comparisons (T8), the
structural built-ins (B4, B5, B10), identity and the tacks (B9),
number-type dictionary passing (closing the Saga 3 gap), the REPL,
and a restructure into component workspaces with a generated
built-in catalog.

What went well:

- Asking before each open semantic question (array typing, fills,
  select order, `c_at` ranks, comparisons) kept every decision the
  user's; each is recorded in lang-choices and pinned by spec cases.
- Blessing and reviewing whole-corpus output again found real bugs:
  a quantified variable defaulted through an alias and through a
  structure (local polymorphism), fixed with unit tests.
- Data-driven code paid off at once: moving the built-ins into
  `builtins.toml` replaced four hand-kept lists, and making the type
  classes a table made `Eq` and `Ord` two rows.
- Goldens replaced 312 lines of assert_cmd tests with no loss of
  coverage.

What to do differently:

- Scripted text edits (Python replace with `count=1`) hit the wrong
  occurrence twice (a table and a spec expectation); check the diff of
  every scripted edit before building on it.
- Design new code to the stricter gates from the start: three
  functions or modules had to be split after sw-checklist failed.
- reg-rs reads a `--desc` starting with `--` as a flag; describe
  goldens in words.
- Known gaps: `e_xp` overflow prints `inf` (not valid input); a REPL
  session replays every accepted line, so its cost grows with the
  session (fine for interactive use). Replaying `r_oll!` is
  deterministic: a session keeps one seed (Saga 5).

## Saga 5 -- higher-order (M4)  [DONE, ARCHIVED]

Quoted functions and lambdas as values (F4), operand binding and
chaining (F8, F9) and applying function values (F5) already work for
user functions. This saga adds `r_/` (right fold) and `s_\` (prefix
reductions) on the leading axis with empty-reduce identities, `e_ach`
(dyadic by currying), `t_able`, `i_nner`, `c_ompose`, `s_wap`, and the
B7 search and random built-ins. Properties: reduce over concat, last
of scan equals reduce. Multi-axis reduce and scan (R1) move to Saga 6.

| #  | Step slug          | Delivers                                                  |
| -- | ------------------ | --------------------------------------------------------- |
| 1  | reduce-scan        | built-ins applying function values; `r_/`, `s_\`, identities |
| 2  | each-table         | `e_ach` (monadic and dyadic by currying), `t_able`         |
| 3  | inner-compose-swap | `i_nner`, `c_ompose`, `s_wap`                              |
| 4  | search-builtins    | `i_ndexOf` `m_ember?` `u_nique` `s_ort` `g_rade` `w_here`  |
| 5  | roll               | `r_oll!`, test-only seed, goldens tolerant of random output |
| 6  | hof-properties     | property tests                                            |
| 7  | m4-docs-release    | README M4 tour, docs sync, retrospective                  |

### Saga 5 retrospective

Delivered: `r_/` (right fold) and `s_\` (prefix reductions) with
typed empty identities, `e_ach` (dyadic by currying), `t_able`,
`i_nner`, `c_ompose`, `s_wap`, the search and order built-ins
(`i_ndexOf`, `m_ember?`, `u_nique`, `s_ort`, `g_rade`, `w_here`) and
`r_oll!`, in two new components (`hof`, `search`); a README M4 tour.

What went well:

- Asking the open semantic questions in two batches at planning time
  (fold direction, scan, dyadic each, identities, inner axes,
  randomness, sorting, where R1 belongs) let seven steps run without
  stopping, and every decision is the user's.
- One callback (`Caller`) let every higher-order built-in run any
  operand by the ordinary rules; `i_nner` reuses `r_/` through it
  instead of duplicating the fold and the identities.
- The elaborator, built for number dictionaries in Saga 4, carried
  typed identities with a small extension: an empty Float sum is
  `0.0` with nothing new in the evaluator.
- A mutation check showed the property tests catch a wrong one-pass
  scan at once; the shrunk case is kept as a regression seed.
- reg-rs `preprocess` pins random output by property (every roll in
  range) rather than by value, so goldens need no seed.

What to do differently:

- Expected spans and printed class contexts in new spec cases were
  guessed and often off by one; write `PLACEHOLDER` and review the
  blessed value instead of guessing.
- A one-pass scan is only safe where it is provably identical to the
  definition (exactly associative operands, Int sums that cannot
  overflow); Float `+` and `*` scans stay quadratic. Revisit with a
  compensated or blocked scheme if it matters.
- Known gaps: a point-free definition such as `u:s_um := r_/ '+` is
  monomorphic (value restriction, T5), so it is Int only; write
  `{ '+ r_/ _r }` for a polymorphic one. Dyadic `e_ach` over an empty
  array decides by the operand's visible arity. Monads can be written
  today with Church encodings (a Maybe with bind type-checks); a
  worked example fits the combinators saga.

## Saga 6 -- rotate-and-axes (M5)

`o_-` rotate (leading axis), `r_ev`, axis subscripts on any function
by the move-to-front rule (A6), multi-axis reduce and scan (R1),
multi-axis rotate giving every combination (A4). Axis validation.
Properties: rotate inverse. Animated 2-D rotate demo via CLI frames.

## Saga 7 -- combinators (M6)

The birds I K S B C W V T and more, written with named parameters
(L4), with inferred types checked by tests; Y working through a lazy
self parameter (E1-E3) and Z; user-written control structures with
lazy parameters. Combinator notebook demo (`demos/combinators.xtl`).
Monads with Church encodings: a Maybe (`n_othing`, `j_ust`, `b_ind`)
with safe division chained by bind, type-checked, as spec cases and a
demo (`demos/monads.xtl`); note what needs nested arrays (the list
monad's bind) or named types.

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
- nesting depth: done in Saga 3 step 1 (bracket and AST depth limits,
  `too-deep`).
- install tooling: `x_etal` alias next to `xetal` (S7) -- with Saga 2's
  release step.
