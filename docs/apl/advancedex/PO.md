# PO

From ADVANCEDEX, Appendix B (Advanced Examples) of the APL\360 User's
Manual: the March 1970 edition (GH20-0683-1), page B.6; the August 1968
edition, page B.6, prints it the same.

What it does, from the manual: evaluates polynomials for arguments of any rank: the vectors
along the first coordinate of the left argument are coefficients (ascending
powers), and each polynomial is evaluated at every element of the right
argument.

```
    &#8711; Z&#8592;C PO X
[1]   Z&#8592;(X&#8728;.*&#175;1+&#9075;1&#9076;&#9076;C)+.&#215;C
    &#8711;
```

The example is in [POLY](POLY.md).
