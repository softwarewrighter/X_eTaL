# RESET

From ADVANCEDEX, Appendix B (Advanced Examples) of the APL\360 User's
Manual: the March 1970 edition (GH20-0683-1), page B.3; the August 1968
edition, page B.3, prints it the same.

What it does, from the manual: empties the lists (the globals NAMES,
DATA, P1, P2) used by ENTER and LOOKUP; use it before them.

```
    ∇ RESET
[1]   NAMES←DATA←⍴P1←P2←0
    ∇
```

The example is in [ENTER](ENTER.md).
