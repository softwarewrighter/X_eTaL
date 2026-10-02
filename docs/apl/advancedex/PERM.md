# PERM

From ADVANCEDEX, Appendix B (Advanced Examples) of the APL\360 User's
Manual: the March 1970 edition (GH20-0683-1), page B.11; the August 1968
edition, page B.11, prints it the same.

What it does, from the manual: A PERM B produces the B-th permutation of
order A, by a method due to L. J. Woodrum; PALL uses it.

```
    ∇ Z←A PERM B;I;Y
[1]   I←⍴Z←1+(⌽⍳A)⊤B-1
[2]   →0×⍳0=I←I-1
[3]   Z[Y]←Z[Y]+Z[I]≤Z[Y←I+⍳A-I]
[4]   →2
    ∇
```

The example is in [PALL](PALL.md).
