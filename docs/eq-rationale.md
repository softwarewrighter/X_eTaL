# Writing a spatial equation: the approaches and the choice

The spatial-equations library turns a sum as a paper prints it into
array code and LaTeX from one macro call. This page compares the ways
the call could be written and says why the chosen one was picked. The
running example is the 2-D CNN cross-correlation, a triple sum:

```
Y[o,h,w] = sum_{c=0}^{C-1} sum_{r=0}^{Kh-1} sum_{s=0}^{Kw-1} X[c,h+r,w+s] W[o,c,r,s]
```

## Constraints every approach meets

- A macro call is `"left" n_ame< "right"`: a string (or `@`) on each
  side, nothing else (MC3, MC17, MC22). A call cannot take another
  call as an argument, so nested macros are text inside one string.
- A macro argument is one string literal on one line; multi-line
  arguments would need raw strings (candidate L4, undecided).
- The text inside the strings is the library's own small language: it
  is parsed by the macro, not by X_eTaL, so it may use `..`, `|` and
  math precedence without any change to the compiler.
- The left argument names the result and its extents:
  `"Y[o,h,w] | O Ho Wo"` (explicit extents, decided).

## Criteria

1. **Locality**: can a reader see which bound goes with which index
   without counting?
2. **Mixed operators**: a max of sums, a product of sums.
3. **Error modes**: what a plausible slip does (a silent wrong answer
   is the worst outcome).
4. **Closeness to the paper**: how directly it maps to `\sum_{c=0}^{C-1}`.
5. **Cost**: works as X_eTaL is, with no compiler change.

## The approaches

### A. Positional regions, operator in the macro name

```
Y := "Y[o,h,w] | O Ho Wo" e:s_um< "C - 1, Kh - 1, Kw - 1 | X[c,h+r,w+s] * W[o,c,r,s] | c:=0, r:=0, s:=0"
```

Upper bounds, summand and lower bindings, in the spatial order of the
printed formula; the n-th bound pairs with the n-th binding.

- Pros: mirrors the printed layout (above, center, below); one
  operator per macro is simple to explain.
- Cons: pairing by position, so three sums mean counting; swapping two
  bounds (`Kh - 1, C - 1, ...`) gives a wrong answer or an index error
  far from the slip; mixed operators need a macro per combination or
  nesting, which a call cannot do.

### B. Positional regions, operators in the center

```
Y := "Y[o,h,w] | O Ho Wo" e:q_< "C - 1, Kh - 1, Kw - 1 | sum sum sum X[c,h+r,w+s] * W[o,c,r,s] | c:=0, r:=0, s:=0"
```

- Pros: one macro for every operator; mixed nests (`max sum`) fit;
  the center reads like the paper's run of sigmas.
- Cons: three lists (operators, bounds, bindings) still pair by
  position; the count must match in three places; the same swapped
  bound slip as A.

### C. Per-sum bindings (chosen)

```
Y := "Y[o,h,w] | O Ho Wo" e:q_< "sum c:=0..C - 1, sum r:=0..Kh - 1, sum s:=0..Kw - 1 | X[c,h+r,w+s] * W[o,c,r,s]"
```

Each sum carries its own operator, index, lower and upper bound,
outermost first, then the summand after `|`.

- Pros: each sum is self-contained, as `\sum_{c=0}^{C-1}` is; nothing
  is counted or paired; mixed operators are a word per sum
  (`max r:=0..Kh - 1, sum c:=0..C - 1`); a slip stays local (a bound
  written on the wrong index is visibly wrong where it is written);
  two regions instead of three make the parse simpler.
- Cons: not the printed spatial layout (bounds sit beside the index,
  not above it); a new `..` range notation to learn; longer than A for
  one sum.

### D. Nested macro calls, one per sum

```
Y := ... e:s_um< (... e:s_um< (... e:s_um< "X[c,h+r,w+s] * W[o,c,r,s]"))
```

- Refused today (a call as a macro argument is `error[bad-macro-call]`),
  and as text inside strings it needs escaped quotes at each level.
  Unreadable for exactly the triple sums that matter.

### E. Labeled regions

```
"upto: C - 1, Kh - 1, Kw - 1; of: X[c,h+r,w+s] * W[o,c,r,s]; from: c:=0, r:=0, s:=0"
```

- Pros: order-free, self-describing regions.
- Cons: still pairs bounds with bindings by position inside each
  region; longer; `u:` or `l:` as labels would read as the private-name
  prefixes, so the labels must be words.

### F. Operator prefix on each binding

```
"C - 1, Kh - 1, Kw - 1 | X[c,h+r,w+s] * W[o,c,r,s] | +c:=0, +r:=0, +s:=0"
```

- Pros: mixed operators in one call.
- Cons: symbols for operators (`+`, `*`, `|`) are cryptic and clash
  with the summand's own arithmetic; bounds still pair by position.

### G. Two-dimensional punctuation (the research's `|~ upper | center | lower _|`)

- Pros: closest to the printed layout.
- Cons: needs new tokens (candidate L5) and, to stack regions on
  lines, multi-line arguments (L4): a compiler change, out of scope for
  a library.

