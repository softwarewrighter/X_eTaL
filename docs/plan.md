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

## Saga 6 -- rotate-and-axes (M5)  [DONE, ARCHIVED]

`o_-` rotate (leading axis), `r_ev`, axis subscripts on any function
by the move-to-front rule (A6), multi-axis reduce and scan (R1),
multi-axis rotate giving every combination (A4). Axis validation.
Properties: rotate inverse. Animated 2-D rotate demo via CLI frames.
With these the Life one-liner runs (step 4 flips its acceptance
case to active); Saga 16 (life) adds the other Life goldens.

| #  | Step slug         | Delivers                                                   |
| -- | ----------------- | ---------------------------------------------------------- |
| 1  | rotate-reverse    | `o_-` (APL direction, amount lists as combinations), `r_ev` |
| 2  | axis-subscripts   | A6 move-to-front on any function, axis validation          |
| 3  | multi-axis        | `o_-_12` (A4), `r_/_12` and `s_\_12` (R1)                  |
| 4  | life-runs         | the Life acceptance case active                            |
| 5  | justfile          | `just` recipes over the scripts (inserted at user request) |
| 6  | rotate-properties | property tests                                             |
| 7  | rotate-demo       | animated 2-D rotate demo                                   |
| 8  | m5-docs-release   | README M5 tour, docs sync, retrospective                   |

### Saga 6 retrospective

Delivered: `o_-` and `r_ev`, axis subscripts on any function by the
move-to-front rule, rotate over several axes with every combination
of amounts, reduce and scan over several axes in turn; the Life
one-liner runs (a blinker demo with golden); a `justfile`; an
animated rotate demo.

What went well:

- Four open questions (rotate direction, amount lists, dyadic axes,
  moving the axis back) were settled with the user before the saga
  started, and the implementation followed them without surprises.
- The axis rule stayed general: the function's argument count comes
  from its type through the elaborator, the evaluator wraps the
  function as a built-in value, and one crate applies the rule; only
  rotate, reduce and scan define several axes, as A6 says.
- Life ran on the first try once the pieces existed, except for one
  real bug it exposed: the typed-identity wrapper hid `r_/` from the
  axis rule inside polymorphic functions. The fix is general (wrap
  the whole subscripted function), with a polymorphic test.

What to do differently:

- A mutation check first went unnoticed, because rotate handles its
  own axes and the properties exercised only rotate; model checks for
  rank-keeping functions under a subscript now catch it. Mutation-test
  each new rule, not just each new built-in.
- A pending acceptance case that starts passing has to flip in the
  same commit as the code, since the gate runs on every commit; plan
  that flip into the implementing step.
- Known gaps: `c_at_k` is refused (only the right argument's axis
  moves); `t_ally_2 M` and other rank-changing functions under a
  subscript are errors by design; the animation is a terminal
  playback of printed frames (no timing in the language).

## Saga 7 -- tui (M5b)  [DONE, ARCHIVED]

See the source as it is displayed while typing it as ASCII. A
front-end-agnostic view model (`components/view`) turns source, valid
or not, into styled segments: decorated Unicode glyphs (underlines,
subscript axes, superscript exponents and namespaces, APL alpha
and omega for `_l` / `_r`), a semantic class per token for highlighting, and a map
between raw byte offsets and rendered columns. Terminal widgets
(`components/tui`, ratatui + crossterm) draw it: an ASCII source
pane, a rendered pane, an output pane and an array viewer. Two apps
use them: `xetal edit FILE`, a nano-like editor with the ASCII text
on the left and the live rendered, highlighted view on the right,
types and diagnostics live below and results on Ctrl-R; and
`xetal repl`, which renders the line as it is typed when on a
terminal (piped input keeps the plain behavior). The view model and
widgets are the base for the stepping debugger (Saga 17) and the web
playground (Saga 10).

