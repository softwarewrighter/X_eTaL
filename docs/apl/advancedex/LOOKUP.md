# LOOKUP

From ADVANCEDEX, Appendix B (Advanced Examples) of the APL\360 User's
Manual: the March 1970 edition (GH20-0683-1), page B.3; the August 1968
edition, page B.3, prints it the same.

What it does, from the manual: for each name entered (by quote-quad
input, until an empty line) it prints the data stored with that name by
ENTER, or NO SUCH NAME (or a message if the name occurs more than once).

```
    &#8711; LOOKUP;X;J
[1]   '?'
[2]   X&#8592;,&#9054;
[3]   &#8594;0&#215;&#9075;0=&#9076;X
[4]   J&#8592;(((1&#8595;P1)-&#175;1&#8595;P1)=&#9076;X)/&#9075;&#175;1+&#9076;P1
[5]   J&#8592;(NAMES[P1[J]&#8728;.+&#9075;&#9076;X]&#8743;.=X)/J
[6]   &#8594;(0 1 =&#9076;J)/ 10 8
[7]   &#8594;1,&#9076;&#9109;&#8592;'MORE THAN ONE SUCH NAME'
[8]   DATA[P2[J]+&#9075;-/P2[1 0 +J]]
[9]   &#8594;1
[10]  'NO SUCH NAME'
[11]  &#8594;1
    &#8711;
```

The example is in [ENTER](ENTER.md).
