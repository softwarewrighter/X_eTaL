# TIME

From ADVANCEDEX, Appendix B (Advanced Examples) of the APL\360 User's
Manual: the March 1970 edition (GH20-0683-1), page B.7; the August 1968
edition, page B.7, prints it the same.

What it does, from the manual: returns the CPU time used since its previous execution, as
minutes, seconds and 60ths of a second, for timing other functions; it
stores the cumulative CPU time in the global variable TIMER.

```
    ∇ Z←TIME;T
[1]   Z← 60 60 60 ⊤(T←⌶21)-TIMER
[2]   TIMER←T
    ∇
```

The example is in [COMB](COMB.md).
