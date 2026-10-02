# ASSOC

From ADVANCEDEX, Appendix B (Advanced Examples) of the APL\360 User's
Manual: the March 1970 edition (GH20-0683-1), page B.12; the August 1968
edition, page B.12, prints it the same.

What it does, from the manual: ASSOC M tests a putative group
multiplication table M for associativity (the manual takes the group
elements to be &#9075;1&#9076;&#9076;M, as printed), giving 1 if it is associative and 0 otherwise.

```
    &#8711; Z&#8592;ASSOC M
[1]   Z&#8592;&#8743;/,M[M;]=M[;M]
    &#8711;
```

The example as printed (1970):

```
      M&#8592;(&#9075;5)&#9021;5 5&#9076;&#9075;5
      M

 2  3  4  5  1
 3  4  5  1  2
 4  5  1  2  3
 5  1  2  3  4
 1  2  3  4  5
      TIME
0  0  13
      ASSOC M
1
      TIME
0  0  9
      M&#8592;0 0 1 0 0&#9021;M
      M

 2  3  4  5  1
 3  4  5  1  2
 5  1  2  3  4
 5  1  2  3  4
 1  2  3  4  5
      ASSOC M
0
      TIME
0  0  10
      M&#8592;?10 10&#9076;10
      &#9076;M
10  10
      TIME
0  0  3
      ASSOC M
0
      TIME
0  0  45
```
