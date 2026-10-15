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
  in turn (a companion macro, designed separately).
- **X_eTaL reads right to left**: both lower to ordinary X_eTaL, which
  `xetal expand` shows.

## Where it lives

`demos/spatial/`: `Spatial.xtlm` (the macros), `Spatial.xtl` (run-time
helpers), the oracle and the demo programs. It is a demo library, not
a standard library; it may move to `lib/` once it has earned it.

## The call

```
"<result>[<free indices>] | <extents>" e:q_< "<op> <index>:=<low>..<high>, ... | <summand>"
```

The CNN cross-correlation
`Y[o,h,w] = sum_{c=0}^{C-1} sum_{r=0}^{Kh-1} sum_{s=0}^{Kw-1} X[c,h+r,w+s] W[o,c,r,s]`:

```
Y := "Y[o,h,w] | O Ho Wo" e:q_< "sum c:=0..C - 1, sum r:=0..Kh - 1, sum s:=0..Kw - 1 | X[c,h+r,w+s] * W[o,c,r,s]"
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
  math precedence: `*` binds tighter than `+` and `-`, which group left
  to right; parentheses group. `2*h+r` means `(2*h)+r`. The macro
  parenthesizes fully when it lowers, so the X_eTaL it writes means the
  same under right-to-left order. Names other than the indices
  (`C`, `Kh`, `X`, `W`) are the caller's X_eTaL values.
- **Brackets** index arrays: `X[c,h+r,w+s]`, one index per axis, origin
  0 as in the paper (lowered to X_eTaL's origin 1).

## The macros

| Macro | Gives |
| ----- | ----- |
| `e:q_<` | the value (the array) |
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
(`e:q_<`, not `e:q<`), reduce is `'+ r_/` (not `+/`), brackets are not
indexing, `C-1` is ambiguous (write `C - 1`), `~` is not quotation, and
a macro call is never another call's argument. Inside the equation
strings these are the library's notation, not X_eTaL.

## Not in scope

- Padding, strides and dilation (a later operator word or option).
- Inferred extents, typed domains and static shape checks.
- Multi-line calls (needs raw strings, L4), punctuation delimiters
  (L5), typed macro arguments (L6), drawing the call itself as math in
  the renderers (L7), choose indexing (L3), an expansion naming its
  own library (L1). Each is reviewed with a failing snippet and decided
  with the user, never assumed.
- A stepping debugger; inspection is by `xetal expand` and an explain
  macro.
