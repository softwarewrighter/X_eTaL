# POL

From ADVANCEDEX, Appendix B (Advanced Examples) of the APL\360 User's
Manual: the March 1970 edition (GH20-0683-1), page B.6; the August 1968
edition, page B.6, prints it the same.

What it does, from the manual: evaluates a polynomial whose coefficients
(in ascending order of power) are the left argument, at the scalar point
given by the right argument, using an inner product of the powers with the coefficients.

```
    ∇ Z←C POL X
[1]   Z←(X*¯1+⍳⍴,C)+.×C
    ∇
```

The example is in [POLY](POLY.md).
