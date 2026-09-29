# A tour of M0 and M1

Part of the X_eTaL milestone tours ([index](tour.md), [README](../README.md)); next: [A tour of M2](tour-m2.md).
Commands run from the repository root after `scripts/build-all.sh --release`
(or use `./target/debug/xetal` after `just build`).

**M0 -- typing and display.** Source is plain ASCII; `xetal render`
draws it decorated and `--raw` turns the decorated text back into the
same source (see [`docs/input.md`](input.md)).

**M1 -- the scalar functional calculus.** Every function takes one
argument; dyadic use is currying. The demo scripts in `demos/` are
executable (`#!/usr/bin/env xetal`):

```
# demos/sub.xtl
u:s_ub := { _l - _r }
10 u:s_ub 3
u:t_enMinus := u:s_ub 10    # partial application fixes the left argument
u:t_enMinus 3
```

```bash
./target/release/xetal run demos/square.xtl       # 49
./target/release/xetal run demos/sub.xtl          # 7 and 7
./target/release/xetal run demos/factorial.xtl    # 3628800
./target/release/xetal run --untyped demos/fixed-point.xtl  # 120
```

`demos/factorial.xtl` uses guards, one statement per line:

```
u:f_act := { n ->
  n <= 1 ? 1
  n * u:f_act n - 1
}
u:f_act 10
```

`demos/fixed-point.xtl` shows the Y combinator working because its
self parameter is lazy (`~s_elf`). Its self-applied argument has no
finite type, so it runs with `--untyped`, which skips the checker:

```
u:Y_ := { f_ -> { x_ -> f_ x_ 'x_ } '{ x_ -> f_ x_ 'x_ } }
u:F_ := { ~s_elf n -> n <= 1 ? 1; n * s_elf n - 1 }
(u:Y_ 'u:F_)_ 5
```

Each stage of the pipeline is visible from the command line; for
`u:s_ub := { _l - _r }; 10 u:s_ub 3`, `xetal parse` prints the tree
shown above and `xetal core` the desugared Core, where built-ins are
marked `#`:

```
(def u:s_ub (lam _l (lam _r (app2 #- _l _r))))
(eval (app2 u:s_ub 10 3))
```

## The pipeline, stage by stage

Build and call the binary directly:

```bash
scripts/build-all.sh --release
./target/release/xetal --version
./target/release/xetal eval -e '1 + 2'                     # 3
./target/release/xetal run demos/factorial.xtl             # 3628800
./target/release/xetal lex -e 'u:s_quare := { _r * _r }; u:s_quare 7'
```

`xetal lex` prints one token per line with its byte span:

```
0..9 Func(u:s_quare)
10..12 Assign
13..14 LBrace
15..17 LamArg(r)
18..19 Sym(*)
20..22 LamArg(r)
23..24 RBrace
24..25 Semi
26..35 Func(u:s_quare)
36..37 Num(7)
```

Source is typed as plain ASCII: a `_` directly after a letter
underlines it and makes the name a function (`r_ev`), `_digits` after
a function name are axis subscripts (`o_-_12`), `ns:` names a
namespace (`u:s_quare`), and `^2` touching a value is an exponent.
Errors carry a code and a byte span, for example `xetal lex -e '3-1'`
reports `error[ambiguous-minus]` at `1..2`.

`xetal render` shows the decorated form; `--raw` converts it back,
`--latex` prints LaTeX math for a post-processor, and `--color`
highlights it for a terminal (text that does not lex is shown in red,
and rendering continues):

```bash
./target/release/xetal render -e 'x r_ev o_-_2 u:s_quare c:K_ x^2 _l _r @'
./target/release/xetal render --latex -e 'o_-_12 x^2'
./target/release/xetal render --color -e 'u:s_quare := { _r * _r } # sq'
```

`xetal fmt` prints the canonical form, with every application in
parentheses (`xetal fmt -e 'x := 3; a f_ b g_ c'` gives `x := 3` and
`(a f_ (b g_ c))`), and `xetal parse` prints the surface tree as
S-expressions, one line per statement:

```bash
./target/release/xetal parse -e 'u:s_ub := { _l - _r }; 10 u:s_ub 3'
```

```
(:= u:s_ub (lambda (_l _r) (- _l _r)))
(u:s_ub 10 3)
```
