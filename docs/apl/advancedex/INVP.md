# INVP

From ADVANCEDEX, Appendix B (Advanced Examples) of the APL\360 User's
Manual: the March 1970 edition (GH20-0683-1), page B.10; the August 1968
edition, page B.10, prints it the same.

What it does, from the manual: INVP M gives the inverse of the matrix M by
Gauss-Jordan (complete) elimination with pivoting; line 4 performs each of
the N complete inversion steps, and the permutation P of the pivot rows is
undone at the end.

```
    ∇ Z←INVP M;I;J;K;P
[1]   M←⍉(1 0 +⍴M)⍴(,⍉M),∼J←1<P←⍳I←1↑⍴M
[2]   M[K,1;⍳⍴P]←M[1,K←(∣M[⍳I;1])⍳⌈/∣M[⍳I;1];⍳⍴P]
[3]   P←1⌽P,0⍴P[K,1]←P[1,K]
[4]   M←1⌽(J,1)⌽[1]M-(J×M[;1])∘.×M[1;]←M[1;]÷M[1;1]
[5]   →2×⍳0≠I←I-1
[6]   Z←M[;⍋P]
    ∇
```

The example is in [INV](INV.md); the manual prints none for INVP itself.
