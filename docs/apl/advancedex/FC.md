# FC

From ADVANCEDEX, Appendix B (Advanced Examples) of the APL\360 User's
Manual: the March 1970 edition (GH20-0683-1), page B.7; the August 1968
edition, page B.7, prints it the same.

What it does, from the manual: an alternative to COMB without recursion: it yields the same
pairs of elements of &#9075;N, but in a different order.

```
    &#8711; C&#8592;FC N;A;B
[1]   B&#8592;(&#9075;N)&#8728;.+N&#9076;0
[2]   A&#8592;(&#9075;N)&#8728;.+&#9075;N
[3]   C&#8592;(2,N&#215;N)&#9076;(,B),,A
[4]   C&#8592;&#9033;(C[2;]&#8804;N)/C
    &#8711;
```

The example is in [COMB](COMB.md).
