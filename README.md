# <img src="images/X_eTaL-logo.jpg" alt="X_eTaL" width="160" align="left" style="margin-right:12px"> X_eTaL

**eXperimental eXtensible Typed Array Language** -- LaTeX reversed,
with the X itself decorated.

> LaTeX uses text to produce typography. X_eTaL uses typography to
> express computation.

X_eTaL is a terse, statically typed, functional array language in the
APL / APL2 / J / BQN tradition, implemented in Rust. It uses no
special glyph alphabet: source is plain ASCII typed on a US keyboard,
and **typographic decoration** changes what an ordinary name means.
An underlined letter makes a name a function, a leading superscript
names its namespace, a subscript gives its axes, and a superscript on
a value is an exponent. The language decisions are recorded in
[`docs/lang-choices.md`](docs/lang-choices.md).

<br clear="left">

| Raw ASCII     | Displayed as                          | Meaning                              |
| ------------- | ------------------------------------- | ------------------------------------ |
| `x`           | x                                     | the variable `x`                     |
| `r_ev`        | rev, r underlined                     | the built-in function reverse        |
| `o_-_2`       | o-, o underlined, subscript 2         | rotate along axis 2                  |
| `'+ r_/ A`    | quote +, then r/ with r underlined    | reduce A by plus (APL `+/A`)         |
| `u:s_quare`   | superscript u, square, s underlined   | a user-defined function              |
| `c:K_`        | superscript c, K underlined           | K from the combinator library        |
| `x^2`         | x squared                             | exponent on a value                  |
| `_l` `_r`     | _l _r                                 | left / right lambda argument         |
| `@`           | @                                     | the Unit value                       |

