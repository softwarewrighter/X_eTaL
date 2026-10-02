# Built-in functions

Every built-in function, what it does and how it is used. Each
example is shown as in a session, the code indented six spaces
and its result under it; every one was run by `xetal`
(`scripts/reference.py` writes this page from
`docs/reference/builtins.ref` and the built-in catalog, and the
gate checks it is current). The examples use three arrays:

```
      v := 3 1 2
      M := 2 3 r_eshape r_ange 6
      N := 2 3 r_eshape 3 1 2 6 4 5
```

`M` has two rows, 1 2 3 and 4 5 6: axis 1 (down the rows) has
length 2, axis 2 (along a row) length 3. A subscript after a
function's name picks the axis it works along: `'+ r_/ M` works
along axis 1, the default; `'+ r_/_1 M` writes that axis out; and
`'+ r_/_2 M` works along axis 2 instead.

## Arithmetic

Arithmetic works item by item on arrays of any shape; a single number
on either side extends to every item of the other. Int and Float mix
as the literals allow; division always gives a Float.

### `+`

`Num a => a -> a -> a`, two arguments.

Plus.

```
      1 + 2
3
      1 2 3 + 10
11 12 13
      M + 100
101 102 103
104 105 106
```

### `-`

`Num a => a -> a -> a`, two arguments.

Minus. Written spaced (`3 - 1`): a minus touching a number makes it
negative.

```
      10 - 3
7
      v - 1
2 0 1
```

### `*`

`Num a => a -> a -> a`, two arguments.

Times, drawn as the times sign.

```
      6 * 7
42
      v * v
9 1 4
```

### `/`

`Num a => a -> a -> Float`, two arguments.

Divide, drawn as the division sign; the result is a Float.

```
      7 / 2
3.5
      1 2 3 / 2
0.5 1.0 1.5
```

### `^`

`Num a => a -> a -> a`, two arguments.

Power, spaced: `x ^ n`. An exponent written touching a value (`x^2`)
is a literal superscript instead; touching a function it is function
power (see p_ower).

```
      2 ^ 10
1024
      v ^ 2
9 1 4
```

### `m_ax`

`Num a => a -> a -> a`, two arguments.

The larger of two numbers; `'m_ax r_/` is the largest item.

```
      3 m_ax 5
5
      'm_ax r_/ v
3
```

### `m_in`

`Num a => a -> a -> a`, two arguments.

The smaller of two numbers.

```
      3 m_in 5
3
      2 m_in v
2 1 2
```

### `d_iv`

`Int -> Int -> Int`, two arguments.

Whole-number division, in maths order: `7 d_iv 2` is 3.

```
      7 d_iv 2
3
```

### `m_od`

`Int -> Int -> Int`, two arguments.

Remainder, in maths order: `7 m_od 3` is 1 (the left argument is
divided by the right).

```
      7 m_od 3
1
      (r_ange 6) m_od 2
1 0 1 0 1 0
```

### `n_eg`

`Num a => a -> a`, one argument.

Negate.

```
      n_eg v
-3 -1 -2
```

### `a_bs`

`Num a => a -> a`, one argument.

Absolute value.

```
      a_bs -3 4 -5
3 4 5
```

### `f_loor`

`Num a => a -> Int`, one argument.

Round down to an Int.

```
      f_loor 2.5 -2.5
2 -3
```

### `c_eiling`

`Num a => a -> Int`, one argument.

Round up to an Int.

```
      c_eiling 2.5 -2.5
3 -2
```

### `e_xp`

`Num a => a -> Float`, one argument.

e to a power.

```
      e_xp 1
2.718281828459045
```

### `l_og`

`Num a => a -> Float`, one argument.

Natural logarithm.

```
      l_og e_xp 2
2.0
```

### `f_loat`

`Num a => a -> Float`, one argument.

An Int as a Float.

```
      f_loat 3
3.0
```

## Trigonometry

Angles are in radians. Each gives a Float, item by item, for any
number.

### `s_in`

`Num a => a -> Float`, one argument.

Sine.

```
      s_in 0 1
0.0 0.8414709848078965
```

