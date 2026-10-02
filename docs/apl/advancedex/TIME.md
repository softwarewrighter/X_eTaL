# TIME

From ADVANCEDEX, Appendix B (Advanced Examples) of the APL\360 User's
Manual: the March 1970 edition (GH20-0683-1), page B.7; the August 1968
edition, page B.7, prints it the same.

What it does, from the manual: returns the CPU time used since its previous execution, as
minutes, seconds and 60ths of a second, for timing other functions; it
stores the cumulative CPU time in the global variable TIMER.

```
    &#8711; Z&#8592;TIME;T
[1]   Z&#8592; 60 60 60 &#8868;(T&#8592;&#9014;21)-TIMER
[2]   TIMER&#8592;T
    &#8711;
```

The example is in [COMB](COMB.md).