Decisions (with the user): live rendering in the REPL; split editor
with a live types/diagnostics pane and results on demand (effects such
as `p_rint!` and `r_oll!` run only on Ctrl-R); `_r` / `_l` render as
APL alpha and omega (first subscript l and r, changed at the
user's request: they read poorly); ratatui + crossterm, screens tested with TestBackend
buffer snapshots.

| #  | Step slug        | Delivers                                                    |
| -- | ---------------- | ----------------------------------------------------------- |
| 1  | lambda-glyphs    | compressed glyphs for `_r` `_l`, round-tripping            |
| 2  | view-model       | tolerant styled segments with classes and a span map; `xetal render --color` |
| 3  | tui-foundation   | text buffer, nano keymap, source and rendered panes, snapshots |
| 4  | editor           | `xetal edit FILE`: split panes, live types, Ctrl-R run, save |
| 5  | alpha-omega-glyphs | `_l` `_r` as APL alpha and omega (user request)           |
| 6  | operand-colors   | symbols light blue; a quote colored as its function (user request) |
| 7  | language-tour    | `demos/tour.xtl`: every feature, commented (user request)   |
| 8  | echo-run         | `xetal run --echo`: each statement, then its output (user request) |
| 9  | just-args        | `just run` / `just eval` pass flags through                 |
| 10 | diamond-glyph    | `;` as a black diamond (user request)                       |
| 11 | comment-rendering | comments keep their column; backquoted code drawn decorated |
| 12 | demo-fixes       | Life iterated, tour axes, `just install` (user request)     |
| 13 | macro-color      | macros bold yellow (user request)                           |
| 14 | array-view       | array viewer widget for results (reused by the debugger)    |
| 15 | repl-live        | live-rendered REPL line editor with history on a terminal   |
| 16 | tui-docs-release | README tour, debugger design notes, retrospective           |

### Saga 7 retrospective

Delivered: the view model (`components/view`: tolerant styled
segments with classes and a raw-to-rendered map, values as grids),
`xetal render --color` / `just pp`, the full-screen editor
`xetal edit` (split ASCII and decorated panes, live types, results as
grids on Ctrl-R, pane focus and scrolling, nano and Emacs keys), the
live-rendered REPL, notebook runs (`xetal run --echo` / `just show`),
a commented language tour (`demos/tour.xtl`), a `justfile` with
`install`, and Life iterated in its demo.

What went well:

- One view model behind every display meant each glyph or color
  decision (alpha and omega, the black diamond, operand colors, macros)
  was one table change that reached `pp`, `show`, the editor and the
  REPL at once.
- The editor and the line editor are state machines tested on
  ratatui's TestBackend and by key scripts; a pseudo-terminal smoke
  test checked the real terminal path (raw mode, events, restore).
- Reusing the REPL session gave notebook runs almost for free, with
  consistent rolls and no repeated printing.
- Most steps in this saga were the user's requests while using the
  tools; inserting them as saga steps kept the history honest.

What to do differently:

- Check what the user actually sees: glyphs that looked right in
  Unicode tables (subscript l, APL's diamond operator) were unreadable
  in the user's terminal font.
- Keep `target/release` current or say that it is not: the user hit a
  stale release binary; `just install` and the README now cover it.
- The user's uncommitted edit had to be set aside for several gates;
  ask sooner what to do with work in progress in the tree.
- Known gaps: the editor evaluates synchronously (a long run blocks
  the screen); the REPL line editor does not wrap lines longer than
  the terminal; screenshots of the editor are described, not shown.

## Saga 8 -- libraries (M6b)  [DONE, ARCHIVED]

The macro phase (MC1-MC9): `u_se<` with a required alias
(`"c:" u_se< "Combinators"`), libraries defining under `l:`, per-file
aliases with private imports, one shared instance per library, the
macro-phase error table. Library names display with their alias as a
superscript and in the library color (already in the view model); the
tour and `pp` / `show` / the editor cover a small library. Decided
with the user: in the pretty views `"c:" u_se< "Combinators"` draws
as superscript c and superscript equals (U+207C) in the library
color, then `u_se<` (u underlined, bold yellow), then the library name
in string color, so an import reads apart from a use such as `c:K_`. `xetal render`
keeps the exact tokens. Library names are capitalized by convention
(style guide). Scheduled before the combinators at the user's
request, so the birds are written once, directly as a library.

| #  | Step slug         | Delivers                                                   |
| -- | ----------------- | ---------------------------------------------------------- |
| 1  | sources           | multi-file source map; diagnostics name their file         |
| 2  | imports           | the macro phase: finding, validating, resolving, loading   |
| 3  | namespaces        | hidden namespaces, exports, private names, error rows      |
| 4  | first-library     | lib/Stats.xtl, a demo, the tour section, import display    |
| 5  | tools             | run / eval / echo / repl / edit expand imports             |
| 5  | readme-rewrite    | README as an overview, tours in docs/, images and video (user request) |
| 6  | org-babel         | xetal-mode, ob-xetal, the literate tour (user request)     |
| 7  | tools             | the REPL, notebooks, org sessions and the editor use libraries |
| 8  | libs-docs-release | M6b tour page, docs sync, retrospective                    |

### Saga 8 retrospective

Delivered: the macro phase (MC1-MC9) as a component of its own: a
multi-file source map, import finding and validation with every MC8
error row, library resolution (beside the file, XETAL_PATH, standard
libraries built in), hidden namespaces for exports and private names,
errors located in the file where they were written; `lib/Stats.xtl`;
every tool loading through `xetal-program`. Alongside, at the user's
request: the README rewritten as an overview with images and a tour
video, the milestone tours moved into docs/, Emacs support
(`xetal-mode`, `ob-xetal`) with a literate tour whose results are
checked, paced notebook runs, and plans for the web demo and for
porting the APL workspaces.

What went well:

- One combined text with a source map kept every later stage
  unchanged: the parser, checker and evaluator never learned about
  files.
- End-to-end tests through the checker and evaluator caught what unit
  tests would not have: a library name found the importer itself on a
  case-insensitive disk, and the mean of Stats was Int-only.
- Screenshots and recordings made by the tools themselves (vhs) found
  real display bugs (the ASCII pane dropping hidden text).

What to do differently:

- Anticipate rustfmt reflow and the checklist limits: several scripted
  edits missed reformatted text, and two modules had to be split after
  the fact; the user called this out, and the rule is now to design
  for the limits and edit formatted text.
- Look where tools are installed before saying they are missing
  (Emacs is an app, not on PATH).
- Known gaps: the REPL replays every accepted line with its libraries
  (fine interactively); `xetal core` and `xetal fmt` do not expand
  imports; a library cannot yet be run on its own (MC8 row 9 note).


## Saga 9 -- combinators (M6)  [DONE]

The birds I K S B C W V T and more, written with named parameters
(L4) in the first real library, `Combinators.xtl`, used as
`"c:" u_se< "Combinators"`, with inferred types checked by tests; Y
working through a lazy self parameter (E1-E3) and Z; user-written
control structures with lazy parameters. Function power (D-7):
`f_^3 x` and the built-in `n 'f_ p_ower x` (Life's generations become
`u:l_ife^4 board`). Combinator notebook demo (`demos/combinators.xtl`,
shown with `just show`). Monads with Church encodings: a Maybe
(`n_othing`, `j_ust`, `b_ind`) with safe division chained by bind,
type-checked, as spec cases and a demo (`demos/monads.xtl`); note what
needs nested arrays (the list monad's bind) or named types. The tour
gains a libraries section.

Decided with the user at planning: every bird Smullyan names that
type-checks goes in Combinators (the self-applying ones, M L U and the
textbook Y and Z, in an untyped demo); Y is in the library as a
recursive definition typed `(a -> a) -> a`; function power on any
function name, including a quoted operand; Maybe is a second standard
library.

| #  | Step slug            | Delivers                                                 |
| -- | -------------------- | -------------------------------------------------------- |
| 1  | aviary               | the birds, their spellings, `xetal type` of a library    |
| 2  | untyped-notebook     | notebook runs honor `--untyped`; every `just` recipe smoke-tested |
| 3  | power                | `f_^n` and `p_ower`                                      |
| 4  | combinators-library  | lib/Combinators.xtl, types pinned; untyped birds demo    |
| 5  | maybe                | lib/Maybe.xtl, demos/monads.xtl                          |
| 6  | editor-panes         | Ctrl-T zoom, Tab among views, strong current pane, visible cursor |
| 7  | tttml                | TTTML ported from sw-apl (demo for the user, same day)   |
| 8  | tttml-literate       | lib/TTTML.xtl (pure model), docs/literate/tttml.org; streaming one-pass notebooks in APL session layout |
| 9  | life-diagram         | `xetal diagram`, the README's annotated Life line        |
| 10 | combinator-diagrams  | six decoded combinator lines in docs/birds.md            |
| 11 | tttml-play           | quad names; files, keyboard, `f_ormat`, `n_umbers`; train-and-save and play demos |
| 12 | notebook             | tour Combinators section, literate birds.org, Life with power |
| 13 | idioms               | docs/idioms.md, X_eTaL beside mainstream and array languages |
| 14 | logo                 | the typeset logo (the README uses the user's modern logo) |
| 15 | keys-demo            | demos/keys.xtl, every input form beside what is typed    |
| 16 | keys-explicit        | explicit comments; axes shown default, explicit, other   |
| 17 | reference            | docs/reference.md, every built-in, generated and checked |
| 18 | raised-decimal       | decimal exponents raised, a middle dot as the point      |
| 19 | m6-docs-release      | M6 tour page, screenshots and video, docs sync, retrospective |

Retrospective. Planned as six steps, the saga ran to nineteen: all
thirteen additions came from the user while using what the earlier
steps built, and most were about seeing the language (diagrams, the
reference, idioms, the keys demo, the logo, the notebook layout)
rather than the language itself. That is the value of working in
view: TTTML, asked for as a same-day demo, forced files, the keyboard,
numbers as text and system names into the language well before their
planned saga, and exposed two real problems, both fixed: the notebook
replayed the whole file for every statement (a training program ran
its training again for each later line), and it showed nothing until
the end. Lessons: generate documentation from what runs (the reference
and the diagrams cannot drift, and generating the reference caught a
wrong example); expect display questions to recur, and answer them
with a rule rather than a special case (the raised decimal point);
and keep the gate honest about time (the smoke test, the literate
documents and the goldens now take about four minutes, most of it
running programs, where the evaluator's per-item operand calls are
the next speedup).

## Saga 10 -- web-playground (M9)

A live demo in the browser (Yew, compiled to WASM, deployed to GitHub
Pages under `pages/`) that is the terminal UI, not a new design: the
same editor (ASCII and decorated panes, the types/output pane, Tab
focus, nano and Emacs keys, Ctrl-R run, Ctrl-T zoom), the notebook
view of the language tour and a live-rendered REPL, drawn as a
monospace character grid with the same colors. It reuses the view
model, the grids, the text buffer and the keymap; the evaluator,
type checker and bundled libraries compile to WASM (the evaluator's
large-stack worker thread needs a single-threaded path there).
Decided with the user: the only parts that are not TUI are a Help
button (a dialog listing the keys and a short language summary) and
the footer the other live demos share (as in sw-mlpl): MIT License,
copyright, a GitHub repo-links dialog, Changes, Literate (the
exported literate tour), Blog, Discord, YouTube, Wiki, and build
info. Also decided with the user:
- The footer's Literate link leads to HTML exports of the literate
  documents (the tour, Life, and others as they are written), built
  into `./pages/literate/` with Org's HTML export.
- Built locally into `./pages` (committed) and published by a GitHub
  Actions workflow that deploys that folder (nothing is built on
  GitHub); the README links to it.
- A drop-down loads the canned `.xtl` files (the demos and the tour).
- Help opens a dialog explaining how the live editor works (moving the
  cursor, Tab between panes, Ctrl-T zoom, Ctrl-R run), dismissed by
  its corner X, a click on the background, or Escape.
- Load and save in the browser's local storage, like the workspaces
  of the sw-apl live demo (../../sw-vibe-coding/sw-apl).
- The standard libraries are bundled, the Combinators library among
  them, and the tour uses Combinators.
- Reprioritized by the user after Saga 9, to show the work in
  progress: the live demo comes before the ports. In the browser,
  `[]R_EAD` reads from a prompt and the files are kept in local
  storage.
Later: the stepping debugger's panes, "why this parse", hover
tooltips, a LaTeX view.

## Lane: classics (parallel to Saga 10, branch feat/classics) -- done

Asked for by the user: the classic APL example programs (Pascal's
triangle, the sieve, GCD, Hanoi, quicksort, inner-product graph
algorithms, finite differences, cellular automata, run-length
encoding, magic squares, Mastermind, Mandelbrot, Roman numerals, word
frequency, N-Queens, an APL subset interpreter, ...), one notebook
demo each in `demos/classics/`, a golden per demo, listed in the live
demo and indexed in `docs/classics.md`. Worked as a parallel lane
whose saga lives in `lanes/classics/.agentrail` (run agentrail with
`--saga lanes/classics`), so the main saga is untouched. The programs
are forcing functions: the ones that need them bring replicate
(B11), encode and decode (B12) and nested arrays (A7, static `Box a`,
printed boxed), decided with the user up front (D35-D37). Graphics came
next at the user's request (QD5, B13, D38-D39): pure SVG builders,
one `[]S_HOW` effect, frames animated, so Life, Queens, tic-tac-toe,
Mandelbrot and turtle drawings are pictures in the terminal's files,
the browser and a future desktop app (a webview host of the same
SVG). What each demo asked of the language (added, planned, and
friction kept for now) is tracked in `docs/dogfooding.md`; every step
that changes the language or the plan updates it.

| #  | Step slug                | Delivers |
| -- | ------------------------ | -------- |
Added at the user's request (an ADVANCEDEX note): IBM shipped a
workspace, ADVANCEDEX (saved 07/20/68), so APL\360 users could load,
run, change and trace real programs; the APL\360 User's Manual (1968,
and March 1970, GH20-0683-1, both at softwarepreservation.org) refers
to its Appendix B, Advanced Examples. The lane recovers those 32
functions with their sources, ports them (a historically grounded
curriculum rather than one compiled in hindsight), and sets about
twelve classics side by side across eras (APL\360, APL2, Dyalog,
X_eTaL) to show how array style evolved and what X_eTaL keeps or
changes. Follow-on outside this repo: a CLASSICS workspace in
sw-apl-workspaces holding the same programs as APL, by era. The
manual's advice to watch a subexpression through an output assignment
feeds the trace saga (Saga 17).

| 1  | classics-index           | docs/classics.md, conventions, Pascal's triangle |
| 2  | classics-broken-pipe     | a closed stdout ends a run quietly (found by the Pascal demo) |
| 3  | classics-trig            | `s_in` `c_os` `a_tan` `p_i` (B13) |
| 4  | classics-draw-grid       | `xetal-draw`, `[]G_RID`, `[]S_HOW`, animated Life (QD5) |
| 5  | classics-draw-path       | `[]P_ATH`, lib/Turtle.xtl, Sierpinski |
| 6  | classics-draw-raster     | large grids as images, palettes, Mandelbrot zoom and fly-over |
| 7  | classics-literate        | ob-xetal `:file` pictures, literate classics.org, animated SVG in the HTML export |
| 8  | classics-numbers         | sieve, primes, GCD, Fibonacci, factorial, Collatz |
| 9  | classics-draw-web        | a Draw pane in the live demo: done on main by the web lane (draw-pane) |
| 10 | classics-recursion       | Tower of Hanoi, quicksort |
| 11 | classics-graphs          | matrix product, transitive closure, Warshall, shortest paths |
| 12 | classics-sequences       | polynomials, moving average, differences, 1-D automaton |
| 13 | classics-data            | histogram, duplicates, sort and grade, run-length encoding |
| 14 | classics-puzzles         | magic square, Mastermind (Mandelbrot is in draw-raster) |
| 15 | classics-replicate       | `r_eplicate` (B11) |
| 16 | classics-encode-decode   | `e_ncode` / `d_ecode` (B12) |
| 17 | classics-radix-programs  | truth tables, base conversion, Roman numerals, RLE decode |
| 18 | classics-nested-design   | the remaining A7 questions, with the user |
| 19 | classics-nested-core     | `Box a`, nested values, string strands, `e_nclose`, `d_isclose`, DISPLAY printing (B16) |
| 20 | classics-nested-builtins | `p_artition`, `m_ap` (B14) |
| 21 | classics-nested-programs | word frequency, N-Queens drawn, ragged Pascal |
| 22 | classics-advancedex-source | IBM's ADVANCEDEX (1968): Appendix B of the APL\360 User's Manual transcribed and inventoried |
| 23 | classics-mini-apl        | an APL subset interpreter in X_eTaL |
| 24 | classics-release         | index, README link, retrospective, lane archived |
| -- | classics-advancedex-ports | backlog: ADVANCEDEX in X_eTaL, after the transcription (main saga, advancedex-transcribe) |
| -- | classics-eras            | backlog: twelve classics across eras: APL\360, APL2, Dyalog, X_eTaL |

Gaps found by the mini APL interpreter (classics-mini-apl), for later
decisions:
- No raze (APL's enlist): the boxes of a nested vector cannot be
  joined into one array, so the interpreter reads its number tokens
  one at a time instead of as one strand of text.
- `s_elect` takes no index from the end (APL's negative index is a
  rotate-and-take there): the last token is `(t_ally t) s_elect t`.
- No table of functions to dispatch on: the interpreter picks its
  function by a chain of guards on the name (a box of functions, or a
  `Map` from names, would say it once).
- No execute: the interpreter is the workaround, and it is also the
  test bed for one (`[]V_ALUE`, reserved in QD3).
- A comparison named at the top level is a Bool, not a number (seen
  again in the tokenizer, written `0 + ...`).

Retrospective (the lane is closed; its saga is archived in
`lanes/classics/.agentrail-archive/classics-20261001T201245/`):
- Delivered: thirty classic programs in `demos/classics/` (each a
  notebook with a golden, in the live demo and indexed in
  `docs/classics.md`), plus magmas and the first LeetCode demo;
  pictures (`[]G_RID`, `[]P_ATH`, `[]S_HOW`, raster images,
  animations); literate walkthroughs (classics, Hanoi three ways, the
  ducks two ways); a tiny APL interpreter with `)ORIGIN`.
- The programs worked as forcing functions: they brought trigonometry,
  replicate, encode and decode, catenate along an axis, nested arrays
  (`Box a`, string strands, enclose, disclose, partition, map, DISPLAY
  printing, `--ascii`), a quiet closed pipe and the ob-xetal picture
  results, each decided with the user and added test-first.
  `docs/dogfooding.md` keeps the full list, with the frictions the
  language keeps for now.
- What worked: one PR per step or two, branched from the newest main;
  decisions asked before building; the handoff file for what the
  cloud sandbox could not run (the wasm build, the live demo, the
  ADVANCEDEX sources).
- What to do differently: no stacked PRs (deleting a merged base
  branch closed the PR on top of it; each PR now branches from main);
  post a status well within ten minutes; a branch's stale reg-rs caches
  (`reg/*.tdb`) can fail another branch's goldens, so clear the ones
  without an `.rgt`.
- Backlog: the ADVANCEDEX ports (once main's advancedex-transcribe step
  has the sources) and the eras Rosetta, taken up by whichever lane
  has room; the gaps the interpreter found (above).

## Next, in order (decided with the user, 2026-10-02)

1. Finish Saga 10: demo-menu (the Open list as collapsible groups:
   Demos, Classics, Libraries, Misc), advancedex-transcribe, then two
   steps from X_eTaL-demos' asks (int-strand-speed, ask D1, a
   quadratic type check of long Int strands; exponent-literals, ask
   D4, `1.5e-7`, decision S8), then Saga 25 and Saga 19 (below),
   then web-release, so the release ships with the terminal and
   macro libraries.
2. Saga 25, the terminal: a CLI emulated in the live demo with Rust,
   Yew and WASM on the web-sw-tos model, replacing `window.prompt`,
   with named screen-control quads (QD6); its steps run inside Saga
   10's agentrail saga, before web-release (the user's order).
3. Saga 19, macros (`.xtlm` macro libraries, `u_se<` finding
   `Name.xtl` and `Name.xtlm`, longer namespace prefixes), moved by
   the user right after Saga 25, also inside Saga 10's agentrail saga
   before web-release; X_eTaL-libraries' Control and Assert macro
   libraries (its saga 4) wait on it.
4. Saga 20, array kinds (empty arrays remember Char, number or box;
   ask X5 from X_eTaL-libraries, a wrong picture they hit now).
5. Saga 13, quads (the user asked for `[]A`, `[]D` and the related
   system names).
6. Saga 21, errors of one's own (assert, raise, catch; ask X3 from
   X_eTaL-libraries).
7. The retrofit saga after it (below), so it can use quads, macros,
   long aliases and errors too.
8. Then Saga 11 and the rest as numbered, with the sagas added for
   the sibling repos' asks (2026-10-02): Saga 22 speed (ask D2),
   Saga 23 host bindings and native packages (asks D3 and E1),
   Saga 24 transpose (ask D9, done as the transpose lane); the
   trace through `xetal-play` (ask D8) is a step of Saga 17. The
   leetcode lane goes on in parallel (the classics, trains and
   transpose lanes are done and archived).

## Saga 25 -- the terminal (browser and CLI)

Asked for by the user and by X_eTaL-games: interactive programs
(anything that reads with `[]R_EAD`) get a real terminal in the live
demo instead of `window.prompt` dialogs, and text-UI programs get screen
control. It is a CLI emulated in the page with Rust, Yew and WASM, as
`../../sw-embed/web-sw-tos` does; nothing else is involved. What is
taken from web-sw-tos (its docs/architecture.md):

- A character grid (cells with a character, foreground, background,
  bold) owned by plain Rust and drawn by Yew, with scrollback.
- Keys from the window, not a focused element (its `src/browser.rs`
  `on_keydown`: Meta and Alt left to the browser); key translation a
  pure, tested function (its `crates/swtos-input/src/translate.rs`).
- The program stepped by the page in ticks within a time budget (its
  `crates/swtos-session/src/driver.rs` `run`), never blocking: a program
  waiting for a line simply does not advance until Enter.
- The browser touched in one module only; time injected through a
  trait; the session logic tested natively; dependencies one way.

Decided with the user (2026-10-02): a steppable evaluator, as
web-sw-tos steps its emulator. web-sw-tos never blocks because its
program's state is data (the emulated CPU's registers and memory): the
page runs a batch per tick and a program waiting for a key is simply
not advancing. X_eTaL's evaluator keeps a running program's state on
the Rust call stack, so it cannot stop inside `[]R_EAD` and continue
later; it becomes an explicit machine (the expression being evaluated,
pending applications and environments as data, the higher-order
built-ins as steps rather than callbacks), run in slices, waiting at
`[]R_EAD` (or a key) until a line arrives. One evaluator serves the
CLI and the page. Rejected: re-running the program on each line
(nonsensical for a stateful loop such as a text adventure) and a worker
sleeping on `Atomics.wait` (needs cross-origin isolation, which GitHub
Pages cannot give without a non-Rust service worker). Screen
control is named quads (QD6), applied to the grid in the page and
written as ANSI by the CLI.

