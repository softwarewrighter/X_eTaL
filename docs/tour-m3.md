# A tour of M3

Part of the X_eTaL milestone tours ([index](tour.md), [README](../README.md)); previous: [A tour of M2](tour-m2.md); next: [A tour of M4](tour-m4.md).
Commands run from the repository root after `scripts/build-all.sh --release`
(or use `./target/debug/xetal` after `just build`).

**M3 -- arrays.** Arrays are dense and 1-origin. A type names only the
element type (`1 2 3` and `7` are both Int), so a scalar function
applies item by item and a scalar extends to every item; shapes are
checked at run time. The count, shape or indices of a structural
built-in go on its left.

```
# demos/arrays.xtl
m := 2 3 r_eshape r_ange 6
m
s_hape m
2 s_elect m
-1 t_ake m
m * 10
(f_irst m) c_at 7 8 9
10 ^ o_ffsets 3
```

```bash
./target/release/xetal run demos/arrays.xtl
```

```
1 2 3
4 5 6
2 3
4 5 6
4 5 6
10 20 30
40 50 60
1 2 3 7 8 9
1 10 100
```

A user function works on arrays unchanged, and a string is a Char
vector:

```bash
./target/release/xetal eval -e 'u:s_quare := { _r * _r }; u:s_quare 1 2 3; 1 2 3 + 10; "hello"'
./target/release/xetal type -e '1 2 3 + 10; "hello"'                  # Int, then Char
./target/release/xetal eval -e '1 2 + 1 2 3'                          # error[shape-mismatch]
./target/release/xetal eval -e '"hello" = "help!"; 3 t_ake "hello"; "ab" c_at "cd"; 10.0 ^ 20'
```

The first prints `1 4 9`, `11 12 13` and `hello`; the last prints
`1 1 1 0 0`, `hel`, `abcd` and `100000000000000000000.0`. `=` and
`!=` compare any one kind of scalar, and `<` `>` `<=` `>=` compare
numbers and characters.

`i_d` is the identity, `x l_eft y` is `x` and `x r_ight y` is `y`.
With them the S combinator is the train `[i_d F G]` (`x F (G x)`),
also written as a lambda:

```bash
./target/release/xetal eval -e "[i_d - n_eg] 5; u:S_ := { f_ g_ x -> x f_ g_ x }; 'n_eg '- u:S_ 5; 3 [l_eft + r_ight] 4"
```

prints `10`, `10` and `7`. `xetal repl` reads lines from standard
input: definitions and types persist, a failing line is reported and
dropped, and an unclosed bracket continues on the next line.
