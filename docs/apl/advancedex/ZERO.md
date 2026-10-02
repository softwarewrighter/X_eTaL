# ZERO

From ADVANCEDEX, Appendix B (Advanced Examples) of the APL\360 User's
Manual: the March 1970 edition (GH20-0683-1), page B.9; the August 1968
edition, page B.9, prints it the same.

What it does, from the manual: finds, by the method of false position, a root of the function
F to within tolerance TOL (left argument), lying between the bounds B[1] and
B[2] (right argument), where F B[1] and F B[2] have opposite signs.

```
    &#8711; Z&#8592;TOL ZERO B;T
[1]   &#8594;0&#215;&#9075;TOL&#8805;&#8739;T&#8592;F Z&#8592;0.5&#215;+/B
[2]   &#8594;1,B[2&#8869;(0<T)&#8800;0<F B]&#8592;Z
    &#8711;
```

The example as printed (1970):

```
      &#9109;&#8592;X&#8592;&#175;4+&#9075;9
&#175;3 &#175;2 &#175;1 0 1 2 3 4 5
      F X
169 12 &#175;29 &#175;20 &#175;3 4 7 36 145
      TIME
0 1 19
      &#9109;&#8592;R&#8592;1E&#175;6 ZERO &#175;2 &#175;1
&#175;1.845121413
      TIME
0 2 36
      F R
7.14140814E&#175;7
      TIME
0 0 2
      &#9109;&#8592;F&#9109;&#8592;R&#8592;1E&#175;10 ZERO 1 2
1.26397094
&#175;1.813305062E&#175;11
      TIME
0 3 46
      &#9109;&#8592;F&#9109;&#8592;R&#8592;1E&#175;6 ZERO 1 2
1.263970852
&#175;8.51888359E&#175;7
      TIME
0 2 13
```
