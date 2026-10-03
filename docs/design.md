# X_eTaL -- Language Design

Status: the language decisions are recorded, with their reasons, in
`docs/lang-choices.md`; this document summarizes the design and says
where each part is pinned by tests. When a test and this document
disagree, the test wins and this document is updated in the same
commit. Research transcripts (`docs/research*.txt`) are archival.

Name: X_eTaL (display name). Code uses the slug `xetal` (crates,
binary) and a single display-name constant `xetal_base::LANG_NAME`,
so a rename touches one constant plus docs.

## 1. Guiding rules

1. Tests define the language.
2. Decoration determines grammatical class: an underlined letter
   makes a function name. Spelling never does, except for the fixed
   dictionary of built-in names and macros.
3. Ambiguity is an error. The parser never "picks something
   reasonable"; types may reject a unique parse but never choose
   between parses.
4. All surface sugar desugars to a small Core IR.
5. Built-ins and user definitions follow identical application rules.
6. Invalid valence/type is an error; arguments are never ignored.
7. Code is read more often than written; definitions may be verbose,
   uses should be terse (lang-choices P1, P2).

## 2. Lexical structure (raw ASCII source)

Source is ASCII, except that string literals and comments may hold any
Unicode (lang-choices I1, ST2). Spaces, tabs, carriage returns
and `#` comments (to the end of the line) separate tokens; each `\n`
is a `Newline` token. Pinned by `components/syntax/crates/xetal-lex/tests/lex/` and
`spec/lex/*.case`; `xetal lex` prints the tokens with byte spans.

| Token | Raw examples | Rules (lang-choices) |
| ----- | ------------ | -------------------- |
| variable | `x`, `board2`, `count!`, `m:pi` | letters and digits; trailing `!` marks a mutable variable (N1, N4, M2) |
| function name | `r_ev`, `self_`, `u:s_quare`, `c:K_` | exactly one `_`, directly after a letter, which underlines that letter (N2) |
| trailing mark | `r_/`, `o_-`, `e_mpty?`, `u_se<` | one of `\| - / \ + * < > ~ ! ? % $ &` ends a function name; `<` marks a macro (N3, MC2) |
| namespace | `u:`, `c:`, `l:`, `m:` | letters then `:`, directly before a name (N5) |
| axis subscript | `r_/_2`, `o_-_12`, `r__2` | `_digits` after a function name, one digit per axis, 1-9, no repeats (D-8, D-9) |
| exponent | `x^2`, `x^-1`, `(a + b)^2` | `^` touching a value token or `)`, then a number literal (D-1 to D-5) |
| number | `42`, `2.5`, `-1` | a `-` before a digit at a token boundary is a negative literal |
| string | `"abc"`, `"a\"b"` | single line, ASCII, escapes `\"` `\\` `\n` `\t` (ST1, ST2) |
| symbol | `+ - * / ^ = != < > <= >= & \|` | all dyadic; spaced `^` is power (section 8, D-6) |
| lambda argument | `_l`, `_r`, `_l_`, `_r_` | `_l_` / `_r_` apply the argument's value (L1, F5) |
| other | `:=` `->` `?` `'` `~` `@` `;` `( ) { } [ ]` | binding, parameter arrow, guard (spaced), quote, lazy marker, Unit, separators |

Rejections (each has a test): an underline after a digit (`a1_`), two
underlines (`a_b_c`), anything after a trailing mark (`f_-1`), axis 0
or a repeated axis, `x^n` (use `x ^ n`), a power on a symbol or with
a count that is not a whole number literal (`+^r`, `r_ev^n`,
`r_ev^1.5`; `r_ev^-1`, the inverse, is reserved), a bare namespace (`u:`), `_@` sugar (`n_ow@`),
`x!=3` (write `x != 3` or `x! = 3`, R2), `3-1` (write `3 - 1` or
`3 -1`), a name touching a string (reserved for `r"..."`), the
complex-number literal `3j4` (reserved), and non-ASCII characters.

## 3. Grammar

Pinned by `components/syntax/crates/xetal-syntax/tests/parse/`, `spec/syntax/*.case`
and the ambiguity corpus `spec/ambiguity/*.case`; `xetal parse` prints
the surface tree as S-expressions (`10 u:s_ub 3` is `(u:s_ub 10 3)`).
The grammar is deterministic: tokens decide every rule, so an input has
at most one parse, and shapes near a rule boundary (`a b`, `- 3`,
`f_ g_`, `{ _l }`, a value in a train, ...) are rejected with specific
errors instead of being resolved.

Limits (no-panic rule): brackets `( ) { } [ ]` nest at most 64 deep
and an expression tree at most 256 levels (each application in a
chain and each statement of a lambda body counts); deeper input is
`error[too-deep]`, so no stage can overflow the stack. Long strands
and long programs (many statements) are not limited.

- Application is right to left with long right scope and no
  precedence: `f_ g_ x` is `f_ (g_ x)`; `x f_ y` is dyadic. Every
  function has one arity and dyadic use is currying:
  `x f_ y = App(App(f, x), y)` (F1, F3). A symbol applied to one
  argument is an error (SC1).
- Numeric strands form vector literals (`-1 0 1`).
- A quote passes a function as a value: `'r_/`, `'+`,
  `'{ x -> x * 2 }`, `'[F G]` (F4). A quoted function directly left of
  a function name is its operand: `'+ r_/ A` (APL `+/A`),
  `A '* t_able B`, `A '+ '* i_nner B` (F8, F9).
- A function value is applied with an underline: `_l_ x`, `(expr)_ x`
  (F5).
- Lambdas: shorthand `{ _r * _r }` (monadic) and `{ _l - _r }`
  (dyadic); named parameters `{ f_ g_ x -> f_ g_ x }` (function
  parameters are underlined), niladic `{ @ -> ... }`, lazy
  parameters `{ ~s_elf n -> ... }` (L1-L7, E1). `_l` / `_r` refer to
  the innermost lambda. An inline lambda is an ordinary function:
  `X { ... } Y`.
- Trains: `[F G H]` fork, `[F G]` atop (TR1-TR3).
- Guards inside lambdas: `condition ? result` (G1, G2).
- Statements: binding `name := expr` (`=` is always equality); a
  newline separates statements at the top level and inside `{ }`, `;`
  separates them on one line (S1-S3).

## 4. Core IR

```
Item := Def(Global, Expr) | Let(Id, Rec, Expr) | Set(Id, Expr) | Eval(Expr)
Expr := Lit | Str | Unit | Array(Expr*) | Var(Id) | Global(Name)
      | Prim(Name) | Axes(Digits, Expr) | Lam(Param, Lazy, Expr)
      | App(Expr, Expr) | App2(Expr, Expr, Expr)
      | Let(Id, Rec, Expr, Expr) | Set(Id, Expr, Expr)
      | If(Expr, Expr, Expr) | NoMatch
```

