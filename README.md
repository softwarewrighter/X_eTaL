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

The design target is Conway's Life in one line:

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

## Status

Early. The lexer (`xetal lex`), the decorated renderer
(`xetal render`), the parser (`xetal parse`) and the canonical
formatter (`xetal fmt`), the desugaring to Core (`xetal core`) and a
strict evaluator for scalars (`xetal eval`, `xetal run`, `xetal FILE`)
work; type checking (`xetal type`) and arrays are not implemented yet
and report `error[unsupported]`. The language is specified by its test suite as it is built;
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
cargo build --release
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

## Architecture

Single cargo workspace of small crates:

`base -> lex -> syntax -> core -> types -> eval -> cli / web`

with `render` (raw / decorated / canonical / expanded printers) and
`array` (dense arrays and primitive kernels) alongside. See
[`docs/architecture.md`](docs/architecture.md).

## Development

Development is test-driven and tracked with
agentrail sagas; see
[`CLAUDE.md`](CLAUDE.md) (also `AGENTS.md`) for the agent workflow and
rules.

```bash
cargo test                                          # unit, spec corpus, property tests
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all
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
