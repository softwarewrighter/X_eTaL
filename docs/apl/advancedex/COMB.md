# COMB

From ADVANCEDEX, Appendix B (Advanced Examples) of the APL\360 User's
Manual: the March 1970 edition (GH20-0683-1), page B.7; the August 1968
edition, page B.7, prints it the same.

What it does, from the manual: builds, by recursion, the (2!N)-by-2 matrix of all pairs of
distinct elements of ⍳N.

```
    ∇ C←COMB N;A;B
[1]   →0×⍳N<2
[2]   →0×⍳N=2×1⍴C← 1 2 ⍴ 1 2
[3]   A←COMB N-1
[4]   C←((⍴A)+(N-1),0)⍴(,A),,(⍳N-1)∘.⌈0,N
    ∇
```

The example as printed (1970):

```
      TIME
0 0 35
      TIME
0 0 2
      COMB 4

 1 2
 1 3
 2 3
 1 4
 2 4
 3 4
      TIME
0 0 12
      FC 4

 1 2
 1 3
 1 4
 2 3
 2 4
 3 4
      TIME
0 0 8
      LFC 4

AB
AC
AD
BC
BD
CD
      TIME
0 0 7
      Z←COMB 15
      ⍴Z
105 2
      TIME
0 1 4
      Z←FC 15
      ⍴Z
105 2
      TIME
0 0 29
```
