# FC

From ADVANCEDEX, Appendix B (Advanced Examples) of the APL\360 User's
Manual: the March 1970 edition (GH20-0683-1), page B.7; the August 1968
edition, page B.7, prints it the same.

What it does, from the manual: an alternative to COMB without recursion: it yields the same
pairs of elements of ⍳N, but in a different order.

```
    ∇ C←FC N;A;B
[1]   B←(⍳N)∘.+N⍴0
[2]   A←(⍳N)∘.+⍳N
[3]   C←(2,N×N)⍴(,B),,A
[4]   C←⍉(C[2;]≤N)/C
    ∇
```

The example is in [COMB](COMB.md).
