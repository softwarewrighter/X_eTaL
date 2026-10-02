# BIN

From ADVANCEDEX, Appendix B (Advanced Examples) of the APL\360 User's
Manual: the March 1970 edition (GH20-0683-1), page B.5; the August 1968
edition, page B.5, prints it the same.

What it does, from the manual: produces all the binomial coefficients up
to order N, as a matrix whose row I holds the coefficients of order I-1.

```
    &#8711; Z&#8592;BIN N
[1]   Z&#8592;&#8970;&#9033;(0,&#9075;N)&#8728;.!0,&#9075;N
    &#8711;
```

The example as printed (1970):

```
      BIN 4

  1  0  0  0  0
  1  1  0  0  0
  1  2  1  0  0
  1  3  3  1  0
  1  4  6  4  1
```
