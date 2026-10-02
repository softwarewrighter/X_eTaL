# PALL

From ADVANCEDEX, Appendix B (Advanced Examples) of the APL\360 User's
Manual: the March 1970 edition (GH20-0683-1), page B.11; the August 1968
edition, page B.11, prints it the same.

What it does, from the manual: PALL N produces the matrix of all
permutations of order N, one per row, building row I with N PERM I.

```
    ∇ Z←PALL N;I
[1]   Z←((!N),N)⍴0
[2]   I←1
[3]   Z[I;]←N PERM I
[4]   →3×(!N)≥I←I+1
    ∇
```

The example as printed (1970):

```
      PALL 3

 1  2  3
 1  3  2
 2  1  3
 2  3  1
 3  1  2
 3  2  1
      TIME
0  3  7
      Z←PALL 3
      TIME
0  0  49
      Z←PALL 5
      TIME
0  25  10
      Z←PER 5
      TIME
0  1  12
```
