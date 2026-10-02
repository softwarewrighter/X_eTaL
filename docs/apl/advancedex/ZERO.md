# ZERO

From ADVANCEDEX, Appendix B (Advanced Examples) of the APL\360 User's
Manual: the March 1970 edition (GH20-0683-1), page B.9; the August 1968
edition, page B.9, prints it the same.

What it does, from the manual: finds, by the method of false position, a root of the function
F to within tolerance TOL (left argument), lying between the bounds B[1] and
B[2] (right argument), where F B[1] and F B[2] have opposite signs.

```
    ∇ Z←TOL ZERO B;T
[1]   →0×⍳TOL≥∣T←F Z←0.5×+/B
[2]   →1,B[2⊥(0<T)≠0<F B]←Z
    ∇
```

The example as printed (1970):

```
      ⎕←X←¯4+⍳9
¯3 ¯2 ¯1 0 1 2 3 4 5
      F X
169 12 ¯29 ¯20 ¯3 4 7 36 145
      TIME
0 1 19
      ⎕←R←1E¯6 ZERO ¯2 ¯1
¯1.845121413
      TIME
0 2 36
      F R
7.14140814E¯7
      TIME
0 0 2
      ⎕←F⎕←R←1E¯10 ZERO 1 2
1.26397094
¯1.813305062E¯11
      TIME
0 3 46
      ⎕←F⎕←R←1E¯6 ZERO 1 2
1.263970852
¯8.51888359E¯7
      TIME
0 2 13
```
