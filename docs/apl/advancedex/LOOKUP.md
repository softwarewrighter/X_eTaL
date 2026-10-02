# LOOKUP

From ADVANCEDEX, Appendix B (Advanced Examples) of the APL\360 User's
Manual: the March 1970 edition (GH20-0683-1), page B.3; the August 1968
edition, page B.3, prints it the same.

What it does, from the manual: for each name entered (by quote-quad
input, until an empty line) it prints the data stored with that name by
ENTER, or NO SUCH NAME (or a message if the name occurs more than once).

```
    ∇ LOOKUP;X;J
[1]   '?'
[2]   X←,⍞
[3]   →0×⍳0=⍴X
[4]   J←(((1↓P1)-¯1↓P1)=⍴X)/⍳¯1+⍴P1
[5]   J←(NAMES[P1[J]∘.+⍳⍴X]∧.=X)/J
[6]   →(0 1 =⍴J)/ 10 8
[7]   →1,⍴⎕←'MORE THAN ONE SUCH NAME'
[8]   DATA[P2[J]+⍳-/P2[1 0 +J]]
[9]   →1
[10]  'NO SUCH NAME'
[11]  →1
    ∇
```

The example is in [ENTER](ENTER.md).