### `c_os`

`Num a => a -> Float`, one argument.

Cosine.

```
      c_os p_i @
-1.0
```

### `a_tan`

`Num a => a -> Float`, one argument.

Arctangent, the angle whose tangent is the argument.

```
      4 * a_tan 1
3.141592653589793
```

### `p_i`

`Unit -> Float`, one argument.

The constant pi. It is niladic, so it is applied to `@`.

```
      p_i @
3.141592653589793
      2 * p_i @
6.283185307179586
```

## Comparisons and logic

Comparisons give 1 for true and 0 for false, item by item. Everything
reads right to left with no precedence, so the left side of `&` or
`|` needs parentheses when it is itself an expression.

### `=`

`(Eq a, Truthy b) => a -> a -> b`, two arguments.

Equal (any scalar type).

```
      3 = 3
1
      v = 1
0 1 0
```

### `!=`

`(Eq a, Truthy b) => a -> a -> b`, two arguments.

Not equal.

```
      v != 1
1 0 1
```

### `<`

`(Ord a, Truthy b) => a -> a -> b`, two arguments.

Less than (numbers and characters).

```
      v < 2
0 1 0
```

### `>`

`(Ord a, Truthy b) => a -> a -> b`, two arguments.

Greater than.

```
      v > 2
1 0 0
```

### `<=`

`(Ord a, Truthy b) => a -> a -> b`, two arguments.

At most.

```
      v <= 2
0 1 1
```

### `>=`

`(Ord a, Truthy b) => a -> a -> b`, two arguments.

At least.

```
      v >= 2
1 0 1
```

### `e_q~`

`(Num a, Truthy b) => a -> a -> b`, two arguments.

Equal within a small tolerance, for Floats.

```
      (0.1 + 0.2) e_q~ 0.3
1
```

### `&`

`(Truthy a, Truthy b) => a -> a -> b`, two arguments.

And.

```
      1 1 0 & 1 0 0
1 0 0
```

### `|`

`(Truthy a, Truthy b) => a -> a -> b`, two arguments.

Or.

```
      1 1 0 | 1 0 0
1 1 0
```

### `n_ot`

`(Truthy a, Truthy b) => a -> b`, one argument.

Not.

```
      n_ot 1 0
0 1
```

## Structure

The structural functions work along the first axis unless given
another: a subscript after the name picks the axis (`t_ake_2`), by
moving it to the front, applying the function and moving it back.
In the examples, `M` is `2 3 r_eshape r_ange 6`, two rows 1 2 3 and
4 5 6: axis 1 has length 2, axis 2 length 3.

### `s_hape`

`a -> Int`, one argument.

The length of each axis.

```
      s_hape v
3
      s_hape M
2 3
```

### `t_ally`

`a -> Int`, one argument.

How many items along the first axis: the rows of a matrix. With a
subscript it counts along that axis only when the result keeps a rank
the rule allows: `t_ally_2` would drop two ranks, an error.

```
      t_ally v
3
      t_ally M
2
      t_ally_1 M
2
      t_ally_2 M
error[axis]: a function under an axis subscript changed the rank from 2 to 0
```

### `r_ange`

`Int -> Int`, one argument.

The numbers 1 to n.

```
      r_ange 5
1 2 3 4 5
```

### `o_ffsets`

`Int -> Int`, one argument.

The numbers 0 to n - 1.

```
      o_ffsets 5
0 1 2 3 4
```

### `r_eshape`

`Int -> a -> a`, two arguments.

The items of the right argument, taken in order and repeated as
needed, into the shape on the left.

```
      2 3 r_eshape r_ange 6
1 2 3
4 5 6
      2 2 r_eshape 7
7 7
7 7
```

### `r_avel`

`a -> a`, one argument.

Every item as one vector, row after row. With axis 2, column after
column.

```
      r_avel M
1 2 3 4 5 6
      r_avel_2 M
1 4 2 5 3 6
```

### `f_irst`

`a -> a`, one argument.

The first item along the first axis: the first row. With a subscript,
along that axis: `f_irst_2` is the first column.

