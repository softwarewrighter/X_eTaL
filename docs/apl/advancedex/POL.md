# POL

From ADVANCEDEX, Appendix B (Advanced Examples) of the APL\360 User's
Manual: the March 1970 edition (GH20-0683-1), page B.6; the August 1968
edition, page B.6, prints it the same.

What it does, from the manual: evaluates a polynomial whose coefficients
(in ascending order of power) are the left argument, at the scalar point
given by the right argument, using an inner product of the powers with the coefficients.

```
    &#8711; Z&#8592;C POL X
[1]   Z&#8592;(X*&#175;1+&#9075;&#9076;,C)+.&#215;C
    &#8711;
```

The example is in [POLY](POLY.md).
