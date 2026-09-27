# X_eTaL -- Language Design (v0 proposal)

Status: PROPOSAL. Every rule below is provisional until a test pins
it. When a test and this document disagree, the test wins and this
document is updated in the same commit. Research transcripts
(`docs/research*.txt`) are archival; where they disagree with each
other this document records the chosen resolution and why.

Name: X_eTaL (display name). Code uses the
slug `xetal` (crates, binary) and a single display-name constant
`xetal_base::LANG_NAME`, so a rename touches one constant plus docs.

## 1. Guiding rules

1. Tests define the language.
2. Decoration determines grammatical class; spelling never does
   (except for the fixed built-in dictionary of names).
3. Ambiguity is an error. The parser never "picks something
   reasonable"; types may reject a unique parse but never choose
   between parses.
4. All surface sugar desugars to a small Core IR.
5. Built-ins and user definitions follow identical application rules.
6. Invalid valence/type is an error; arguments are never ignored.

## 2. Lexical structure (raw ASCII source)

Source is ASCII (Latin-1 tolerated only inside string/char literals,
which are deferred); any other character is a `non-ascii` error, and
ASCII characters outside the token set (`# , : ' " $ % ! ? ~ &` ...)
are `unexpected-char` (so there are no comments yet). Spaces, tabs and
carriage returns separate tokens. Each `\n` is a `Newline` token; the
parser decides whether it acts like `;` (D2).

The raw ASCII is also the typing shorthand: `_` underline, `_12`
subscript, `^r` superscript. No input method is needed; `xetal render`
shows the decorated Unicode form and `--raw` converts it back (section
8). The lexer does not accept decorated Unicode input (D13).

Pinned by `crates/xetal-lex/tests/lex/` and `spec/lex/*.case`.

### 2.1 Names and decoration

A *stem* is `[A-Za-z][A-Za-z0-9]*` (no underscores inside stems).
Decoration suffixes, in fixed canonical order:

```
stem [ '^' SUP ] [ '_' [ SUB ] ]          -- canonical order
SUP  := letter-word  (derivation: r reduce, s scan, e each, o outer)
SUB  := digits       (axis list: 1..9, each digit one axis)
      | '@'          (niladic application sugar)
      | empty        (bare underline: "this is a function")
```

| Raw        | Token                                   |
| ---------- | --------------------------------------- |
| `r`        | `Noun(r)`                               |
| `r_`       | `Func(r)`                               |
| `t_2`      | `Func(t, axes=[2])`                     |
| `t_12`     | `Func(t, axes=[1,2])`                   |
| `now_@`    | `Func(now, niladic)` -> sugar           |
| `+^r`      | `Func(+, deriv=r)`                      |
| `+^r_2`    | `Func(+, deriv=r, axes=[2])`            |
| `f^s`      | `Func(f, deriv=s)`                      |
| `m.f_`     | `Func(f, ns=m)`                         |

The lexer keeps the superscript word as written (`r`, `reduce`); the
dictionary resolves it later, so an unknown word is not a lex error.

Pinned lexical rules (each has a rejection test):

- A name is a function iff it has a symbol stem, a superscript or an
  underline; a plain named stem is a noun.
- A bare underline must add something: `+_` and `f^r_` are
  `redundant-underline` (one spelling per meaning). `+_1`, `+^r_2`,
  `f^r` are fine.
- Axes are digits 1-9, each at most once: `r_0`, `t_11` are `bad-axis`.
- Canonical order only: `r_2^r`, `r_^r` are `non-canonical-order`;
  `r__`, `a_b`, `t_2x`, `now_@x`, `f_.g` are `bad-decoration`; one
  superscript only (`+^r^s`); `r^` needs a word; `+^2` is not a word.
- A decorated or named name must end at a delimiter (not a letter,
  digit, `_ ^ . @`). An undecorated symbol may touch anything (`1+2`).
