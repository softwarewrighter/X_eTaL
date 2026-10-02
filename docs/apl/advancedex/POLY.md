# POLY

From ADVANCEDEX, Appendix B (Advanced Examples) of the APL\360 User's
Manual: the March 1970 edition (GH20-0683-1), page B.6; the August 1968
edition, page B.6, prints it the same.

What it does, from the manual: evaluates a polynomial whose coefficients
(in ascending order of power) are the left argument, at the scalar point
given by the right argument, by summing coefficients times powers.

```
    &#8711; Z&#8592;C POLY X
[1]   Z&#8592;+/C&#215;X*&#175;1+&#9075;&#9076;,C
    &#8711;
```

The example as printed (1970):

```
      C&#8592;1 2 3 4
      C POLYB 3
142
      (C POLY 3)&#8743;.=(C POLYB 3),(C POL 3),C PO 3
1
      C PO 1 2 3 4 5 6
10 49 142 313 586 985
      &#9109;&#8592;M&#8592;&#9033;BIN 5

  1  1  1  1  1  1
  0  1  2  3  4  5
  0  0  1  3  6 10
  0  0  0  1  4 10
  0  0  0  0  1  5
  0  0  0  0  0  1

      &#8970;M PO &#9075;6

      1     2     4     8    16    32
      1     3     9    27    81   243
      1     4    16    64   256  1024
      1     5    25   125   625  3125
      1     6    36   216  1296  7776
      1     7    49   343  2401 16807
```
