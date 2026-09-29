# A tour of M4

Part of the X_eTaL milestone tours ([index](tour.md), [README](../README.md)); previous: [A tour of M3](tour-m3.md); next: [A tour of M5](tour-m5.md).
Commands run from the repository root after `scripts/build-all.sh --release`
(or use `./target/debug/xetal` after `just build`).

**M4 -- higher-order.** A quoted function is a value, and a quoted
function written just left of a function name is its operand:
`'+ r_/ v` reduces v by plus. With two operands the nearest one binds
first, so `A '+ '* i_nner B` reads like APL's `A +.x B`. Reduce is a
right fold along the leading axis (`'- r_/ 1 2 3` is `1 - (2 - 3)`),
and item k of a scan is the reduce of the first k items. Any function
value can be an operand: a symbol, a built-in, a lambda or a user
function.

```
# demos/higher-order.xtl
v := 3 1 4 1 5
'+ r_/ v
'- r_/ 1 2 3
'+ s_\ v
m := 2 3 r_eshape r_ange 6
'+ r_/ m
1 2 3 '* t_able 1 2 3
m '+ '* i_nner 1 1 1
u:s_ign := { x -> x < 0 ? -1; x = 0 ? 0; 1 }
'u:s_ign e_ach -2 0 7
1 2 3 '= e_ach 1 5 3
'n_eg 'a_bs c_ompose -3 4
2 '/ s_wap 1
s_ort v
g_rade v
u_nique v
v i_ndexOf 4 9
w_here v > 2
```

```bash
./target/release/xetal run demos/higher-order.xtl
```

```
14
2
3 4 8 9 14
5 7 9
1 2 3
2 4 6
3 6 9
6 15
-1 0 1
1 0 1
-3 -4
0.5
1 1 3 4 5
2 4 1 3 5
3 1 4 5
3 6
1 3 5
```

`u:s_ign` has guards, whose condition must be a single value, so it
is applied with `e_ach`; the dyadic `1 2 3 '= e_ach 1 5 3` is plain
currying. Reducing an empty array gives the operand's identity, of the
element type, and a derived function fits in a train:

```bash
./target/release/xetal eval -e "u:a_vg := ['+ r_/ / t_ally]; u:a_vg 1 2 3 4; '+ r_/ 0 t_ake 2.5"
```

prints `2.5` and `0.0`. `r_oll! n` gives a random number from 1 to n
for every item of n, so `r_oll! 6 6` rolls two dice:

```bash
./target/release/xetal eval -e 'r_oll! 6 6'
```

Each run rolls differently; `--seed N` (or `XETAL_SEED`) repeats a
run's rolls when a test needs that.