| Step | Slug | Content |
| ---- | ---- | ------- |
| 1 | terminal-decisions | Done: the steppable evaluator (above); the quads' names, key codes and palette are settled at screen-control. |
| 2 | steppable-core | The core evaluator as an explicit machine over a heap stack, run in slices with a budget; every spec case and golden unchanged. |
| 3 | steppable-hof | The higher-order built-ins that call user functions become steps of the machine, not callbacks. |
| 4 | resumable-run | A run returns a waiting state at `[]R_EAD` (or a key) and resumes with the line; tested natively, the CLI unchanged. |
| 5 | terminal-grid | Crates on the web-sw-tos split: the grid (cells, attributes, scrollback, line editing with echo, backspace and history), key translation, and the session that feeds lines to the waiting run; all pure Rust, tested natively. |
| 6 | terminal-pane | The live demo's output pane becomes the terminal (Yew, keys from the window); `[]R_EAD` reads from it; no `window.prompt` left. Checked in headless Chrome. |
| 7 | screen-control | The quads (QD6): write at a row and column, clear, style, one key, the terminal's facts, standard error in red; the same in the grid and as ANSI at the CLI. |
| 8 | terminal-retrofit | TTTML play, Mastermind and the other interactive demos on the terminal; goldens rebased on purpose. |
| 9 | terminal-release | README, Help, CHANGES, pages, retrospective; X_eTaL-games told how to use the terminal with `xetal-play`. |

