# PACK

From ADVANCEDEX, Appendix B (Advanced Examples) of the APL\360 User's
Manual: the March 1970 edition (GH20-0683-1), page B.2; the August 1968
edition, page B.2, prints it the same.

What it does, from the manual: PACK and UNPACK show decode and encode
converting between a four-number form (serial number 1 to 9999, month,
day, year) and a single number holding the same data; PACK makes the
single number (by decode).

```
    ∇ Z←PACK X
[1]   Z← 10000 12 31 100 ⊥X-1
    ∇
```

The example as printed (1970):

```
      P←PACK 2314 7 17 68
      P
86063867
      UNPACK P
2314  7  17  68
      UNPACK PACK 2311 9 21 72
2311  9  21  72
      PACK UNPACK 92137142
92137142
      PACK 1 1 31 1
3000
      UNPACK 3000
1  1  31  1
```
