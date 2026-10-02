# PER

From ADVANCEDEX, Appendix B (Advanced Examples) of the APL\360 User's
Manual: the March 1970 edition (GH20-0683-1), page B.11; the August 1968
edition, page B.11, prints it the same.

What it does, from the manual: PER M produces all permutations of order M
by a recursive definition, much faster than PALL, giving the permutations
in the opposite order.

```
    ∇ P←PER M;X;Y;Z
[1]   →0×⍳M=P← 1 1 ⍴1
[2]   Z←PER M-1
[3]   P←⍳X←0
[4]   →0×⍳M<X←X+1
[5]   Y←(∼(⍳M)∊X)\Z
[6]   Y[;X]←M
[7]   P←((X×!M-1),M)⍴(,P),,Y
[8]   →4
    ∇
```

The example is in [PALL](PALL.md) (the timing of Z←PER 5).