Pinned by `components/core/crates/xetal-core/tests/core/` and the `CORE` sections of
`spec/syntax/*.case`; `xetal core` prints it with built-ins marked `#`.

- `x f y` lowers to `App2(f, x, y)`, which means `App(App(f, x), y)`
  but evaluates the function, then the right argument, then the left
  (E4); `(f x)_ y` lowers to the nested `App`.
- A quoted operand is curried: `'+ r_/ A` is `App(App(r_/, +), A)`,
  the same Core as `(r_/ '+)_ A`. `x^2` is `App2(^, x, 2)`, the same as
  `x ^ 2`.
- Names: `ns:` names are module globals, defined once per file and
  late-bound (so definitions may refer to each other); plain variables
  and parameters are lexical, so a lambda keeps the values it captured
  (M1); an unqualified function name is a parameter or local binding
  if one is in scope (L7), otherwise a built-in.
- A binding whose value is a lambda or train is recursive (`letrec`);
  other bindings are not, so `x := x + 1` refers to the previous `x`.
  Rebinding a mutable `!` variable already in scope is `Set`.
- Lambda bodies fold into `Let` / `If` chains; a guard with nothing
  after it falls through to `NoMatch` (G2).
- Trains take their arity from position (TR4): monadic trains lower to
  a lambda, a dyadic train written in place binds its arguments (right
  first) and expands `(x F y) G (x H y)` in place. Each application
  a train expands to carries the span of its element, so a type or
  run-time error in a train points at the element at fault (`[n_ot +]
  1 2` points at `n_ot`); a nested train keeps its own elements' spans.
  Lowering also leaves notes for each element's span (`Program::notes`),
  which the checker and the evaluator add to an error reported there
  (`Program::annotate`): the train, the element, what the train means
  at that element written out with x (and y) for its arguments, and a
  hint from the built-ins' arities (a built-in that takes one argument
  given two; one that takes two given one in a monadic train, so its
  result is a function).

Every Core node carries the NodeId and source span of the surface
construct it came from.

## 5. Types

Hindley-Milner inference (Algorithm W) with let-polymorphism over
`Unit Bool Int Float Char T -> U` (rank-erased: an array has its
element type, T7), type variables and two
constraints, `Num` (Int, Float) and `Truthy` (Bool, Int), plus `Eq`
(any scalar) and `Ord` (numbers and Char) for comparisons (T8); see
`components/types/crates/xetal-types`. Lambda values are generalized (value
restriction); other bindings are monomorphic, and at the top level
their numbers default (Int; a condition Bool) when defined. Module
definitions may refer to later ones: a use before the definition
shares one monomorphic type until the definition is inferred, and
mutually recursive definitions form a binding group generalized
together once no forward reference is pending. A top-level item that
is not a function is evaluated as soon as it is defined, so every
constrained variable it introduces defaults then. After inference the
program is elaborated (`xetal-elab`), so values agree with their
types. A binding generalized over `Num` variables takes one hidden
number-type argument per variable (dictionary passing): the zero of
the type it is used at, `0` or `0.0`, or the caller's own hidden
argument when the caller is polymorphic too; a variable nothing
quantifies defaults to Int. An integer literal of a quantified type
becomes `literal + zero`, one of type Float a Float literal. So
`u:k_ := { @ -> 1 }` used where a Float is expected gives `1.0`, also
through `p_rint!`, recursion, binding groups and local
let-polymorphism. A built-in that makes a result with no typed item
(the identity of `r_/` or `i_nner` for an empty axis) is elaborated
at its result type: at Float `r_/` becomes
`{ #a1 #a2 -> f_loat (r_/ #a1 #a2) }`, at a
quantified number type `... + zero`, so `'+ r_/` of an empty Float
vector is `0.0`. The evaluator runs the result as ordinary Core; the
Core shown by `xetal core` is before elaboration.
`xetal eval` and `xetal run` type-check first and refuse ill-typed
programs; `--untyped` skips the checker (T6). Laziness (`~`) is not part of the type. Scalar functions will be
lifted over arrays by one scalar-extension rule. Shape is runtime
metadata in v0. Self-application (`x_ 'x_`, as in the textbook Y)
has no simple type: `infinite-type`; run it with `--untyped`.

- Bool is a real type; Bool -> Int implicitly (true 1, false 0); Int
  -> Bool only from 1 or 0, anything else is an error (T1). Typed
  Haskell-style (T5): polymorphic number literals, comparisons return a
  `Truthy` value; an Int variable mixed with a Float needs `f_loat`.
- `/` always returns Float; `d_iv` and `m_od` are the integer
  operations; division by zero is an error (T2).
- `=` is exact; `e_q~` is tolerant equality (T3).
- `Int ^ Int` is Int, a negative exponent at run time is an error;
  a Float anywhere gives Float (D-10).
- No annotations in v0; `::` is reserved for checked signatures (T4).

## 6. Arrays

- Leading-axis defaults: with no subscript, every axis-taking
  function acts on the first axis (A1).
- Index origin 1, not configurable; `o_ffsets n` gives `0 .. n-1` (A5,
  B5).
- An axis subscript works on any function: `f_k X` moves axis k to the
  front, applies f and moves it back (A6).
- Row-major dense arrays (`components/eval/crates/xetal-array`, generic
  over the item type); a scalar is not an array, and every array has
  rank 1 or more. Number strands and strings are vectors.
- Types are rank-erased (T7): a type names the element type, so
  scalar functions apply to arrays with no extra rule. At run time a
  scalar function applies item by item, a scalar extends to every
  item, and two arrays must have the same shape (`shape-mismatch`).
  Nested arrays come later (A7).
- Built-in names: see lang-choices B1-B10. Structural kernels live in
  `xetal-struct`, generic over the item type.

### 6.1 Multi-axis rotate

A list of amounts with a multi-axis subscript means every
combination, one leading result axis per subscripted axis:
`-1 0 1 o_-_12 B` on an n by m board has shape 3 3 n m (A4). Reduce
and scan with a multi-digit subscript work over each listed axis in
turn, so `'+ r_/_12` sums the nine boards (R1).

### 6.2 The Life one-liner

```
u:l_ife := { ('+ r_/_12 -1 0 1 o_-_12 _r) { (_l = 3) + _r * _l = 4 } _r }
```

Conway's rule, with N the count of the 8 neighbours and c the cell:
a cell is alive next generation iff `N = 3`, or `c` and `N = 2`.
Summing the nine rotated boards gives S = N + c (the cell *included*),
and the same rule is `(S = 3) or (c and S = 4)`. The two terms are
mutually exclusive, so with 0/1 values "or" is `+`:
`next = (S = 3) + c * (S = 4)`. The inner dyadic lambda receives S as
`_l` and the board as `_r`, so S is computed once.

Reference implementations (sw-apl, `samples/51-life.apl`), in ASCII
transliteration:

- APL\360 (eight explicit rotations): `(3=N) or B and 2=N`.
- The classic APL2 one-liner
  `life <- {disclose 1 w or.and 3 4 = +/ , -1 0 1 outer-rotate-first
  -1 0 1 rotate-each enclose w}`: `3 4 = S` gives the boards `S=3` and
  `S=4`, and `1 w or.and ...` combines them as
  `(1 and S=3) or (w and S=4)`. Its ravel `,` before `+/` plays the
  role of our `_12` on reduce.

History: the research's line `{ (+^r ... _r) = 3 + _r }` computed
`S = 3 + c`, which keeps a live cell only when N = 3; a blinker's
centre died. Verified with sw-apl: on the 5x5 blinker the old rule
gives row 3 = `0 1 0 1 0`, Conway gives `0 1 1 1 0`; the corrected
rule equals the APL\360 reference on the blinker, a glider (1, 2 and
4 generations) and a random 8x8 torus (1 and 3 generations).
Pinned by `spec/integration/life-blinker.case` (pending until M8).

## 7. Evaluation

Strict by default; parameters marked `~` are call-by-need (evaluated
on first use, then remembered). A function is evaluated before its
arguments, and arguments right to left (APL order). The Y combinator
works through a lazy self parameter; Z works too (E1-E4). Values are
immutable; rebinding shadows; `!`-named variables are the mutation
escape hatch (M1, M2). The evaluator consumes Core only
(`components/eval/crates/xetal-eval`, `spec/eval/*.case`): values are Int, Float,
Bool, Char, Unit, arrays of those (`xetal-value`) and functions; variables live in a persistent environment
of shared slots, so a closure keeps the values it captured while a `!`
variable is updated in place; a `~` argument is a thunk forced on
first use and remembered; module definitions are late-bound. In a
dyadic call a lazy parameter is honoured when the function is a
lambda written with both parameters; built-ins are strict.
Higher-order built-ins (`components/hof`, B6) are kernels
(`xetal-kernel`, D50): each asks the evaluator for one call of its
operand at a time, so an operand runs by the ordinary rules whatever
it is, and a run can stop between any two calls. `r_/` is a
right fold along the leading axis, taken from the last major cell in
one pass; item k of `s_\` is the reduce of the first k cells. A scan
accumulates from the left in one pass only when that is provably the
same: `m_ax m_in & |` always, Int `+` and `*` when the magnitudes of
all items are small enough that no grouping can overflow; otherwise
each prefix is folded (quadratic, exact). `e_ach` calls f on every
item in order; when the results are functions it returns a pending
item-wise application (a built-in value `#each` holding them) that
zips with the next argument, which is how dyadic each works by
currying. `t_able` applies f to each item of A once, then the partial
result to every item of B. Both require single values from every call
until nested arrays exist. `i_nner` pairs the last axis of A with the
first of B (a single value extends), applies the nearest operand to
each pair and reduces each run by a call of `r_/`,
so it shares reduce's right fold and identities. `c_ompose` and
`s_wap` are one-line kernels. The search and order built-ins
(`components/search`, B7) work on major cells: `i_ndexOf` looks for
cells of `B` shaped like a major cell of `A`, `u_nique`, `s_ort` and
`g_rade` treat each major cell as one item (rows compare item by
item), `m_ember?` is item by item. Items compare like `=` and order
like `<`; the sort is stable. `w_here` takes a vector of 1s and 0s.
`r_oll!` draws from a SplitMix64 generator owned by the evaluator,
seeded unpredictably unless `--seed N` or `XETAL_SEED` is given (for
tests that need repeatable rolls). A REPL session picks one seed and
replays every accepted line with it, so a rolled value keeps its
value from line to line.
An axis subscript (A6) is Core `Axes { axes, arity, f }`. The checker
records f's type and the elaborator fills `arity`, the number of
arguments f takes (unchecked programs use what f's value shows: a
built-in's missing arguments, a lambda's parameters). The evaluator
turns it into a built-in value `#axes` that collects those arguments;
`xetal-axes` then moves axis k of the last (data) argument to the
front, applies f, and moves the axis back when the result kept its
rank (a result one rank lower consumed it; other rank changes and
axes beyond the rank are `error[axis]`). `c_at` defines its own axis rule
(B15), because its left argument is data too: `c_at_k` moves axis k of
both full-rank arguments to the front, joins, and moves it back,
after checking the other axes agree in the original shapes. Rotate defines several axes and
amount lists itself (every combination, one leading result axis per
listed axis, A4); reduce and scan with several axes apply the rule to
each listed axis in turn, renumbering the later ones when a reduce
consumes an axis (R1). A typed-result built-in under a subscript
(`'+ r_/_12` in a polymorphic function) is elaborated as a whole, so
the axis rule still sees the built-in.
Evaluation runs on its own large stack; runaway recursion is a
`stack-overflow` error. Each top-level expression's value is printed
(section 10a of lang-choices); runtime errors are diagnostics with
spans, and parameters that shadow a built-in give a warning (L7).
A trace tree (`NodeId, span, value, type, shape, children`) comes
with the trace-and-explain saga.

## 7a. The macro phase (libraries)

Between the lexer and the parser (MC1), `xetal-macro` expands a
program's imports. Each file's top-level statements of exactly
`"alias:" u_se< "Library"` are its imports; any other use of a macro
is an error from the MC8 table (missing, malformed or reserved alias,
non-string arguments, a macro that is not a top-level statement, an
unknown macro), as is one alias for two libraries or one library under
two aliases in a file. Libraries are found through a `Libraries`
trait: on disk (`FsLibraries`) a path is relative to the importing
file and a name is `Name.xtl` beside it, then in `userlibs/` (libraries
of your own, in the current directory), then in XETAL_PATH, then
among the standard libraries built from `lib/` (`xetal-libs`).
Programs (`demos/`) are executable files starting with `#!`; libraries
(`lib/`, `userlibs/`) are not, and `xetal run` on one lists its exports'
types (`scripts/check-modes.sh`, in the gate, keeps the modes right). They
load recursively, each once per resolved path (MC7), and an import
cycle is reported with its chain. The result is one combined text
(`xetal-sources`): each library before the files that use it, the
import statements removed, and a source map so that any later
diagnostic is reported in the file and at the place it was written
(`FILE:LINE:COLUMN` when several files are involved).

Namespaces: each library instance gets two hidden namespaces, `LA`
for its exports and `PA` for its private names (`LB`/`PB` for the
next, and so on); they are uppercase, and aliases lowercase, so no
file can write one. A library's `l:` names are renamed to its `LA`,
its unprefixed top-level functions and variables to its `PA` (except
where a lambda parameter or local binding of the same name shadows
them), and an importer's alias letters to the named library's `LA`,
checked against its exports (`not-exported`, listing them). Core
accepts top-level definitions in hidden namespaces (functions and
variables), so a library's names are late-bound globals like `u:`
names. The source map records how each file writes each hidden
namespace, so a message is shown with that file's letters (MC6).
A name finds a file only when its directory entry matches exactly, so
`"Stats"` never finds `stats.xtl` on a case-insensitive disk.

