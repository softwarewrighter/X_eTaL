# Spatial equations: paper math to array code

A library of macros that takes a sum as an ML paper prints it and
gives, from one call, the array computation, its LaTeX, or both. It is
a library on the language as it is: no compiler change. Why the call
looks the way it does: `docs/eq-rationale.md`. The review behind it:
`docs/spatial-equations-review.md`.

## Directions

Each notation keeps its own reading direction, and the expansion is
where they meet:

- **Equations read inside out**, as the paper prints them: sums
  outermost first, the summand inside.
- **Pipelines read left to right**: initial data, then each transform
  in turn (`e:p_ipe<`, below).
- **X_eTaL reads right to left**: both lower to ordinary X_eTaL, which
  `xetal expand` shows.

## Where it lives

`demos/spatial/`: `Spatial.xtlm` (the macros), `Spatial.xtl` (run-time
helpers), the oracle and the demo programs. It is a demo library, not
a standard library; it may move to `lib/` once it has earned it.

## The call

```
"<result>[<free indices>] | <extents>" e:e_q< "<op> <index>:=<low>..<high>, ... | <summand>"
```

The CNN cross-correlation
`Y[o,h,w] = sum_{c=0}^{C-1} sum_{r=0}^{Kh-1} sum_{s=0}^{Kw-1} X[c,h+r,w+s] W[o,c,r,s]`:

```
Y := "Y[o,h,w] | O Ho Wo" e:e_q< "sum c:=0..C - 1, sum r:=0..Kh - 1, sum s:=0..Kw - 1 | X[c,h+r,w+s] * W[o,c,r,s]"
```

- **Left argument**: the result's name, its free indices in brackets,
  then `|` and one extent per free index, in order (explicit; extents
  are never inferred). The result has shape `O Ho Wo`.
- **Right argument**: the sums, comma-separated, outermost first, then
  `|` and the summand. Each sum is an operator word, an index, `:=`, an
  inclusive range `low..high`. `sum` first; `prod`, `max` and `min` as
  the library grows; mixed operators are a word per sum.
- **Expressions** inside the strings (bounds, extents, index
  arithmetic, the summand) are the library's own math notation, with
  math precedence: `^` binds tightest (right to left), then `*` and
  `/`, then `+` and `-` (left to right); parentheses group. `2*h+r`
  means `(2*h)+r` and `(y[i] - p[i])^2` is the squared difference. The
  macro parenthesizes fully when it lowers, so the X_eTaL it writes
  means the same under right-to-left order. Names other than the
  indices (`C`, `Kh`, `X`, `W`) are the caller's X_eTaL values.
- **Every reading rule uses all of its text or refuses it.** A group
  is one parenthesized term, a reference ends at its closing bracket,
  an atom is a name or a number, and only the grammar's characters may
  appear; anything else is `error[bad-equation]` naming the text. The
  rule came from three silent wrong answers found while the library
  was written about (2026-10-17): `(y[i] - p[i])^2` summed as `y - p`,
  `y[i]2` and `y[i] % 2` as `y`, with the LaTeX printing what was
  written. A reader that uses part of its input and drops the rest is
  the class to design against (docs/type-check-macros.md). A function
  in the summand (`exp(y[i])`) is not in the grammar yet: it is
  refused, and the function is applied to the array before the call.
- **Brackets** index arrays: `X[c,h+r,w+s]`, one index per axis, origin
  0 as in the paper (lowered to X_eTaL's origin 1).

## The macros

| Macro | Gives |
| ----- | ----- |
| `e:e_q<` | the value (the array) |
| `e:t_ex<` | the LaTeX, a string |
| `e:b_oth<` | the pair `(value, latex)` |

All three parse the same text the same way; the value and the LaTeX of
one call always describe the same equation.

## Lowering

- A coordinate grid over the free indices and the summed indices
  (`t_able`, `r_eshape`, `o_ffsets`), extents from the left argument
  and the ranges.
- Each array reference becomes a gather: the index expressions
  evaluated on the grid, combined by Horner's rule over the array's
  shape, then `s_elect` on its ravel.
- A bounds check per axis before each gather (below).
- One `'+ r_/_k` (or the operator's reduction) per sum, innermost
  first.
- Helpers are written into each expansion (inline), since an expansion
  cannot yet name its own library (candidate L1, deferred); fresh names
  come from `[]G_ENSYM`.

## Pipelines

A pipeline writes ML data flow left to right: the initial data, then
each transform in turn. It lives beside the equations in
`Spatial.xtlm`, and a stage may be an equation (`@ e:e_q< "..."` is just
a value).

```
Y := @ e:p_ipe< r"
      X
   |> W1 nn:d_ense it
   |> nn:r_elu
   |> W2 nn:d_ense it
   |> nn:s_oftmax"
```

Until raw strings land, the same call fits on one line with an
ordinary string: `@ e:p_ipe< "X |> W1 nn:d_ense it |> nn:r_elu |> ..."`.

- **The call**: `@ e:p_ipe< "<data> |> <stage> |> ..."`; the left side
  is `@`. The first stage is the data, any X_eTaL expression.
- **Separator**: `|>`, split outside brackets, braces and strings (it
  has no meaning in X_eTaL, so a stage never holds one).
- **Stages** are X_eTaL expressions, type-checked like any code and
  copied into the expansion as written (so an error in one is reported
  inside the string). A stage is either a function (a name, a train,
  a quoted lambda, `nn:r_elu`), applied to the previous value, or an
  expression naming the previous value `it` (an anaphor, `## binds:
  it`). Order is explicit for noncommutative stages: `it - m` or
  `m - it`. A stage that is neither (a value without `it`) is rejected
  at the argument; the first stage may not name `it`.
- **Names**: ` : name` at a stage's end (spaces around the colon, so it
  is never a namespace prefix) names its value; later stages use the
  name directly, so fan-out and joins need nothing more
  (`e / '+ r_/ e`). ` : (Q, K, V)` destructures a tuple. Names are
  local to the pipeline and each stage is evaluated once; a name bound
  twice is rejected.
- **Shapes**: an optional ` [N O Ho Wo]` after a name, extents being the
  caller's values, is a contract: a shape lint proves or refutes it
  before the program runs where the extents are known symbolically, and
  the macro emits a run-time check after the stage for what the lint
  cannot decide:
  `error[shape]: stage 3 (conv): shape 2 8 6 6, expected 2 8 4 4`.
- **Value**: the last stage's value. A trace (each stage's name, shape
  and a sample) and a drawing come with the report step.

