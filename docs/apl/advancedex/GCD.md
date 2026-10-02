# GCD

From ADVANCEDEX, Appendix B (Advanced Examples) of the APL\360 User's
Manual: the March 1970 edition (GH20-0683-1), page B.5; the August 1968
edition, page B.5, prints it the same.

What it does, from the manual: GCD and GC both find the greatest common
divisor by the Euclidean algorithm; GCD takes two scalar arguments.

```
    ∇ Z←M GCD N
[1]   Z←M
[2]   M←M∣N
[3]   N←Z
[4]   →0≠M
    ∇
```

The example as printed (1970):

```
      84 GCD 90
6
      90 GCD 84
6
      GC 90 84
6
      GCV 90 84
6
      GCV 90 84 105
3
```
