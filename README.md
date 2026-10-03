<p align="center">
  <img src="images/modern-xetal-logo.jpg" alt="X_eTaL: X underlined, a raised e, T, a raised a, L" width="360">
</p>

<p align="center">
  <b><a href="https://softwarewrighter.github.io/X_eTaL/">Try it live in your browser</a></b>
  -- the editor, running in WebAssembly: type ASCII, see it decorated, run it
</p>

# X_eTaL

XeTaL is an experimental, statically typed array language that asks:
what would APL look like if it were designed today for composition,
readability, tooling, and machine learning?

Rather than inventing another collection of syntax, XeTaL explores a
specific design space: APL-style whole-array programming, functional
composition, and shape-aware operations, expressed with ASCII source
that can be rendered typographically without changing the underlying
program.

The goal isn't to replace existing languages. It's to make the powerful
ideas of array programming easier to read, reason about, type-check,
visualize, and experiment with.

In the live demo, type ASCII on the left and watch it drawn decorated
on the right; Run shows the output, Open lists the demos, the classics, the
libraries and your saved files in collapsible groups, and Save keeps them in the browser.
Output appears as it is printed, with Stop while a program runs;
Notebook runs it showing each statement above its output, and Step runs
one statement at a time ([notebooks](docs/notebook.md)). On a
phone the toolbar wraps and the panes stack, and the demo installs as
an app (Add to Home Screen) that works offline.

