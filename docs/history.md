# X_eTaL -- A History of the Language

This is the story of how the language got its shape: what changed in
the syntax and semantics, when, and what pressure caused each change.
It covers the repository from its first commit (2026-09-27) to
2026-10-18.

It is a companion to two other documents:

- `docs/chronology.md` tells the story of the whole ecosystem (demos,
  games, libraries, extensions, ML) through 2026-10-05. Read it for
  what was built; read this for why the language moved.
- `docs/design.md` section 9, the decisions register (D1 to D174),
  and `docs/lang-choices.md` are the normative record. Every decision
  named here by its ID is there with its full text. Where this history
  and those documents differ, they govern.

Dates are UTC, taken from the commit that first recorded a decision.
Sources: git history (1,057 commits and 143 merged pull requests on
`main`), `docs/plan.md` and its saga retrospectives, the decisions
register, `docs/lang-choices.md`, the archived sagas in
`.agentrail-archive/`, `CHANGES.md`, and the archival research
conversations `docs/research.txt` to `docs/research8.txt`.

## Contents

1. How the language is decided
2. Before the code: the research (09-27 morning)
3. The first reversal: from the research to `lang-choices.md` (09-27)
4. Calculus, types and arrays (09-27 to 09-28)
5. Axes, Life, and the display (09-28)
6. Libraries, namespaces and system names (09-29)
7. The classics: nested arrays and Unicode in strings (09-30 to 10-01)
8. Trains made trustworthy; literals and transpose (10-01 to 10-02)
9. A steppable machine and typed screens (10-02 to 10-03)
10. Macros become the extension mechanism (10-03 to 10-04)
11. Errors of one's own, events and tables (10-05)
12. Kinds, private names and tuples (10-06 to 10-10)
13. Immutability tightened, and a linter (10-08 to 10-13)
14. Notation inside strings (10-12 to 10-18)
15. Readable errors (10-17)
16. Decisions that were reversed
17. What drives a change

## 1. How the language is decided

The process is part of the history, because it explains why the
language changed in the way it did.

- **One question at a time, with the user.** On the first day the
  language was settled as fifty numbered questions (Q1 to Q50) and six
  review items (R1 to R6), each put to the user and recorded in
  `docs/lang-choices.md`. Every later language change went the same
  way: a proposal document (often with alternatives), the user's
  choice, an entry in `lang-choices.md`, and a row in the decisions
  register. The register grew from about 20 entries on 09-27 to 174 on
  10-18; its busiest days were 10-04 (23 entries, the macros) and
  10-17 (14, notation and errors).
- **Tests define the language.** The spec cases in `spec/`, crate
  tests and the reg-rs goldens in `reg/` are the specification. A
  syntax change starts with a failing test, and a decision a test pins
  is written into `docs/design.md` in the same commit.
- **Ambiguity is an error.** The parser never chooses between two
  readings, and grammatical class comes from tokens alone, never from
  types or spelling. This rule has decided more syntax questions than
  any other: several proposals were dropped because they would have
  needed the parser to guess.
- **Sagas and lanes.** Work is planned as agentrail sagas, each a
  milestone of `docs/PRD.md` (M0 to M9) or a feature, broken into
  steps. From 09-30 work also ran in parallel lanes on branches
  (classics, trains, macros, tuples, lint, readable errors), each
  merged by pull request. On 10-17 this became a release train: lanes
  check themselves cheaply, an integrator merges ready pull requests
  in sequence, and a nightly full suite is the test of record.
- **Dogfooding generates the requests.** From 10-01 the sibling
  repositories (demos, games, libraries, extensions, ML) filed asks
  against the core; `docs/asks.toml` and the generated `docs/asks.md`
  track them. Many decisions below cite an ask by its code (G19, M13,
  X19, E5 and so on).

## 2. Before the code: the research (09-27 morning)

`docs/research.txt` opens with the founding idea: an array language
as terse as APL, but typed in plain ASCII, where *typographic
decoration* (underline, subscript, superscript) turns an ordinary
letter into a function, an axis specialization or a derived function.
The decorated form would be produced by an input method, LaTeX or
Unicode rendering, so the programmer types letters and sees notation.

