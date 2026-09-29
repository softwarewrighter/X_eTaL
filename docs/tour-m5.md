# A tour of M5

Part of the X_eTaL milestone tours ([index](tour.md), [README](../README.md)); previous: [A tour of M4](tour-m4.md); next: [A tour of M5b](tour-m5b.md).
Commands run from the repository root after `scripts/build-all.sh --release`
(or use `./target/debug/xetal` after `just build`).

**M5 -- rotate and axes.** `n o_- X` rotates the major cells of X, a
positive amount toward the front, and `r_ev` reverses them. An axis
subscript works on any function by one rule: `f_k X` moves axis k of
X to the front, applies f, and moves it back (unless f consumed it, as
reduce does). On a dyadic function the axis is the right argument's.

```bash
./target/release/xetal eval -e "m := 2 3 r_eshape r_ange 6; 1 o_- 1 2 3 4; r_ev_2 m; 1 o_-_2 m; '+ r_/_2 m"
```

```
2 3 4 1
3 2 1
6 5 4
2 3 1
5 6 4
6 15
```

User functions take axes the same way. Rotate with several axes and
a list of amounts gives every combination, one leading axis per listed
axis, and reduce with several axes works on each in turn: these are
the two halves of the Life one-liner.

```bash
./target/release/xetal eval -e "u:f_lip := { r_ev _r }; b := 3 3 r_eshape r_ange 9; u:f_lip_2 b; s_hape -1 0 1 o_-_12 b; '+ r_/_12 b"
```

prints `b` with each row reversed, the shape `3 3 3 3` and the sum
`45`. `demos/rotate.xtl` prints frames of a picture moving diagonally
(`-1 o_-_12` each step, wrapping); `just animate` plays them.
