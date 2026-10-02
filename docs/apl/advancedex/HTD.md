# HTD

From ADVANCEDEX, Appendix B (Advanced Examples) of the APL\360 User's
Manual: the March 1970 edition (GH20-0683-1), page B.8; the August 1968
edition, page B.8, prints it the same.

What it does, from the manual: works on hexadecimal numbers of at most 8 digits from
0123456789ABCDEF, negatives in 2's complement (8 through F in the leftmost of
eight places), and leading zeros may be omitted. HTD converts hexadecimal to decimal, printing
NUMBER IS NOT HEX for a non-hex digit.

```
    ∇ R←HTD X
[1]   R←((8-⍴,X)⍴'0'),X
[2]   R←⌊(16⊥¯1+'0123456789ABCDEF'⍳R)-(2*32)×R[1]∊'89ABCDEF'
[3]   →4×~∧/X∊'0123456789ABCDEF'
[4]   R←''
[5]   'NUMBER IS NOT HEX'
    ∇
```

The example is in [DTH](DTH.md).
