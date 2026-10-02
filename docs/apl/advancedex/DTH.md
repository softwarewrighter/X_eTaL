# DTH

From ADVANCEDEX, Appendix B (Advanced Examples) of the APL\360 User's
Manual: the March 1970 edition (GH20-0683-1), page B.8; the August 1968
edition, page B.8, prints it the same.

What it does, from the manual: works on hexadecimal numbers of at most 8 digits from
0123456789ABCDEF, negatives in 2's complement (8 through F in the leftmost of
eight places), and leading zeros may be omitted. DTH converts decimal to hexadecimal.

```
    ∇ R←DTH X
[1]   R←,('0123456789ABCDEF')[1+(8⍴16)⊤X]
    ∇
```

The example as printed (1970):

```
      Z←DTH 1776
      Z
000006F0
      HTD Z
1776
      Z AH Z
00000DE0
      HTD Z AH Z
3552
      HTD '000006F0'
1776
      HTD '90000000'
¯1879048192
      HTD '00049HFG'
NUMBER IS NOT HEX
```
