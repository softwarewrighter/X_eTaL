# TRUTH

From ADVANCEDEX, Appendix B (Advanced Examples) of the APL\360 User's
Manual: the March 1970 edition (GH20-0683-1), page B.4; the August 1968
edition, page B.4, prints it the same.

What it does, from the manual: produces the matrix of arguments of the
truth table for N logical variables (all 2*N rows of N bits).

```
    &#8711; Z&#8592;TRUTH N
[1]   Z&#8592;2&#8739;&#8970;(&#175;1+&#9075;2*N)&#8728;.&#247;2*N-&#9075;N
    &#8711;
```

The example as printed (1970):

```
      TRUTH 3

  0  0  0
  0  0  1
  0  1  0
  0  1  1
  1  0  0
  1  0  1
  1  1  0
  1  1  1
      (TRUTH 3)+.&#215;&#9021;2*&#175;1+&#9075;3
0  1  2  3  4  5  6  7
```