- Namespace prefixes: one level, on named functions only. `m.x`,
  `m.+`, `a.b.f_`, `m.` are `bad-namespace`.
- Lambda arguments are exactly `_l` / `_r`, undecorated: `_x`, `_lx`,
  `_r_`, `_l^r`, `_@`, `_` are `bad-lambda-arg` (D12).
- Numbers are `[-]digits[.digits]`, 64-bit integers or floats; `1.`,
  `1.2.3`, `2x`, `2_`, `2^r` are `bad-number`, `.5` is
  `unexpected-char`, too-large integers are `number-out-of-range`.

Symbol stems are always functions: `+ - * / = < > |` (set to be
pinned; `%` etc. reserved). A symbol may carry `^SUP` and `_SUB`.

Resolved research conflict: the research used superscripts both for
provenance (`r^u`, `r^m`) and for derivation (`a^r` reduce). v0
reserves superscript for **derivation**; provenance/namespace uses a
dotted prefix `m.f_` (the research's own stated preference).

Resolved research conflict: the research used `r` for both rotate and
reduce and `s` for both scan and shape. v0 dictionary (terse / long):

| Terse | Long        | Monadic            | Dyadic                  |
| ----- | ----------- | ------------------ | ----------------------- |
| `i_`  | `range_`    | range 1..n         | (reserved: index-of)    |
| `p_`  | `shape_`    | shape-of (rho)     | reshape                 |
| `x_`  | `index_`    | (reserved)         | select / slice (axis)   |
| `t_`  | `rotate_`   | reverse on axis    | rotate by n on axis     |
| `^r`  | `^reduce`   | derivation: reduce |                         |
| `^s`  | `^scan`     | derivation: scan   |                         |
| `^e`  | `^each`     | derivation: each   |                         |
| `^o`  | `^outer`    | derivation: outer  |                         |

Terse and long spellings must desugar to the identical Core node
(normalization-equivalence tests).

### 2.2 Other tokens

| Raw        | Token                                          |
| ---------- | ---------------------------------------------- |
| `_l` `_r`  | `LamArg(Left)`, `LamArg(Right)` (leading `_`)  |
| `@`        | `Unit`                                         |
| `;`        | statement separator                            |
| `{ }`      | lambda                                         |
| `( )`      | grouping                                       |
| `[ ]`      | train                                          |
| `=`        | equality function, or binding (see 3.4)        |
| `-1`, `2.5`| numbers; negative-literal rule below           |

Negative literals (no high-minus in ASCII): `-` immediately followed by
a digit, and preceded by start-of-input, whitespace (including a
newline), `(`, `{`, `[` or `;`, lexes as part of a number. So `-1 0 1`
is a 3-vector, `3 - 1` is subtraction, `3 -1` is the 2-vector `3 -1`,
and `3-1` is an `ambiguous-minus` error (requires spaces). The same
applies after any token: `a-1`, `(x)-1`, `2 +-1`, `--1`, `f_-1` are
errors. `-x` (minus before a letter) is the function `-` then `x`.
Each case is a pinned test.

Rejected at lex time (examples): `r__`, `r_0` (axes are 1-based),
`_x` (unknown lambda arg), `r^` (empty superscript), `r_2^r`
(non-canonical order), `a b_c` stems containing `_`.

## 3. Grammar

### 3.1 Classes

Every expression is syntactically a **noun** or a **function**:

- noun: number, strand of numbers, `@`, plain name, `_l`, `_r`,
  parenthesized noun, application result.
- function: decorated name, symbol, `{ ... }`, `[ ... ]` train,
  parenthesized function.

Class is decided from tokens alone, never from types or bindings.

### 3.2 Application (APL-style, right to left)

- `f_ Y`         monadic: `App(f, Y)`
- `X f_ Y`       dyadic: `App(App(f, X), Y)` where X is the single
  noun (or strand) immediately left of `f_`.
- Functions have long right scope: `f_ g_ X` = `f_ (g_ X)`.
- No precedence among functions; parentheses group.
- Strands: adjacent numeric literals form a vector literal. Adjacent
  plain names (`a b`) are a v0 error, not a strand.

### 3.3 Lambdas

- `{ ... _r ... }` without `_l`: monadic `Lam(r, body)`.
- `{ ... _l ... _r ... }`: dyadic `Lam(l, Lam(r, body))`.
- Every function is monadic. A "dyadic" lambda is curried, and
  `X f_ Y` is `App(App(f, X), Y)`; niladic `f_@` is `App(f, Unit)`.
  So `f_ X` on a dyadic lambda is partial application: it fixes `_l`
  and returns a function of `_r`.
- Scoping (decided by the user; pinned by the parser/desugar tests):
  `_l` / `_r` always refer to the innermost enclosing lambda, as APL
  dfns treat alpha / omega. Outer arguments are passed in explicitly
  (Life passes the board as the inner lambda's `_r`).
- An inline lambda is an ordinary function, so `X { ... } Y` is a
  dyadic application by the general rule (decided by the user; pinned
  by the parser tests).
- `{ _l }` (left only): OPEN -- reject or K-like; a test will decide.
- `{ }` with neither: OPEN -- constant function taking `@`?

### 3.4 Bindings and equality

`=` is the equality function. A statement whose first two tokens are
`plain-name =` is a binding. A comparison at statement start must be
parenthesized: `(x = 3)`. Alternative under consideration: a distinct
binding token (`<-` or `:=`). This choice is pinned by the first
parser saga's ambiguity tests.

Calling a bound function requires decoration: `square = { _r * _r };
square_ 7`. Plain `square` is the function as a noun (a value passed
to higher-order functions).

### 3.5 Derivations (built-in operators)

A superscript derives a new function from the function it decorates:
`+^r` = `App(reduce, add)`. `sum = +^r; sum_ 1 2 3 4` must work: a
derived function is an ordinary first-class value. Axis subscripts
apply to the derived function: `+^r_2`.

Research note: the research also wrote Life as `+ r_ ...` (reduce as a
separately spaced HOF). With right-to-left application, `+ r_ A`
parses as `+ (r_ A)`, so the operand relationship would need a
separate operator class; the superscript form expresses the same
thing lexically and cannot be misparsed.

### 3.6 Niladic / Unit

`@ : Unit`. `now_ @` applies `now : Unit -> Time`. `now_@` is sugar,
normalizing to the identical Core. `now_ 42` is a type error.

### 3.7 Trains

`[f g]` hook/atop and `[f g h]` fork, defined purely by desugaring:
`[f g h] x` == `(f x) g (h x)`; dyadic forms pinned in M7.

## 4. Core IR

```
Expr := Lit(Scalar | Array) | Unit | Var(Id) | Prim(PrimId, Axes)
      | Lam(Id, Expr) | App(Expr, Expr) | Let(Id, Expr, Expr)
```

Every Core node carries the NodeId and source span of the surface
construct it came from (for traces and the explainer).

## 5. Types (v0)

Types: `Unit Bool Int Float Char Array<T> T -> U`, type variables,
constraint `Num a`. Algorithm W / unification with let-polymorphism.
Scalar functions are typed on scalars and lifted over arrays by a
single lifting rule (scalar extension), not per-function overloads.
Shape is runtime metadata (not in types) in v0.

Bool (D5, direction decided by the user; exact typing rule pinned by
the types saga): there is a real `Bool` type and `=` `<` `>` return
Bool. Coercion is implicit in both directions:

- Bool -> Int in arithmetic: true is 1, false is 0, so
  `(S = 3) + c * (S = 4)` type-checks and Life sums Bool boards.
- Int -> Bool where a Bool is required: 1 is true, 0 is false, any
  other value is an error (a runtime error when the value is only
  known at run time).

## 6. Arrays

- Index origin 1 (matches axis subscripts starting at 1). OPEN until
  M3 tests pin it.
- Row-major dense arrays, shape `Vec<usize>`, scalars are rank 0.
- Scalar extension: scalar op array and array op scalar; equal-shape
  elementwise; otherwise a shape error.
- Empty arrays and reduce identities: `+^r` of empty = 0 etc. (pinned
  in M4).

### 6.1 Multi-axis rotate (M5 key question)

`-1 0 1 t_12 A` with A of shape `[n,m]` yields every rotation for the
Cartesian product of offsets over axes 1 and 2. Result shape options:
(a) rank-4 `[3,3,n,m]`, then Life reduces with `+^r_12`;
(b) nested `3x3` array of boards, then `+^r` reduces items.
The research's Life one-liner assumes a plain `+^r` sums all nine
boards. The M5 saga must pick one and pin it before M8; Life may then
need `+^r_12` in option (a).

### 6.2 The Life one-liner (D11, decided)

```
life = { (+^r -1 0 1 t_12 _r) { (_l = 3) + _r * _l = 4 } _r }
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
  `(1 and S=3) or (w and S=4)`. Note the ravel `,` before `+/`: a
  plain `+/` on the nested 3x3 would reduce only the last axis (see
  6.1 and D7).

History: the research's line `{ (+^r ... _r) = 3 + _r }` computed
`S = 3 + c`, which keeps a live cell only when N = 3; a blinker's
centre died. Verified with sw-apl: on the 5x5 blinker the old rule
gives row 3 = `0 1 0 1 0`, Conway gives `0 1 1 1 0`; the corrected
rule equals the APL\360 reference on the blinker, a glider (1, 2 and
4 generations) and a random 8x8 torus (1 and 3 generations).
Pinned by `spec/integration/life-blinker.case` (pending until M8).

The line relies on rules still to be pinned: an inline lambda used
dyadically (`X { ... } Y`), innermost-lambda scoping of `_l`/`_r`
(saga 1 parser/desugar), Bool results of `=` used in `+` and `*` (D5),
and `+^r` summing all nine boards (D7).

## 7. Evaluation

Strict, call-by-value in v0. OPEN (M6): whether to add laziness so a
genuine Y combinator works, or ship Z and a test that documents Y
diverges under strict evaluation.

The evaluator consumes Core only and emits a trace tree:
`NodeId, span, value, type, shape, children`.

## 8. Display modes (CLI and web)

| Mode      | Example (Life)                                              |
| --------- | ----------------------------------------------------------- |
| Raw       | the line in 6.2, exactly as typed                           |
| Decorated | same, with underline/subscript/superscript glyphs (Unicode) |
| Canonical | fully parenthesized raw form                                |
| Expanded  | long names: `reduce(add, rotate(axes=[1,2], ...))` etc.   |
| Core      | `Lam(r, App(App(Lam(l, Lam(r, ...)), ...), Var(r)))`        |

Source is raw ASCII only (D13); the decorated and LaTeX forms are
output. Editing always happens on raw text (prettify-style display,
never destructive substitution). `xetal render` prints the decorated
form, `xetal render --raw` converts it back (and lexes the result),
and `xetal render --latex` prints LaTeX math for a post-processor.

### 8.1 Decorated Unicode (`xetal render`)

Whitespace between tokens is copied verbatim; only names change.

| Raw           | Decorated                                            |
| ------------- | ---------------------------------------------------- |
| `r_`          | U+0332 COMBINING LOW LINE after each stem character   |
| `t_12`        | underlined stem, then subscript digits U+2081 U+2082 |
| `+_1`         | `+` then U+2081 (symbols are already functions)      |
| `+^r`, `f^s`  | stem, then the word in superscript letters           |
| `+^s_2`       | stem, superscript word, subscript digits             |
| `now_@`       | underlined stem immediately followed by `@`          |
| `+_@`         | underlined symbol immediately followed by `@`        |
| `f^e_@`       | stem, superscript word, immediately `@`              |
| `m.f_`        | `m.` then the decorated name                         |
| other tokens  | unchanged (`_l`, `_r`, `@`, numbers, brackets)       |

The underline is drawn only where it is what makes the name a
function (a named stem without a superscript, or any stem that is
niladic). `now_@` and `now_ @` differ only by the space, which is
preserved. Unicode has no subscript `@`, so niladic is shown as the
decorated name touching `@`.

Superscript letters (lowercase only), code points:

| a 1D43 | b 1D47 | c 1D9C | d 1D48 | e 1D49 | f 1DA0 | g 1D4D |
| ------ | ------ | ------ | ------ | ------ | ------ | ------ |
| h 02B0 | i 2071 | j 02B2 | k 1D4F | l 02E1 | m 1D50 | n 207F |
| o 1D52 | p 1D56 | q none | r 02B3 | s 02E2 | t 1D57 | u 1D58 |
| v 1D5B | w 02B7 | x 02E3 | y 02B8 | z 1DBB |        |        |

A name whose superscript word contains `q` or any uppercase letter
(no reliable superscript glyph) is shown in raw ASCII, so the inverse
is still exact: ASCII passes through `--raw` unchanged.

Inverse (`--raw`): an underlined run becomes `stem_`; subscript digits
become `_digits`; superscript letters become `^word`; `@` touching an
underlined or superscripted name becomes `_@`; any other non-ASCII
character is `not-decorated`, a stray underline is `bad-underline`.
Pinned by `crates/xetal-render/tests/render/` (proptest: raw ->
decorated -> raw is the identity on lexable sources) and
`spec/render/*.case`.

### 8.2 LaTeX (`xetal render --latex`)

One way, complete (LaTeX has every glyph): the body of a math
environment for KaTeX, MathJax or pdflatex. Each token is braced so
TeX adds no operator spacing; each source space is `\ ` and each
newline `\\`. Names: `\underline{\mathrm{t}}_{12}`, `+^{\mathrm{r}}`,
`\underline{\mathrm{now}}_{@}`; `*` is `\ast`, `|` is `\mid`, `{ }`
are escaped, `_l` is `\_\mathrm{l}`.

## 9. Open decisions register

| ID | Question                                  | Decide in   |
| -- | ----------------------------------------- | ----------- |
| D1 | Binding token: `name =` vs `<-` / `:=`    | Saga 1 parser |
| D2 | Newline as statement separator            | Saga 1 parser |
| D3 | `{ _l }` and `{ }` lambda semantics       | Saga 1 desugar |
| D4 | Sections: is `2 +` a partial app (`+_` is a lex error) | Saga 1 parser |
| D5 | Bool as Num                               | DIRECTION DECIDED (section 5): Bool type, implicit Bool <-> Int (1/0, other Int is an error); rule pinned in Saga 2 types |
| D6 | Index origin                              | Saga 3 arrays |
| D7 | Multi-axis rotate result shape            | Saga 5 rotate |
| D8 | Strict vs lazy; Y vs Z                    | Saga 6 combinators |
| D9 | Dyadic train forms                        | Saga 7 trains |
| D10| File extension (`.xtl` provisional)       | Saga 1 CLI |
| D11| Life one-liner rule                       | DECIDED: 6.2, `spec/integration/life-blinker.case` |
| D12| Applying a function-valued lambda argument (`_l` / `_r` used as a function, needed by S, B, C combinators) | Saga 6 combinators; lexer rejects `_r_` until then |
| D13| Accept decorated Unicode as input          | DECIDED: raw ASCII only (revisit much later); section 8 |
| D14| Namespaces as superscripts (user direction: user functions carry an explicit namespace such as `u`, libraries their own letter such as `c` for combinators, plus an `as`-style alias for clashes). Conflicts with superscript = derivation (`+^r`) and with the implemented dotted prefix `m.f_`; options to be put to the user | Before the Saga 1 parser step |

Each decision is recorded here and in the test that pins it
(test name or spec case referenced in the table when decided).
