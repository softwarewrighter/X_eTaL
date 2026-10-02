# PER

From ADVANCEDEX, Appendix B (Advanced Examples) of the APL\360 User's
Manual: the March 1970 edition (GH20-0683-1), page B.11; the August 1968
edition, page B.11, prints it the same.

What it does, from the manual: PER M produces all permutations of order M
by a recursive definition, much faster than PALL, giving the permutations
in the opposite order.

```
    &#8711; P&#8592;PER M;X;Y;Z
[1]   &#8594;0&#215;&#9075;M=P&#8592; 1 1 &#9076;1
[2]   Z&#8592;PER M-1
[3]   P&#8592;&#9075;X&#8592;0
[4]   &#8594;0&#215;&#9075;M<X&#8592;X+1
[5]   Y&#8592;(&#8764;(&#9075;M)&#8714;X)\Z
[6]   Y[;X]&#8592;M
[7]   P&#8592;((X&#215;!M-1),M)&#9076;(,P),,Y
[8]   &#8594;4
    &#8711;
```

The example is in [PALL](PALL.md) (the timing of Z&#8592;PER 5).