Built-in names are words (`r_eshape`, `t_ally`) so code stays
recognizable; a punctuation mark appears only where it carries APL
meaning (`r_/` reduce, `s_\` scan, `o_-` rotate).

At a glance, versus classic APL:

| Category            | APL                          | X_eTaL                                  |
| ------------------- | ---------------------------- | -------------------------------------- |
| Character set       | APL glyphs                   | ASCII source; Unicode/LaTeX for display |
| Function vs value   | fixed glyph identity         | an underlined letter in the name       |
| Axis specification  | separate glyphs / brackets   | subscript digits, e.g. `o_-_2`         |
| Operators           | `/` `\` `.` etc.             | ordinary curried functions: `'+ r_/ A` |
| Typing              | dynamic                      | static, inferred (Hindley-Milner)      |
| Ambiguous syntax    | resolved by fixed rules      | rejected with an explanation           |
| Core model          | niladic/monadic/dyadic       | curried one-argument functions         |
| Implementation      | C / assembly                 | Rust (CLI + WASM playground)           |

The acceptance test is Conway's Life in one line
(`spec/integration/life-blinker.case`, whose expected generation is
checked against the sw-apl APL\360 reference):

```
u:l_ife := { ('+ r_/_12 -1 0 1 o_-_12 _r) { (_l = 3) + _r * _l = 4 } _r }
```

Read right to left: rotate the board `_r` by every offset in `-1 0 1`
along axes 1 and 2 (`o_-_12`), giving a 3 by 3 arrangement of boards,
and sum over those two axes (`'+ r_/_12`), giving S, each cell plus
its neighbours. The inner lambda then gets S as `_l` and the board as
`_r` and computes `(S = 3) + board * (S = 4)`: born or surviving with
3 neighbours when S is 3, surviving with 2 neighbours when a live cell
has S of 4. The two tests never both hold, so `+` acts as "or".

It runs:

```bash
./target/release/xetal run demos/life.xtl
```

steps a blinker (a vertical bar of three live cells on a 5 by 5 board)
one and two generations:

```
0 0 0 0 0
0 0 0 0 0
0 1 1 1 0
0 0 0 0 0
0 0 0 0 0
0 0 0 0 0
0 0 1 0 0
0 0 1 0 0
0 0 1 0 0
0 0 0 0 0
```

## Status

Early. The lexer (`xetal lex`), the decorated renderer
(`xetal render`), the parser (`xetal parse`) and the canonical
formatter (`xetal fmt`), the desugaring to Core (`xetal core`), type
inference (`xetal type`), a strict evaluator (`xetal eval`,
`xetal run`, `xetal FILE`, which type-check first) and an interactive
session (`xetal repl`) work, with dense arrays, strings, scalar
extension, the structural built-ins, the higher-order built-ins
(reduce, scan, each, table, inner product, compose, swap) and the
search, order and random built-ins, rotate and reverse, and axis
subscripts on any function; the Life one-liner runs.
The language is specified by its test suite as it is built;
the design documents describe the intended language.

## Documentation

- [`docs/input.md`](docs/input.md) -- how to type X_eTaL expressions
- [`docs/lang-choices.md`](docs/lang-choices.md) -- the language decisions
- [`docs/PRD.md`](docs/PRD.md) -- product requirements and milestones
- [`docs/design.md`](docs/design.md) -- language design and decisions register
- [`docs/architecture.md`](docs/architecture.md) -- crates, pipeline, testing
- [`docs/plan.md`](docs/plan.md) -- implementation plan
- `docs/research.txt`, `docs/research2.txt` -- archival design research

## Quick Start

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

`xetal render` shows the decorated form; `--raw` converts it back and
`--latex` prints LaTeX math for a post-processor:

```bash
./target/release/xetal render -e 'x r_ev o_-_2 u:s_quare c:K_ x^2 _l _r @'
./target/release/xetal render --latex -e 'o_-_12 x^2'
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

## A tour of M0 and M1

**M0 -- typing and display.** Source is plain ASCII; `xetal render`
draws it decorated and `--raw` turns the decorated text back into the
same source (see [`docs/input.md`](docs/input.md)).

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

## A tour of M2

**M2 -- static types.** Types are inferred, never written: `xetal type`
prints one line per top-level item, and `xetal eval` and `xetal run`
check the program before anything runs. A niladic function takes Unit,
written `@`:

```
# demos/unit.xtl
u:a_nswer := { @ -> 42 }
u:a_nswer @
```

```bash
./target/release/xetal type demos/unit.xtl       # u:a_nswer : Num a => Unit -> a, then Int
./target/release/xetal run demos/unit.xtl        # 42
./target/release/xetal type demos/factorial.xtl  # u:f_act : Num a => a -> a, then Int
./target/release/xetal eval -e 'u:a_nswer := { @ -> 42 }; u:a_nswer 42'
```

The last command is refused before evaluation:
`error[type-mismatch]: expected Unit, found a number at 26..38`.

Numbers are typed as in Haskell. A literal fits any number type, so
`3 + 2.5` is fine, but a top-level variable has one type: after
`n := 3`, `n` is an Int and `n + 2.5` is a type error. Convert with
`f_loat`:

```bash
./target/release/xetal eval -e 'n := 3; (f_loat n) + 2.5'   # 5.5
```

Comparisons give Bool, which counts as 1 or 0 in arithmetic, and `/`
always gives a Float. The Y combinator's self-applied argument has no
finite type; `--untyped` skips the checker (see above).

## A tour of M3

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

## A tour of M4

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

## Install

```bash
scripts/build-all.sh --release
cp target/release/xetal ~/.local/bin/          # any directory on PATH
ln -sf xetal ~/.local/bin/x_etal               # the alias x_etal
```

With `xetal` on the PATH, `.xtl` scripts run directly:
`./demos/factorial.xtl`.

## Architecture

Component workspaces (`components/<name>/`, each a few small crates)
sharing one `target/` directory:

`base -> lex -> syntax -> core -> types -> eval -> cli / web`

with `render` (raw / decorated / canonical / expanded printers),
`array` (dense arrays and primitive kernels), `hof` (higher-order
built-ins, applying operands through the evaluator) and `search`
(search and order built-ins) alongside. See
[`docs/architecture.md`](docs/architecture.md).

## Development

Development is test-driven and tracked with
agentrail sagas; see
[`CLAUDE.md`](CLAUDE.md) (also `AGENTS.md`) for the agent workflow and
rules.

```bash
scripts/build-all.sh                                # build every component
(cd components/syntax && cargo test)                # test one component
scripts/reg.sh run                                  # reg-rs CLI golden tests
scripts/gate.sh                                     # full pre-commit gate
```

## Related Projects

- [sw-mlpl](https://github.com/sw-ml-study/sw-mlpl) -- Software
  Wrighter's Machine Learning Programming Language, a Rust array
  language inspired by APL, APL2, J, and BQN.
- [sw-apl](https://github.com/sw-vibe-coding/sw-apl) -- a clean-room
  APL interpreter in Rust modelled on APL\360 and IBM 5100 APL, for
  the terminal, a local service and the browser.
- [web-sw-cor24-apl](https://github.com/sw-embed/web-sw-cor24-apl) --
  browser-based APL environment running the sw-cor24-apl interpreter
  on an emulated COR24 CPU via WebAssembly.

## Links

- Blog: [Software Wrighter Lab](https://software-wrighter-lab.github.io/)
- Discord: [Join the community](https://discord.com/invite/Ctzk5uHggZ)
- YouTube: [Software Wrighter](https://www.youtube.com/@SoftwareWrighter)

## Copyright

Copyright (c) 2026 Michael A Wright

## License

MIT. See [`LICENSE`](LICENSE) and [`COPYRIGHT`](COPYRIGHT).