`docs/research2.txt` found the name: LaTeX reversed is XeTaL, with
the same alternating upper and lower case, and the backronym became
the eXperimental Extensible Typed Array Language. `docs/name.md`
records how it is spelled and said.

The research converged on a notation in which a plain name is a
noun, an underlined name a function, subscript digits an axis, and a
superscript letter a namespace. Its Life rule was
`life = { (+ r_ -1 0 1 t_12 _r) = 3 + _r }`.

## 3. The first reversal: from the research to `lang-choices.md` (09-27)

The bootstrap commit (b1a8faf, 09-27) wrote `docs/design.md` and a
first lexer and renderer, and in doing so departed from the research:

- Superscripts became *derivation* (`+^r` reduce, `+^s` scan) and
  namespaces moved to a dotted prefix (`m.f_`). The reason was real:
  under right-to-left application `+ r_ A` parses as `+ (r_ A)`, so
  reduce seemed to need a separate operator class.
- `_` served two roles, underline and subscript.

Re-reading the research the same day produced
`docs/syntax-proposal.md`, which listed every divergence and proposed
one key per decoration. It was never adopted as written. Instead the
user answered Q1 to Q50, and `docs/lang-choices.md` superseded both
documents. The lexer was then rewritten to it (step
`001-lexer-revision`, 09-27). What came out of that day, and still
holds:

| Question | The research or bootstrap | Decided (09-27) |
| -------- | ------------------------- | --------------- |
| Operators | a separate class, or superscript derivation | no operator class: every function takes one argument, dyadic use is currying, and a quoted function binds to the function on its right as its operand: `'+ r_/ A` (F1, F7, F8) |
| Namespaces | superscript letter, then dotted prefix | a leading prefix: `u:` for the program, `l:` for a library's exports, aliases chosen by the importer through the `u_se<` macro (N5, D14) |
| Superscript | namespace, then derivation | an exponent after a value (`x^2`); spaced `^` is the power function (D-1 to D-6) |
| Function names | underline anywhere | exactly one `_`, directly after a letter; a later `_digits` is an axis subscript (N2, D-8) |
| Binding | `=` | `:=`; `=` is always equality (D1) |
| Index origin | open | 1, not configurable, with `o_ffsets` for 0-based offsets (D6) |
| Evaluation | open | strict, with `~` marking a lazy parameter (D8) |
| Trains | none | in brackets: fork `[F G H]` and atop `[F G]` (D9) |
| Source text | open | ASCII; the decorated form is display only (D13) |
| Built-in names | single letters | words with one underlined letter (`r_ev`, `s_elect`); marks only where APL gave them a meaning (`r_/`, `o_-`) (Q34) |

Two principles from that day explain much of what followed. "One key,
one job" rejected any spelling where one key meant two things.
"A single letter cannot stand for every word starting with it" is why
built-ins are words rather than the research's one-letter `t_` and
`b_`.

## 4. Calculus, types and arrays (09-27 to 09-28)

Saga 2 (calculus, M0 and M1) built the pipeline the decisions
described: lexer, renderer, parser, canonical formatter, Core IR and a
strict scalar evaluator. Two questions surfaced in building it: the
arity of trains (TR4) and how the grammar stays deterministic.

Saga 3 (types-and-unit, M2) added Hindley-Milner inference over Core.
Two choices were made with the user here and shaped every later
feature: numbers are Haskell-style classes `Num` and `Truthy` with
defaulting (D15, T5), and `eval` and `run` type-check first, with
`--untyped` as an escape hatch for experiments such as Y (D16, T6).
`Bool` became a real type with an implicit conversion to 0 and 1
(D5), which Life needed.

Saga 4 (arrays, M3) chose rank-erased array types: a type names the
element type and shape is checked at run time (D17, T7). This is the
decision that keeps X_eTaL's types light enough for array code, and
it was revisited (but kept) when tuples and shape lints arrived.
Saga 5 (higher-order, M4) fixed fold direction (`r_/` is a right
fold, as in APL, J and BQN; D21), dyadic `e_ach` by currying (D22),
and 1-origin, stable `g_rade` (D25).

## 5. Axes, Life, and the display (09-28)

Saga 6 (rotate-and-axes, M5) made an axis subscript work on *any*
function by one rule: move axis k to the front, apply, move it back
(A6, D27). Rotate over several axes gives every combination of
amounts (A4, D7, D26). With that, the Life one-liner ran at 12:46 UTC
on 09-28 with no special case:

```
u:l_ife := { ('+ r_/_12 -1 0 1 o_-_12 _r) { (_l = 3) + _r * _l = 4 } _r }
```

The research's rule had been wrong: `= 3 + _r` keeps a live cell only
when it has three neighbors, so a blinker's center dies. It was
corrected against sw-apl's reference implementations before any code
depended on it (`docs/design.md` section 6.2).

The same day, the display of lambda arguments changed twice. `_l` and
`_r` were first drawn as subscript l and r, then as APL's alpha and
omega, because subscript l was unreadable in the user's terminal
font. The lesson, recorded in the Saga 7 retrospective, was to check
what the user actually sees, not what a Unicode table promises.

Function power arrived as a superscript on a function: `f_^3 x`
applies f three times, with computed counts through `p_ower` (D-7,
D28). It reuses the exponent's literal-only rule rather than adding a
new form.

## 6. Libraries, namespaces and system names (09-29)

Saga 8 (libraries, M6b) built the macro phase as a source map over
several files. The parser, checker and evaluator never learned about
files: imports become hidden namespaces in one combined program.
Library lookup was fixed as the importing file's directory, then
`userlibs/`, then `XETAL_PATH`, then the built-in standard libraries
(D29).

System names were settled as quads: `[]NAME` is one token in the
system namespace, drawn with the quad glyph (D30). Quad functions
take no `!` (09-29), and files and typed input followed as TTTML, the
first substantial application, needed them (D32, QD4).

The combinators (Saga 9, M6) tested how far typed functional
abstraction fits an array language: every bird Smullyan names that
type-checks, spelled by letter (`B_1`, `C_s`), with Y by recursion
(D31).

## 7. The classics: nested arrays and Unicode in strings (09-30 to 10-01)

The classics lane ported the classic APL programs and, in doing so,
forced the first large semantic addition since Life:

- **Nested arrays with static depth.** An enclosed item has type
  `Box a` (D37, D41). A strand of string literals is nested, as a
  strand of numbers is flat, so `"ab" "cde"` is a two-item `Box Char`.
  `e_ach` stays scalar-only and `m_ap` boxes its results (B14).
  Static depth was chosen over APL2's dynamic depth so the type
  checker can still see the structure.
- **Replicate, encode and decode, catenate on an axis, partition**
  (D35, D36, D40, D42), each because more than one classic needed
  it and a local workaround would have been copied around.
- **Match** `m_atch` as APL's whole-array match (D34).

Two display decisions came from the same period. Decimal exponents
are drawn raised with a middle dot as the point, since Unicode has no
superscript full stop (D33). And the source rule was revised: string
literals and comments may hold any Unicode, so a library can greet
with the drawn logo, while code stays ASCII and decorated input is
still rejected (I1, D13 revised, 09-30).

## 8. Trains made trustworthy; literals and transpose (10-01 to 10-02)

Trains existed from the first day, but terse syntax that cannot be
explained is a liability. The trains lane (Saga 15, M7) made them
trustworthy rather than larger: property tests check that every train
form evaluates exactly as its desugaring (D45), an error inside a
train points at the element at fault (D46), and the message says what
the train means at that point (D47). The tour, the reference and the
syntax poster show each train beside the code it means, and lambdas
that only spelled out a train were rewritten as trains.

Requests from the sibling repositories added two small pieces of
syntax:

- **Exponent literals** `1.5e-7` (S8, D49), asked for by the demos'
  physics.
- **Transpose** `o_\` and `t_ranspose` (B17, D48), which had been
  reserved in A2 on 09-27 and which three repositories were carrying
  local copies of. The copies were deleted once it landed.

## 9. A steppable machine and typed screens (10-02 to 10-03)

The games asked for a terminal. The first idea, a WASI target, was
dropped in favor of making the evaluator an explicit machine whose
state is data, so a program can pause for typed input and resume
(D50, Saga 25). The terminal is a grid and line editor over that
machine (D51). Screen control is typed: the checker gained nominal
built-in types for `Color` and `Key`, rather than magic numbers
(D52, QD6).

The same week settled two inference details from real programs: a
top-level value whose type is still a condition is generalized rather
than defaulted to Bool (D53), and a file starting with `#!` is a
program, never a library (D54).

