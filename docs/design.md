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

Conway's rule, with N the count of the 8 neighbors and c the cell:
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
center died. Verified with sw-apl: on the 5x5 blinker the old rule
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
dyadic call a lazy parameter is honored when the function is a
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

System macros (MC14-MC20): `lib/System.xtlm`, built in, is loaded as
a macro library before every file, its `s:` macros (`i_f<`, `u_nless<`,
`e_ach<`, written in X_eTaL) called unprefixed. Each file's macro calls
other than `u_se<` are expanded by `xetal-expand` (components/expand):
each call is found in the file's tokens with its two arguments (a
string, or `@` for none), classified as a statement of its own or part
of an expression (MC12), and run through a `Macros` table (the system
macros and the file's macro libraries alike); the text it gives
replaces the call, parenthesized inside an expression, and is expanded
again to a depth of 32. The result is an `xetal_mapped::Mapped`: the
expanded text with a map from each byte back to where it was written
(runs of the macro's text found in an argument map into its string,
escapes to their backslash; the macro's own text to the whole call).
The hooks (MC20) are quad built-ins in `xetal-system` reading the call
being expanded, which the runner sets (`expanding`); `[]R_EJECT`
fails the call with a note naming the side to report at.
The macro phase then reads the expanded text; `Sources` keeps the map
(`add_expanded`), so every later diagnostic, from the imports to the
evaluator, is located in the text as written. `xetal expand` prints
the expanded text, and a spec case's `== EXPAND` section pins it.

Macro libraries (MC10-MC12, MC23): libraries are found through
`xetal-lookup` (components/lookup: `Libraries::find_both` gives the
`.xtl` and `.xtlm` of the first directory holding either, on disk, in
the store, or built in from `lib/`). A file's imports are read before
its macros are expanded; each macro library is loaded once by a
`Loader` of its own (its `m:` exports renamed to a hidden namespace,
the names rules checking that every export ends in `<` and no other
macro is defined) and kept as a `MacroLib` (its program and exports).
`xetal-expand` asks a `Macros` table for each `alias:n_ame<` call; the
table appends the call to the macro library's program and hands it to
`Libraries::run_macro`, which `xetal-program` provides (`Running`
wraps every loader's libraries: lower, check that the call gives
`Char`, evaluate, take what it prints), since the macro phase cannot
depend on the evaluator. The text is glued in at the call and expanded
again, to the depth limit.

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
matrix slices labeled by their leading indices. The editor's Ctrl-R
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

`docs/emacs/xetal-mode.el` colors X_eTaL source in the classes of
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
| D53| A bound condition (T1, T5) | a top-level value whose type is a condition variable (`Truthy`, not yet Bool or Int) is generalized rather than defaulted to Bool (`closing.rs` `condition`), so each use is an Int in arithmetic and a Bool in a guard, as inline; its item type is shown as Bool; the value is the same at run time (the evaluator already took a Bool in arithmetic). Spec: eval/bound-condition, eval/reject-bound-condition-float. |
| D54| A `#!` file is a program | `is_library` (xetal-program) answers no for a file starting with `#!` (S5), whatever it names, so `xetal run` reports a program's `l:` names as MC8 row 9 instead of taking the file for a library (X_eTaL-games' ask). Golden run-program-with-library-names. |
| D55| Built-in operands called at once (Saga 30) | a first-order built-in operand (`'*`, `'+`, `'r_ight`, partly applied or not) runs no user code and cannot pause, so `t_able`, `i_nner`, `e_ach` and reduce call it directly, in the kernel's order, instead of asking the machine for each call: `xetal-kernel` `Direct` (whether an operand is called at once, and the call), implemented by `xetal-prim` `Now` (not a higher-order built-in, not `[]R_EAD` or `[]K_EY`), passed through `xetal_hof::call`. A lambda operand still runs call by call (D50), so a run can stop inside it. Per operand call: a primitive `t_able` 0 transitions and 0 allocations (were 1.05 and 6.5), reduce 0 and 1 (2.1, 7.1), `i_nner` 0 and 0.25 (3.9, 16.4); bench/inner.xtl 8.3 s to 0.86 s. Test: xetal-step tests/cost.rs. |
| D56| Lean kernels (Saga 30) | a higher-order built-in whose operand is a function of the user's is one kernel over its own state (`xetal-kernel` `lean`: a state, the next `Application`, one call or two curried, and what each result does), not nested combinators: `e_ach`, `m_ap`, the dyadic each, `t_able` (`f a` once per row, then the row on each right item), `i_nner` (g on each pair, then `f r_/` per cell), reduce (the right fold) and scan (a running scan on the previous item when f is associative, else each prefix folded leftwards); every call in the same order as before (xetal-map and xetal-hof tests/order.rs). Nothing is allocated per element by the kernel: a lambda operand of `t_able` or `e_ach` went from 11.6 and 11.1 allocations per call to 7.2 and 7.1, the rest being the closure call itself (xetal-step tests/cost.rs). `p_ower`, `c_ompose`, `s_wap` and the axis forms keep the combinators. |
| D60| System macros and the expansion map | `components/expand`: `xetal-mapped` (a text with a map back to the written text: slices, glue mapped to a call, unescaping) and `xetal-expand` (calls found and classified, the three system macros, the depth limit); `xetal-macro` expands each file before reading its imports, and `xetal-sources` keeps each file's map (`add_expanded`, `Location::to`), so errors in expanded code are located as written, one file or several (MC14-MC17, proposed). Tests: expand tests (system.rs, map.rs, mapped.rs), spec/macros. |
| D61| Seeing an expansion | the subcommand `xetal expand FILE` (or `-e`), not a `--expand` flag, as the CLI's other views (`fmt`, `core`) are subcommands; the spec harness's `== EXPAND` section runs it. The spelling can be switched if the user prefers (ask X2). |
| D62| Finding libraries and macro libraries | `components/lookup` (`xetal-lookup`: `Found`, `Libraries` with `find_both` and `run_macro`, `FsLibraries`, `StoreLibraries`, moved out of `xetal-macro`, which re-exports them); `xetal-libs` embeds `lib/*.xtlm` as `MACROS` beside `LIBRARIES`; the not-found message names both files (MC11). Tests: lookup tests/pairs.rs (each tier, explicit paths, the store). |
| D63| Running a macro | a macro library is loaded by its own `Loader` into a `MacroLib`; a call appends `"left" LX:n_ame< "right"` as a file of its own and `xetal-program`'s `Running` lowers, checks (the call's type must be `Char`) and evaluates it (seed 0), its printed text being the macro's result; `xetal type`/`run` on a `.xtlm` lists its macros (`is_library` sees `m:n_ame< :=`). Tests: xetal-program tests/macros.rs, xetal-expand tests/user.rs (the depth limit by a macro calling itself), spec/macros/user-*.case, goldens macros-user-*. (MC23, proposed.) |
| D64| Long namespace prefixes | the lexer's `namespace` reads `[a-z][a-z0-9]*:` (as written) or `[A-Z]+:` (the macro phase's hidden namespaces, which the combined program holds), anything else `bad-namespace`; `valid_alias` applies the written rule; a hidden prefix written in a file is `hidden-namespace` (names phase); render, LaTeX and the view already draw a prefix raised only when every character has a superscript form, and a digit has none, so `b2:` is drawn as written and inverse rendering stays unambiguous; xetal-mode colors `[a-z][a-z0-9]*:` prefixes (MC13). Tests: lex accept/reject, xetal-macro tests/imports.rs, spec lex/long-prefixes, integration/long-aliases and three rejections, goldens long-prefixes-latex, long-prefixes-color, an ERT test. |
| D65| System macros in X_eTaL | `lib/System.xtlm` (built in as a standard macro library, key `std:System.xtlm`, its own letter `s`) is loaded by `Loader::load_system` before every file and every macro library; `Table` runs unprefixed calls against it and aliased ones against the file's macro libraries; the Rust generators for `i_f<`, `u_nless<`, `e_ach<` are gone. Their string work is X_eTaL (trim, words by `p_artition`, statement separators by scans over brackets and quotes, `$w` replaced through `m_ap`). Hooks `[]R_EJECT` and `[]S_TATEMENT` in the catalog and `xetal-system` (`hooks.rs`, a call context set by the runner, one call at a time); a rejection carries a `macro-place` note. `Running` checks a call's sides against the macro's parameter types when the call does not type (MC22). The example macro library's `m:u_nless<` became `m:w_hen<` (a macro library may not define a system macro). Tests: xetal-expand tests (user.rs, map.rs with stand-in macros), xetal-program tests/macros.rs (system redefinition, sides, rejection), spec/macros (unchanged expansions; s: as an alias; hook outside; @ for text). (MC18-MC22, MC24.) |
| D66| The compiler-only system macros | `l_ine<`, `f_ile<`, `i_nclude<`, `c_fg<`, `e_rror<` in System.xtlm over the hooks `[]L_INE`, `[]F_ILE`, `[]I_NCLUDE`, `[]C_FG` (catalog; `xetal-system` facts.rs: the platform by `cfg!(target_arch = "wasm32")`, flags from `xetal --cfg NAME`, files through `xetal-store` relative to the call's file) and `[]R_EJECT`; the call's file and line reach the hooks through `MacroCall::at` (the call's written offset), `MacroRun::file`/`row` and `Expanding`; a string result is quoted by System.xtlm's own `q_uote`. `u_se<` stays in Rust, with a comment in System.xtlm modeling it (MC21). Tests: spec/macros line, file, cfg, error and rejections, xetal-program tests (a user macro refusing with `e_rror<`), goldens macros-include, macros-cfg, the reference's hook entries. (MC20, MC21, MC25.) |
| D67| d_bg< | System.xtlm's `s:d_bg<` writes `{ @ -> dbgValue := (expr); []E_RR "[file:line] expr = " c_at f_ormat dbgValue; dbgValue } @`, its place from `[]F_ILE` and `[]L_INE`. Tests: spec macros/dbg (expansion, value), golden macros-dbg (standard error). (MC26.) |
| D68| a_ssert< | System.xtlm's `s:a_ssert<` writes `{ @ -> (cond) ? @; []E_RR "assertion failed: ..."; @ } @`; its message parameter is polymorphic (`f_ormat note`), so `Running`'s side check skips type-variable parameters. Tests: spec macros/assert, golden macros-assert (standard error, the program goes on). (MC27.) |
| D69| f_ormat< | System.xtlm's `p_ieces` walks the text recursively to the next brace, quoting literal runs (`l_it`) and writing `(f_ormat (expr))` per hole, joined by `c_at` (`c_j`); errors by `[]R_EJECT` at the right argument. Tests: spec macros/format, three rejections, a left-side rejection, a type error located in a hole. (MC28.) |
| D57| p_anic< | `[]P_ANIC` in the catalog (`Char -> a`) and `xetal-system` calls.rs (an error with code `panic`); System.xtlm's `s:p_anic<` writes `[]P_ANIC (pieces)` with `f_ormat<`'s pieces. Tests: spec macros/panic (a Char position), panic-int (an Int position, the message formatted), golden macros-panic, the reference entry. (MC29.) |
| D58| Performance regression gates (Saga 30, research4) | `scripts/bench-check.sh` (`just bench-check`, `just bench-bless`): every program in `bench/` (elementwise Int and Float, reduce, scan, each with a lambda, table with a primitive and with a lambda, inner, a small matrix product, rotate, transpose, Life) timed with the release build, best of 5, against a per-machine baseline `bench/baseline/HOST.tsv`; more than 15% and 15 ms slower fails; a new baseline is recorded only with the user's approval for any slowdown; part of the release checklist, not the gate (timings depend on the machine and its load), where the deterministic cost guard (xetal-step tests/cost.rs) stays. |
| D59| Combinators.xtlm | `lib/Combinators.xtlm` (built in with the other `lib/*.xtlm`): `m:Y_<` finds the parameters before `->`, drops the first (and its `~`), and replaces it as a whole word (`w_ord`: not inside a longer name) by the name on the left; `m:B_<` writes `{ f g _r }`. Timing, fib 25 in the release build (best of three): through `c:Y_` 0.39 s, through `c:Y_<` 0.43 s: tying the knot at expansion buys no speed here, since `c:Y_`'s lazy self parameter is evaluated once per call either way; its gain is a plain named function (no lazy parameter, readable expansion). Tests: spec macros/combinators-y, combinators-b, reject-combinators-y-shape; the fibonacci demo's fifth way (golden run-classics-fibonacci rebased on purpose). (CB5.) |
| D76| The user's macro decisions of 2026-10-04 | MC14-MC17, MC23, MC25, `c_fg<` as an Int and the `[]R_EJECT` shape recorded as decided. L6 extended: `xetal-ast` `Param::name` is an `Option<Target>`, none for `@` among named parameters, lowered to Core's `Param::Unit` (so it types Unit); the canonical and surface printers write `@`. System.xtlm uses `@` for every side that takes nothing (its `n_othing` helper gone). `d_bg<` writes `{ v -> ...; v } (expr)` (hygienic; `dbgValue` gone). `t_odo<` writes `@ p_anic< "not yet implemented: ..."`. Tests: spec syntax/unit-parameter, unit-parameter-normal, reject-unit-parameter-value, reject-lazy-unit-parameter; macros/dbg-hygienic, todo; golden macros-dbg and system-library-types rebased on purpose. |
| D77| Hygiene | `xetal-expand` `hygiene.rs`: after `copied` maps a macro's text, each `{` the macro wrote whose body holds some of the call's code (copied, not inside a string) has its plain binders (parameters before `->`, top-level `name :=`) renamed, with every macro-written occurrence inside it, to `gN:name` (`Cell` counter per file); `binds.rs` reads `## binds:` lines in the doc block above a definition (`MacroLib::binds`, `Macros::binds`). `copied` now finds whole arguments first, then runs of at least 3 bytes of arguments not found whole. `is_fresh` (xetal-token) marks the namespace; Core lowers `g1:` names as locals (parameters, bindings, references); the names phase leaves them alone; a program writing one is `reserved-namespace`, an alias `"g1:"` `reserved-alias`. Example macros `m:t_wice<` (hygiene) and `m:w_ith<` (an anaphor) in lib/Macros.xtlm. Tests: xetal-expand tests/hygiene.rs; spec macros/hygiene-capture, hygiene-anaphor, hygiene-numbering, reject-fresh-namespace, reject-fresh-alias; every earlier expansion unchanged. (MC30.) |
| D78| Macro libraries of your own, in the live demo too | `userlibs/Repeat.xtlm` (`m:t_imes<`: statements written as many times as asked, a loop unrolled when the program is expanded) and the demo `demos/user-macros.xtl`; the live demo seeds `Repeat.xtlm` among your files (store), and Open lists the standard macro libraries (`macros:Name`, labeled `Name.xtlm`) beside the libraries; saved `.xtlm` files are your files like any other, found by `u_se<` through `StoreLibraries` (MC11). Help says so. Tests: xetal-web tests (Open offers the macro libraries; the demo runs from the store), golden run-user-macros (the run and its expansion). |
| D79| Signature lines in System.xtlm | `xetal-sigs` (components/lookup): when a file is the system macro library (letter `s`), each line `s:n_ame< :: Type` is read (the name must be `s:` and end in `<`; the type through `xetal_prim_types::read` plus a check that every word is a known type, class or type variable, else `bad-signature`) and blanked to spaces, keeping offsets, before the file is lexed; its `##` block above is its doc. `xetal_macro::system_signatures()` (re-exported) gives the built-in System.xtlm's as `Signature { name: "u_se<", ty: "Char -> Char -> Unit", doc: Vec<String> (the ## lines without "## ", including ">> " example lines), span }` for the doc model (kind: built-in macro). `is_library` ignores `::` lines. Anywhere else `::` is the lexer's `unexpected-char`, as before. Tests: xetal-macro tests/signatures.rs, xetal-program tests/macros.rs (a program and a .xtlm refuse it), spec lex/reject-signature-line. (MC21, T4.) |
| D20| Comparing characters | `=` / `!=` on any scalar type (`Eq`), orderings on numbers and Char (`Ord`), table-driven classes (T8) |
| D70| The status table | `docs/status.md` is generated by `scripts/status.py` from the built-in catalog, `docs/lang-choices.md` (a row saying "not yet implemented" is planned, "partly implemented" partial), the spec cases (a pending case is planned; each built-in counts the cases that use it and the reference examples it runs, and one nothing runs is partial) and `lib/`; `--check` in the gate keeps it current, `just status` regenerates it, and the README's Status points at it (research3's "what works today"); launch-docs lane |
| D71| The asks ledger | `docs/asks.md` is generated by `scripts/asks.py` from `docs/asks.toml` (every ask in the sibling repositories' `docs/xetal-asks.md`, its state here: landed with the commit, partly landed, planned with the saga, declined with the reason, new); each ask with a `repro` program is run against the release build and passes when a whole output line equals its `works` text with no error (or the output holds the `error[code]` it names); a landed ask whose repro fails fails the gate's `--check`, and a planned, declined or new ask whose repro passes is listed under "To look at"; `--siblings DIR` compares each repository's count of filed asks with the ledger's; launch-docs lane |
| D72| The doc model (Saga 32 step 1) | `components/doc`: `xetal-doccom` reads S9 doc comments from the written text (the lexer drops comments): the `##` block directly above a definition, the file's block (at the top, after any `#!`, followed by a blank line), the `###` section a line falls under, and `## >> code` examples whose output is the following `##` lines up to a bare `##`, the next `## >>` or the block's end. `xetal-doc` builds the model from the program's load (`xetal-program`): one entry per file (program, library, macro library, system macros), each item with its written name, kind (a macro ends in `<`, a function has an underlined letter, else a value), public (an `l:` export, or any item of a program), inferred type, line, section, doc, examples, source lines and the items it uses (Core globals and top-level names not shadowed, resolved to the defining file and written name, sorted). `.xtlm` libraries load on their own; `System.xtlm` is included only when a file calls one of its public macros, so a program's docs do not carry it otherwise. `xetal doc --json FILE` prints it (hand-written JSON: no serde in the workspaces); `--json` is required until the site (`--out`) lands. Expansions come with step doc-macros. Tests: doc tests/comments.rs, tests/model.rs; goldens doc-json-app, doc-json-library, doc-needs-json. |
| D80| The doc site (Saga 32 step doc-site) | `xetal doc --out DIR FILE` writes a flat static site, HTML and CSS written by Rust (no build tooling; the one script switches light and dark, following the system until the reader picks, remembered in local storage): `index.html` (each file with its doc's first paragraph, every item with kind and type), `builtins.html` (the catalog's implemented built-ins: name drawn, type, rules), and per documented file `PAGE.html` (kind and name, file doc, imports linked to their files, the items grouped by `###` section with a table of contents in the side bar; each item with its name drawn decorated, type, kind, privacy, a link to its line, doc prose (paragraphs; `code` drawn; an indented paragraph a code block), `## >>` examples as a session transcript (input indented six spaces), its source drawn decorated, and where it is used) and `PAGE.src.html` (the file drawn decorated, each line numbered and anchored `#L3`). PAGE is the file's name without leading `/`, `./`, `../`, `/` as `-` (`std/System.xtlm` is `std-System.xtlm`; `-e` is `program`); an item's anchor is its written name with `:` as `.` and other characters outside letters, digits and `_` as `-` and hex (`s:i_f<` is `s.i_f-3c`). Links (`xetal-doclink`) come from the written text, so the reader sees what was written: a name resolves to an item of the same file, else (prefixed) to the export of the file its alias imports (`g:s_hout` to `l:s_hout`), else (an unprefixed macro) to a System.xtlm macro, else to a catalog built-in (unprefixed or a quad); lambda parameters and names bound inside a lambda body hide items of the same name, and a top-level `name :=` defines rather than uses; an import's library string links to the library's page. "Used in" lists every link to an item across the documented sources, by the item holding it, else by file and line. The model gains each file's imports (alias, spec, the files found; in the JSON) and its written text (not in the JSON); `--json` or `--out` is required. Names inside macro arguments (strings) are linked with the expansions (step doc-macros). Tests: doc tests/links.rs, tests/html.rs, tests/site.rs, tests/model.rs; goldens doc-site-files, doc-site-links (doc-json-app, doc-json-library and doc-needs-json rebased on purpose for the imports and the new option). |
| D73| Decode on any numbers (B18) | `d_ecode` is `Num a => a -> a -> a` in the catalog; `xetal-radix` folds Horner's rule over a `Number` (`i64` with overflow checked, `f64` with `mul_add`), and `call` picks Floats when either argument holds one (the checker has made the two one type). Shapes, the scalar radix and the length check are shared. Number literals take either type (T5), so `2.0 d_ecode 3 -2 1` is 9.0, while an Int variable with Float digits is a type error. `e_ncode` stays Int. Spec: eval/decode-any, eval/reject-decode-mixed, eval/reject-encode-float-radix; radix tests/radix.rs; ask X9 (decode). |
| D74| Doc tests (S10, Saga 32 step doc-test) | `components/doc` `xetal-doctest`: the examples of the file given (its own doc block, then each item's), each block one `xetal_repl::Session` (origin `-e`, seed 1): a program file's text is fed first, silently (`xetal_store::muted`), a library's nothing, then each example in turn, so later examples see earlier ones' definitions. A run passes when its output lines equal the shown lines that are not `error[...]`, and each shown `error[code]` begins an error line it printed (with none shown, none may be printed); warnings and `d_bg<` traces are not compared. The report prints like `cargo test` (one line per example, each failure's expected and actual, the counts); a failure exits 1 with `error[doc-test-failed]`. `xetal doc --test FILE` (`--json` and `--test` are one required choice); the gate runs it over `lib/*.xtl` and `lib/*.xtlm`. Documenting `lib/System.xtlm` itself no longer lists it twice. Tests: doc tests/run.rs, tests/model.rs; goldens doc-test-library, doc-test-program, doc-test-failures, doc-needs-json. |
| D75| System values and character codes (QD2, QD3, QD7) | a quad name without an underline lexes as a value token in the system namespace (as before) and now lowers to a niladic built-in, `Kind::Prim("[]A")`, instead of a global; the catalog takes arity 0 (`["[]A", 0, "Char"]`), the checker gives it the catalog type, and the evaluator calls a niladic built-in where it is used (`xetal-step` `leaf`), so a system value is read each time. New crate `components/system` `xetal-quad` (`[]A`, `[]D`, `[]AV` the 128 ASCII characters, `[]IO` always 1, `[]U_CS` Char to Int and `[]U_CHAR` Int to Char, 0 to 127 or `error[domain]`), called from `xetal-prim` after `xetal-system`. The view colors a quad value as a built-in. Spec: eval/quad-values and the rejections reject-quad-unknown-value, reject-quad-value-applied, reject-u-cs-number, reject-u-char-range; quad tests/quad.rs; view test. |
| D81| Macro expansions in the docs (Saga 32 step doc-macros) | `components/docexpand` (`xetal-docexpand`): a documented file is expanded as the loader expands it, through `xetal-program`'s `Running` (now public) wrapped by a recorder that keeps each call's text by its written form (statement or not, sides quoted, the macro as written); then the calls written in the file (`xetal-expand`'s `calls`, now public) are matched to what they gave, the text parenthesized inside an expression as the macro phase substitutes it, and the calls in that text expanded in turn to the depth limit. The model gains each file's `expansions` (line, macro, call, text, nested; in the JSON) and each item's `binds` (a `## binds: it that` line in a macro's doc comment, the names a macro binds for the text it is given on purpose; a field, not prose). On the site, under each line holding a call (source page) and under an item's source, a collapsed `<details>` shows the macro (linked to its definition in its `.xtlm` or System.xtlm) and its text drawn decorated and linked, nested expansions inside it; fresh names the macro phase gives appear as `xetal expand` shows them. An example that imports a library (`## >> "g:" u_se< "Greet"`, S10) makes its names linked in the rest of that doc comment's examples and in its prose (the doc tests' session rule). 'Used in' counts a use inside an expansion (a name in a macro's argument) at the line of the call. The source page is a `div` of lines (white space kept) so the blocks can sit between them. Tests: docexpand tests/expansions.rs, expand tests/calls.rs, doc tests (model, links, site, comments); goldens doc-site-expansions, doc-site-links restored. |
| D82| Searching the docs (Saga 32 step doc-search) | `components/docsearch` (`xetal-docsearch`): `xetal doc --out` also writes `search-index.js` (`window.XETAL_DOC_INDEX`, one row per item of the documented files and per implemented built-in of the catalog: name, kind, type, the type normalized, file, link, the first line of its doc or the built-in's rules) and `search.js`, a plain script (no build tooling, no framework); every page has a search box in its side bar and a results section. A query reads as a type when it holds `->` or `=>` or starts with a capital, else as a name. By type, Hoogle-like: constraints are dropped and the type split into names and runs of symbols, each type variable renamed `a`, `b`, ... in the order it first appears (Rust `normalize`, mirrored by the script), so `Num a => a -> a -> a` finds `+`, `-`, `m_ax`, `m_in` (an exact match first, then a type holding the query as a run of its tokens). By name: without prefix, underline or mark, case aside (`mean` finds `l:m_ean`, `if` finds `s:i_f<`): exact, then prefix, then contained; up to 60 results. Tests: docsearch tests/search.rs, docsite tests/site.rs; goldens doc-search-index (doc-site-files and doc-site-links rebased on purpose: the two scripts, the search box). |
| D83| The doc release (Saga 32 step doc-release) | `xetal doc --out DIR FILE MORE...` (and `--json`) documents several programs and libraries as one site (`xetal-doc` `models`): each file once, also when found under two names (a given `lib/Stats.xtl` and the built-in `std/Stats.xtl` a program imports, the same file name and text), kept under the first, references to the other moved to it. `scripts/doc-site.sh` (`just doc`, and `just pages` through `scripts/build-pages.sh`, whose rsync keeps `pages/doc/`) writes `pages/doc`: every `lib/*.xtl` and `lib/*.xtlm` (System.xtlm among them), the built-ins, and two programs to read from the top down, `demos/life.xtl` and `demos/tttml-play.xtl` (the TTTML game); linked from the README (its Documentation list and a section saying what `xetal doc` is and how to use it), docs/reference.md's introduction, and the live demo's Help. The built-ins page and the search index take each built-in's description from docs/reference/builtins.ref (built in), else its rules. `lib/Stats.xtl` and `lib/Maybe.xtl` are documented with `##`, `###` and S10 examples (the gate runs them). The index page is titled for the whole site. Goldens: doc-site-several (just-list, doc-needs-json, doc-json-library, doc-search-index rebased on purpose). |
| D84| The clock (QD2, QD3, QD7) | `[]TS` (Int, seven numbers: local year, month, day, hour, minute, second, millisecond) and `[]D_L` (`Num a => a -> Float`, the seconds waited) read the clock in use: new crate `components/base` `xetal-clock` (`Clock`: `now`, `delay`, each refusable; `install`; the system clock by default, chrono's local time and a sleep, compiled out of wasm32), so a host without a clock answers `error[no-clock]`, never a panic; a negative, infinite or unknown delay is `error[domain]` before any waiting. The live demo's worker installs the browser's clock (`components/web` `xetal-webclock`: JavaScript's `Date` in the browser's zone, a delay by watching it; the worker, not the page, waits). `[]TS` changes, so spec cases and the reference show its shape and range. Spec: eval/quad-clock, eval/reject-delay-negative; clock tests (system.rs, install.rs), quad tests. |
| D85| `xetal doc` shows `u_se<` | `xetal-doc` `items.rs` `declared`: in the system macros file, each signature line from `xetal_macro::system_signatures()` (MC21) becomes an item in line order: kind `built-in macro`, its type, its line, section and `##` doc (`doc_above`), source the declaration line, no uses; so `--json` has it, the site draws it like the other system macros (and calls of `u_se<` link to it), and `xetal doc --test lib/System.xtlm` runs its examples. Tests: xetal-doc tests/model.rs; golden doc-site-system rebased on purpose (the Importing section and the u_se< item). |
| D100| The web host (Saga 34 step 12) | A component of its own, `rosetta`, one Yew crate `xetal-rosetta` built by trunk to `pages/rosetta/` (scripts/build-pages.sh) around web's worker: the request carries the program with `Stone.xtl`, `Comparison.xtl` and `demos/rosetta/data.toml` as its files (the worker's store finds libraries beside a program there) and `frames: true`, a new request flag under which the worker posts `Event::Frame` instead of `Picture` and the page keeps only the latest (`Run::frame`), so a long run never piles pictures up. Events: the page's pointer in the picture's own pixels (the element's box scaled to the viewBox; moves at most once a frame), keys by `[]K_EY`'s names, a tick per wait (`use_ticks`), and `choose AXIS ITEM` from the three lists (a new event kind, RS1 refined). The program prints `at IDIOM TOP BOTTOM` whenever it moves; the page reads the last such line for its lists and writes it to the address bar (`?idiom=&top=&bottom=`, read on load as choices). `prefers-reduced-motion` sends the three pauses. The page knows no geometry, traversal or language. The sandbox had neither trunk nor the wasm32 target: the page is built and first seen where `just pages` runs. |
| D99| Faces (RS5, Saga 34 step 11) | `[]V_IEW` in a new crate `xetal-source` (the system component's fourth), over `xetal-view`, runs of one class merged as the HTML export merges them; `v:s_pan` and `v:m_arkup` in Svg; Stone's panel lines are markup, the first a small gray title; `rosetta.xtl` colors X_eTaL cells through `[]V_IEW` and other cells through the optional `spans` table of data.toml (`"class text|class text"`, parsed with `p_artition`), both into the same class-to-color map (`u:c_olor`), shows the output in gray and the note of a missing cell in place of the code; the idiom's name is a caption above the stone. The rest-picture golden rebased for the new faces. |
| D98| Attract mode (Saga 34 step 10) | A fifth state row, the attract clock: seconds since the last touch, seconds since the last step, and the two sweep counters. `cm:dwell` (3 s) and `cm:idleBefore` (8 s) are data at the top of Comparison.xtl. `t_ick` advances the clock and, when the stone has been left alone for the idle time and the last comparison has held for a dwell, calls `a_ttract`: the bottom ring steps; when its count reaches one less than the languages (a sweep of the others), the top ring steps in the same instant and the count starts over; when the top has swept, the drum rolls too: the tumble. A paused axis is counted but not turned, so the rhythm holds; a ring never lands on the other's language (the step skips it). Any event but a tick is a touch (`t_ouched`, applied in `u_pdate`), and a drag no longer pauses an axis for good: the idle wait does that; a click's pause is the sticky one. `r_esume` after a chosen start. Ten more claims (36 in all); golden run-rosetta-attract (24 seconds of ticks, 48 frames: the bottom steps every three seconds, the top with it after six). |
| D97| The pointer (Saga 34 step 9) | The state grows a fourth row, the pointer (the axis being dragged: 0 none, 0.5 pressed and not yet moved, else the axis; where it last was; how far it has moved in all; the last turn in degrees), so a drag is a state transition like any other and is tested the same way. `d_own` remembers; the first `m_ove` of more than four pixels decides the axis (sideways: the half under the pointer, `cm:middle` splitting the picture; up or down: the drum) and pauses it, and each move turns it by `cm:perPixel` (a quarter turn across half the picture), a drag right or down turning toward the previous item (pulling a side to the front); `u_p` settles a drag on the nearest item, one further when the last move was a flick (over 6 degrees), by setting the steps so the angle eases the rest of the way, and treats a press that never moved as a click; `c_lick` toggles the half under it. The decided axis is kept for the whole drag. Thirteen more transition claims (26 in all); a scripted drag session (golden run-rosetta-drag, 7 frames). |
| D96| The axes (Saga 34 step 8) | `demos/rosetta/Comparison.xtl`: the state is a 3 x 4 Float matrix (an axis per row: count, steps, angle, playing); steps are the truth (the item facing is `1 + steps` wrapped) and the angle eases toward `90 * steps` on ticks (half a second, landing exactly when near); `s_tep`, `c_hoose` (the short way round, pausing the axis, never onto the other ring's language; stepping onto it steps past), `t_oggle`, `t_ick`, `s_ettle`, `u_pdate` over an `[]E_VENT` (ticks, the arrow keys, a/d, space/s/w). Cells are read and replaced through masks (`a_t`, `p_ut`), never by position arithmetic. The Stone draws a quarter turn: an axis advanced by `a` is turned by `a - 90 floor(a/90)` and its sides show items counted from `floor(a/90)`, so a turn ends with the picture it would have started from and the rings always stand at the drum's front (and roll with it). `rosetta.xtl` is now the event loop (a frame per tick; `end` prints the state). Transition tests: reg/fixtures/comparison-tests.xtl (golden run-rosetta-transitions, thirteen claims), a scripted key session (run-rosetta-keys), the rest picture (run-rosetta-stone, now one tick); the doc examples of the two libraries run under rosetta-check. |
| D95| The split face (Saga 34 step 7) | `demos/rosetta/Stone.xtl` (a library beside the demo): the stone is three square prisms sharing one place, a drum of idioms turning about x (its bottom, back and top sides, the front being the rings; the corners of each side ordered so text reads upright as it rolls round) and two half-height rings of languages turning about y, each side showing an item found by index offset from the facing one (`drumOffsets`, `ringOffsets`), so the geometry never changes; all sides are painted far to near as one solid (`g:o_rder`), which makes clipping unnecessary: a turning ring's back side simply pokes out of the drum, as an impossible object may. Text lands on a face through `v:m_atrix` (Svg; an affine map of the unit square onto three projected corners), in face coordinates. `demos/rosetta/rosetta.xtl` reads the data with `[]L_IST`/`[]T_ABLE` into aligned arrays and draws three still pictures (at rest; the top ring turning; the drum rolling with the bottom ring turning); golden run-rosetta-stone. Not in the live demo's list yet: a program there has no files beside it, so neither the Stone library nor data.toml can be found until the Rosetta page (step 12) supplies them. Found on the way: a local function needs an underlined name (`d_rumPanel`, not `drumPanel`) and must be defined before the lambda that calls it; a guard cannot stand inside a binding (`i_f<` can); `j_oin` over no texts must give the empty text. |
| D94| Tables (RS2, Saga 34 step 6) | A crate of its own, `xetal-table` (the system component's third; `xetal-quad` and `xetal-system` are at their module limits), over the `toml` crate with only parsing on: `[]L_IST` and `[]T_ABLE` read the file through the store (`xetal_store::read`, so the browser's files work too), take strings only (error[bad-table] naming the file and key for anything else; a file that does not parse the same, with TOML's message), and build the vector or matrix of boxed texts, a missing cell the empty text. `demos/rosetta/data.toml` holds the 31 idioms of docs/idioms.md's array table across APL2, Dyalog, J, BQN, K, Uiua and X_eTaL (209 cells; the K and Uiua cells reviewed by the user), with inputs and outputs for 16 X_eTaL cells; `scripts/rosetta-check.py` (`just rosetta-check`, run by the gate) checks names, keys and runs every X_eTaL cell with its input. Goldens run-table, run-table-errors. |
| D93| Events (RS1, Saga 34 step 5) | `Value::Event(Rc<Event>)` (`xetal-value` `event.rs`: kind, numbers, key; parsed from one line, printed as one) and the nominal type `Event` (`ENUMS`); the fed lines of a run moved from three `Machine` fields into `xetal_frame::Inbox` (`Wants::{Line, Key, Event}`, `next` giving the value a built-in wants or marking the wait), so `[]E_VENT` joins `[]R_EAD` and `[]K_EY` in `caller.rs` without a new function; `Status::WaitingEvent` through `xetal-play`, the runner's protocol (`v`) and `Output::wants_event`; without a queue `xetal-quad` `event::next` reads standard input and gives `end` at its end; `--events FILE` installs the `Drawing` store with a script (blank lines and `#` comments skipped); the live demo gives a tick on the next animation frame each time the program waits (`xetal_typing::use_ticks`, one event per wait so the worker is never flooded) and `key NAME` for keys, pointer events being the Rosetta page's (step 12). Goldens run-events, run-events-bad; tests xetal-value tests/event.rs, xetal-step tests/input.rs. |
| D91| The error macros (ER4, Saga 21 step 4) | Seven `s:` macros in `lib/System.xtlm`, plain text over the built-ins: `t_ry<` writes `'{ @ -> body } []T_RAP '{ e -> handler }` and declares `## binds: e` so hygiene leaves the handler's `e` as written; `c_atch<` writes a disjunction of `([]E_CODE e) m_atch "code"` over its words (a guard whose else is `[]H_ALT e`); `f_inally<` writes `[]E_NSURE`; `r_ecover<` parenthesizes its value; `r_etry<`, `h_alt<`, `c_ontinue<` take `@` on both sides (the side check of MC22 rejects text) and write the built-in applied to `e`. No Rust changed. Spec: macros/try, catch, catch-halts, finally, retry-halt-continue, reject-try-at, reject-retry-text; reject-unknown lists the new names. |
| D90| The warning (ER3, Saga 21 step 3) | `default []W_ARN "code" "message"` (`a -> Box Char -> a`) and `[]C_ONTINUE e` (`Error -> Outcome a`). The pair is parsed and checked by `xetal-quad` (`signal::warn`, always the error), and the machine intercepts the call (`trapping`): `Machine::resumable` finds the nearest `Kont::Trap` below (stopping at `Items`), pushes `Kont::Continuing { error, default, trap_at }` on the stack as it stands and applies the handler; its answer, Continue, returns the default into the intact stack; Halt offers the warning to the next trap below `trap_at`; Recover or Retry call `Machine::unwind(e, Some(outcome))`, which pops to the trap running the cleanups on the way (`Cleaning` carries the decided outcome) and applies the outcome at the trap instead of calling the handler again. `catch` is `unwind(e, None)`. `Outcome::Continue(e)` reaching an ordinary trap is error[not-resumable]. Spec: eval/warn, warn-uncaught, warn-recover, warn-halt, warn-ensure, warn-ensure-recover, reject-continue-signal, reject-warn-type. |
| D92| The Rosetta stone's host boundary (RS1-RS4, Saga 34 step 1) | Decided before building: `[]E_VENT` (a nominal `Event` with `[]E_KIND`, `[]E_AT`, `[]E_KEY`, fed by the line queue `[]K_EY` uses, `xetal run --events FILE`), `[]L_IST` and `[]T_ABLE` (strings-only TOML reading with the axes named in the call), the data file's schema (axes as lists, `names`, `input`, and `source`/`output`/`notes` keyed idiom then language), and the first data. The alternatives weighed are in lang-choices 9c and docs/rosetta.md; `demos/rosetta/data.toml` starts with the schema and the first rows. (D90 and D91 are the errors lane's warning and macros.) |
| D89| The trap (ER2, Saga 21 step 2) | catching is the machine's, not a kernel's: `[]T_RAP` and `[]E_NSURE` push a frame (`xetal-step` `Kont::Trap`, `Kont::Ensure`) and run the body as `f @`; an error no longer ends the run at once but goes to `Machine::catch`, which pops frames (a lazy thunk restored as before) to the nearest Trap (the handler is applied to `Value::Error`, under a `Handling` frame whose value, a `Value::Outcome`, recovers, retries with the Trap frame pushed again, or halts by catching the original error again) or Ensure (the cleanup runs under a `Cleaning` frame holding the body's result, which then returns or is caught again); at `Items` the stack is cleared and the run ends with the error, as before. The checker gains `Type::Outcome(a)` beside `Box` and `Error` among the nominal types; the catalog types everything (`[]T_RAP : (Unit -> a) -> (Error -> Outcome a) -> a`), so a wrong recovery is a type error at the call. The readers and constructors are plain built-ins in `xetal-quad` (`outcome.rs`). The frame types moved to a crate of their own, `xetal-frame`, since `xetal-step` had its seven modules. Spec: eval/trap, trap-halt, trap-retry, trap-nested, ensure, ensure-error, reject-trap-recover-type; ty tests/outcomes.rs. |
| D86| `[]S_IGNAL` (ER1, Saga 21 step 1) | `"code" []S_IGNAL "message"` is a built-in of type `Char -> Char -> a` (as `[]P_ANIC` is `Char -> a`) in `xetal-quad` that always fails with `Diagnostic::new(code, message)`, so it is reported, located at the call and given exit status 1 by the machinery every error already has; nothing new in the evaluator. The code must be spelled as xetal's own codes (lowercase letters, digits, hyphens between them), else `error[bad-code]`, so a signal can later be caught by code (ER2) without ambiguity. Spec: eval/signal, eval/reject-signal-code; golden signal-stops (reg/fixtures/signal.xtl); quad tests. |
| D87| The fast gate | `scripts/gate.sh` is fast by default: `scripts/affected.py` reads the files changed since the merge base with `origin/main` (`GATE_BASE` names another) and plans it: a component whose files changed is checked (format, clippy, tests), a component depending on one (by its crates' own dependencies, transitively) is tested only, the rest are skipped; `lib/`, `spec/`, `demos/` and `userlibs/` count for the components that build them in or test against them; the literate, diagram, Emacs, recipe, asks and browser-build checks run only when their inputs changed; locks, goldens, doc tests, reference, status, checklist and markdown always run. `--full` runs everything: between features, after a batch of merges, before a release. A change to the gate's machinery checks everything. Each step taking 5 seconds or more prints its time. Components are not run in parallel: they share one target directory, which cargo locks. Asked for by the user after 20-minute gates on every merge. |
| D88| American spellings only | `scripts/check-spelling.py` (the checker X_eTaL-demos uses, with a self-test) scans every tracked text file except pages/, the saga records, the archival research conversations and itself, and fails on British forms (its word list has the families: the -or, -er, -ize and single-l words, gray and the rest); text inside a URL is not checked, and `analyses` (the American plural) is allowed. In the gate, always. The 107 forms found at the audit are fixed in docs, comments, identifiers, demos, literate documents and the reference. The user is American and asked for the test and the audit. |
| D108| The stone released (Saga 34 step 16) | `docs/literate/rosetta.org`: the stone built up from its two libraries with three pictures the libraries draw (at rest, the top half turned, the stone rolled), the quarter turn, the state machine and the attract tour as recorded results; `scripts/literate.sh` and `literate-html.sh` put `demos/rosetta` on XETAL_PATH for it. Pictures pinned: goldens run-rosetta-angled and run-rosetta-tumble beside run-rosetta-stone (front, angled, mid-tumble), and `images/rosetta-three.png` in the README from the same frames. `scripts/literate.sh` takes the documents to run, and the fast gate passes only the changed ones unless a library, demo or the Emacs code changed (`affected.py --literate`): one new document cost a 4.7-minute run of all thirty-two. Frictions of the lane in docs/dogfooding.md; Saga 34 marked done in docs/plan.md; the lane archived. |
| D109| The stone's footer and the space bar | The stone's page ends with the site's footer, shared with the live demo (`xetal_chrome::footer_at("../")`: the same links, reaching the root through a prefix), and the footer gains Live editor and Rosetta stone links, so each page reaches the other. The space bar paused nothing on the page: its event line is `key` and two spaces, and `Event::parse` split on whitespace (a key without a name), as did the CLI's scripted queue, which trimmed the line. A `key` line whose rest is one character is that key, `key Space` names it too, and scripted lines keep their trailing spaces; the keys golden presses it both ways. |
| D111| The stone's cells are run in their own languages | `scripts/rosetta-run.py` (`just rosetta-run`): every BQN and Uiua cell with an input (`input.IDIOM.LANG`, the same values X_eTaL's cells use) is run by its interpreter and compared with `output.IDIOM.LANG`; `--bless` records; a missing interpreter is a note, not a failure, so the gate passes without the tools. `scripts/install-array-langs.sh` (`just install-array-langs`) builds CBQN from GitHub and Uiua 0.19.1 from crates.io under `tools/bin` (gitignored). Corrections with their evidence: Uiua, from running 0.19.1 and its primitive docs (uiua_parser 0.19.1, src/defs.rs, which uiua.org renders): `≡` rows for the deprecated `∵` each, `˜F x y` (backward) for the deprecated flip, `˙×` (self) for the deprecated duplicate, `⊗ v x` (indexin, array first), `∊ v x` (memberof, haystack first), `⨬(∘|¯)⊸<0`, and `↬` gone (recursion by name with a declared signature). APL2 against IBM's APL2 Idioms list: the count is `×/⍴V`; the rest confirmed. 44 cells run and agree with X_eTaL's; a face shows a multi-line result on one line with ` / ` between the lines (`u:o_neLine`). Dyalog, J and ngn/k are the next interpreters to add. |
| D118| The tour's mode is visible, and a link opens touring | The program prints `mode touring|holding|paused[: axes]` whenever it changes (`cm:m_ode`) beside the `at` line, and the page shows it as a badge under the stone with the key to press (green dot touring or holding, amber paused). A `choose` pauses its axis (choosing is navigating), so a page opened from a `?idiom=&top=&bottom=` link started with every axis paused and never toured; the page now sends `key r` after its opening choices, and `r` resumes the whole tour at once, the idle wait over (`cm:k_ey`); a touch still holds the tour for the idle time and the space bar still toggles it. Five mode claims in the transition tests. The user, 2026-10-06. |
| D116| The tour is `for top, for idiom, for bottom` | The attract traversal: the bottom sweeps the other languages one dwell each; then the stone rolls to the next idiom in the same instant and the bottom goes on round; only when every idiom has been shown does the top advance (`cm:a_ttract`, counters BOTTOMS and IDIOMS). The language compared from stays put through the whole table and no comparison comes round again within minutes. The program starts at the tour's beginning (first idiom, first language over the second) instead of Rotate with X_eTaL over K. Before (D98) the top stepped after each sweep of the bottom and the stone rolled after the top's sweep: two minutes per idiom with six languages, and the same pairs again and again; the user saw it as a regression. Also: the face transform's six numbers are rounded to a thousandth of a pixel after the differences are taken (`st:a_ffine`), because the last digits of sine and cosine differ between machines and the pictures are pinned as text; the goldens had flipped between the sandbox and the user's machine. Claims, doc examples and the literate tour table follow; goldens attract, drag, keys, stone, angled, tumble and transitions rebased on purpose. |
| D115| The axes are alphabetical | The page's lists show the data's order, so the data keeps both axes in alphabetical order of their display names (case aside): `idioms`/`idiom_names` and `languages`/`language_names` sorted together, and `scripts/rosetta-check.py` fails on any pair out of order (`out_of_order`, with a `--self-test` the gate runs), so an idiom or language added later lands in its place or the gate says so. The stone's tour and the generated table of docs/idioms.md follow the same order. Six stone goldens rebased on purpose (the neighbors changed). The user, 2026-10-06. |
| D114| Dyalog dropped; the APL column is GNU APL | The user's decisions (2026-10-06): Dyalog APL leaves the stone entirely (its key, name and every cell; a vetted set of idioms across languages, with three K dialects and no Dyalog, is being built elsewhere and will replace the hand-kept cells when ready), and the `apl2` column is shown as GNU APL, the APL2 dialect people can run today; its cells stay IBM's APL2 idioms until GNU APL's own extensions are vetted. Six languages, 176 cells; the stone's rings have five languages to turn through. The key `apl2` stays, so links keep working. |
| D113| The stone's cells are run in their own languages | `scripts/rosetta-run.py` (`just rosetta-run`): every BQN and Uiua cell with an input (`input.IDIOM.LANG`, the same values X_eTaL's cells use) is run by its interpreter and compared with `output.IDIOM.LANG`; `--bless` records; a missing interpreter is a note, not a failure, so the gate passes without the tools. `scripts/install-array-langs.sh` (`just install-array-langs`) builds CBQN from GitHub and Uiua 0.19.1 from crates.io under `tools/bin` (gitignored). Corrections with their evidence: Uiua, from running 0.19.1 and its primitive docs (uiua_parser 0.19.1, src/defs.rs, which uiua.org renders): `≡` rows for the deprecated `∵` each, `˜F x y` (backward) for the deprecated flip, `˙×` (self) for the deprecated duplicate, `⊗ v x` (indexin, array first), `∊ v x` (memberof, haystack first), `⨬(∘|¯)⊸<0`, and `↬` gone (recursion by name with a declared signature). APL2 against IBM's APL2 Idioms list: the count is `×/⍴V`; the rest confirmed. 44 cells run and agree with X_eTaL's; a face shows a multi-line result on one line with ` / ` between the lines (`u:o_neLine`). Dyalog, J and ngn/k are the next interpreters to add. |
| D110| The stone after the user's hands-on hour | Zoom: two buttons scale the stone (50 to 250 percent, `--scale` on the pane; it fills the window's shorter side at 100) and one shows it full screen; the picture is vector and its own size never changes. Pace (Comparison): dwell 5 s (was 3: too quick to read two code faces), the tour held 20 s after any key or drag (was 8: an arrow-key look round was interrupted), and the space bar pauses and resumes the whole tour (`cm:t_oggleTour`; it paused the top half alone), s and w still each their axis; the reduced-motion opening sends the space bar alone. The lists and the address follow the picture: `u:w_here` names the faces (`u:i_tem`), printed when a turn lands, not the steps, which lead by the easing; the page sets the lists from that line (`use_follow`, node refs) instead of binding them every render, so a choice never snaps back. The data reviewed cell by cell after APL2's anonymous function turned out to be a defined-function header: it and K's swap are honest nones, K's shape, inner, member and Uiua's swap and conditional corrected, notes added; the mainstream row of idioms.md follows. Goldens attract (40 s now), drag, keys, stone rebased on purpose. |
| D112| The worker is never served stale | trunk content-hashes the page's own files but not the worker's (`xetal-runner.js`, `_bg.wasm`, the loader shim), and GitHub Pages caches for ten minutes, so right after a deploy a browser ran the new page (and the new `rosetta.xtl`) with the old worker and its old embedded libraries (`v:s_ized is not defined`). The page now makes the worker itself: a blob script that is trunk's shim with the page's address as base and a build id (`XETAL_RUNNER_BUILD`, the build's time from xetal-runner's build.rs, rerun when lib/ or any component changes) on each file's URL, so every build's worker is a new cache entry. Both pages (the live demo and the stone) share the session code, so both are fixed. |
| D106| Text that fits its face (Saga 34 step 15) | The trap idiom's X_eTaL cell is the errors lane's macro form (`"1 d_iv 0" t_ry< "@ r_ecover< \"-1\""`, -1), in both tables of docs/idioms.md and the spec case; the data had said none yet. Code longer than 15 characters is split at the space nearest its middle (`u:s_plit`, each half colored on its own: `[]V_IEW` classifies fragments) and each line is shrunk to fit (`u:f_itted`, `v:s_ized`, never below 0.05); notes wrap at the spaces (`u:w_rap`, `u:l_ines`, 18 characters). A panel draws its text only when its corners go round clockwise on the screen (`st:f_acing`, the signed area): a face turned away is covered anyway, but text laid on a face seen nearly edge-on poked out past it. The view tilts 22 degrees (was 12) so the lid's idiom label reads. Goldens run-rosetta-stone and type-svg rebased on purpose. |
| D107| The kind kept by the primitives (T9, Saga 20 step 2) | the structural helpers of `xetal-struct` (reshape, take, drop, select, replicate, first, partition, cat), the rotate and move kernels of `components/axes`, and `e_ach` and `t_able` in `xetal-map` carry the argument's kind to their result (`with_kind`), so an empty result says what it would have held; `cat` takes the kind of the side that has items; `as_array` and `as_vector` give a scalar the kind of its value (`kind_of`), and the pieces of a partition are boxes. Spec: eval/empty-kind-through-take and eval/empty-kind-boxes (active now), eval/empty-kind-primitives (a line per primitive). |
| D105| The stone's frame budget, measured (Saga 34 step 14) | Three benchmarks, `bench/rosetta-{update,scene,svg}.xtl` (the phases apart; `scripts/bench.sh` and `bench-check.sh` set `XETAL_PATH=demos/rosetta` for them), and the whole frame from the program under 300 scripted ticks. Release CLI, this sandbox: update 0.2 ms, scene 0.5 ms, SVG 34 ms, the frame 37 ms against a budget of 33. The cost was `v:e_scape` (a lambda per character, a boxed reduce to join; every attribute and text goes through it). The fix is in X_eTaL, not Rust: text without `&<>"` is itself after one `m_ember?` test; the slow path only for text with a special. After: SVG 9 ms, the frame 16 ms. The worker's share is measured in the browser by the page's owner (no wasm32 here). What remains is written into Saga 30's lean-kernels step as its measured case. docs/speed.md has the table. |
| D102| idioms.md generated (Saga 34 step 13) | The array table of `docs/idioms.md` is generated from `demos/rosetta/data.toml` by `scripts/idioms.py` (`just idioms`): the prose and the mainstream table live in `docs/idioms.template.md`, whose one marker line becomes the table (a column per language in the data, a row per idiom; a cell is the source in `<code>`, HTML-escaped with every non-ASCII glyph an entity so the document stays ASCII, then the cell's note in parentheses; a note alone where there is no source; `none` where there is neither). The gate runs `--self-test` (the cell rendering on samples) and `--check` (the document is current). The table gains K and Uiua; a `(a matrix)` found in BQN's transpose source moved to its note. The mainstream table stays in the template until Saga 35 puts those languages in the data. |
| D103| The kind of an empty array (T9, Saga 20 step 1) | `xetal_array::Kind` (Number, Char, Box; Number unless told) is a field of `Array`, kept by `map` and `zip`, set to Char by a string literal in the evaluator, and read by DISPLAY's bottom mark when the array is empty; two empty arrays are equal only in one kind, non-empty ones as before. Spec: eval/empty-char-kind (active); eval/empty-kind-through-take and eval/empty-kind-boxes pending for step 2. Test: xetal-array tests/kind.rs. |
| D104| The page's first viewing (Saga 34, after step 12) | Keys: a window-level listener (`use_keys`) instead of `onkeydown` on a focusable `main`, which heard nothing until something in the page had the focus; a key typed into a list or field is left to it. The address bar: its `choose` lines were posted right after the start request, before the worker had the program, so the worker dropped them; they are now read at once and sent the first time the program waits for an event (`wants_event`), the reduced-motion pauses with them. Geometry: each half is a closed box of six faces (`st:r_ing` gives front, right, back, left, lid, floor; `st:capOffsets` -1 1 says which idiom the lid and the floor name) and the drum is gone; the open drum showed its inside, and its far corners as stray planes, whenever a ring stood turned. Faces: code bold (700), black, 0.1 of the face high, the title 0.06; faces near white (#fdfcfa, #f4f1ec, caps #e9e4db); cells without spans colored by `u:g_uess` (digits numbers, letters names, else built-ins, one span per run, primitives only); zoom 90 to 112. |
| D101| Pages in parts; the gate's build tier | `scripts/build-pages.sh` builds pages/ in five parts (web: the live demo and its screenshot; literate; doc; poster; latex), each only when the hash of its inputs in `pages/INPUTS` (one line per part) changed, `--all` or part names on demand; `scripts/check-pages.sh` names the stale parts; the inputs are the demos, libraries, documents and templates, not the Rust sources (as before: after a change to how programs are drawn or run, `just pages --all`). The fast gate gains a third tier: a component that only depends on a change has its library code compiled (`cargo check`), not its tests, since the spec cases (now run whenever cli is not otherwise tested) and the goldens cover it end to end; a change to gate.sh or affected.py turns on every document check but leaves the components to the plan. The user, after a 41-minute merge. |
| D117| Three gates: sample, affected, full | `scripts/gate.sh` is the sample gate by default: after one build of the CLI, the end-to-end checks (goldens, run in parallel by `reg.sh`; spec cases; doc tests; reference, status, Rosetta data and idioms tables; spelling; markdown) run at once, each to its own log, about 30 seconds; `--affected` adds the planned component work, the browser build, the document checks whose inputs changed, and sw-checklist; `--full` everything, run nightly by `scripts/nightly.sh` (a launchd job, `scripts/nightly.plist`; the log under work/nightly/; a GitHub issue opened on failure, closed when it passes) and before a release. The literate check runs every document in its own Emacs at once (69 s, from several minutes). Merges are pushed after the sample gate; a nightly failure is fixed forward or the merge reverted (`git revert -m 1`). The user, after days of 10-45 minute merges. |
| D119| Private names, the decisions (Saga 38 step 1) | PN1-PN7 recorded in lang-choices section 14 from docs/private-names.md: `h:` for file-private definitions in apps and libraries, never part of a file's interface (PN7); bare top-level functions in a library deprecated (PN2); an app's bare top-level function names both choices (PN3); a library that exports nothing is an error (PN4); bare variables untouched (PN5); lambda locals unchanged (PN6). MC8 row 5 widened to `h:`, rows 21-23 added. Spec: names/h-helpers-in-an-app, names/reject-h-alias, names/reject-bare-function-in-app, all pending; the library-side rules get fixture libraries and goldens in steps 2-4. |
| D124| Arrays and Any (Saga 39 step 3) | A tuple must not stand in for an array, yet the plumbing built-ins must carry one (TU5, TU9). A hidden class `Arr` (everything but a tuple) is given to every variable of a built-in's signature unless the signature writes `Any a`, and to an array literal's item type; a box passes its classes on to its item without `Arr`, so a box may hold a tuple (TU8). Printing inverts it, so T7 reads as before: `Arr` is never printed, a variable with no class prints `Any a`. Alternatives: every array variable printed `Arr a` (honest, but every signature and golden changes), or a hidden constraint printed nowhere (fewest changes, but `i_d` and `r_ev` would both print `a -> a`); the user chose `Any`. Cost: purely higher-order code prints `Any` on every variable (the Combinators birds, Maybe). |
| D123| Tuple values (Saga 39 step 2) | The comma is a token; a parenthesized group with a comma at its own level is a tuple (surface `Tuple`, Core `Tuple`), its parts evaluated right to left like an array's items, each part a whole expression (`(1 2 3, 2 * 3 + 1)` is `(1 2 3, 8)`). Errors: `comma-outside-tuple`, `empty-tuple-part`, and `record-field` for `name: ` opening a part (TU12, from the lexer). The type is `Type::Tuple`, unified part by part at one size; the value `Value::Tuple`. Printing (TU6): one line when every part fits one, else `(1:` and one block per part, a tall part indented one space under its position, a tall last part closing with `)` on its own line. TU7 needed `m_atch` and `=` told apart, both `Eq` before: a class `Match` (Eq's types and tuples) types `m_atch`, which compares each part as a whole array; a tuple where `Eq` or `Num` is wanted is "expected an array, found a tuple". |
| D122| Tuples, the decisions (Saga 39 step 1) | TU1-TU12 in lang-choices section 9d, decided with the user (2026-10-07), every one the recommendation of docs/tuples.md: `(w, m, v, k)` with the comma legal only directly inside parentheses or a pattern; types printed as written, parts rank-erased (T7); patterns only, in bindings and lambda parameters, nested, `_` ignoring a part; mismatches as type errors; one-line printing with blocks for tall parts; `m_atch` only (no `=`, no arithmetic into tuples); boxed tuples only in arrays (v1); the higher-order built-ins unchanged; tuples and pattern projections in Core; pattern names as binders for hygiene and L7; `(w: ...)` kept free for records. Grammar (rule 3): a parenthesized group is a tuple when a comma stands at its own level and grouping otherwise; a statement is a pattern binding when `:=` follows its closing parenthesis; in a lambda the parameter list is everything before `->`, where a parenthesized group is a pattern, and a lambda without `->` has none, so `{ (a, b) }` is a lambda whose body is a tuple. Every rule's spec case is written, pending (spec/tuples/). |
| D120| The h: namespace (Saga 38 step 2) | `h:` is reserved as an alias (MC8 row 5; the RESERVED message names it); the macro phase leaves a program's `h:` names as written and hides a library's with its other internals (MC6), so an importer naming one gets row 11's `not-exported`; core lowering accepts `h:` functions and variables at the top level; `xetal type` (`program_types`, also the live demo's Types pane) leaves an app's `h:` names out (PN7). The tour's and hello's import of Hello moved from `h:` to `hi:`. Spec: names/h-helpers-in-an-app and names/reject-h-alias active. Goldens: names-app-runs, names-app-interface, names-library-private, names-library-hidden. |
| D121| A library that exports nothing (PN4, Saga 38 step 3) | when `u_se<` links an `.xtl` whose rewrite found no `l:` name, the macro phase fails with `library-exports-nothing` ("library Name exports nothing; mark its exports with l:") at the import statement (MC8 row 22); the library's own imports do not count. Run as an app the same file is a script with no interface (PN7). Goldens: names-helpers-run, names-library-exports-nothing. |
| D126| Tuple patterns (Saga 39 step 4) | A statement is a pattern binding when `:=` follows the `)` closing its first token; a lambda's parameters may be tuple patterns (TU3), nested, with `_` for a part bound to nothing (TU4; `_` alone is a token of its own, an error outside a pattern, and not a whole parameter). Each name appears once in a pattern and among a lambda's parameters. Core gets one form, `Proj { index, size, tuple }` (printed `(part i n t)`): a pattern desugars to the value under a fresh name (`%1`, never shown by `xetal type`) and a binding of each named part's projection; a pattern parameter is a fresh parameter whose parts the body binds first (TU10). The checker unifies the value with a tuple of `size` fresh parts, so a size mismatch reads "expected a tuple of 2, found a tuple of 3" (TU5). Hygiene and the shadowing warning see pattern names as binders (TU11). |
| D125| Bare library functions deprecated; xetal migrate (PN2, PN3, PN6, Saga 38 step 4) | a library's `h:` names are hidden in their own namespace (`HA`, beside the bare privates' `PA`), so the lint warns `deprecated-private` ("write h:j_oin; ...") on a bare top-level function of a library only, never on a helper or a bare variable (PN5); a test in xetal-program fails on any library under lib/, userlibs/ or demos/rosetta/ that warns, so the gate treats it as an error. `xetal migrate FILE` prints the file with each bare top-level function and its uses written `h:`: a token rename, so comments and layout stay and the parse differs only in those names; a lambda's own local of the same name is left bare (PN6). A bare function in an app is `bad-binding` naming both choices, `u:` and `h:` (PN3). lib/, Greetings and Stone migrated. Goldens: names-migrate, names-deprecated-private; doc-json-library rebased. |
