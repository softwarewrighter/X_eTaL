# GCV

From ADVANCEDEX, Appendix B (Advanced Examples) of the APL\360 User's
Manual: the March 1970 edition (GH20-0683-1), page B.5; the August 1968
edition, page B.5, prints it the same.

What it does, from the manual: yields the greatest common divisor of all
the elements of a vector of two or more elements.

```
    ∇ Z←GCV W;A
[1]   →1≠⍴W←Z,(A≠0)/A←(Z←⌊/W)∣W
    ∇
```

The example is in [GCD](GCD.md).
