# IN

From ADVANCEDEX, Appendix B (Advanced Examples) of the APL\360 User's
Manual: the March 1970 edition (GH20-0683-1), page B.4; the August 1968
edition, page B.4, prints it the same.

What it does, from the manual: given a word (a vector) and a second
vector, IN returns the index of the first letter of every occurrence of
the word in the second vector, overlapping occurrences included.

```
    ∇ Z←A IN B;J
[1]   J←(A[1]=B)/⍳⍴B
[2]   J←(J≤1+(⍴B)-⍴A)/J
[3]   Z←(B[J∘.+¯1+⍳⍴A]∧.=A)/J
    ∇
```

The example as printed (1970):

```
      W←'THE'
      T←'THE MEN THEN WENT HOME.
      W IN T
1  9
      W IN1 T
1  9
      'ABA' IN 'NOWABABABABABABABA'
4  6  8  10  12  14  16
      'ABA' IN1 'NOWABABABABABABABA'
4  8  12  16
```

The 1970 edition prints the second line of the example without its closing
quote; the August 1968 edition prints it `T←'THE MEN THEN WENT HOME.'`.