On 10-03 a release review (`docs/research3.txt`, then
`docs/research4.txt`) reprioritized the plan: stop adding breadth,
fix the visible problems (a higher-order slowdown found by the demos,
Saga 30), finish macros, and document what exists.

## 10. Macros become the extension mechanism (10-03 to 10-04)

Some abstractions cannot be eager functions: control forms that
evaluate selectively, domain notation checked at compile time,
repetitive bindings. The macros lane (Saga 19, pull requests #41 to
#71, 26 steps where 9 were planned) answered with a macro phase that
runs X_eTaL itself.

- A macro is an ordinary function whose name ends in `<` (N3, MC2),
  taking and returning source text. `xetal expand` shows what it
  wrote (D61).
- Macro libraries are `.xtlm` files, found beside the `.xtl` under one
  alias (MC10, MC11, D63).
- Namespace prefixes may be long (`nn:`, `cs:`), so a large ecosystem
  does not run out of letters (MC13, D64).
- The system macros were rewritten from Rust generators into
  X_eTaL itself, in `lib/System.xtlm`, over small compiler hooks
  (D65, D66). From then on a new system macro was a few lines of
  X_eTaL: `d_bg<`, `a_ssert<`, `f_ormat<`, `p_anic<` and `t_odo<`
  followed Rust's macros (D57, D67 to D69).
- `@` may stand among named parameters for a side that takes nothing
  (L6 extended, 10-04), so a macro with an empty side has a type.
- Hygiene was made automatic: a macro's binders were renamed apart
  from its call's text, with declared anaphora (`## binds:`) as the
  exception (D77). This was reversed on 10-12 (section 13).
- `##` doc comments with `## >>` examples became syntax that tools
  read: `xetal doc` builds a site from them and runs the examples as
  tests (S9, S10, D72 to D83).

`u_se<` stays built into the compiler, because what it does (finding
and binding a library) is not text, and a macro can only give text
(MC21).

## 11. Errors of one's own, events and tables (10-05)

The libraries asked for raise and catch. Rather than an untyped
exception, the design used the type system and the steppable machine:
`"code" []S_IGNAL "message"` has a polymorphic result because it never
returns (D86); `[]T_RAP` runs a body and a typed handler that answers
recover, retry or halt, and `[]E_NSURE` guarantees cleanup (D89);
`default []W_ARN "code" "message"` is a resumable warning that knows
the type of the value it stands for, so continuing is sound (D90).
Seven system macros give the common shapes (`t_ry<` and friends,
D91).

The Rosetta stone (Saga 34), a flagship demo written mostly in
X_eTaL, defined the host boundary as values rather than callbacks:
a nominal `Event` type fed from a queue (D93) and data tables read
from TOML (D94). The decision to keep TOML and not invent a data
notation was deliberate; a safe, data-only `.xtln` notation was
planned as a separate saga (Saga 37).

The same day American spellings became a rule checked by the gate
(D88).

## 12. Kinds, private names and tuples (10-06 to 10-10)

- **Empty arrays remember their kind** (D103, D107, T9). An empty
  vector of characters must still print and compare as text; the
  array records whether it holds numbers, characters or boxes, and
  every structural primitive keeps it.
- **Private names, `h:`** (decided 10-07; D119 to D121, D125, PN1 to
  PN7). Bare top-level helpers in a library leaked into the namespace
  where built-ins live, so adding a built-in could be shadowed by any
  file. `h:` marks a file-private definition; a bare top-level
  function in a library is deprecated with a `xetal migrate` fix; a
  library that exports nothing is an error. An app's interface is its
  `u:` names.
- **Tuples** (decided 10-07, released 10-10; Saga 39, D122 to D136,
  TU1 to TU12). X_eTaL-ML (ask M13) and the demos (ask D7) were
  packing unrelated state into Float matrices and reading it back by
  index. `docs/tuples.md` compared other languages and recommended
  `(w, m, v, k)`: the comma is a token legal only directly inside
  parentheses or a pattern, parts are evaluated right to left like an
  array's, and tuples are destructured by patterns only, in bindings
  and lambda parameters (D126). A hidden class `Arr` keeps a tuple
  from standing where an array is expected (D124), while `e_nclose`
  and `m_ap` take any value, so tuples can be boxed (D132). The
  Rosetta stone's packed state matrix was rewritten as a tuple to
  prove it (D134).

Records, enums and matching (the rest of Saga 29) are under review
since 10-10: the user does not want them in the ML style, and
`docs/adt.md` proposes an array-first alternative (accessor macros
over tuples, perhaps one small tag type).

## 13. Immutability tightened, and a linter (10-08 to 10-13)

**Names are bound once** (M1 changed with the user 10-08, built
10-09; D135). Until then, binding a name again in the same scope
silently shadowed it. A survey found 235 such bindings across the
repositories, 106 of them throwaway names like `shown :=` used only
to run an effect. The rule became: a name without `!` is bound once
per scope (`error[rebind]`), a name ending in `!` is a variable that
changes, and `_ := expr` runs an expression for its effect and binds
nothing (M3).

**A clippy-style linter** (Saga 40, from 10-10; D137 onward, S12).
`xetal lint` runs passes over Core and, later, over types and macro
contracts. Lint levels are ordinary comments (`# lint: allow NAME`),
so no new syntax was needed; `xetal run` honors them too (D138).
Fixes are machine-applicable, as rustc's are, and `--fix` refuses to
touch a file git could not restore (D144, D149). Building the lints
exposed real language questions:

- **Case carries no meaning** (D139). Since decoration decides what a
  name is, the naming style guide became advice, and a combinator may
  be a capital with any variant mark.
- **Power is not a section** (D146). `redundant-lambda` suggested
  `'^` for `{ a b -> a ^ b }`, but a touching `^` is an exponent
  (D-6), so `'^` is an error; the lint learned the exception.
- **Inference must be deterministic** (D148, ask G20). A group of
  mutually dependent definitions was closed in hash-map order, so a
  default could differ between runs.

**Hygiene by choice** (MC30 changed with the user 10-12; D151, ask
X19). Automatic renaming (section 10) depended on guessing which
bytes of an expansion came from the call, and broke macros that bind
a name for their caller on purpose. Hygiene became a choice, as in
Common Lisp rather than Scheme: names a macro writes stay as written,
`[]G_ENSYM` gives a fresh name when one is wanted, and declared
anaphora are protected. The linter took over the safety the renaming
had given (origin-aware lints, D162).

## 14. Notation inside strings (10-12 to 10-18)

The most recent arc is about mathematical and pipeline notation that
is not X_eTaL syntax, and how to admit it without making the grammar
ambiguous.

- **Spatial equations as a library, not syntax** (planned 10-12,
  decided 10-15; D154, `docs/spatial.md`, `docs/eq-rationale.md`).
  Index-notation equations such as a convolution are written as a
  string given to a macro, which checks extents and expands to array
  code. The compiler was not changed. `docs/research8.txt` had
  explored two-dimensional expressions; keeping the notation inside a
  string kept the core grammar line-based and unambiguous.
- **Pipelines** (D155) from the user's `docs/xetal-ml-pipelines-design.md`:
  stages joined in a macro's text.
- **Raw strings** `r"..."` and `r#"..."#` (decided 10-15, built 10-16;
  ST4, D156), with Rust's rules, because a notation full of quotes
  and backslashes is unreadable when escaped. A raw string is another
  spelling of the same Char vector, not a new type.
- **Plain names** (10-17; D160, D161). A library's names must read
  under whatever alias an importer chooses: `"m:" u_se< "Equations"`
  turned `e:q_<` into `m:q_<`, which reads as nothing. The equation
  macro became `e:e_q<`, and a survey renamed every export chosen for
  being short and clever rather than plain.
- **Macro contracts** (10-17; MC31, D163). In the Equations case a
  reader dropped `^2` and summed the wrong thing in silence. A macro
  now declares in its `##` block what its sides take, what it binds
  and what it uses, and the runner and the linter check it (D168,
  D174). The shape lint's check was named plainly too:
  `r_equireShape` (B19, D166, renamed 10-18).
- **Code strings** `c"..."` and `c#"..."#` (decided 10-17, built 10-18;
  ST5, `docs/code-strings.md`). A macro's argument is often X_eTaL,
  but to every tool a plain string is opaque. A code string declares
  in a token that its inside is code, so the renderer draws it
  decorated and the formatter and linter can read it, while an
  equation (a notation, not X_eTaL) stays a plain string. A combined
  raw code form `cr"..."` was proposed and dropped.
- **Two pipeline tokens** (10-18). `|>` joins stages and is drawn as a
  white triangle (S13); `$h` labels a stage's value so later stages
  can fan out and in (S14). Both are single tokens that are errors in
  code and appear only in a macro's text. They were chosen over a
  draw-only ligature and over spellings such as `v>`, which the lexer
  would read as a name.

The display gained matching glyphs for `##` doc comments and `###`
headings (I4, D165).

## 15. Readable errors (10-17)

Saga 27, planned with the user on 10-03, landed as a lane on 10-17:
every diagnostic is placed as `FILE:LINE:COLUMN` with the line
underlined (D169); a type mismatch says where each side's type came
from (D170) and gives a hint for the common mistakes, such as mixing
Int and Float (D171); and an error raised inside a library also names
the program line that called it (D173, ask E5). No syntax changed,
but it was the release review's most visible gap.

## 16. Decisions that were reversed

The language changed its mind in the open. Each reversal below is
recorded in `docs/lang-choices.md` with its earlier form.

| When | Was | Became | Why |
| ---- | --- | ------ | --- |
| 09-27 | superscript as namespace (research) | prefix namespaces `u:`, `l:` | under right-to-left reading the superscript forms needed an operator class; prefixes are typeable and unambiguous |
| 09-27 | superscript as derivation (bootstrap) | superscript as exponent and function power | one key, one job; reduce became an ordinary function taking a quoted operand |
| 09-27 | one-letter built-ins (`t_`, `b_`) | words (`o_-`, `t_able`) | one letter cannot stand for every word that starts with it |
| 09-28 | Life rule `= 3 + _r` | Conway's rule | checked against sw-apl; a blinker's center died |
| 09-28 | lambda arguments drawn as subscript l, r | alpha and omega | unreadable in the user's terminal font |
| 09-30 | source wholly ASCII | strings and comments may hold any Unicode (I1) | a library could not greet with the drawn logo; code stays ASCII |
| 10-02 | a WASI target for the terminal | a steppable evaluator (D50) | pausing for input fits a machine whose state is data |
| 10-08 | rebinding shadows in the same scope | names bound once, `_ :=` (M1, M3) | silent shadowing hid mistakes; most rebindings were throwaway names |
| 10-12 | automatic macro hygiene | hygiene by choice, `[]G_ENSYM` (MC30) | the renaming guessed at bytes and broke intentional captures |
| 10-17 | `e:q_<` and other short names | plain names, `e:e_q<` (D160, D161) | names must read under any alias |
| 10-17 | `cr"..."` proposed | `c"..."` and `c#"..."#` only (ST5) | one level of code is enough; a verbatim code form already exists |

## 17. What drives a change

Looking across three weeks, almost every language change came from
one of five pressures:

1. **An acceptance program.** Life forced axes, rotate combinations
   and Bool as a number before the rest of the language existed.
2. **Dogfooding.** The classics forced nested arrays; the games forced
   the steppable machine; TTTML forced files; X_eTaL-ML and the demos
   forced tuples; the Equations library forced contracts and plain
   names. The rule is that a workaround copied in more than one place
   is evidence, and a general mechanism moves down into the core.
3. **The user's hands-on use.** Glyph choices, Unicode in strings,
   naming relaxed, the Rosetta stone's controls, and the pipeline
   tokens were all decided while the user was using the tools.
4. **The no-ambiguity rule.** It rejected the research's superscript
   reduce, kept equations out of the grammar, made `'^` an error, and
   decided that `|>` and `$` are tokens rather than ligatures.
5. **Release reviews.** The 10-03 review turned the project from
   breadth to quality: speed, macros, docs and, eventually, readable
   errors.

The open questions at the time of writing are listed in
`docs/lang-choices.md` section 15 and `docs/plan.md`: records, enums
and matching (Saga 29), complex numbers (Saga 31), the data notation
`.xtln` (Saga 37), integrals (Saga 42), and whether `a_ssert<` should
stop the program as Rust's `assert!` does.