```
      f_irst v
3
      f_irst M
1 2 3
      f_irst_1 M
1 2 3
      f_irst_2 M
1 4
```

### `t_ake`

`Int -> a -> a`, two arguments.

The first n items (the last n for a negative n) along the first axis,
or along the subscript's axis.

```
      2 t_ake v
3 1
      -2 t_ake v
1 2
      1 t_ake M
1 2 3
      1 t_ake_1 M
1 2 3
      1 t_ake_2 M
1
4
```

### `d_rop`

`Int -> a -> a`, two arguments.

All but the first n items (the last n for a negative n), along the
first axis or the subscript's.

```
      1 d_rop v
1 2
      1 d_rop M
4 5 6
      1 d_rop_1 M
4 5 6
      1 d_rop_2 M
2 3
5 6
```

### `s_elect`

`Int -> a -> a`, two arguments.

The items at the given positions (from 1) along the first axis, or
along the subscript's axis.

```
      3 1 s_elect v
2 3
      2 s_elect M
4 5 6
      2 s_elect_1 M
4 5 6
      2 s_elect_2 M
2 5
```

### `r_eplicate`

`Truthy a => a -> b -> b`, two arguments.

Replicate: each item (each row of a matrix) repeated as many times as
its count on the left, so a 0 drops it and a mask of 1s and 0s keeps
the items where it is 1; one count extends to every item, and with a
subscript the counts go along that axis. A negative count is an error.

```
      1 0 2 r_eplicate "abc"
acc
      (v > 1) r_eplicate v
3 2
      2 r_eplicate v
3 3 1 1 2 2
      0 2 r_eplicate M
4 5 6
4 5 6
      1 0 2 r_eplicate_2 M
1 3 3
4 6 6
      1 -1 2 r_eplicate v
error[domain]: a count cannot be -1
```

### `e_ncode`

`Int -> Int -> Int`, two arguments.

