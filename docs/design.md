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
which are deferred). Whitespace separates tokens; newlines act like
`;` at top level (to be pinned).

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
| `+^r`      | `Func(+, deriv=reduce)`                 |
| `+^r_2`    | `Func(+, deriv=reduce, axes=[2])`       |
| `m.f_`     | `Func(f, ns=m)`                         |

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
a digit, and preceded by start-of-input, whitespace, `(`, `{`, `[` or
`;`, lexes as part of a number. So `-1 0 1` is a 3-vector, `3 - 1` is
subtraction, `3 -1` is the 2-vector `3 -1`, and `3-1` is an error
(ambiguous; requires spaces). Each case is a pinned test.

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

OPEN: Bool as Num. Life adds Bool arrays (`+^r` over rotated boards)
and compares Int with Bool. Options: (a) Bool is a `Num` instance
(APL behavior), (b) explicit conversion, (c) boolean-to-int lifting
rule. Decided by tests in the types saga before M8.

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

### 6.2 The Life one-liner is not yet Conway's rule

Conway's rule, with N the count of the 8 neighbours and c the cell:
a cell is alive next generation iff `N = 3`, or `c` and `N = 2`.
With S = N + c (the 3x3 sum *including* the cell, which is what
summing the nine rotated boards gives) the same rule is
`(S = 3) or (c and S = 4)`.

Reference implementations (sw-apl, `samples/51-life.apl`), written
here in ASCII transliteration:

- APL\360 (eight explicit rotations): `(3=N) or B and 2=N`.
- The classic APL2 one-liner
  `life <- {disclose 1 w or.and 3 4 = +/ , -1 0 1 outer-rotate-first
  -1 0 1 rotate-each enclose w}`: `3 4 = S` gives the boards `S=3` and
  `S=4`, and the inner product `1 w or.and ...` combines them as
  `(1 and S=3) or (w and S=4)`. Note the ravel `,` before `+/`: a
  plain `+/` on the nested 3x3 would reduce only the last axis (see
  6.1 and D7).

The design target `life = { (+^r -1 0 1 t_12 _r) = 3 + _r }` computes
`S = 3 + c`: a live cell survives only when S = 4 (N = 3), so a live
cell with 2 neighbours dies. Checked with sw-apl on the 5x5 blinker:
Conway's rule turns the vertical bar into `0 1 1 1 0` on row 3, while
`S=3+B` gives `0 1 0 1 0` (the centre dies). The pending acceptance
case `spec/integration/life-blinker.case` expects Conway's result, so
the one-liner must change before M8 (it needs an `or` / `and` pair or
an equivalent). OPEN (D11).

## 7. Evaluation

Strict, call-by-value in v0. OPEN (M6): whether to add laziness so a
genuine Y combinator works, or ship Z and a test that documents Y
diverges under strict evaluation.

The evaluator consumes Core only and emits a trace tree:
`NodeId, span, value, type, shape, children`.

## 8. Display modes (CLI and web)

| Mode      | Example (Life)                                              |
| --------- | ----------------------------------------------------------- |
| Raw       | `life = { (+^r -1 0 1 t_12 _r) = 3 + _r }`                  |
| Decorated | same, with underline/subscript/superscript glyphs (Unicode) |
| Canonical | fully parenthesized raw form                                |
| Expanded  | long names: `reduce(add, rotate(axes=[1,2], ...)) == ...`  |
| Core      | `Lam(r, App(App(eq, ...), ...))`                            |

Decorated rendering uses Unicode combining low line (U+0332) for the
underline, subscript digits (U+2080..U+2089) and modifier letters for
superscripts; the renderer and its inverse are tested for lossless
round-trips. Editing always happens on raw text (prettify-style
display, never destructive substitution).

## 9. Open decisions register

| ID | Question                                  | Decide in   |
| -- | ----------------------------------------- | ----------- |
| D1 | Binding token: `name =` vs `<-` / `:=`    | Saga 1 parser |
| D2 | Newline as statement separator            | Saga 1 parser |
| D3 | `{ _l }` and `{ }` lambda semantics       | Saga 1 desugar |
| D4 | Sections: is `2 +_` / `2 +` a partial app | Saga 1 parser |
| D5 | Bool as Num                               | Saga 2 types |
| D6 | Index origin                              | Saga 3 arrays |
| D7 | Multi-axis rotate result shape            | Saga 5 rotate |
| D8 | Strict vs lazy; Y vs Z                    | Saga 6 combinators |
| D9 | Dyadic train forms                        | Saga 7 trains |
| D10| File extension (`.xtl` provisional)       | Saga 1 CLI |
| D11| Life one-liner rule (see 6.2)             | Saga 8 life |

Each decision is recorded here and in the test that pins it
(test name or spec case referenced in the table when decided).
