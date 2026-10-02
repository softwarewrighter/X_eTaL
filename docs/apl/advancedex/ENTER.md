# ENTER

From ADVANCEDEX, Appendix B (Advanced Examples) of the APL\360 User's
Manual: the March 1970 edition (GH20-0683-1), page B.3; the August 1968
edition, page B.3, prints it the same.

What it does, from the manual: ENTER, LOOKUP and RESET build and use
lists of variable-length data, each list kept as a character vector plus
a vector of indices. ENTER asks (by quote-quad input) for name and data
items in turn, appending them to the global lists, until an empty line
is entered.

```
    &#8711; ENTER;X
[1]   'ENTER NAME'
[2]   X&#8592;,&#9054;
[3]   &#8594;0&#215;&#9075;0=&#9076;X
[4]   NAMES&#8592;NAMES,X
[5]   P1&#8592;P1,&#9076;NAMES
[6]   'ENTER DATA'
[7]   DATA&#8592;DATA,&#9054;
[8]   P2&#8592;P2,&#9076;DATA
[9]   ''
[10]  &#8594;1
    &#8711;
```

The example as printed (1970):

```
      RESET
      ENTER
ENTER NAME
J. ARMSTRONG
ENTER DATA
PRESIDENT

ENTER NAME
H. LEVINE
ENTER DATA
VICE-PRESIDENT

ENTER NAME

      LOOKUP
?
H. LEVINE
VICE-PRESIDENT
?
L. YAVNER
NO SUCH NAME
?
```