[![The live demo: the ASCII source, its decorated form, and the types below](images/live-demo.png)](https://softwarewrighter.github.io/X_eTaL/)

<p align="center">
  <img src="images/live-demo-phone.png" alt="The live demo on a phone: the toolbar wrapped, the panes stacked" width="260">
</p>

**eXperimental Extensible Typed Array Language** -- LaTeX reversed,
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


| Raw ASCII     | Displayed as                          | Meaning                              |
| ------------- | ------------------------------------- | ------------------------------------ |
| `x`           | x                                     | the variable `x`                     |
| `r_ev`        | rev, r underlined                     | the built-in function reverse        |
| `o_-_2`       | o-, o underlined, subscript 2         | rotate along axis 2                  |
| `'+ r_/ A`    | quote +, then r/ with r underlined    | reduce A by plus (APL `+/A`)         |
| `u:s_quare`   | superscript u, square, s underlined   | a user-defined function              |
| `c:K_`        | superscript c, K underlined           | K from the combinator library        |
| `x^2`         | x squared                             | exponent on a value                  |
| `x^0.5`       | x, raised 0.5 (a middle dot as point) | a decimal exponent: the square root  |
| `_l` `_r`     | APL alpha and omega                   | left / right lambda argument         |
| `:=` `;`      | a left arrow, a black diamond         | binding, statement separator         |
| `@`           | @                                     | the Unit value                       |
| `[f_ g_ h_]`  | in square brackets                    | a train: `(f_ x) g_ (h_ x)`          |
| `"ab" "cde"`  | two strings side by side              | a nested vector of two strings       |

Built-in names are words (`r_eshape`, `t_ally`) so code stays
recognizable; a punctuation mark appears only where it carries APL
meaning (`r_/` reduce, `s_\` scan, `o_-` rotate).

At a glance, versus classic APL:

| Category            | APL                          | X_eTaL                                  |
| ------------------- | ---------------------------- | -------------------------------------- |
| Character set       | APL glyphs                   | ASCII source; Unicode/LaTeX for display |
| Function vs value   | fixed glyph identity         | an underlined letter in the name       |
| Axis specification  | separate glyphs / brackets   | subscript digits, e.g. `o_-_2`         |
| Transpose           | the transpose glyph          | `o_\ A`, `2 1 3 t_ranspose A`          |
| Operators           | `/` `\` `.` etc.             | ordinary curried functions: `'+ r_/ A` |
| Typing              | dynamic                      | static, inferred (Hindley-Milner)      |
| Ambiguous syntax    | resolved by fixed rules      | rejected with an explanation           |
| Core model          | niladic/monadic/dyadic       | curried one-argument functions         |
| Trains              | forks and atops (Dyalog, J)  | `[F G H]` fork, `[F G]` atop           |
| Nested arrays       | APL2 boxes, DISPLAY          | `Box a` types, APL2 DISPLAY printing   |
| Implementation      | C / assembly                 | Rust (CLI + WASM playground)           |

How to read it, on one page: what each decoration says about a name
(function or value, yours or a library's), imports, definitions,
passing a function, axes, powers, lambdas, system names, guards, trains,
and what you type for each. Every sample on it is drawn by `xetal render
--html` itself (`scripts/poster.py`), so it cannot drift from the
language; the [poster as a web page](https://softwarewrighter.github.io/X_eTaL/poster/)
is linked from the live demo too.

[![XeTaL syntax you can see: name decorations, importing a library, definitions, applying vs. passing functions, axes, power, lambdas, system names, the ? symbol, ASCII input vs. the rendered form, and trains](images/xetal-syntax-poster.png)](https://softwarewrighter.github.io/X_eTaL/poster/)

The acceptance test is Conway's Life in one line
(`spec/integration/life-blinker.case`, checked against the sw-apl
APL\360 reference); `just life` runs `demos/life.xtl`, which steps a
blinker and a glider with it:

```
u:l_ife := { ('+ r_/_12 -1 0 1 o_-_12 _r) { (_l = 3) + _r * _l = 4 } _r }
```

That is the line as typed. Here it is drawn decorated, as `just pp`
shows it, with each part explained (generated by `xetal diagram` from
[`docs/diagrams/life.notes`](docs/diagrams/life.notes); every callout is
anchored to the tokens it points at):

![The Life line decorated, with callouts explaining each part, read right to left](images/life-annotated.svg)

Read right to left: rotate the board by every offset in `-1 0 1` along
axes 1 and 2, giving a 3 by 3 arrangement of boards, and sum over those
two axes, giving S, each cell plus its neighbours. The inner function
gets S as its left argument and the board as its right, and computes
`(S = 3) + board * (S = 4)`: a cell lives next when S is 3, or when it
is alive and S is 4.


## The name

Say it **Ecks-e-tal**, as the file type `.xtl` is said eks-tee-ell.
Spell it with the logo (an underlined X, a raised e, T, a raised a, L)
where it can be drawn, `X_eTaL` in plain ASCII, XeTaL in a sentence,
and `xetal` for the binary and the code. [`docs/name.md`](docs/name.md)
lists every spelling and why the logo is itself XeTaL (`X_ e:T a:L`).

## Seeing it

Source is typed as ASCII and shown decorated. `xetal edit FILE` puts
the two side by side, with the types (or the first error) below as
you type and the results on Ctrl-R. Tab moves between the panes (the
current one has a thick border) and Ctrl-T zooms it to the full
screen and back:

![xetal edit: the ASCII source on the left, its decorated form on the right, results below](images/editor.png)

`just tour` runs the commented language tour, `demos/tour.xtl`, as a
notebook: each statement decorated, followed by its output. A part of
it (the full output is on the [tours page](docs/tour.md)):

![Part of the language tour: each statement decorated and followed by its output](images/tour-excerpt.png)

`just slow-show FILE` does the same with a pause after each statement,
for watching; here is the whole tour as it scrolls by (also as a
smaller, sharper [WebM video](videos/tour.webm)):

![The language tour running as a notebook, scrolling by](videos/tour.webp)

## Status

[`docs/status.md`](docs/status.md) is the generated table of what works
today: every built-in, language decision and standard library, marked
works, partial or planned, with the spec cases behind it (regenerated
by `just status`, checked by the gate). In short:

Early, and specified by its test suite as it is built. Working: the
whole pipeline (lexer, parser, formatter, Core, Hindley-Milner type
inference, a strict evaluator), dense 1-origin arrays with strings,
scalar extension and the structural built-ins, the higher-order
built-ins (reduce, scan, each, table, inner product, compose, swap,
power), search, order and random built-ins, rotate, reverse and axis
subscripts on any function, function power (`f_^3`), the Life
one-liner, trains (forks, atops, hooks with the tacks, and errors
that say what the train means where it fails), replicate (`r_eplicate`),
encode and decode, catenate along any axis (`c_at_2`), transpose (`o_\`
reverses the axes, `t_ranspose` permutes them, `o_\_23` swaps two), whole-array match
(`m_atch`), nested arrays (string strands, `e_nclose`, `d_isclose`,
`p_artition`, `m_ap`, printed as APL2's DISPLAY, `d_isplay`, `--box`,
`--ascii`), files, the keyboard and numbers as text (`[]N_GET`,
`[]N_PUT`, `[]R_EAD`, `f_ormat`, `n_umbers`), trigonometry, pictures
(`[]G_RID` grids and `[]P_ATH` paths as SVG, animated by frames,
shown with `[]S_HOW`), and libraries imported with `u_se<`: the
standard libraries `Stats`, `Combinators` (Smullyan's birds), `Maybe`,
`TTTML` (a machine that learns tic-tac-toe) and `Turtle` (turtle
graphics as arrays) are built in. The decorated views: `xetal render
--color`, streaming notebook runs laid out as an APL session, the
editor, a REPL that draws each line decorated as you type, and
annotated diagrams, and a live web demo. What comes next is in
[`docs/plan.md`](docs/plan.md).

## Prerequisites

- [Rust](https://rustup.rs) (stable, via rustup) and
  [`just`](https://github.com/casey/just) (`brew install just` or
  `cargo install just`): enough to build, run and edit programs.
- For the live demo in the browser: the WebAssembly target and
  [trunk](https://trunkrs.dev):
  `rustup target add wasm32-unknown-unknown`, then `brew install trunk`
  or `cargo install trunk`.
- For the full pre-commit gate (maintainers): `reg-rs` (the CLI
  goldens), `sw-checklist` and `sw-markdown-checker`; Emacs for the
  literate documents; vhs, ffmpeg and ImageMagick to regenerate the
  screenshots and the video; Google Chrome for the live demo's
  screenshot; Node.js with KaTeX (`cd tools/katex && npm ci`) to check
  every line's LaTeX and draw the gallery (`scripts/latex-gallery.sh`).

Vendoring X_eTaL into another repository: `xetal --version` and the
live demo's footer report the commit they were built from, found with
`git` where they are built, which inside another repository is that
repository's commit. Set `XETAL_BUILD_SHA` (and, if wanted,
`XETAL_BUILD_HOST` and `XETAL_BUILD_TIMESTAMP`) when building to report
the vendored commit instead.

## Quick Start

With [`just`](https://github.com/casey/just) installed (`just` alone
lists the tasks):

```bash
just tour                                       # the language tour, each line with its output
just eval "'+ r_/_2 2 3 r_eshape r_ange 6"      # row sums: 6 15
just life                                       # Life: a blinker and a glider
just repl                                       # drawn decorated as you type; Up/Down history; Ctrl-D ends
just show demos/stats.xtl                       # a program using the built-in Stats library, as a notebook
just show demos/combinators.xtl                 # Smullyan's birds from the built-in Combinators library
just show demos/monads.xtl                      # the Maybe monad: safe division chained by bind
just tttml                                      # TTTML: learns tic-tac-toe by playing itself (optimized build)
just tttml-train                                # TTTML: train, then save the model in work/tttml.model
just tttml-play                                 # TTTML: play the saved model; you are X, type squares 1 to 9
just draw demos/classics/life-drawn.xtl         # Life animated, as SVG pictures opened in the browser
just pp demos/factorial.xtl                     # print a file decorated and highlighted
just edit demos/life.xtl                        # ASCII left, decorated right
just serve                                      # the live demo locally, at http://127.0.0.1:8095/
just pages                                      # build the live demo into pages/ (published by a workflow)
```

`just run` and `just eval` pass flags through to `xetal`
(`just run --echo FILE`). Without `just`, build with
`scripts/build-all.sh --release` and call `./target/release/xetal`;
the [M0 and M1 tour](docs/tour-m0-m1.md) walks through every stage
(`lex`, `render`, `parse`, `fmt`, `core`, `type`, `eval`).

## Fonts

Use **JuliaMono**, a free monospace font that has every character the
decorated display uses (the combining underline, small raised letters
for namespaces, raised digits for powers and exponents, APL's lamp,
alpha and omega, arrows, the diamond and the math signs), with all
raised digits at one height. It comes from the Julia language
community but is an ordinary font; Julia is not needed.

To install it on macOS, with [Homebrew](https://brew.sh):

```bash
brew install --cask font-juliamono
```

or without Homebrew: download `JuliaMono-ttf.zip` from the
[JuliaMono releases](https://github.com/cormullion/juliamono/releases),
unzip it, double-click `JuliaMono-Regular.ttf` and click Install in
Font Book. Then quit and restart the terminal (iTerm reads its font
list at launch) and choose JuliaMono (iTerm: Settings, Profiles,
Text, Font). To keep another font for plain text, iTerm's "Use a
different font for non-ASCII text" in the same tab can take just the
decorated characters from JuliaMono.

Other fonts, checked against the font files:

| Font | Characters it has | Notes |
| ---- | ----------------- | ----- |
| JuliaMono (free) | all | recommended |
| DejaVu Sans Mono (free) | all | `brew install --cask font-dejavu` |
| Andale Mono, PT Mono (macOS) | about a third | work on macOS, which borrows the rest from other fonts; the raised 1, 2 and 3 are the font's own and the other raised digits are borrowed, so a raised 10 shows its 1 and 0 at different heights |
| Menlo (macOS) | all but the lamp | a system font that iTerm does not offer on current macOS |
| JetBrains Mono, BQN386 | most | no small raised letters or combining underline of their own |
| Monaco (macOS) | all but six | avoid: its arrow, alpha, omega, lamp, raised c and quad come out broken |

## Documentation

- [`docs/tour.md`](docs/tour.md) -- the language tour and the milestone
  tours (M0 to M6)
- [The literate documents as web pages](https://softwarewrighter.github.io/X_eTaL/literate/)
  -- the Org documents below exported to HTML, with an index
- [`docs/literate/tour.org`](docs/literate/tour.org) -- the language tour
  as a literate Org document, every block run and its result recorded
  (`docs/emacs/`: `xetal-mode` and `ob-xetal` for Org Babel)
- [`docs/literate/hello.org`](docs/literate/hello.org) -- a library of
  your own: write `userlibs/Hello.xtl`, import it, call it
- [`docs/literate/life.org`](docs/literate/life.org) -- Conway's Life,
  the one line built up a piece at a time
- [`docs/literate/libraries.org`](docs/literate/libraries.org) -- what a
  library is, and the standard libraries (Stats, Maybe, Combinators,
  TTTML) at work
- [`docs/literate/birds.org`](docs/literate/birds.org) -- the Combinators
  library run as you read: Smullyan's birds, with diagrams of the key
  definitions
- [`docs/literate/tttml.org`](docs/literate/tttml.org) -- TTTML, a machine
  that learns tic-tac-toe by playing itself, as a literate program over the
  `TTTML` library
- [`docs/literate/classics.org`](docs/literate/classics.org) -- classic APL
  programs (Pascal's triangle, Life, turtle graphics) run as you read, with
  their pictures; the animated ones play on the web page
- [`docs/literate/hanoi.org`](docs/literate/hanoi.org) -- the Tower of Hanoi
  three ways: recursion, currying with the combinators, and every move at
  once from the bits of the move number, checked to agree and drawn
- [`docs/literate/trains.org`](docs/literate/trains.org) -- trains beside
  the Combinators birds that do the same (Bluebird and atop, Starling and
  hook, Phoenix and fork), checked to agree, and how to read a train's
  errors
- [`docs/literate/duck.org`](docs/literate/duck.org) -- swimming ducks by
  rotate, and joining frames along axis 2 two ways: a recursion, and
  `c_at_2`
- [`docs/reference.md`](docs/reference.md) -- every built-in function, with
  examples (and, for the ones that work along an axis, the default axis,
  axis 1 written out, and another axis)
- [`docs/idioms.md`](docs/idioms.md) -- everyday idioms in X_eTaL beside Java,
  JavaScript, Python, C++ and Rust, and beside APL2, Dyalog APL, J and BQN
- [`docs/classics.md`](docs/classics.md) -- the classic APL programs
  (Pascal's triangle, Life, tic-tac-toe, ...) as commented X_eTaL notebooks
- [`docs/leetcode.md`](docs/leetcode.md) -- LeetCode problems answered the
  array way, with the cousins the same arrays answer
- [`docs/dogfooding.md`](docs/dogfooding.md) -- the language features and
  fixes the demos asked for: added, planned, and friction kept for now
- [`docs/permission-response.md`](docs/permission-response.md) -- XeTaL's
  answers to ngn's [Array Language Implementation Permission
  Request](https://ngn.codeberg.page/funny/reg.html)
- [`docs/notebook.md`](docs/notebook.md) -- notebooks: a program shown
  a statement at a time, its output under each, and stepping, in the
  live demo, on the command line and in the literate documents
- [`docs/wish-list.md`](docs/wish-list.md) -- ideas the language could
  use that no saga plans yet, prioritized and sized (ideas, not
  commitments)
- [`docs/name.md`](docs/name.md) -- how to say XeTaL (Ecks-e-tal) and
  every way it is spelled
- [`docs/why-another-language.md`](docs/why-another-language.md) --
  XeTaL fills in the [Programming Language
  Checklist](https://www.mcmillen.dev/language_checklist.html)
- [`docs/xetal-apl-skeptics-response.md`](docs/xetal-apl-skeptics-response.md)
  -- XeTaL plays the APL Wiki's complaint-bingo card
  ([Humour](https://aplwiki.com/wiki/Humour))
- [`docs/input.md`](docs/input.md) -- how to type X_eTaL expressions
- [`docs/birds.md`](docs/birds.md) -- the aviary: Smullyan's combinators,
  which type-check, and how the `Combinators` library spells them
- [`docs/lang-choices.md`](docs/lang-choices.md) -- the language decisions
- [`docs/PRD.md`](docs/PRD.md) -- product requirements and milestones
- [`docs/design.md`](docs/design.md) -- language design and decisions register
- [`docs/architecture.md`](docs/architecture.md) -- components, pipeline, testing
- [`docs/plan.md`](docs/plan.md) -- implementation plan and retrospectives
- `docs/research.txt`, `docs/research2.txt` -- archival design research

## Install

```bash
just install                                   # or: just install /some/dir/on/PATH
```

which builds the optimized binary and copies it, with the `x_etal`
link, into `~/.local/bin`. Without `just`:

```bash
scripts/build-all.sh --release
cp target/release/xetal ~/.local/bin/          # any directory on PATH
ln -sf xetal ~/.local/bin/x_etal               # the alias x_etal
```

An installed copy does not update itself: run `just install` again
after pulling changes. The `just` recipes run the optimized build
(`target/release/xetal`), which they rebuild when a source changed
(saying "building xetal...").

With `xetal` on the PATH, `.xtl` scripts run directly:
`./demos/factorial.xtl`.

## Architecture

Component workspaces (`components/<name>/`, each a few small crates)
sharing one `target/` directory:

`base -> lex -> syntax -> core -> types -> eval -> cli / web`

with `render` (raw / decorated / canonical / expanded printers),
`array` (dense arrays and primitive kernels), `hof` (higher-order
built-ins, applying operands through the evaluator), `search`
(search and order built-ins), `axes` (rotate, reverse, axis
subscripts), `system` (files, the keyboard, numbers as text), `view`
(the view model every display draws from: styled,
span-mapped segments, and values laid out as grids), `tui` (the
editor) and `line` (the REPL's line editor) alongside. See
[`docs/architecture.md`](docs/architecture.md).

## Development

Development is test-driven and tracked with
agentrail sagas; see
[`CLAUDE.md`](CLAUDE.md) (also `AGENTS.md`) for the agent workflow and
rules.

```bash
just build                                          # build every component
just test                                           # every component's tests
(cd components/syntax && cargo test)                # test one component
just reg                                            # reg-rs CLI golden tests
just gate                                           # full pre-commit gate
```

Each recipe calls a script in `scripts/` (`build-all.sh`, `gate.sh`,
`reg.sh`, `check-locks.sh`), which work without `just` too. The gate
also runs `scripts/just-smoke.sh`, which runs every recipe (the file
recipes on every demo) and fails on a recipe it has no test for.

## The X_eTaL repositories

The language lives here; programs and libraries written in it, and
native extensions for it, each have a repository of their own:

- [X_eTaL-demos](https://github.com/softwarewrighter/X_eTaL-demos) --
  small programs worth watching: a cellular automaton, fractals, a
  reaction-diffusion texture, a neural network seeing a digit, tokens
  routed to experts in a sparse model
  ([live catalog](https://softwarewrighter.github.io/X_eTaL-demos/)).
- [X_eTaL-games](https://github.com/softwarewrighter/X_eTaL-games) --
  board games, puzzles, simulations and quizzes whose rules are
  array-shaped, each playable on the command line
  ([live catalog](https://softwarewrighter.github.io/X_eTaL-games/)).
- [X_eTaL-libraries](https://github.com/softwarewrighter/X_eTaL-libraries)
  -- libraries written in X_eTaL itself, typed, tested and documented:
  text, sets, number theory, combinatorics, matrices, randomness,
  formatting, dates.
- [X_eTaL-extensions](https://github.com/softwarewrighter/X_eTaL-extensions)
  -- small native Rust libraries for what the interpreter cannot do by
  itself (a clock, hashing, regular expressions, fast linear algebra,
  image files), each used from X_eTaL like any other library.

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