## Comparison

| Approach | Locality | Mixed operators | Slip is local | Close to paper | No compiler change |
| -------- | -------- | --------------- | ------------- | -------------- | ------------------ |
| A positional, name | no | no | no | layout | yes |
| B positional, center | no | yes | no | layout | yes |
| C per-sum (chosen) | yes | yes | yes | notation | yes |
| D nested calls | yes | yes | yes | notation | no (refused) |
| E labeled regions | no | no | no | partly | yes |
| F operator prefix | no | yes | no | no | yes |
| G punctuation | no | yes | no | layout | no |

## Rationale

The common case in ML papers is not one sum but two or three (a
convolution, attention, a batched matrix product), so the form must
stay readable at three sums. Approaches A, B, E and F all keep the
bounds and the indices in separate lists paired by position; at three
sums the reader has to count, and the writer's most likely slip,
putting bounds in the wrong order, produces a wrong number or an index
error with no hint of where. Approach C attaches each bound to its
index, which is also how the paper's notation attaches a subscript and
superscript to each sigma: it gives up the printed vertical layout but
keeps the notation's meaning, and the LaTeX the macro writes from it
restores the layout on the page. It needs no compiler change, handles
mixed operators with one macro, and has the simplest grammar of the
string forms (sums, `|`, summand). D and G read well but are not
available without language changes (a call as a macro argument; new
punctuation and multi-line arguments); they remain candidates for the
limitations review, not prerequisites.

## The chosen form, in brief

```
"<result>[<free indices>] | <extents>" e:q_< "<op> <index>:=<low>..<high>, ... | <summand>"
```

- `<op>` is a word: `sum` first; others (`prod`, `max`, `min`) as the
  library grows. Sums are listed outermost first.
- Bounds are inclusive, as in the paper's `\sum_{c=0}^{C-1}`.
- The summand indexes arrays with brackets, `X[c,h+r,w+s]`; it is the
  library's notation, lowered to ordinary X_eTaL array code.
- Errors are reported at the argument: an index bound twice, a summand
  index with no sum and no extent, an empty region, a malformed range.

## Pipelines: the choices

Equations read inside out; pipelines read left to right, initial data
first (`e:p_ipe<`, docs/spatial.md). The input was the ML pipelines
design proposal (`docs/xetal-ml-pipelines-design.md`); each choice
below was made with the user.

### What a stage is

- **X_eTaL expressions (chosen)**: any function, built-in or library
  (X_eTaL-ML's `nn:r_elu`, `nn:d_ense`); type-checked like all code,
  copied into the expansion as written, so errors point inside the
  string. One vocabulary.
- An ML call vocabulary (`dense(W1,b1)`, `windows(Kh,Kw,stride=1)`):
  familiar to ML readers, but a second language whose names, arity and
  errors the macro must translate and keep in step with the libraries.
- Both: the cost of the second with little gain.

### Where the previous value goes

- **`it`, or a bare function (chosen)**: a function stage is applied
  to the previous value; any other stage names it `it`. Noncommutative
  order is written (`it - m`, `m - it`), never guessed.
- Appending it on the right: unsafe in X_eTaL, where juxtaposed values
  are a strand (`W1 nn:d_ense b1` followed by `X` reads `b1 X`).
- Always `it`: uniform but longer for the common `|> nn:r_elu`.
- Always a function: dyadic stages become lambdas,
  `{ x -> x - m }` instead of `it - m`.

### The separator

- **`|>` (chosen)**: F#'s and Elixir's pipe; meaningless in X_eTaL, so
  a stage never contains one.
- `->`: reads as flow, but is also the lambda arrow inside a stage.
- `;`: reads as sequencing, not flow.

### Naming intermediates

- **` : name` (chosen)**: as in the design proposal; later stages use
  the name directly, so the proposal's `from(name)` is unnecessary.
  Spaces around the colon keep it apart from namespace prefixes.
- `as name`: a reserved word inside the strings.
- `name := stage`: X_eTaL's binding, but it reads right to left
  inside a left-to-right pipeline.

### The initial data

- **The first stage, the left argument `@` (chosen)**: the whole flow
  in one string, read top to bottom in a raw string.
- The left argument: input and transforms in two places.
- Either: two ways to write the same thing.

### Layout: one stage per line

A vertical column of `|>` needs a string that spans lines, which ST2
forbids. Chosen: raw strings, `r"..."` and `r#"..."#` with Rust's
rules (contents verbatim, newlines kept), anywhere a string may stand.
A raw string is another spelling of the same Char vector, not a new
string type, so types, built-ins and the evaluator are untouched.
Rejected: display-only vertical layout (the source would still be one
line); splitting a pipeline into named pieces per line (works today,
kept as a fallback); escaped multi-line strings (LaTeX's backslashes
would double).

### Shapes

- **Optional annotations as a contract (chosen)**: a shape lint proves
  or refutes them before the program runs where extents are known
  symbolically; a run-time check after the stage catches the rest.
  Preventable run-time errors should be prevented.
- Display only: annotations that can drift from the truth.
- Lint only: data-dependent sizes would pass unchecked.
