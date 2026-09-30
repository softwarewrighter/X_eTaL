# A tour of M6

Part of the X_eTaL milestone tours ([index](tour.md), [README](../README.md)); previous: [A tour of M6b](tour-m6b.md).
Commands run from the repository root; `just` runs the optimized build,
and `./target/release/xetal` is that binary.

**M6 -- combinators, and what grew around them.** The combinators
arrive as a library, with function power beside them; a second
library shows a monad; TTTML, a program that learns tic-tac-toe,
brings files, the keyboard and numbers as text; and the notebook, the
editor and the documentation grow to show it all.

## The Combinators library

`Combinators` holds every bird Raymond Smullyan names in *To Mock a
Mockingbird* that type-checks, under its letter (`docs/birds.md` lists
them all, and decodes a few in annotated diagrams). `xetal type` on a
library file lists its exports and their types:

```bash
./target/release/xetal type lib/Combinators.xtl
```

begins

```
l:I_ : a -> a
l:K_ : a -> b -> a
l:T_ : a -> (a -> b) -> b
l:W_ : (a -> a -> b) -> a -> b
```

`just show demos/combinators.xtl` runs the library as a notebook: K
keeps its first argument, C swaps arguments (`10 '- c:C_ 3` is -7), B
composes, W uses a value twice, S, T and V hold and pass values, and Y,
the sage bird, makes recursion from a function handed itself (a
factorial of 10 is 3628800). The birds that apply an argument to
itself have no finite type; `demos/birds-untyped.xtl` runs them with
`--untyped` (the mockingbird, the textbook Y, Z, and Turing's fixed
point).

## Function power

A superscript on a function name repeats the function: `n_eg^3 5` is
n_eg applied three times, -5, and the Life demo steps its blinker with
`u:l_ife^2`. `n 'f_ p_ower x` does the same with a computed count:

```bash
./target/release/xetal eval -e "3 'n_eg p_ower 5"
```

prints `-5`.

## A monad: Maybe

The `Maybe` library, Church-encoded, makes a computation that can fail
into a value: `demos/monads.xtl` divides safely and chains steps with
`b_ind`, the first failure giving the default.

```bash
./target/release/xetal run demos/monads.xtl
```

prints `25.0`, `-1.0`, `2.5`, `-1.0`, `30.0`, `4.0` and `0.0`.

## TTTML: a machine that learns tic-tac-toe

Ported from the TTTML workspace of sw-apl's library 1, as the library
`TTTML` and a literate program (`docs/literate/tttml.org`). `just tttml`
trains it by playing itself (2000 games, about ten seconds, printing
its progress), then plays a random player as X and as O, and itself
from random first moves:

```
games to go, positions known:
1750 559
...
0 682
682
48 0 2
45 0 5
0 0 50
```

The rows are games won, lost and drawn: it never loses, and against
itself every game is a draw. `just tttml-train` saves the model it
learned as text in `work/tttml.model`; `just tttml-play` reads it back
and plays you, reading your moves from the keyboard.

## Files, the keyboard and numbers as text

System names are `[]` touching an uppercase name, drawn with APL's
quad: `t []N_PUT path` writes text to a file, `[]N_GET path` reads one,
and `[]R_EAD @` reads a line typed at the keyboard. `f_ormat` turns a
value into the text it prints as, and `n_umbers` reads the numbers in a
text back:

```bash
./target/release/xetal eval -e 'n_umbers f_ormat 2 2 r_eshape 1.5 2 3 4'
```

prints `1.5 2.0 3.0 4.0`.

## The notebook and the editor

`just show FILE` runs a file as a notebook laid out as an APL session:
each statement indented six spaces, its output flush left. It streams:
the file runs once, each statement is shown just before it runs, and
its output appears as it is written, so a long computation can report
its progress. In the editor (`just edit FILE`), Tab moves between the
panes, the current one drawn with a thick bright border, and Ctrl-T
zooms a pane to the full screen and back.

## Seeing and reading the language

- `xetal diagram NOTES` draws a line of source decorated, with
  callouts anchored to its tokens; the README's Life line and the
  diagrams in `docs/birds.md` are made this way, and the gate checks
  they are current.
- `docs/reference.md` explains every built-in, its examples run (and,
  for the ones that work along an axis, the default axis, axis 1
  written out, and another axis).
- `docs/idioms.md` sets X_eTaL beside Java, JavaScript, Python, C++
  and Rust, and beside APL2, Dyalog APL, J and BQN.
- `demos/keys.xtl` shows every input form drawn beside what is typed.
- A decimal exponent is now raised like any other, its point drawn as
  a middle dot: `x^0.5` shows as a raised 0, dot, 5.
