# <img src="images/X_eTaL-logo.jpg" alt="X_eTaL" width="160" align="left" style="margin-right:12px"> X_eTaL

**eXperimental eXtensible Typed Array Language** -- LaTeX reversed,
with the X itself decorated.

> LaTeX uses text to produce typography. X_eTaL uses typography to
> express computation.

X_eTaL is a terse, statically typed, functional array language in the
APL / APL2 / J / BQN tradition, implemented in Rust. It uses no
special glyph alphabet: source is plain ASCII typed on a US keyboard,
and **typographic decoration** changes what an ordinary name means.
A plain name is a noun; an underlined name is a function; a subscript
specializes it (for example an axis); a superscript derives a new
function from it (for example reduce or scan).

<br clear="left">

| Raw ASCII  | Displayed as                | Meaning                          |
| ---------- | --------------------------- | -------------------------------- |
| `t`        | t                           | the variable `t`                 |
| `t_`       | t, underlined               | rotate                           |
| `t_2`      | t, underlined, subscript 2  | rotate along axis 2              |
| `+^r`      | +, superscript r            | reduce by `+` (sum)              |
| `+^s_2`    | +, superscript s, sub 2     | running sum along axis 2         |
| `now_@`    | now, underlined, touching @ | apply `now` to Unit              |
| `_l` `_r`  | _l _r                       | left / right lambda argument     |
| `@`        | @                           | the Unit value                   |

Terse and long spellings are the same grammar -- `rotate_2` and `t_2`
are the same function -- so code can be written in words while
learning and abbreviated when fluent.

At a glance, versus classic APL:

| Category            | APL                          | X_eTaL                                  |
| ------------------- | ---------------------------- | -------------------------------------- |
| Character set       | APL glyphs                   | ASCII source; Unicode only for display |
| Function vs noun    | fixed glyph identity         | typographic decoration of any name     |
| Axis specification  | separate glyphs / brackets   | numeric subscript, e.g. `t_2`          |
| Operators           | `/` `\` `.` etc.             | superscript derivations `^r` `^s`      |
| Typing              | dynamic                      | static, inferred (Hindley-Milner)      |
| Ambiguous syntax    | resolved by fixed rules      | rejected with an explanation           |
| Core model          | niladic/monadic/dyadic       | curried one-argument functions         |
| Implementation      | C / assembly                 | Rust (CLI + WASM playground)           |

The design target is Conway's Life in one line:

```
life = { (+^r -1 0 1 t_12 _r) { (_l = 3) + _r * _l = 4 } _r }
```

Read right to left: rotate the board `_r` by every offset in `-1 0 1`
along both axes (`t_12`) and sum the nine boards (`+^r`), giving S,
each cell plus its neighbours. The inner lambda then gets S as `_l`
and the board as `_r` and computes `(S = 3) + board * (S = 4)`: born
or surviving with 3 neighbours when S is 3, surviving with 2 neighbours
when a live cell has S of 4. The two tests never both hold, so `+`
acts as "or".

## Status

Early. The lexer (`xetal lex`) and the decorated renderer
(`xetal render`) work; parsing and evaluation are not implemented
yet, and those commands report `error[unsupported]`. The language is specified by its test suite as
it is built; the design documents describe the intended language.

## Documentation

- [`docs/input.md`](docs/input.md) -- how to type X_eTaL expressions
- [`docs/PRD.md`](docs/PRD.md) -- product requirements and milestones
- [`docs/design.md`](docs/design.md) -- language design and open decisions
- [`docs/architecture.md`](docs/architecture.md) -- crates, pipeline, testing
- [`docs/plan.md`](docs/plan.md) -- implementation plan
- `docs/research.txt`, `docs/research2.txt` -- archival design research

## Quick Start

```bash
cargo build --release
./target/release/xetal --version
./target/release/xetal lex -e 'square = { _r * _r }; square_ 7'
```

`xetal lex` prints one token per line with its byte span:

```
0..6 Noun(square)
7..8 Func(=)
9..10 LBrace
11..13 LamArg(r)
14..15 Func(*)
16..18 LamArg(r)
19..20 RBrace
20..21 Semi
22..29 Func(square)
30..31 Num(7)
```

Source is typed as plain ASCII: a trailing `_` underlines a name
(makes it a function), digits after it are axis subscripts (`t_12`),
and `^word` is a superscript derivation (`+^r`); see
[`docs/input.md`](docs/input.md). Errors carry a code and a byte
span, for example `xetal lex -e '3-1'` reports
`error[ambiguous-minus]` at `1..2`.

`xetal render` shows the decorated form (underline, subscript and
superscript glyphs) of the table above; `--raw` converts it back and
`--latex` prints LaTeX math for a post-processor:

```bash
./target/release/xetal render -e 't t_ t_2 +^r +^s_2 _l _r @'
./target/release/xetal render --latex -e 't_12 +^r'
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
