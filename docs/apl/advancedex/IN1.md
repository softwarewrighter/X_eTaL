# IN1

From ADVANCEDEX, Appendix B (Advanced Examples) of the APL\360 User's
Manual: the March 1970 edition (GH20-0683-1), page B.4; the August 1968
edition, page B.4, prints it the same.

What it does, from the manual: like IN, but returns only non-overlapping
occurrences: it first applies IN and then suppresses every occurrence
that overlaps an earlier one.

```
    ∇ T←A IN1 B
[1]   T←A IN B
[2]   →2×J<⍴T←(∼(⍳⍴T)∊J←1+((⍴A)>∣-/[1](2,1+⍴T)⍴T)⍳1)/T
    ∇
```

The example is in [IN](IN.md).