`xetal-program` loads a program for the tools: the macro phase with
libraries on disk and built in, then Core, every error located.
Every tool loads through it: `xetal run`, `eval` and `type`, notebook
runs (`--echo`), `--context` (org-babel sessions), the REPL and the
editor. A file's libraries are looked for beside it, `-e` text's and
the REPL's in the current directory. `type` and the editor's types
pane list the program's own items (`program_types`). The REPL and the
editor show an error in the program at its place in the text typed
(`in_program`: the combined text starts with the libraries), and one
in a library at `FILE:LINE:COLUMN`. The first standard library is
`lib/Stats.xtl` (`m_ean`, `v_ariance`, `s_d`, `r_ange`), shown in
`demos/stats.xtl`. In the pretty views an import is drawn as its
alias bound to the macro: superscript letters and a superscript
equals in the library color, then `u_se<`.

## 8. Display modes (CLI and web)

| Mode      | What it shows                                              |
| --------- | ---------------------------------------------------------- |
| Raw       | the source exactly as typed (the stored form)              |
| Decorated | Unicode glyphs (`xetal render`), losslessly invertible     |
| LaTeX     | LaTeX math for a post-processor (`xetal render --latex`)   |
| Canonical | fully parenthesized raw form (`xetal fmt`)                 |
| Expanded  | long names (later)                                         |
| Core      | the Core IR (`xetal core`)                                 |

Source is raw ASCII (strings and comments aside); the decorated and
LaTeX forms are output. The inverse (`xetal render --raw`) copies
strings and comments as they are, since the drawing never touches
them.
Editing always happens on raw text (prettify-style display, never
destructive substitution).

### 8.1 Decorated Unicode (`xetal render`)

Whitespace and comment text between tokens are copied verbatim.

