# HILB

From ADVANCEDEX, Appendix B (Advanced Examples) of the APL\360 User's
Manual: the March 1970 edition (GH20-0683-1), page B.10; the August 1968
edition, page B.10, prints it the same.

What it does, from the manual: HILB N produces the Hilbert matrix of order
N, whose element in row I and column J is the reciprocal of I+J-1.

```
    ∇ Z←HILB N
[1]   Z←÷¯1+(⍳N)∘.+⍳N
    ∇
```

The example as printed (1970):

```
      HILB 3

1                  0.5                0.3333333333
0.5                0.3333333333       0.25
0.3333333333       0.25               0.2
```
