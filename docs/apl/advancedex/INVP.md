# INVP

From ADVANCEDEX, Appendix B (Advanced Examples) of the APL\360 User's
Manual: the March 1970 edition (GH20-0683-1), page B.10; the August 1968
edition, page B.10, prints it the same.

What it does, from the manual: INVP M gives the inverse of the matrix M by
Gauss-Jordan (complete) elimination with pivoting; line 4 performs each of
the N complete inversion steps, and the permutation P of the pivot rows is
undone at the end.

```
    &#8711; Z&#8592;INVP M;I;J;K;P
[1]   M&#8592;&#9033;(1 0 +&#9076;M)&#9076;(,&#9033;M),&#8764;J&#8592;1<P&#8592;&#9075;I&#8592;1&#8593;&#9076;M
[2]   M[K,1;&#9075;&#9076;P]&#8592;M[1,K&#8592;(&#8739;M[&#9075;I;1])&#9075;&#8968;/&#8739;M[&#9075;I;1];&#9075;&#9076;P]
[3]   P&#8592;1&#9021;P,0&#9076;P[K,1]&#8592;P[1,K]
[4]   M&#8592;1&#9021;(J,1)&#9021;[1]M-(J&#215;M[;1])&#8728;.&#215;M[1;]&#8592;M[1;]&#247;M[1;1]
[5]   &#8594;2&#215;&#9075;0&#8800;I&#8592;I-1
[6]   Z&#8592;M[;&#9035;P]
    &#8711;
```

The example is in [INV](INV.md); the manual prints none for INVP itself.