## Saga 20 -- array kinds (empty arrays remember their kind)

Asked for by X_eTaL-libraries (ask X5): `d_isplay ""` draws the
numbers mark `~`, because at run time an empty array has no items to
say what kind it is, and `""` and `0 r_eshape 1` are the same value.
Decision T9 (made with the user): an array keeps the kind of its items,
as APL2's prototype does.

| Step | Slug | Content |
| ---- | ---- | ------- |
| 1 | kind-in-arrays | Failing tests first (`d_isplay ""`, an empty piece of a split, `0 t_ake "abc"`, boxes); the array (or the evaluator's array value) carries an item kind; literals and `""` set it. |
| 2 | kind-through-primitives | Every primitive that can make an empty array keeps its argument's kind (reshape, take, drop, compress, replicate, partition, where, each, catenate of empties, outer products); a test per primitive. |
| 3 | kind-shown | DISPLAY, Boxed output and the live demo mark empty arrays by kind; goldens rebased on purpose; reference and design register. |

## Saga 21 -- errors of one's own (assert, raise, catch)

Asked for by X_eTaL-libraries (ask X3) and on the wish list (error
handling, tests in XeTaL): a program cannot stop with an error it
chooses, nor recover from one (a failing `[]N_GET`). Decided with the
user to come after macros. The design is open and is made with the
user first: the spelling (`a_ssert`, a raise built-in, a typed result
or a handler-taking `t_ry`), error kinds and messages, the exit
status, and how a caught error is typed.

| Step | Slug | Content |
| ---- | ---- | ------- |
| 1 | errors-decisions | Decide the design with the user; record it in lang-choices and the design register. |
| 2 | assert-and-raise | Stop with one's own message and a non-zero exit status (`k:a_ssert 5 = 6` style); spec cases and goldens. |
| 3 | catch | Recover from an error, typed as decided; `[]N_GET` on a missing file as the first case. |
| 4 | errors-retrofit | The Check library's text-report workaround and other workarounds rewritten; goldens rebased on purpose. |
| 5 | errors-release | README tour, reference, CHANGES, pages, retrospective. |

## Saga 13a -- retrofit (newer features in older programs)

Asked for by the user: programs written before a feature existed carry
workarounds for it (two Float planes for a complex number, rows stacked
by hand before `c_at_2`, digit strings written out before `[]D`).
`docs/dogfooding.md` lists each gap a lane met and the workaround it
used; this saga works through that list and an audit of the demos,
libraries, userlibs and literate documents, and rewrites each
workaround with the feature now there, the goldens rebased on purpose
and each change in CHANGES.md. Where the before and after teach
something, a short note keeps both (as `docs/literate/duck.org` does).

| #  | Step slug        | Delivers                                                   |
| -- | ---------------- | ---------------------------------------------------------- |
| 1  | audit            | every workaround found (dogfooding.md and a search of the sources), each with the feature that replaces it; a table in docs/dogfooding.md |
| 2+ | one per feature  | the programs using that workaround rewritten (`c_at_2`, nested arrays and `p_artition`/`m_ap`, `d_isplay`, `[]A`/`[]D`, and so on), goldens rebased |
| n  | retrofit-release | the lesson, CHANGES, retrospective |

From then on, every saga that adds a feature ends with a retrofit step
for that feature (cross-cutting, below).

## Saga 19 -- macros (`.xtlm` macro libraries and long prefixes), right after Saga 25

Asked for by the user (2026-10-02); X_eTaL-libraries files it as asks
X1 (`.xtlm` macro libraries) and X2 (seeing expansions) in its
docs/xetal-asks.md, and its saga 4 (Control and Assert macro
libraries) waits for this one. The design is from its
docs/research.txt. Decisions MC10 to MC13 in
`lang-choices.md` (made with the user the same day). Today the macro
phase (components/macro) knows one macro, `u_se<`, hard-coded in
xetal-names; libraries are `.xtl` only; aliases are lowercase letters
(the lexer and the alias check disagree on uppercase and digits).

| Step | Slug | Content |
| ---- | ---- | ------- |
| 1 | long-prefixes | MC13 test-first: `"combinators:" u_se< "Combinators"`, `b2:`, rejections (`Abc:`, `2b:`) with the lexer and `valid_alias` agreeing; the raw-prefix fallback when a letter has no superscript (render, view, LaTeX, Emacs mode); spec cases and a golden. |
| 2 | lookup-xtlm | MC11: library lookup returns the `.xtl` and/or `.xtlm` of the first directory holding either (files, `userlibs/`, `XETAL_PATH`, standard libraries, the browser store); explicit paths; the not-found message names both; each search tier tested. A `.xtlm` is parsed and its exports listed, not yet run. |
| 3 | macro-calls | MC10 and MC12: a `.xtlm` defines `m:name<` exports (rejections: `l:` names, `m:` names without `<`, `<` names without `m:`, top-level expressions); the call shapes: the names phase looks up `alias:name<` among the imported `.xtlm` exports (monadic and dyadic, string arguments), in statement and expression position, replacing the "only u_se<" error; MC8 rows updated with rejections for each new rule. |
| 4 | macro-engine | Running a macro: a runner injected into the macro phase (it cannot depend on eval) evaluates the `.xtlm` function on the argument strings; the result is re-lexed with spans mapped to the call and expanded again to the depth limit (cycle and depth errors show the chain); `.xtlm` exports are type-checked as text to text. |
| 5 | expand-tool | Ask X2: print the program after macro expansion (spelling to settle with the user: X2 says `xetal --expand FILE`; the CLI's other views are subcommands, `xetal fmt`, `xetal core`), and one call's expansion with where the macro is defined; goldens of expansions. |
| 6 | macro-example | A small macro library for the tests and a demo (X1's `u_nless<` example as the first case) and a literate document. The useful macro libraries (Control, Assert) belong to X_eTaL-libraries, not here. |
| 7 | user-macros | A `userlibs/` macro library example; the live demo's store and Open menu carry `.xtlm` files. |
| 8 | retrofit-macros | Older programs and libraries rewritten where long aliases or macros read better; goldens rebased on purpose. |
| 9 | macros-release | README tour, reference, design.md register, CHANGES, pages, retrospective. |

## Saga 11 -- ports-first (LEARN, COURSE and PLOT)

Asked for by the user after Saga 9, sooner rather than later: an
inventory of sw-apl's library 1 and sw-apl-workspaces (every
function mapped to a target file, marked "works now" or "needs
feature X", docs/apl-ports.md), then LEARN, COURSE and PLOT ported as
libraries with demos, goldens and literate documents, with the
features they need added first (each decided with the user). Reading
input and numbers as text exist already; PLOT's character plots may
call for nested arrays (A7).

## Saga 12 -- ports-now (the other libraries that work as the language is)

Phased at the user's request, to demo more libraries soon: the APL
ports are split in two. This saga ports every workspace function that
X_eTaL can already say cleanly; Saga 14 adds the features the rest
need, then ports those. The inventory decides which is which.

| #  | Step slug       | Delivers                                                   |
| -- | --------------- | ---------------------------------------------------------- |
| 1  | review          | the Saga 11 inventory brought up to date |
| 2+ | one per library | each group of "works now" functions as `lib/*.xtl` (for example MATH, POLY, Stats additions, LIFE, BIRDS into Combinators), with a `just show` notebook demo, goldens and a tour or literate link; steps added by the inventory |
| n  | ports-now-release | README and tour links to the new libraries, retrospective |

## Saga 13 -- quads (system names), next after Saga 10

APL's quad names, as decided with the user (lang-choices section 13a,
QD1-QD3): `[]` touching a name lexes as one system-name token,
displayed as the quad glyph; the values `[]A`, `[]D`, `[]AV`, `[]TS`
and `[]IO` (always 1); the functions `[]D_L` (delay), `[]U_CS`
(character codes), `[]R_EAD` and `[]V_ALUE` (input, for the course
and drill ports). Each with spec cases, rejection tests (an unknown
system name, a quad function used as a value), types, the view
model's colors and the Emacs mode. Scheduled before the remaining APL
ports (Saga 14), which need them.

## Saga 14 -- ports-later (dogfooding)

After Sagas 11 and 12 have ported what works as the language is, port the
rest of every workspace, function and demo of the user's APL projects
into X_eTaL libraries (`lib/*.xtl`), demos (`demos/`) and literate
documents (`docs/literate/`): sw-apl's library 1 (BIRDS in both
modes, EDIT, LEARN, LIFE, RACE, TTTML; about 70 functions) and
sw-apl-workspaces (CALC, COURSE, DRILL in both modes, MATH, MATRIX,
PLOT, POLY, STATS; about 220 functions), with their sample transcripts
as goldens. Decided with the user: the ports are forcing functions.
They are written as X_eTaL should say them, not transliterated; a
port that cannot be written cleanly names a missing feature, which is
added first, test-first, with the user's decision recorded in
lang-choices, rather than worked around. Features the inventory
already points to: reading input (the course and drill quizzes),
numbers as text (formatting for PLOT and the drills), matrix
division (APL's domino, for MATRIX and STATS regression), and
whatever nested arrays (A7) the character-plot and course material
needs.

| #  | Step slug       | Delivers                                                   |
| -- | --------------- | ---------------------------------------------------------- |
| 1  | review          | the Saga 11 inventory brought up to date: what is left and the features it needs |
| 2  | features        | the missing features, one decision and step each, test-first |
| 3  | math            | MATH, POLY, CALC, MATRIX as libraries; demos and goldens    |
| 4  | stats           | STATS merged into lib/Stats.xtl (regression, tests, distributions) |
| 5  | plots           | PLOT: character plots, graphs, scatter, bars               |
| 6  | games           | LIFE, RACE, TTTML (tic-tac-toe learning), with BIRDS joining Combinators |
| 7  | teaching        | COURSE, LEARN, DRILL as interactive programs (REPL and web) |
| 8  | edit            | EDIT: port what fits (the del editor itself is the editor's job) |
| 9  | ports-release   | literate documents for each library, tour links, retrospective |

## Saga 15 -- trains (M7)  [DONE, ARCHIVED]

`[F G H]` forks and `[F G]` atop (TR1-TR3), purely by desugaring;
fork-law property test; type errors for ill-typed trains.

Worked as a lane, `lanes/trains/.agentrail` (run agentrail with
`--saga lanes/trains`), one PR per step from main. The parsing and the
desugaring were already done (spec/syntax/train-*.case), so the lane's
steps are: trains-fork-law (property tests that each train equals its
desugared form), trains-errors (an ill-typed train names and points at
the element at fault), trains-docs (reference, README, and a literate
document of trains beside their Combinators birds) and trains-release.

Retrospective (the lane is closed; its saga is archived in
`lanes/trains/.agentrail-archive/`):
- Delivered, in six PRs (#26-#28, #30, #31 and this one): property
  tests that every train shape is its desugaring (fork, atop, dyadic
  fork, long trains, named trains, text); errors spanned by the
  element at fault, with notes giving the train, the element, what the
  train means there written out, and an arity hint for built-ins
  (`xetal-explain`, `Program::annotate`, D46, D47); trains in the
  tour, README, reference, the syntax poster and
  `docs/literate/trains.org` (each train beside its Combinators bird,
  checked to agree); the retrofit (the Stats library and the demos'
  train-shaped lambdas written as trains).
- Two steps were added as the lane went: better diagnostics (the
  user asked for more than a span) and the retrofit (the user's rule
  that a feature saga ends by rewriting what worked around it).
- What worked: the parsing and desugaring were already done, so the
  lane began by proving the law, and the property tests found no bug;
  every error case was a spec case first; a survey of the sources
  (`docs/dogfooding.md`, Retrofit audit) gave the retrofit its list.
- What to watch: a type error in a train is still reported where the
  train is applied, over the whole call and without notes, when the
  mismatch comes from its elements' types together (the mean
  `['+ r_/ / t_ally]` on Floats); that is the checker's error
  locality in general, kept as friction in `docs/dogfooding.md`.
  Previews of the poster need JuliaMono, or the combining underline
  draws as an underscore.

## Saga 16 -- life (M8)

The Life case is active from Saga 6; block, blinker, glider and
random-board goldens (random boards checked against a reference
implementation in Rust test code). No Life-specific code paths.

## Saga 17 -- trace-and-explain

Trace tree (NodeId, span, value, type, shape, children) following the
evaluation order (E4); `xetal explain` prints the right-to-left
derivation; expanded (long-name) printer. A visual stepper, `xetal debug FILE`,
reuses the Saga 7 view model and widgets: the rendered source pane
highlights the node being evaluated (by its span), and the array
viewer shows its intermediate value, stepping forward and back
through the trace tree. A step exposes the trace tree through
`xetal-play` (ask D8 from X_eTaL-demos: a per-operation trace that
programs embedding XeTaL can show).

## Saga 18 -- transducers (research, then perhaps a library)

Asked for by the user (2026-10-01), to research and possibly
implement, in step with sw-mlpl's planned literate document on
transducers (`../../sw-ml-study/sw-mlpl`, step transducers-literate:
Clojure-style transducers taught without closures, through partial
application). XeTaL has what that design works around: closures,
curried functions, quoted operands, composition and the Combinators
library. A reducing function is `{ a x -> ... }` (accumulator on the
left, item on the right), and a transducer turns one reducing function
into another, so mapping, filtering and their composition are ordinary
higher-order functions.

What the research must settle with the user (each recorded in
lang-choices):

- folding: XeTaL's reduce is APL's (a right fold, no starting value);
  a transducer needs a left fold from an initial accumulator, so a
  built-in or library fold with a seed (and its name) is the first
  decision;
- early termination (Clojure's `reduced`): how a reducing function
  says "stop" (a Maybe-like wrapper, or a flag in the accumulator);
- stateful transducers (taking, deduplicating): state carried in the
  accumulator, or closures over mutable (`!`) names;
- streams: the same pipeline over an array, the lines of a file and
  typed input, in constant memory, which is where transducers beat
  whole-array code.

| #  | Step slug        | Delivers                                                   |
| -- | ---------------- | ---------------------------------------------------------- |
| 1  | research         | docs/transducers.md: Clojure's design, sw-mlpl's plan, APL/BQN folds, what XeTaL lacks; the decisions above put to the user |
| 2  | fold             | the seeded left fold (as decided), test-first, with its type and reference entry |
| 3  | library          | `lib/Transducers.xtl`: mapping, filtering, taking, mapcat, composition, transduce, early termination; each export typed and tested |
| 4  | examples         | one pipeline reused with sum, count, max, collect and join; over an array, a file's lines and typed input; a data-cleaning pipeline feeding a running mean and a histogram; goldens |
| 5  | literate         | docs/literate/transducers.org: the idea, the library, and an honest comparison with the whole-array idiom (masks and reduce), saying when each wins |
| 6  | release          | README, reference, CHANGES, retrospective; lessons fed back to sw-mlpl |

## Deferred (from `lang-choices.md` section 15)

Nested arrays (A7; now in the classics lane), raw strings `r"..."`, Unicode text and complex
numbers via a type-extension mechanism, checked `::` signatures, axes
above 9, count-from-the-end axes, the `_` wildcard
parameter. Each gets a saga (or steps) when scheduled.

## Saga 22 -- speed (vector kernels)

Ask D2 from X_eTaL-demos: whole-array arithmetic costs about 28 to 50
ns per item per operation (measured on 1M Floats), and the
cross-cutting evaluator-speed entry below found the same in TTTML.
Steps: checked-in benchmarks first (elementwise arithmetic, rotate,
reduce and scan with primitive operands, TTTML training); then
elementwise kernels on whole Int and Float arrays; then a primitive
operand of reduce, scan, outer and inner product applied as a vector
kernel; then the retrofit (demos' workarounds) and a release with the
numbers before and after. No language change.

## Saga 23 -- host bindings and native packages

Asks D3 (X_eTaL-demos: bind host arrays into a run of `xetal-play`
and read results after, or keep a session) and E1 (X_eTaL-extensions:
host-registered typed functions, then `[]S_VO` loading a native
package), one design for both. Steps: decisions with the user (the
API for binding typed host values, a session object reusing the
REPL's, the names, the ABI, which X_eTaL-extensions' ABI V1 can start
from); host bindings and sessions in `xetal-play`; host-registered
typed functions; `[]S_VO` and a package manifest (absorbing the
`[]S_VO` part of "Later -- system I/O" below); the retrofit; the
release.

## Saga 24 -- transpose  [DONE, ARCHIVED]

Ask D9 from X_eTaL-demos, and the cross-cutting entry below: `o_\`
(reserved in A2 and B2), monadic transpose of a matrix, the mirror of
rotate, with axis subscripts (their rules confirmed with the user
first); then `[]P_ATH` may accept points as an n by 2 matrix. Needed
by ADVANCEDEX's BIN, FC and INV too. Steps: decisions, transpose,
axes, the retrofit, the release.

Retrospective (worked as the transpose lane; its saga is archived in
`lanes/transpose/.agentrail-archive/`):
- Decided with the user first (B17): `o_\ A` reverses the axes (APL,
  J); a name has one arity, so permuting is a word, `p t_ranspose A`
  (axis i to p[i], no diagonals); `o_\_jk` swaps two axes, and a
  repeated digit is an explicit error, never a swap that does nothing;
  `[]P_ATH` keeps 2 rows.
- Delivered in five PRs (#33-#36 and this one): the `xetal-transpose`
  crate (one `permute` kernel, property-tested), the axis rule, the
  tour, README, idioms and poster, and the retrofit (Mastermind's
  codes, Hanoi's moves, TTTML's position code), outputs unchanged.
- What worked: asking the four questions before any code, and the
  retrofit list from `docs/dogfooding.md`; the question that turned
  out flawed (one name for both monadic and dyadic transpose) was
  caught by B9 and asked again before building.
- Left for later: diagonals (`1 1 t_ranspose M`), if a program asks;
  the X_eTaL-demos attention and PCA demos can now be written.

## Later -- system I/O

After the quads: file functions in the quad namespace (read and write
text and bytes; the web demo maps them to local storage), `[]S_VO`
reserved for channels to special facilities (graphics, a Rust dynamic
library) as the FFI-like escape hatch, and networking as a library on
it (lang-choices section 15). Names to be decided with the user; the
APL ports (Sagas 11, 12 and 14) may call for it sooner.

## Cross-cutting (insert as steps when due)

- editor panes (requested by the user): zoom any one pane to full
  screen and back with Ctrl-T (chosen with the user; Ctrl-F stays
  Emacs forward-char) is done (Saga 9, editor-panes); resizing the
  panes (split ratio, the output pane's height) remains, a natural
  step of the stepping-debugger saga, which reuses the panes.
- transpose: done (Saga 24, B17).
- retrofit (the user's rule, 2026-10-02): a saga that adds a feature
  ends with a step rewriting the programs that worked around its
  absence (docs/dogfooding.md lists them), goldens rebased on purpose.
- tail calls (found streaming tttml-train in the live demo): a
  recursion thousands of calls deep overflows a browser's stack (a
  worker's especially; the CLI runs on a large thread stack), so
  TTTML trains in rounds with `p_ower` instead. Evaluating a call in
  tail position as a loop would let any program recurse that deep.
- fuzzing (`cargo-fuzz` for lexer, parser, fmt, eval) -- after Saga 2.
- evaluator speed (found porting TTTML): a primitive operand of reduce,
  scan, inner product or table (`'+ r_/`, `'+ '* i_nner`, `'m_in r_/`)
  is applied one element at a time through the general call path, and
  most of TTTML's training time is allocation there; applying it as a
  vector kernel instead would speed up every such program.
- nesting depth: done in Saga 3 step 1 (bracket and AST depth limits,
  `too-deep`).
- install tooling: `x_etal` alias next to `xetal` (S7) -- with Saga 2's
  release step.
