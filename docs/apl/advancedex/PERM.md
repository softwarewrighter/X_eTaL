# PERM

From ADVANCEDEX, Appendix B (Advanced Examples) of the APL\360 User's
Manual: the March 1970 edition (GH20-0683-1), page B.11; the August 1968
edition, page B.11, prints it the same.

What it does, from the manual: A PERM B produces the B-th permutation of
order A, by a method due to L. J. Woodrum; PALL uses it.

```
    &#8711; Z&#8592;A PERM B;I;Y
[1]   I&#8592;&#9076;Z&#8592;1+(&#9021;&#9075;A)&#8868;B-1
[2]   &#8594;0&#215;&#9075;0=I&#8592;I-1
[3]   Z[Y]&#8592;Z[Y]+Z[I]&#8804;Z[Y&#8592;I+&#9075;A-I]
[4]   &#8594;2
    &#8711;
```

The example is in [PALL](PALL.md).
