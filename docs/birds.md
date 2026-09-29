# The aviary

The combinators Raymond Smullyan names in *To Mock a Mockingbird*
(1985), which the standard library `Combinators` provides. Each bird
is a function of curried arguments that only rearranges, repeats,
drops or groups them: `B x y z = x (y z)` means that `B` applied to
x, y and z gives x applied to (y applied to z).

## Spelling

A bird is its letter as a function name, exported by the library
(`l:B_` in the library, `c:B_` after `"c:" u_se< "Combinators"`).
Smullyan's variants are spelled from the letter:

- a digit follows the letter: `B_1` for B1, `Q_4` for Q4 (a digit
  after the underlined letter is part of the name; an axis subscript
  would be `B__1`, lang-choices D-8);
- a star is `s`: `C_s` for C* ("C once removed"), `C_ss` for C**
  ("C twice removed");
- the hat is `h`: `E_h` for the bald eagle (E with a hat).

## Typability

X_eTaL is typed (Hindley-Milner). A bird that applies an argument to
itself, such as M (`M x = x x`), has no finite type: the checker
rejects it with `error[infinite-type]`. Those birds run only with
`--untyped`. Y is the exception: the library defines it by recursion,
`l:Y_ := { f_ -> f_ l:Y_ 'f_ }`, of type `(a -> a) -> a`, which works
because a function's parameter is lazy. The textbook Y (built from
self-application) and the untyped birds are shown in an untyped demo.

In the tables, the column Typed says whether the bird type-checks.

## The birds

| Bird | Smullyan's name | Definition | Typed | Spelling |
| ---- | --------------- | ---------- | ----- | -------- |
| B | Bluebird | `B x y z = x (y z)` | yes | `B_` |
| B1 | Blackbird | `B1 x y z w = x (y z w)` | yes | `B_1` |
| B2 | Bunting | `B2 x y z w v = x (y z w v)` | yes | `B_2` |
| B3 | Becard | `B3 x y z w = x (y (z w))` | yes | `B_3` |
| C | Cardinal | `C x y z = x z y` | yes | `C_` |
| D | Dove | `D x y z w = x y (z w)` | yes | `D_` |
| D1 | Dickcissel | `D1 x y z w v = x y z (w v)` | yes | `D_1` |
| D2 | Dovekie | `D2 x y z w v = x (y z) (w v)` | yes | `D_2` |
| E | Eagle | `E x y z w v = x y (z w v)` | yes | `E_` |
| E-hat | Bald eagle | `Eh x y1 y2 y3 z1 z2 z3 = x (y1 y2 y3) (z1 z2 z3)` | yes | `E_h` |
| F | Finch | `F x y z = z y x` | yes | `F_` |
| G | Goldfinch | `G x y z w = x w (y z)` | yes | `G_` |
| H | Hummingbird | `H x y z = x y z y` | yes | `H_` |
| I | Identity bird | `I x = x` | yes | `I_` |
| J | Jay | `J x y z w = x y (x w z)` | yes | `J_` |
| K | Kestrel | `K x y = x` | yes | `K_` |
| L | Lark | `L x y = x (y y)` | no | `L_` |
| M | Mockingbird | `M x = x x` | no | `M_` |
| M2 | Double mockingbird | `M2 x y = x y (x y)` | no | `M_2` |
| O | Owl | `O x y = y (x y)` | yes | `O_` |
| Q | Queer bird | `Q x y z = y (x z)` | yes | `Q_` |
| Q1 | Quixotic bird | `Q1 x y z = x (z y)` | yes | `Q_1` |
| Q2 | Quizzical bird | `Q2 x y z = y (z x)` | yes | `Q_2` |
| Q3 | Quirky bird | `Q3 x y z = z (x y)` | yes | `Q_3` |
| Q4 | Quacky bird | `Q4 x y z = z (y x)` | yes | `Q_4` |
| R | Robin | `R x y z = y z x` | yes | `R_` |
| S | Starling | `S x y z = x z (y z)` | yes | `S_` |
| T | Thrush | `T x y = y x` | yes | `T_` |
| U | Turing bird | `U x y = y (x x y)` | no | `U_` |
| V | Vireo | `V x y z = z x y` | yes | `V_` |
| W | Warbler | `W x y = x y y` | yes | `W_` |
| W1 | Converse warbler | `W1 x y = y x x` | yes | `W_1` |
| Y | Sage bird | `Y x = x (Y x)` | yes, by recursion | `Y_` |

The once and twice removed birds pass their first argument, or
first two, through unchanged and act as the plain bird on the rest:

| Bird | Definition | Typed | Spelling |
| ---- | ---------- | ----- | -------- |
| C* | `C* x y z w = x y w z` | yes | `C_s` |
| R* | `R* x y z w = x z w y` | yes | `R_s` |
| F* | `F* x y z w = x w z y` | yes | `F_s` |
| V* | `V* x y z w = x w y z` | yes | `V_s` |
| W* | `W* x y z = x y z z` | yes | `W_s` |
| C** | `C** x y z w v = x y z v w` | yes | `C_ss` |
| R** | `R** x y z w v = x y w v z` | yes | `R_ss` |
| F** | `F** x y z w v = x y v w z` | yes | `F_ss` |
| V** | `V** x y z w v = x y v z w` | yes | `V_ss` |
| W** | `W** x y z w = x y z w w` | yes | `W_ss` |

That is 38 typed birds in the library (Y among them) and 4 untyped
ones (L, M, M2, U), which are in the untyped demo,
`demos/birds-untyped.xtl`, with the textbook Y, Z (the sage bird for
strict evaluation) and Turing's fixed point U U. `demos/combinators.xtl`
shows the library at work (`just show demos/combinators.xtl`).
Birds from other aviaries (the kite `K I`, Curry's phoenix and psi)
are not in Smullyan's list; they are one line each in a user file.
