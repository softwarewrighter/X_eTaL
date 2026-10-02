# INV

From ADVANCEDEX, Appendix B (Advanced Examples) of the APL\360 User's
Manual: the March 1970 edition (GH20-0683-1), page B.10; the August 1968
edition, page B.10, prints it the same.

What it does, from the manual: INV M gives the inverse of the matrix M by
Gauss-Jordan (complete) elimination without pivoting. Line 1 appends the
unit vector 1≥⍳N as a last column and line 2 performs, on each pass of the
loop, one of the N complete inversion steps (Iverson, A Programming
Language, exercise 1.40).

```
    ∇ Z←INV M;I;J
[1]   M←⍉(1 0 +⍴M)⍴(,⍉M),∼J←1<⍳I←1↑⍴M
[2]   M←1⌽(J,1)⌽[1]M-(J×M[;1])∘.×M[1;]←M[1;]÷M[1;1]
[3]   →2×⍳0≠I←I-1
[4]   Z←M[;⍳1↑⍴M]
    ∇
```

The example as printed (1970):

```
      ⎕←N←INV M←HILB 3

  9                ¯36                 30
¯36                192               ¯180
 30               ¯180                180
      M+.×N

 1.000000000E0     2.842170943E¯14  ¯6.039613254E¯14
 1.421085472E¯14  1.000000000E0     ¯1.065814104E¯14
 4.662936703E¯15  3.197442311E¯14   1.000000000E0
```
