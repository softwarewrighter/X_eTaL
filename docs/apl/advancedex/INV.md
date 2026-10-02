# INV

From ADVANCEDEX, Appendix B (Advanced Examples) of the APL\360 User's
Manual: the March 1970 edition (GH20-0683-1), page B.10; the August 1968
edition, page B.10, prints it the same.

What it does, from the manual: INV M gives the inverse of the matrix M by
Gauss-Jordan (complete) elimination without pivoting. Line 1 appends the
unit vector 1&#8805;&#9075;N as a last column and line 2 performs, on each pass of the
loop, one of the N complete inversion steps (Iverson, A Programming
Language, exercise 1.40).

```
    &#8711; Z&#8592;INV M;I;J
[1]   M&#8592;&#9033;(1 0 +&#9076;M)&#9076;(,&#9033;M),&#8764;J&#8592;1<&#9075;I&#8592;1&#8593;&#9076;M
[2]   M&#8592;1&#9021;(J,1)&#9021;[1]M-(J&#215;M[;1])&#8728;.&#215;M[1;]&#8592;M[1;]&#247;M[1;1]
[3]   &#8594;2&#215;&#9075;0&#8800;I&#8592;I-1
[4]   Z&#8592;M[;&#9075;1&#8593;&#9076;M]
    &#8711;
```

The example as printed (1970):

```
      &#9109;&#8592;N&#8592;INV M&#8592;HILB 3

  9                &#175;36                 30
&#175;36                192               &#175;180
 30               &#175;180                180
      M+.&#215;N

 1.000000000E0     2.842170943E&#175;14  &#175;6.039613254E&#175;14
 1.421085472E&#175;14  1.000000000E0     &#175;1.065814104E&#175;14
 4.662936703E&#175;15  3.197442311E&#175;14   1.000000000E0
```