Examples, each with its hand-written equivalent in
`docs/spatial-poc/pipes-oracle.xtl` (the pipeline must print exactly
what the equivalent prints):

| Example | Pipeline (one line) | Expected |
| ------- | ------------------- | -------- |
| Linear regression | `X \|> it '+ '* i_nner W : P \|> P + (s_hape P) r_eshape b` | `14 25` / `20 31` |
| Stable softmax | `z \|> 'm_ax r_/ it : m \|> e_xp z - m : e \|> e / '+ r_/ e` | `0.0900... 0.2447... 0.6652...` |
| Layer norm | `x \|> ('+ r_/ it) / f_loat t_ally it : mu \|> x - mu : c \|> ('+ r_/ c * c) / f_loat t_ally c : v \|> c / v ^ 0.5` | `-1.3416... -0.4472... 0.4472... 1.3416...` |
| MLP | `X \|> it '+ '* i_nner W1 \|> 0 m_ax it \|> it '+ '* i_nner W2` | `2` / `2` |
| Residual | `x : s \|> 2 * it \|> it + s` | `3.0 6.0 9.0 12.0` |

The residual shows why the direction matters: written by hand as
`2 * x + x` it is `2 * (x + x)` (right to left), not the residual.
Linear regression shows a fact the design must keep visible: adding a
bias vector to every row is explicit (`(s_hape P) r_eshape b`), as
X_eTaL has no implicit broadcasting.

## Errors

- **At the argument** (`[]R_EJECT`, at expansion): a malformed range, a
  sum without an operator word or `:=`, an empty region, an index bound
  twice, a summand index that is neither summed nor free, a free index
  without an extent, an extent count that differs from the free
  indices.
- **At run time**: an index outside its array is an error naming the
  array, the axis, the index and the expression it came from, never a
  silent read or wrap:
  `error[index]: X axis 2: index 5 is outside 0..3 (from h+r)`.
  Valid (unpadded) convolution never trips it. Padding is not in
  scope.

## Expected outputs

These outputs, from the proof of concept in `docs/spatial-poc/` (old
call syntax) checked against an independent oracle, become goldens when
the library is written. Index origin 0 in the strings; numbers are the
same in either syntax.

Fixture A, all ones: `X := 2 4 4 r_eshape 1`, `W := 1 2 2 2 r_eshape 1`:
shape `1 3 3`, every element `8`.

Fixture B: `X := 2 4 4 r_eshape r_ange 32`,
`W := 1 2 2 2 r_eshape r_ange 8`: shape `1 3 3`,

```
560 596 632
704 740 776
848 884 920
```

Fixture C (2 output channels, 3 input channels, a 2 by 3 kernel on 5
by 6 pictures): `X := 3 5 6 r_eshape r_ange 90`,
`W := 2 3 2 3 r_eshape 1 -2 3 0 4 -1 2`: shape `2 4 4`,

```
 520  536  552  568
 616  632  648  664
 712  728  744  760
 808  824  840  856

 728  748  768  788
 848  868  888  908
 968  988 1008 1028
1088 1108 1128 1148
```

The LaTeX of the CNN call:

```
Y_{o,h,w} = \sum_{c=0}^{C - 1} \sum_{r=0}^{Kh - 1} \sum_{s=0}^{Kw - 1} X_{c,h+r,w+s} \, W_{o,c,r,s}
```

## The language as it is

What the research assumed and the language refuses is pinned by the
spec cases in `spec/spatial/`: a macro name needs its underline
(`e:e_q<`, not `e:q<`), reduce is `'+ r_/` (not `+/`), brackets are not
indexing, `C-1` is ambiguous (write `C - 1`), `~` is not quotation, and
a macro call is never another call's argument. Inside the equation
strings these are the library's notation, not X_eTaL.

## Not in scope

- Padding, strides and dilation (a later operator word or option).
- Inferred extents and typed domains. Static shape checks come from
  the planned shape lint (docs/type-check-macros.md), not the macros.
- Punctuation delimiters
  (L5), typed macro arguments (L6), drawing the call itself as math in
  the renderers (L7), choose indexing (L3), an expansion naming its
  own library (L1). Each is reviewed with a failing snippet and decided
  with the user, never assumed.
- A stepping debugger; inspection is by `xetal expand` and an explain
  macro.
