# IN1

From ADVANCEDEX, Appendix B (Advanced Examples) of the APL\360 User's
Manual: the March 1970 edition (GH20-0683-1), page B.4; the August 1968
edition, page B.4, prints it the same.

What it does, from the manual: like IN, but returns only non-overlapping
occurrences: it first applies IN and then suppresses every occurrence
that overlaps an earlier one.

```
    &#8711; T&#8592;A IN1 B
[1]   T&#8592;A IN B
[2]   &#8594;2&#215;J<&#9076;T&#8592;(&#8764;(&#9075;&#9076;T)&#8714;J&#8592;1+((&#9076;A)>&#8739;-/[1](2,1+&#9076;T)&#9076;T)&#9075;1)/T
    &#8711;
```

The example is in [IN](IN.md).