| Raw | Decorated |
| --- | --------- |
| `r_ev` | U+0332 COMBINING LOW LINE after the underlined letter |
| `u:s_quare`, `m:pi` | the namespace as leading superscript letters (U+1D58 for u) |
| `o_-_12` | subscript digits U+2081 U+2082 after the name |
| `x^2`, `x^-1` | superscript digits (U+00B2 ...) and U+207B for minus |
| `_r`, `_l` | APL omega U+2375 and alpha U+237A (the dfn argument names); applied (`_l_`) also underlined |
| `:=` `->` `;` | U+2190 left arrow, U+2192 right arrow, U+25C6 black diamond (larger than APL's U+22C4, which reads as a dot in terminal fonts) |
| `#` (comment start) | U+235D APL lamp |
| `-` `*` `/` (symbols) | U+2212 minus, U+00D7 times, U+00F7 division |
| `!=` `<=` `>=` | U+2260, U+2264, U+2265 |
| `&` `\|` | U+2227 logical and, U+2228 logical or |
| everything else | unchanged |

Ligatures apply to standalone tokens only: `r_/` keeps its slash, a
negative literal keeps its ASCII `-`, strings and comment text are
unchanged.

Superscript letters for namespaces (lowercase), code points:

| a 1D43 | b 1D47 | c 1D9C | d 1D48 | e 1D49 | f 1DA0 | g 1D4D |
| ------ | ------ | ------ | ------ | ------ | ------ | ------ |
| h 02B0 | i 2071 | j 02B2 | k 1D4F | l 02E1 | m 1D50 | n 207F |
| o 1D52 | p 1D56 | q none | r 02B3 | s 02E2 | t 1D57 | u 1D58 |
| v 1D5B | w 02B7 | x 02E3 | y 02B8 | z 1DBB |        |        |

A namespace containing `q` or an uppercase letter is shown raw
(`q:x`). An exponent with a decimal point is raised like any other,
its point drawn as a middle dot (U+00B7), since Unicode has no
superscript full stop; the inverse accepts that dot only inside a
raised exponent, so it stays exact: ASCII
passes through `--raw` unchanged. The inverse maps each glyph back to
its ASCII spelling; a superscript namespace must precede a name
(`bad-namespace`), an underline must be under a letter
(`bad-underline`), and any other non-ASCII character is
`not-decorated`. Pinned by `components/render/crates/xetal-render/tests/render/`
(proptest: raw -> decorated -> raw is the identity on lexable
sources) and `spec/render/*.case`.

### 8.1a View model (`xetal-view`, `xetal render --color`)

Front ends (the terminal editor and REPL, later the debugger and the
web playground) draw source from one view model: `view(src)` gives
segments in order, each with its raw byte span, its decorated text
(from the renderer's per-token rules, so it always agrees with
`xetal render`) and a class for highlighting (built-in, user
function, library function, variable, lambda argument, number,
exponent, string, symbol, quote, punctuation, unit, comment, space,
error). It never fails: text that does not lex becomes an error
segment shown as typed, and viewing resumes after it; the segments
cover every byte exactly once (a property test over arbitrary text).
`lines` splits segments at newlines and `column` maps a raw offset to
its rendered column (terminal width, combining underlines take none).
`xetal render --color` prints the segments with ANSI colors, and
`xetal render --html` as escaped `<span class="c-...">` runs (the
class names of the live demo's Rendered pane, so one stylesheet serves
both); the editor uses the same palette: system functions blue (symbols such as
`+` light blue), macros such as `u_se<` bold yellow, the program's
functions green, a library's cyan,
lambda arguments magenta, numbers yellow, comments dim, errors red. A
quote takes the class of the function it quotes, so an operand reads
as one unit (`'+` light blue, `'r_/` blue, `'u:p_lus` green); before
a lambda or a train it keeps its own bold style. This is lexical (the
quote and the next token), decided with the user in the tui saga. Two
more display rules, also only in the view (plain `xetal render`
stays convertible back to the exact source): a comment after code
keeps the column it has in the source, the space before it padded or
trimmed (at least one space), so comments line up although the code
is drawn shorter; and code in backquotes inside a comment is drawn
decorated and highlighted, the backquotes hidden, so comments can
show the glyphs while the source stays ASCII (I1). Backquotes are for
X_eTaL code only: a command or a path in a comment goes in double
quotes, which are drawn as written (a `/` in a backquoted path would
be drawn as division).

### 8.1b The editor (`xetal edit FILE`)

A full-screen editor on the view model (`components/tui`): the text
as typed on the left, its decorated and highlighted form on the
right, the cursor mapped into both and the panes scrolled together.
Below, the types of the top-level items (or the first diagnostic,
whose span is marked in both panes) update as you type; Ctrl-R
type-checks and runs, and shows the output there, so effects such as
`p_rint!` and `r_oll!` never run on a keystroke. File keys follow nano
(Ctrl-S or Ctrl-O save, Ctrl-Q or Ctrl-X quit, asking once more when
there are unsaved changes, Ctrl-R run) and motions follow Emacs as
well as the arrow keys (Ctrl-A and Ctrl-E line start and end, Ctrl-B
and Ctrl-F back and forward, Ctrl-P and Ctrl-N previous and next
line, PageUp and PageDown); the keymap is a table in `xetal-keys`.
Tab and Shift-Tab move the focus between the ASCII, Rendered and
Output panes. The focused pane is marked strongly: a thick bright
border and a reversed title with a triangle, the other panes dim. The
cursor's cell is drawn reversed in the ASCII pane and, at the same
place in the text, in the Rendered pane, besides the terminal cursor,
so it shows whatever the terminal's cursor settings. Ctrl-T zooms:
the focused pane alone fills the screen, Tab and Shift-Tab switch
which of the three is shown, and Ctrl-T again returns to all three
(a pane hidden by zoom keeps its scroll). In the ASCII pane
the motions move the cursor and both panes scroll to follow it, up
and down and sideways; in the other two they scroll that pane by
hand, and typing returns to the ASCII pane. A missing file starts empty
and is created on the first save. The editor is a state machine
(keys in, screen and file out), tested on ratatui's TestBackend;
without a terminal `xetal edit` is `error[no-terminal]`.

### 8.1d Values for display (`xetal-grid`, `eval_events`)

`eval_events` runs a program for display: instead of printing each
top-level value it keeps it as a grid (element type, shape, formatted
items), in order with the text `p_rint!` printed around it. A grid
lays itself out as lines: a scalar or vector on one line with its type
and shape (`1 2 3  : Int 3`, a Char vector as a quoted string), a
matrix in a box with right-aligned columns, higher ranks as boxed
matrix slices labelled by their leading indices. The editor's Ctrl-R
output uses it; the stepping debugger will show intermediate values
the same way. `xetal-grid` knows nothing of the evaluator or the
terminal.

### 8.1e The live REPL (`xetal repl` on a terminal)

When both standard input and output are terminals, `xetal repl` reads
each line with a live line editor (`components/line`): the line is
drawn decorated and highlighted as it is typed and redrawn after every
key, the cursor at its rendered column; the editor's keys work
(Emacs motions, Home/End, Backspace/Delete), Up and Down walk the
history, Enter submits, Ctrl-C clears the line and Ctrl-D on an empty
line ends the session. Raw mode is held only while a line is typed.
Piped input keeps the plain line reader, so scripted sessions and
goldens are unchanged.

### 8.1c Notebook runs (`xetal run --echo`)

`--echo` shows a file as an APL session would: each statement
decorated and colored, indented six spaces where APL prompts for
input, and under it, flush left, exactly the output it produced. It
streams: the program runs once, the evaluator reports each top-level
statement just before running it (`xetal_eval::eval_items`), so the
statement is shown then and its output appears as it is written (a
long computation can print its progress). A file with warnings or
type errors, and the rest of a file after a runtime error, go a
statement at a time instead, as the REPL runs typed lines: a session
fed line by line (a statement with an open bracket takes the following
lines too), whose new output is streamed the same way (definitions
persist, `p_rint!` output is not repeated, and one seed keeps
`r_oll!` consistent across the replays). An error is shown in red
under its statement and the run continues; the command then
fails with `error[failed]`.

### 8.1f Planned: the stepping debugger (`xetal debug FILE`)

The trace saga adds a trace tree (NodeId, span, value, type, shape,
children) in evaluation order (E4). The debugger is a third app on the
pieces built for the editor and the REPL, with no new display code:

- Source: the rendered pane (`xetal-panes`), with the span of the node
  being evaluated marked the way an error span is marked today (the
  view model's raw spans map it into the decorated text).
- Values: `xetal-grid` lays out each node's value with its type and
  shape, as the editor's output pane does; a node's children (its
  function and arguments) are listed as grids beside it.
- Stepping: keys from the keymap table (`xetal-keys`) for step into,
  step over, step out and back, moving through the trace in the
  right-to-left order the explainer uses; Tab moves between the
  source, the values and a list of the steps.
- Running: the trace comes from an evaluator entry point like
  `eval_events`, which already shows values as grids in order.

The web playground (Saga 10) draws the same view model and grids in
the browser.

### 8.1g Emacs and literate documents

`docs/emacs/xetal-mode.el` colours X_eTaL source in the classes of
`xetal render --color` and, with `prettify-symbols-mode`, shows `:=`
`->` `_l` `_r` `!=` `<=` `>=` as the decorated glyphs while the file
keeps its ASCII. `docs/emacs/ob-xetal.el` runs Org Babel blocks with
`xetal run`: `:seed`, `:echo yes` (a notebook run) and `:untyped yes`
map to the flags, and `:session NAME` continues the earlier blocks of
that session in the buffer (`xetal run --context FILE`: the context
runs silently in a REPL session and only the block's output is
shown). The session is read from the buffer each time, so rerunning a
block or the buffer gives the same results. `docs/literate/tour.org`
is the language tour as a literate program, one statement per block
with its result recorded; `scripts/literate.sh` reruns it in a batch
Emacs and `--check` fails when a recorded result is out of date. The
gate runs the ERT tests (`just test-emacs`) and the check, skipped
where there is no Emacs. `scripts/literate-html.sh` exports the
documents to HTML under `pages/literate/` (with an index): each xetal
block is shown drawn, by `xetal render --html`, with its lines as
typed after it as comments, and the recorded results as they are. In
the `.org` files themselves the drawn form comes first too:
`scripts/literate-draw.py` (run by `scripts/literate.sh`, whose
`--check` fails when one is stale) puts `xetal render`'s drawing of
each xetal block just above it, as an example block marked by an Org
comment; the ASCII block below it is what runs. The HTML export drops
these copies, since it draws the blocks itself (decided with the user:
Unicode text in the `.org`, not LaTeX images).

A block that draws takes `:results file :file PATH` (PATH, in the
documents, is `../../images/literate-NAME.svg`): `ob-xetal` runs it
with `--draw` into a scratch directory, copies the last picture the
block showed (`[]S_HOW`) to PATH and returns nothing, so Org records a
link to the picture; a `:file` block that shows no picture is an
error. The block's printed text is not recorded. In a session, the
context runs with pictures muted, and a session replaying its accepted
source skips the pictures it showed before (`xetal_store::replay`), so
the picture is the block's own. The HTML export shows the picture as
an image, where a picture with frames animates by itself; Emacs shows
it as a still (`org-display-inline-images`). `scripts/literate.sh
--check` runs each document in a copy of the repository's layout and
fails when a picture it draws differs from the committed one.
`scripts/frames-to-webp.py PICTURE.svg` turns a picture's frames into
an animated WebP for places that do not play SVG animation (it needs
Python's cairosvg and Pillow).

### 8.1h Graphics (`[]G_RID`, `[]S_HOW`)

Programs compute what to draw as ordinary arrays (QD5, D38). `[]G_RID
a` is pure: it returns one self-contained SVG document as a Char
vector, built by `xetal-draw`, which knows shapes and cells and
nothing of the language. A scalar is one cell, a vector one row, a
matrix one grid of 24-pixel cells on paper, ruled by one path of thin
lines; a rank-3 array is frames along the leading axis, each shown in
turn for 0.4 s in an SMIL loop, so the file animates on its own in
any browser. Numbers that are all 0 or 1 draw their 1s as dark cells
(Bool and Int alike); other numbers are colored on a viridis scale
from the least to the greatest over every frame; characters are drawn
in their cells (spaces left empty). Above rank 3 is `error[rank]`, an
empty array `error[empty]`, anything but numbers or characters
`error[domain]`; the type is `Eq a => a -> Char`. A number grid of
more than 4096 cells a frame (64 by 64, `RASTER_CELLS`, confirmed by
the user) is drawn as one PNG image per
frame instead, a pixel per cell, scaled up by a whole number to at most
400 pixels on its longer side with `image-rendering="pixelated"`, and
without grid lines: a Mandelbrot frame is a few kilobytes rather than
tens of thousands of rectangles (the image is embedded as a data URI,
so the file stays self-contained). Characters are always drawn as
text.

`[]P_ATH xy : Num a => a -> Char` draws points given as 2 rows, x
over y (at least 2 points), joined in order: the box round every point
is scaled so its longer side is 400 pixels, with a 12-pixel margin and
y pointing up; a rank-3 array is frames of paths sharing one fit, so a
growing prefix of a curve animates its drawing. Any other shape is
`error[shape-mismatch]`. Turtle graphics is the standard library
`Turtle`, written in the language: a walk is a vector of turns, the
headings are its running sum, the positions running sums of cosines
and sines (`t:p_oints`, `t:w_alk`, `t:t_urn`, `t:p_olygon`).

`[]S_HOW svg : Char -> Char` is the one effect: it hands the picture
to the store the host installed (`xetal_store::Store::show`) and
returns it, like `p_rint!`, so a program binds it (`torus := []S_HOW
[]G_RID frames`). The command line writes numbered files (section
5 of architecture.md); the browser's Draw pane and a future desktop
app (a webview host of the same SVG) show them; a store with no place
for pictures says so (`error[io]`).

### 8.2 LaTeX (`xetal render --latex`)

One way and complete: the body of a math environment for KaTeX,
MathJax or pdflatex. Each token is braced so TeX adds no operator
spacing; each source space is `\ ` and each newline `\\`; comments are
dropped. Names: `\mathrm{\underline{r}ev}`, namespaces as
`{}^{\mathrm{u}}`, a function's mark braced as an ordinary symbol
(`{-}`), axes subscripting the whole name (`{\mathrm{\underline{o}}{-}}_{12}`,
never a bare mark), exponents unbraced so they attach to the token they
touch (`{\mathrm{x}}^{0.5}`, never an empty group); no space is
written before a line end or a dropped comment; a string is `\text`
spelled as in the source, TeX's specials escaped and a letter with a
combining underline as `\underline` (other Unicode passes through, for a
Unicode-aware engine); symbols
`\times \div \neq \leq \geq \wedge \vee`, binding `\leftarrow`, arrow
`\to`, separator `\diamond`, lazy marker `\sim`.

### 8.3 The live demo runs programs in a worker

The live demo runs each program in a Web Worker (`xetal-runner`, a
second wasm built by trunk), so the page never freezes and output is
not buffered: the worker posts each line as it is printed, each
picture as it is shown and each file as it is written (a plain-text,
length-framed protocol), and the page appends them as they arrive, with
a spinner while the run goes on and Run turned into Stop (which
terminates the worker). Workers have no local storage, so the page
sends the saved files with the program and saves back what it writes;
the worker says when it is ready, and only then is sent the program.
The worker runs a program a slice at a time (D50, `xetal_play::Interactive`,
200 000 transitions, then `setTimeout(0)` so it hears messages), so
Stop always works; a program reading a line (`[]R_EAD`) makes it post
Waiting, the output pane becomes a terminal (the line typed drawn with
a cursor, keys translated by `xetal-lineedit` in `xetal-typing`; the
pane takes the keyboard unless the ASCII pane is the current one), and
Enter echoes the line and sends it to the worker, which goes on from
that very call; Ctrl-C stops the run. No browser dialog is used. A
worker's failure ends the run with the error shown.

### 8.3a The notebook and stepping in the live demo

The Notebook button runs the program shown as `just show` shows it
(Run shows only the output; there is no switch, so Notebook and Step
both always show the notebook): each
statement (with the comments above it) drawn decorated and indented
six spaces, its output and pictures under it. `xetal-play`'s
`notebook_to` uses the evaluator's before-statement hook (the one the
CLI notebook uses) to hand over each statement's source just before it
runs; the worker posts it as a Source event, and the page groups the
output and pictures that follow under it. Step runs the program cut off
after its next statement (`statements` counts them; Step k runs the
first k, again from the start, as a REPL session replays), showing the
notebook with the statement just run marked; Reset (or Clear, Run or
Notebook) starts the steps again.

### 8.4 The live demo on a phone, and as an app

Below 720 pixels wide the toolbar wraps (Open on a line of its own,
buttons big enough to tap), the panes stack (ASCII, drawing, output),
each scrolling inside, and the page scrolls to the footer, with
nothing wider than the screen. The demo is a progressive web app: a
manifest (name, icons from `images/app-icon.svg` by
`scripts/app-icons.sh`, a maskable one included, standalone display)
and a service worker (`sw.js`) that answers from the network first and
keeps what it fetched, so a new deploy is picked up at once online and
the last version opened runs offline. `scripts/live-screenshot.sh`
also takes the phone screenshot (`scripts/phone-screenshot.mjs`,
phone emulation over the DevTools protocol).

## 9. Decisions register

All decisions below are made; the entries point to
`docs/lang-choices.md`, which records each decision and its reasons.
The pinning tests are written as the implementing saga reaches them
(`docs/plan.md`).

| ID | Question | Decision |
| -- | -------- | -------- |
| D1 | Binding token | `:=`; `=` is always equality (lang-choices S1) |
| D2 | Newline as statement separator | yes at top level and inside `{ }`; whitespace inside `( )` and `[ ]`; `;` for one line (S2, S3) |
| D3 | `{ _l }` and `{ }` lambdas | `_l` requires `_r`, else use named parameters; niladic is `{ @ -> ... }` (L1, L6) |
| D4 | Sections (`2 +`, `- 3`) | a symbol applied to one argument is an error (SC1) |
| D5 | Bool as Num | real Bool type; implicit Bool -> Int (1/0), Int -> Bool only from 1/0 (T1) |
| D6 | Index origin | 1, not configurable; `o_ffsets` for 0-based offsets (A5, B5) |
| D7 | Multi-axis rotate result shape | every combination, one leading axis per subscripted axis (A4); Life reduces with `'+ r_/_12` (R1) |
| D8 | Strict vs lazy; Y vs Z | strict by default, `~` lazy parameters (call-by-need); Y works, Z too (E1-E4) |
| D9 | Dyadic train forms | fork `x [F G H] y` is `(x F y) G (x H y)`; `[F G]` is atop (TR1-TR3) |
| D10| File extension | `.xtl`, shebang `#!/usr/bin/env xetal` (S5) |
| D11| Life one-liner rule | Conway's rule, section 6.2 |
| D12| Applying a function-valued argument | `_l_ x`, `(expr)_ x`; named function parameters `f_` (F5, L4) |
| D13| Decorated Unicode as input | not accepted as code; source is ASCII except strings and comments, which may hold any Unicode (I1, ST2, revised with the user) |
| D14| Namespaces | leading prefixes `u:` (program), `l:` (library), aliases via `u_se<` (N5, MC1-MC9); superscripts after a value are exponents (D-1 to D-6) |
| D15| Numeric typing | Haskell-style `Num` / `Truthy` classes with defaulting (T5) |
| D16| Checked evaluation | `eval` / `run` type-check first, `--untyped` skips; binding groups; number-type dictionary passing so literals follow their types (T6) |
| D17| Array types | rank-erased: a type names the element type; shape checked at run time (T7) |
| D18| Identity and tacks | `i_d`, `l_eft`, `r_ight` as separate built-ins (B9) |
| D19| Structural built-in details | control argument on the left, 1-origin `s_elect`, overtake fill from the items, empty errors, leading-axis `c_at` (B10) |
| D21| Reduce and scan | `r_/` is a right fold on the leading axis (J, BQN, APL); `s_\` gives prefix reductions, so its last item is the reduce; empty identities for `+ - * / & \| = !=` only, typed by elaboration; multi-axis forms (R1) come with axis subscripts (B6) |
| D22| Dyadic `e_ach` | one built-in `(a -> b) -> a -> b`; dyadic use by currying, a pending item-wise application zipped with the next argument (B6) |
| D23| Inner product | pairs the last axis of A with the first of B, as APL and J (B6) |
| D24| Random numbers | `r_oll!` is random by default; a seed only for tests that need it (B7) |
| D25| Sorting | `s_ort` / `g_rade` stable ascending on `Ord` types over major cells; `g_rade` is 1-origin (B7) |
| D26| Rotate direction and amounts | APL direction (positive toward the front); a list of amounts always means every combination, one leading axis per rotated axis (A2, A4) |
| D27| Axis subscript on a dyadic function | moves axis k of the right (data) argument only; the axis moves back when the result keeps its rank, stays consumed when it loses one, else `error[axis]` (A6) |
| D29| Libraries | `Name` resolves to `Name.xtl` in the importing file's directory, then `userlibs/`, then `XETAL_PATH`, then the standard libraries built into `xetal`; libraries hold definitions only; unprefixed functions in a library are private; imports display as superscript alias and superscript equals before `u_se<` (MC4, MC9) |
| D30| System names (quads) | `[]NAME` is one token in the system namespace, shown with the quad glyph; values `[]A` `[]D` `[]AV` `[]TS` `[]IO` (always 1), functions `[]D_L` `[]U_CS` `[]R_EAD` `[]V_ALUE` (lang-choices 13a) |
| D28| Function power | `f_^3 x` applies f three times (superscript on a function, D-7), lowered to `3 'f_ p_ower` waiting for its argument; computed counts use `n 'f_ p_ower x`, `p_ower : (a -> a) -> Int -> a -> a`; counts are whole numbers, `^-1` reserved; libraries (Saga 8) come before the combinators (Saga 9), which are written directly as a library |
| D31| Combinators library | every bird Smullyan names that type-checks, spelled by letter (`B_1`, `C_s`, `E_h` for the variants), types pinned; Y by recursion; the self-applying birds in an untyped demo; `Maybe` as a second library; `xetal type` of a library lists its exports (CB1-CB4, `docs/birds.md`) |
| D32| Files and numbers as text | quad functions `[]N_PUT` / `[]N_GET` (text files) and `[]R_EAD` (a typed line); `f_ormat` and `n_umbers` for numbers as text; lexed as a name in the system namespace `[]`, drawn with the quad, a built-in (QD1-QD4) |
| D33| Decimal exponents | raised like whole ones, the point drawn as a middle dot (Unicode has no superscript full stop); the inverse accepts the dot only inside a raised exponent (I2) |
| D34| Match | `a m_atch b` is 1 when both sides have the same shape and equal items (APL's match), typed like `=`; a word, drawn with its m underlined (B7) |
| D35| Replicate | `r_eplicate`, counts on the left over major cells, a scalar count extends, negative is `error[domain]` (B11); classics lane. As built: `Truthy a => a -> b -> b`, so the counts are Ints or a Bool mask, never Floats; a vector of counts of another length than the cells is `error[length-mismatch]` and a matrix of counts `error[rank]`; a scalar right argument is one cell, as for `t_ake` (B10); an axis subscript moves the axis to the front (A6), so `r_eplicate_2` repeats columns |
| D36| Encode and decode | `e_ncode` / `d_ecode`, radix on the left, APL shapes (B12); classics lane. As built: both `Int -> Int -> Int`, in their own component (`radix`); `e_ncode`'s result has the radix's shape then the right argument's (each a scalar or a vector, else `error[rank]`), a radix of 0 takes all that is left, and a digit is APL's residue (it has the radix's sign, so `2 2 2 e_ncode -1` is `1 1 1`); `d_ecode` takes a vector of digits or a matrix (one number per column), a radix vector of another length is `error[length-mismatch]`, and past the largest Int is `error[integer-overflow]`; with an axis subscript (A6), `d_ecode_2` decodes rows |
| D37| Nested arrays | static depth: an enclosed item has type `Box a`; `e_nclose` / `d_isclose`; printed boxed (A7); classics lane. Settled with the user (B14): string-literal strands are nested; `e_ach` stays scalar-only and `m_ap` boxes each result; `p_artition` takes APL2's keys; `Box a` unifies only with `Box a` and prints `Box Char` |
| D38| Graphics | pure SVG builders `[]G_RID` and `[]P_ATH` (points as 2 rows, x over y) returning Char vectors, frames along the leading axis animated with SMIL/CSS, one effect `[]S_HOW` handed to the host (CLI files, web Draw pane, future desktop webview); a component `xetal-draw` with no syntax knowledge (QD5); classics lane |
| D39| Trigonometry | `s_in` `c_os` `a_tan` in radians to Float, `p_i @` (B13); classics lane |
| D40| Catenate along an axis | `c_at_k` moves axis k of both arguments (its own axis rule, like rotate); other axes must match; a scalar or a one-rank-lower argument is one cell; one axis only (B15); asked for by the swimming-ducks demo |
| D41| Nested arrays in the core | `Type::Box`, unifying only with a box, in `Eq` (passing Eq to its item) and no other class; `Value::Boxed`; a strand of string literals lowers to an array of `e_nclose` applications; `e_nclose` / `d_isclose` (one box); boxes compare by content; nested values print as APL2 DISPLAY through `xetal_grid::display`, which a `d_isplay` built-in can reuse (B16) |
| D42| Partition and map | `p_artition : Truthy a => a -> b -> Box b` over major cells (a new piece where the key increases, 0 drops; keys checked like replicate's counts: a scalar extends, negative is `error[domain]`, another length `error[length-mismatch]`); `m_ap : (a -> b) -> a -> Box b` calls f on each item and boxes each result, keeping the shape (B14) |
| D43| ASCII pictures | `--ascii` (a global CLI switch, like `--draw`) sets `xetal_grid::set_ascii`, and `display` maps each box character to one ASCII character (`to_ascii`), so widths are unchanged; `scripts/reference.py` runs its examples with it, so the ASCII-only reference shows nested values (B16) |
| D44| Display and boxed printing | `d_isplay : a -> Char` gives any value as APL2's DISPLAY draws it (B16's frames and marks), flat arrays framed too, as a character matrix (a simple scalar is itself); `xetal --box` (a global switch, `xetal_grid::set_boxed`) prints every array result that way, and the live demo's Boxed toggle does the same (the request carries it to the worker); `p_rint!` prints through the same rule (user request) |
| D45| Trains are their desugaring, tested | property tests (xetal-eval tests/eval/trains_props.rs) check that a fork, an atop, a dyadic fork, a long train, a named train and a fork over text each print what their written-out forms print, over random arguments and pools of built-ins (TR1-TR4); trains lane |
| D46| Errors inside a train | each application a train expands to is spanned by its element, not the whole train, so the checker and the evaluator point at the element at fault, with their usual messages (spec/eval/reject-train-*.case); trains lane |
| D47| Train error notes | `xetal-explain` builds, per train element, a note `in the train [..], F is applied as FORM, where x is the train's argument` and an arity hint (`F takes one argument, but here it is given two`; `G takes two arguments, so G x is a function waiting for the other`), from the source spelling and the catalog arity of a built-in that is not shadowed by a local name; `Program::annotate` adds the notes of an error's exact span, called by `infer_program` and the evaluator's run (user request: better diagnostics for train errors); trains lane |
| D48| Transpose | `o_\` (`a -> a`) reverses the axes and `t_ranspose` (`Int -> a -> a`) permutes them, in a new crate `xetal-transpose` (axes component): one kernel `permute` (axis i to position to[i]) with `reverse_axes` and `swap_axes` on it, and `permutation` checking the 1-origin list (B17); `o_\_jk` is transpose's own axis rule in `xetal-axes` (`transpose_on`, two axes exactly, through the same `checked` as rotate; the lexer already refuses a repeated digit); transposing twice and permuting by the reversed identity are property-tested against it; transpose lane |
| D49| Exponent literals | `1.5e-7`, `6.02e23`, `2E3` lex as one Float token (S8): `e`/`E` touching the digits, an optional `-`, digits (`literal.rs` `exponent`); `1.5e`, `1.5e-` and `1.5e+3` are `bad-number`, `1e2.5` keeps the follow rule, and a Float that overflows (`1e400`) is `number-out-of-range`. This also mends the formatter's round trip: the canonical form already printed `0.0000001` as `1e-7`, which did not lex before. Spec cases lex/exponent-literals, the rejections, eval/exponent-literals, eval/exponent-canonical. |
| D50| A steppable evaluator (decided, Saga 25) | the evaluator becomes an explicit machine whose state is data (the expression in hand, pending applications, environments, the higher-order built-ins' progress), run in slices with a budget, as web-sw-tos steps its emulated CPU; `[]R_EAD` with no line yet leaves the run waiting, and a typed line resumes it. One evaluator for the CLI and the page. Chosen with the user over re-running the program per line and over a worker sleeping on Atomics.wait (cross-origin isolation, a non-Rust service worker). The core is in `components/step` (`xetal-step`: `Machine::run(budget)` returns Running or Done; frames `Kont`, control `Control`); each closure call keeps a frame, so a runaway recursion is still `stack-overflow` (at two million frames, on the heap). The higher-order built-ins are kernels (`xetal-kernel`): a kernel asks for one call of its operand at a time and is resumed with the result, the call running above it as ordinary frames, so a run stops inside `e_ach`, reduce, `p_ower` and the rest as anywhere else; built with combinators (`apply`, `then`, `all`, `fold`) that keep every call's order, and `Caller` is gone (xetal-step tests/inside.rs). Waiting for input: `Machine::waiting_for_input` gives the run a queue of typed lines; `[]R_EAD` takes the next, or, when none has been typed, leaves its call pending (the argument handed back to a `PrimArg` frame) and the run stops as `Status::Waiting` until `feed` adds a line, every loop, recursion and binding intact (xetal-step tests/input.rs: a text-adventure loop fed three commands prints what the CLI prints with them on standard input). Without a queue (the CLI) `[]R_EAD` reads standard input as before. Tested: xetal-eval tests/eval/slices.rs (any slice size prints what one run prints). |
| D51| The terminal's grid and line editor (Saga 25) | `components/console`, on web-sw-tos's model and with no terminal crates: `xetal-screen` keeps what a program writes as lines of `Cell`s (a character and a `Style`: foreground, background, bold; a small named palette), complete lines in a bounded scrollback and the line still being written, and gives the last rows that fit, wrapped at the width and padded; `xetal-lineedit` translates browser key names (`KeyboardEvent.key`, Ctrl held) to keys in a pure function (Ctrl letters are the Emacs and shell editing commands; Meta and Alt stay with the browser) and edits a line (cursor, Backspace, Delete, Home, End, Ctrl-U, history on Up and Down, Enter submits, Ctrl-C interrupts). Tests: console tests/screen.rs, tests/edit.rs. |
| D52| Typed screen control (QD6) | the checker gains nominal built-in types (`Type::Named`, `ENUMS`: Color and Key, admitted to `Eq`), the evaluator `Value::Tag(type, index)` printed by name (`xetal-value` `tags`); `lib/Terminal.xtl` names the values with the library constructors `[]C_OLOR` and `[]K_NAMED`; `xetal-prim` `screen` builds the typed text (`[]F_G`, `[]B_G`, `[]B_OLD`, `[]A_T`, `[]C_LS`: ANSI with matching resets so builders nest); `xetal-tty` (components/console) is the terminal a program runs on (`[]E_RR`, `[]K_EY`, `[]T_E`): `Plain` by default, `xetal_line::Terminal` at the CLI (a raw-mode key), `Forward` in the worker; the machine waits for `[]K_EY` through its queue (`Status::WaitingKey`), the worker posts WaitingKey and the page sends the key's name; the live demo draws output holding sequences as a 24 by 80 `xetal_screen::Grid`. Tests: xetal-prim tests/screen.rs, xetal-step tests/input.rs, console tests (grid, tty, edit), spec eval/terminal-types and two rejections; checked in headless Chrome. |
| D20| Comparing characters | `=` / `!=` on any scalar type (`Eq`), orderings on numbers and Char (`Ord`), table-driven classes (T8) |
