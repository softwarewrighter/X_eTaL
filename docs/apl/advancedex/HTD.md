# HTD

From ADVANCEDEX, Appendix B (Advanced Examples) of the APL\360 User's
Manual: the March 1970 edition (GH20-0683-1), page B.8; the August 1968
edition, page B.8, prints it the same.

What it does, from the manual: works on hexadecimal numbers of at most 8 digits from
0123456789ABCDEF, negatives in 2's complement (8 through F in the leftmost of
eight places), and leading zeros may be omitted. HTD converts hexadecimal to decimal, printing
NUMBER IS NOT HEX for a non-hex digit.

```
    &#8711; R&#8592;HTD X
[1]   R&#8592;((8-&#9076;,X)&#9076;'0'),X
[2]   R&#8592;&#8970;(16&#8869;&#175;1+'0123456789ABCDEF'&#9075;R)-(2*32)&#215;R[1]&#8714;'89ABCDEF'
[3]   &#8594;4&#215;~&#8743;/X&#8714;'0123456789ABCDEF'
[4]   R&#8592;''
[5]   'NUMBER IS NOT HEX'
    &#8711;
```

The example is in [DTH](DTH.md).
