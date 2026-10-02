# GC

From ADVANCEDEX, Appendix B (Advanced Examples) of the APL\360 User's
Manual: the March 1970 edition (GH20-0683-1), page B.5; the August 1968
edition, page B.5, prints it the same.

What it does, from the manual: like GCD (the Euclidean algorithm for the
greatest common divisor), but with a single argument, expected to be a
two-element vector.

```
    ∇ Z←GC M
[1]   →0≠1↓M←⌽M[1],Z←∣/M
    ∇
```

The example is in [GCD](GCD.md).