Encode: the digits of a number in the radix on the left, most
significant first (APL's encode). Radixes may differ from digit to
digit (hours, minutes, seconds), a radix of 0 takes all that is left,
and digits beyond the radix are dropped. A vector gives one column of
digits per item.

```
      2 2 2 2 e_ncode 11
1 0 1 1
      24 60 60 e_ncode 3725
1 2 5
      0 10 e_ncode 123
12 3
      2 2 2 e_ncode 0 1 2 3
0 0 0 0
0 0 1 1
0 1 0 1
      2 2 e_ncode M
error[rank]: e_ncode needs a scalar or a vector, got shape 2 3
```

### `d_ecode`

`Int -> Int -> Int`, two arguments.

Decode: the number whose digits, in the radix on the left, are on the
right (APL's decode), the inverse of `e_ncode`. One radix extends to
every digit; a matrix gives one number per column.

```
      2 d_ecode 1 0 1 1
11
      10 d_ecode 1 2 3
123
      24 60 60 d_ecode 1 2 5
3725
      2 d_ecode 2 2 2 e_ncode 0 1 2 3
0 1 2 3
      2 2 d_ecode 1 0 1
error[length-mismatch]: 2 radix values for 3 digits
```

### `e_nclose`

`a -> Box a`, one argument.

Enclose: the whole value as one item, a box, so arrays can be items of
other arrays; a strand of strings encloses each string. A nested array
prints in frames, as APL2's DISPLAY draws them (shown here as `xetal
--ascii` draws them; a terminal gets box characters): an arrow along
the top, a down arrow for each leading axis, and a mark at the bottom
for what it holds (`~` numbers, `e` boxes).

```
      e_nclose "abc"
.-------.
| .>--. |
| |abc| |
| '---' |
'e------'
      "ab" "cde"
.>-----------.
| .>-. .>--. |
| |ab| |cde| |
| '--' '---' |
'e-----------'
      (e_nclose v) c_at e_nclose 1 2
.>--------------.
| .>----. .>--. |
| |3 1 2| |1 2| |
| '~----' '~--' |
'e--------------'
      t_ally "ab" "cde"
2
```

### `d_isclose`

`Box a -> a`, one argument.

Disclose: what a box holds. It opens one box (select one item first).

```
      d_isclose e_nclose "abc"
abc
      d_isclose 2 s_elect "ab" "cde"
cde
      d_isclose "ab" "cde"
error[rank]: d_isclose opens one box, got shape 2
```

### `d_isplay`

`a -> Char`, one argument.

Display: any value as APL2's DISPLAY draws it, as a character matrix,
flat arrays framed too: an arrow along the top, a down arrow for each
leading axis, and a mark at the bottom for what it holds (`~` numbers,
a plain line characters, `e` boxes); a simple scalar is itself. `xetal
--box` prints every array result this way (shown here as `xetal
--ascii` draws them).

```
      d_isplay M
.>----.
v1 2 3|
|4 5 6|
'~----'
      d_isplay "abc"
.>--.
|abc|
'---'
      d_isplay "ab" "cde"
.>-----------.
| .>-. .>--. |
| |ab| |cde| |
| '--' '---' |
'e-----------'
      s_hape d_isplay v
3 7
      d_isplay 5
5
```

### `p_artition`

`Truthy a => a -> b -> Box b`, two arguments.

Partition: the items (rows of a matrix) cut into pieces by the keys on
the left, one per item: a new piece starts where the key goes up, and
an item with key 0 is left out, so a mask of 1s and 0s cuts out the
runs of 1s. Each piece is a box; one key extends to every item.

```
      t_ally (1 1 0 1 1 1 0 1) p_artition "ab cde f"
3
      d_isclose 2 s_elect (1 1 0 1 1 1 0 1) p_artition "ab cde f"
cde
      '{ t_ally d_isclose _r } e_ach ("a bb  ccc" != f_irst " ") p_artition "a bb  ccc"
1 2 3
      1 1 p_artition 1 2 3
error[length-mismatch]: 2 keys for 3 cells
```

### `c_at`

`a -> a -> a`, two arguments.

Join along the first axis, or with a subscript along that axis of both
arguments: `c_at_2` puts matrices side by side. The other axes must
match; a single value extends, and an argument of one rank less is one
row (or, along axis 2, one column).

```
      1 2 c_at 3 4
1 2 3 4
      M c_at M
1 2 3
4 5 6
1 2 3
4 5 6
      M c_at_2 M
1 2 3 1 2 3
4 5 6 4 5 6
      M c_at_2 0 9
1 2 3 0
4 5 6 9
      M c_at_2 1 2 3
error[shape-mismatch]: shapes differ: 2 3 and 3
```

## Reduce, scan, each and table

These take a function as an operand, written quoted before them
(`'+ r_/`): the nearest quoted function is their first argument.

### `r_/`

`(a -> a -> a) -> a -> a`, two arguments.

Reduce: combine the items along the first axis with the function, from
the right (`'- r_/ 1 2 3` is 1 - (2 - 3)). On a matrix that combines
the rows. A subscript picks the axis; two digits reduce both axes.

```
      '+ r_/ v
6
      '- r_/ 1 2 3
2
      '+ r_/ M
5 7 9
      '+ r_/_1 M
5 7 9
      '+ r_/_2 M
6 15
      '+ r_/_12 M
21
      'm_ax r_/_2 M
3 6
```

### `s_\`

`(a -> a -> a) -> a -> a`, two arguments.

Scan: every running reduce, the last being the whole reduce. On a
matrix down the rows; with a subscript along that axis.

```
      '+ s_\ v
3 4 6
      '+ s_\ M
1 2 3
5 7 9
      '+ s_\_1 M
1 2 3
5 7 9
      '+ s_\_2 M
1 3  6
4 9 15
```

### `e_ach`

`(a -> b) -> a -> b`, two arguments.

Apply a function to every item. Given a dyadic function it pairs items
of two arrays.

```
      'n_eg e_ach v
-3 -1 -2
      '{ _r * 10 } e_ach v
30 10 20
      1 2 3 '+ e_ach 10 20 30
11 22 33
```

### `m_ap`

`(a -> b) -> a -> Box b`, two arguments.

Map: the function on each item, every result boxed, so the function may
give an array (e_ach wants one value per item).

```
      t_ally 'r_ange m_ap 1 2 3
3
      d_isclose 3 s_elect 'r_ange m_ap 1 2 3
1 2 3
      '{ t_ally d_isclose _r } e_ach 'r_ange m_ap 1 2 3
1 2 3
```

### `t_able`

`(a -> b -> c) -> a -> b -> c`, three arguments.

Every pairing of an item on the left with one on the right (APL's
outer product): the result has both shapes.

```
      1 2 3 '* t_able 1 2 3
1 2 3
2 4 6
3 6 9
```

### `i_nner`

`(a -> b -> c) -> (c -> c -> c) -> a -> b -> c`, four arguments.

Inner product: `A '+ '* i_nner B` is matrix multiplication (the last
axis of A with the first of B).

```
      (2 2 r_eshape 1 2 3 4) '+ '* i_nner 2 2 r_eshape 1 2 3 4
 7 10
15 22
```

### `c_ompose`

`(a -> b) -> (b -> c) -> a -> c`, three arguments.

`'f 'g c_ompose x` is f applied to g applied to x.

```
      'n_eg 'a_bs c_ompose -5
-5
```

### `s_wap`

`(a -> b -> c) -> b -> a -> c`, three arguments.

`x 'f s_wap y` is `y f x`: the arguments swapped.

```
      10 '- s_wap 3
-7
```

### `p_ower`

`(a -> a) -> Int -> a -> a`, three arguments.

`n 'f p_ower x` applies f to x n times; `f_^n` is the same with a
literal count.

```
      3 'n_eg p_ower 5
-5
      n_eg^3 5
-5
```

## Search and order

These work on the items along the first axis (the rows of a matrix).

### `i_ndexOf`

`Eq a => a -> a -> Int`, two arguments.

Where each item of the right argument is first found among the items
of the left, from 1; one past the end when it is not there.

```
      5 6 7 i_ndexOf 7 9
3 4
      M i_ndexOf 4 5 6
2
```

### `m_ember?`

`(Eq a, Truthy b) => a -> a -> b`, two arguments.

Whether each item of the left is among the items of the right.

```
      2 9 m_ember? 1 2 3
1 0
```

### `m_atch`

`(Eq a, Truthy b) => a -> a -> b`, two arguments.

Whether both sides have the same shape and equal items (APL's match):
one result for the whole arrays, where `=` compares item by item. The
same items in another shape do not match.

```
      1 2 3 m_atch 1 2 3
1
      1 2 3 m_atch 1 2 4
0
      M m_atch M
1
      M m_atch 1 2 3 4 5 6
0
      1 m_atch 1 s_elect 1 2 3
1
```

### `u_nique`

`Eq a => a -> a`, one argument.

The items without repeats, in first-seen order.

```
      u_nique 3 1 3 2 1
3 1 2
```

### `s_ort`

`Ord a => a -> a`, one argument.

The items in ascending order (the rows of a matrix, compared in
order); with a subscript, sorted along that axis.

```
      s_ort 3 1 2
1 2 3
      s_ort N
3 1 2
6 4 5
      s_ort_2 N
1 2 3
4 5 6
```

### `g_rade`

`Ord a => a -> Int`, one argument.

The positions that would sort the items: `(g_rade v) s_elect v` is
`s_ort v`.

```
      g_rade 3 1 2
2 3 1
      g_rade_2 N
2 3 1
```

### `w_here`

`Truthy a => a -> Int`, one argument.

The positions of the 1s in a vector.

```
      w_here 0 1 1 0
2 3
      w_here_2 M
error[rank]: w_here needs a vector (nested arrays come later), got shape 3 2
```

## Rotate and reverse

### `o_-`

`Int -> a -> a`, two arguments.

Rotate: shift the items round by n (APL's rotate); positive n moves
items toward the front. Along the first axis by default, so a matrix's
rows move; with a subscript along that axis. A list of amounts gives
one result per amount.

```
      1 o_- v
1 2 3
      -1 o_- v
2 3 1
      1 o_- M
4 5 6
1 2 3
      1 o_-_1 M
4 5 6
1 2 3
      1 o_-_2 M
2 3 1
5 6 4
      -1 0 1 o_- v
2 3 1
3 1 2
1 2 3
```

### `r_ev`

`a -> a`, one argument.

Reverse the order of the items along the first axis (the rows of a
matrix), or along the subscript's axis.

```
      r_ev v
2 1 3
      r_ev M
4 5 6
1 2 3
      r_ev_1 M
4 5 6
1 2 3
      r_ev_2 M
3 2 1
6 5 4
```

## Effects, identity, text and files

### `p_rint!`

`a -> a`, one argument.

Print a value and give it back (the ! marks an effect).

```
      p_rint! v
3 1 2
3 1 2
```

### `r_oll!`

`Int -> Int`, one argument.

A random whole number from 1 to n for every n (seeded here, so the
example repeats).

```
      r_oll! 6 6 6
6 2 1
```

### `i_d`

`a -> a`, one argument.

The value itself.

```
      i_d v
3 1 2
```

### `l_eft`

`a -> b -> a`, two arguments.

The left argument.

```
      1 l_eft 2
1
```

### `r_ight`

`a -> b -> b`, two arguments.

The right argument.

```
      1 r_ight 2
2
```

### `f_ormat`

`a -> Char`, one argument.

A value as the text it prints as.

```
      f_ormat 3.5
3.5
      t_ally f_ormat 1 2 3
5
```

### `n_umbers`

`Char -> Float`, one argument.

The numbers in a text, as Floats.

```
      n_umbers "1 2.5 -3"
1.0 2.5 -3.0
```

### `[]N_PUT`

`Char -> Char -> Int`, two arguments.

Write the text on the left to the file on the right (made, with its
directories, if missing); how many characters it wrote.

```
      "hello" []N_PUT "work/reference.txt"
5
```

### `[]N_GET`

`Char -> Char`, one argument.

A file's text.

```
      []N_GET "work/reference.txt"
hello
```

### `[]R_EAD`

`Unit -> Char`, one argument.

A line typed at the keyboard (here, the line "a typed line").

```
      []R_EAD @
a typed line
```

## Graphics

A program computes what to draw as an array. Drawing is pure: the
picture comes back as text, an SVG document. Showing it is the effect.

### `[]G_RID`

`Eq a => a -> Char`, one argument.

An array as a picture, returned as SVG text: a vector as one row of
cells, a matrix as a grid, a rank-3 array as frames shown in turn.
Numbers that are all 0 or 1 draw their 1s dark; other numbers are
colored from the least (dark purple) to the greatest (yellow);
characters are drawn in their cells. A frame of more than 64 by 64
numbers is drawn as an image, a pixel per cell.

```
      []G_RID 1 0
<svg xmlns="http://www.w3.org/2000/svg" width="48" height="24" viewBox="0 0 48 24" role="img">
<rect width="48" height="24" fill="#f8fafc"/>
<rect x="0" y="0" width="24" height="24" fill="#1f2937"/>
<path d="M0 0H48M0 24H48M0 0V24M24 0V24M48 0V24" stroke="#cbd5e1" stroke-width="1" fill="none"/>
</svg>
```

### `[]P_ATH`

`Num a => a -> Char`, one argument.

Points as a picture, returned as SVG text: 2 rows, x over y, joined in
order and fitted into a 400-pixel picture with y pointing up; a rank-3
array is frames of paths shown in turn.

```
      []P_ATH 2 3 r_eshape 0 1 2 0 1 0
<svg xmlns="http://www.w3.org/2000/svg" width="424" height="224" viewBox="0 0 424 224" role="img">
<rect width="424" height="224" fill="#f8fafc"/>
<polyline points="12,212 212,12 412,212" fill="none" stroke="#1f2937" stroke-width="1.5" stroke-linejoin="round"/>
</svg>
```

### `[]S_HOW`

`Char -> Char`, one argument.

Show a picture, and return it: the command line writes it to a
numbered file (`--draw DIR`, else `XETAL_DRAW`, else the current
directory), the browser shows it beside the program.

```
      9 t_ake []S_HOW []G_RID 1 0
<svg xmln
```
