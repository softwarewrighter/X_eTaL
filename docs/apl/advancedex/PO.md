# PO

From ADVANCEDEX, Appendix B (Advanced Examples) of the APL\360 User's
Manual: the March 1970 edition (GH20-0683-1), page B.6; the August 1968
edition, page B.6, prints it the same.

What it does, from the manual: evaluates polynomials for arguments of any rank: the vectors
along the first coordinate of the left argument are coefficients (ascending
powers), and each polynomial is evaluated at every element of the right
argument.

```
    ∇ Z←C PO X
[1]   Z←(X∘.*¯1+⍳1⍴⍴C)+.×C
    ∇
```

The example is in [POLY](POLY.md).
