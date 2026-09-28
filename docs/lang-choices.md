# X_eTaL -- Language Choices

Status: DECISIONS IN PROGRESS. Each entry below was chosen by the
user, one question at a time. Nothing here is implemented yet: the
implemented lexer and renderer, `docs/design.md` sections 2, 3 and 8,
and `docs/input.md` still describe the earlier syntax. Once all
decisions are made and later ones no longer change earlier ones, this
document becomes the specification, and design.md, input.md, the
code, the spec cases and the goldens are updated to match.

This document supersedes `docs/syntax-proposal.md`.

## 1. Principles

- **P1. Code is read more often than written.** Prefer the spelling
  that reads clearly.
- **P2. A definition is written once and used many times.** Verbose
  definitions are fine; terseness matters at the point of use.
- **Letters mainly.** Names are made of letters; decoration marks
  namespace and function versus variable. Any of the ~80 keys of a US
  keyboard may be used, but code should stay recognizable. A single
  letter cannot stand for every word starting with it.
- **One key, one job.** No key has two unrelated meanings.
- **One spelling per meaning.** Redundant spellings are rejected.
- **Ambiguity is an error.** The parser never picks a reading; class
  comes from tokens, never from types.
- **ASCII in, rich display out.** Source is ASCII; the display may use
  Unicode and a subset of LaTeX, and may render a multi-character
  token as one glyph (a ligature).
- **Array language surface, functional core.** Right-to-left, no
  precedence, strands, pervasive scalar functions and axes on the
  surface; curried one-argument functions underneath.

## 2. Functions, values and application

| #  | Decision |
| -- | -------- |
| F1 | Every function takes one argument. There is no ambivalence: each function has exactly one arity, and dyadic use is currying (`X f Y` is `App(App(f, X), Y)`). Monadic meanings get their own names (negate `n_eg`, reverse `r_ev`). |
| F2 | The underline is part of a function *name*. A name with an underlined letter is a function name; a name without one is a variable. The two are separate namespaces: `o\|` (if it were legal) and `o_\|` are unrelated. |
| F3 | Juxtaposition applies: `f_ x`, `x f_ y`. |
| F4 | A quote passes a function without applying it: `'r_/`, `'+`. Quoting a variable is an error (a variable is already a value). |
| F5 | A function held in a value is applied by an underline after a lambda argument or a closing parenthesis: `_l_ x`, `(expr)_ x`, and `(f)_ x` for a variable `f`. |
| F6 | Functions that take functions put them first, data last: `'+ r_/ A` is `reduce '+ A`; rotate takes the amount first: `1 o_- A`. Swapping order is done with a flip combinator (C, APL's commute), e.g. `u:o_ver := s_wap 'r_/` then `A u:o_ver '+`. |
| F7 | There is no separate operator class. Reduce, scan, each and table are ordinary curried functions; the function-first convention gives the APL look (`'+ r_/ A` for APL `+/A`). |

## 3. Names

| #  | Decision |
| -- | -------- |
| N1 | A name starts with a letter. A variable name is letters and digits only (`x`, `n`, `board2`). |
| N2 | A function name contains exactly one `_`, directly after a letter, which underlines that letter (`r_ev`, `s_quare`, `o_-`). |
| N3 | A function name may end in one punctuation character from `\| - / \ + * < > ~ ! ? % $ &` (`r_/`, `o_-`, `e_mpty?`). |
| N4 | A mutable variable name ends in `!` (see M2). |
| N5 | Namespaces are a leading prefix: `u:` for the user's definitions, `c:` for the combinator library; no prefix means the system (built-ins). The display shows the prefix as a leading superscript. Plain variables (`x`, `n`) are in the user namespace implicitly. |
| N6 | User functions are defined and called with the same full name: `u:l_ife := { ... }`, `u:l_ife board`. |
| N7 | Aliasing a library namespace (Python-style `as`) is wanted when two libraries share a default letter; syntax to be decided. |

## 4. Decorations

| #  | Decision |
| -- | -------- |
| D-1 | Trailing superscript key is `^`, so `^` never appears inside a name. |
| D-2 | An exponent may follow a value token (number, variable, lambda argument) or a parenthesized expression: `x^2`, `_r^2`, `(a + b)^2`. |
| D-3 | It binds to the token it touches: `1 2 3^2` is `1 2 9`; `(1 2 3)^2` squares all three. |
| D-4 | Any number literal may be an exponent, including negative and decimal (`x^-1`, `x^0.5`). An Int base with a whole non-negative exponent gives Int; otherwise Float. A negative base with a fractional exponent is an error, as is integer overflow. LaTeX renders what Unicode cannot. |
| D-5 | For now exponents are literal numbers only (phased); `x^n` is an error with a hint to use the power function. Variable or expression exponents may be added later. |
| D-6 | Power with a computed exponent is the spaced symbol `^`: `x ^ n`, `2 ^ (k - 1)`. Touching `^` is the literal superscript; spaced `^` is the function (like `-1` versus `- 1`). |
| D-7 | Superscripts on functions (including symbols) are reserved and are an error for now. Function power (f squared = apply twice, inverse) is the leading candidate for later. |
| D-8 | An axis subscript is `_` followed by digits after a function name: `r_/_2`, `o_-_12`. Because a function name has exactly one underline, a later `_` always starts the subscript (`r_2` is the function `r2`; the function `r` on axis 2 is `r__2`). |
| D-9 | One digit per axis, axes 1 to 9: `_12` means axes 1 and 2. A spelling for axes above 9 is reserved for later. |

## 5. Arrays and axes

| #  | Decision |
| -- | -------- |
| A1 | Leading-axis theory: with no subscript, every axis-taking function acts on the first axis (as in J and BQN). |
| A2 | Rotate is `o_-` (like APL's circle-minus, first-axis rotate). `o_\|` is reserved; `o_\` (transpose) and `o_/` are future candidates for the mirror family. |
| A3 | Reverse is `r_ev` (`r_ev_2` along axis 2). |
| A4 | A list of amounts with a multi-axis subscript means every combination, with one leading result axis per subscripted axis: `-1 0 1 o_-_12 B` on an n by m board has shape 3 3 n m. |

## 6. Lambdas

| #  | Decision |
| -- | -------- |
| L1 | Shorthand arguments: `_r` alone makes a monadic lambda; `_l` and `_r` make a dyadic (curried) one. `_l` without `_r` is an error: use named parameters. |
| L2 | `_l` and `_r` refer to the innermost enclosing lambda. |
| L3 | An inline lambda is an ordinary function, so `X { ... } Y` is a dyadic application. |
| L4 | Named parameters, decorated like names, separated from the body by `->`: `{ f_ g_ x -> f_ g_ x }`. A function parameter (`f_`) is called directly. They desugar to nested one-argument lambdas. |
| L5 | Shorthand and named parameters may not be mixed in one lambda. |
| L6 | A niladic function is defined with `@` as its only parameter: `u:n_ow := { @ -> ... }`, and called as `u:n_ow @`. There is no `_@` call sugar; the subscript slot is for axes only. |

## 7. Statements, bindings and comments

| #  | Decision |
| -- | -------- |
| S1 | Binding is `:=` (`x := 3`); `=` is always equality. The display may render `:=` as APL's left arrow. Accepting the arrow as input is a far-future idea. |
| S2 | A newline separates statements at the top level and inside `{ }`; inside `( )` and `[ ]` it is whitespace. |
| S3 | `;` separates statements on one line (APL's diamond), at the top level and inside `{ }`; it is an error inside `( )` and `[ ]`. Empty statements are ignored. The display may render `;` as the diamond. |
| S4 | `#` starts a comment anywhere on a line and runs to the end of the line; the newline still separates statements. The display renders `#` as APL's lamp. |
| S5 | Scripts use the extension `.xtl` and start with `#!/usr/bin/env xetal` (a plain comment). `xetal FILE` runs FILE. |

## 8. Symbols

All symbol functions are dyadic. Two-character symbols are single
tokens; the display may render each as one glyph.

| Input | Meaning | Displayed as |
| ----- | ------- | ------------ |
| `+` | add | plus |
| `-` | subtract | minus sign |
| `*` | multiply | times sign |
| `/` | divide | division sign |
| `^` | power (spaced) | superscript |
| `=` | equal | equals |
| `!=` | not equal | not-equal sign |
| `<` `>` | less, greater | same |
| `<=` `>=` | less or equal, greater or equal | single glyphs |
| `&` | and | logical and |
| `\|` | or | logical or |

Named instead of symbols: `n_eg` (negate), `n_ot` (not), `m_od`
(APL's residue), `m_ax`, `m_in`.

## 9. Types and values

| #  | Decision |
| -- | -------- |
| T1 | A real `Bool` type; `=` and the comparisons return Bool. Bool converts to Int implicitly in arithmetic (true 1, false 0). Int converts to Bool where a Bool is required: 1 is true, 0 is false, anything else is an error (at run time when only known then). |
| M1 | Values are immutable. Rebinding a name creates a new binding (shadowing); a lambda keeps the value it captured. There is no indexed assignment; updates return new arrays. |
| M2 | Mutation is an explicit escape hatch: only variables named with a trailing `!` may be reassigned in place (`count! := count! + 1`), so every read and write shows it. |

## 10. Input and display

| #  | Decision |
| -- | -------- |
| I1 | Source is ASCII only. Decorated Unicode input is not accepted (revisit much later). |
| I2 | The display is Unicode where it can be, plus a LaTeX subset for anything Unicode lacks (for example superscript `q`, subscript `@`, superscript decimal point). |
| I3 | The display may render a multi-character token as one glyph: `:=` as the left arrow, `->` as an arrow, `;` as the diamond, `#` as the lamp, `!=` `<=` `>=` `*` `/` `&` `\|` as their mathematical glyphs. |

## 11. Sections

| #  | Decision |
| -- | -------- |
| S6 | A symbol function applied to one argument is an error (`- 3`, `/ 2`, `2 +`): under currying it would fill the *left* argument, so `- 3` would mean "3 minus ...". Named functions still curry (`r_/ '+`, `o_- 1`), and a quoted symbol may be partially applied explicitly (`('-)_ 3`). |

What each reading of `- 3` is written as:

```
-3                        # the number negative three (negative literal)
x - -3                    # x minus negative three
n_eg 3                    # negate a value: -3
n_eg 1 2 3                # -1 -2 -3 (element by element); -1 2 3 is a literal
u:l_ess3 := { x -> x - 3 }        # "subtract 3"
u:l_ess3 := (s_wap '-)_ 3         # the same, point-free (s_wap: flip, placeholder)
u:f_romThree := { x -> 3 - x }    # "3 minus something"
u:f_romThree := ('-)_ 3           # the same, point-free: quoted minus, left filled
```

The error for `- 3` names all three fixes: write `-3` for the
number, `n_eg 3` to negate, `{ x -> x - 3 }` to subtract 3.

## 12. Queue of open questions

- String literals (`"..."` likely; `'` is the quote).
- Aliasing syntax for namespaces (N7).
- Binary name: `xetal` or `x_etal`.
- Optional, later: a spelling for axes above 9; function power on
  functions; an explicit `_` wildcard parameter (`{ x _ -> x }`);
  count-from-the-end axes.
- Names for the remaining built-ins (reshape, shape, range, index,
  each, table, tally, and others) under the naming rules above.

## 13. Style guide (conventions; a linter may check them later)

Only camelCase can express multi-word names: snake_case is impossible
(variables have no `_`, a function name has exactly one) and
kebab-case lexes as subtraction.

| Kind | Convention | Example |
| ---- | ---------- | ------- |
| variable | lowerCamel | `boardSize`, `n`, `x` |
| function | lowerCamel, first letter underlined | `n_extGen`, `r_ev` |
| combinator (bird) | single capital | `K_`, `S_`, `c:B_` |
| constant | UpperCamel | `MaxSize` |
| type (future) | UpperCamel | `Board` |

Recommended function-name suffixes:

| Suffix | Meaning | Example |
| ------ | ------- | ------- |
| `?` | predicate, returns Bool (instead of an `is` prefix) | `e_mpty?` |
| `!` | has a side effect (I/O, clock, randomness) | `p_rint!` |
| `/` | reduce-like, collapses an axis | `r_/`, `m_ax/` |
| `\` | scan-like, running results | `s_\` |
| `<` `>` | direction: from the front / from the back | `t_ake<` |
| `~` | approximate or tolerant | `e_q~` |
| `$` | produces text | `f_mt$` |

## 14. Examples

Names other than `r_/`, `o_-`, `r_ev`, `n_eg` are placeholders.

```
x := 3
x^2 + 4^2                        # 25
v := 1 2 3 4
'+ r_/ v                         # 10: reduce by plus
u:s_um := r_/ '+                 # partial application: sum
u:s_um v                         # 10
1 o_- M                          # rotate M's rows by 1 (leading axis)
1 o_-_2 M                        # rotate along axis 2
u:s_quare := { _r * _r }         # shorthand lambda
u:s_quare := { x -> x * x }      # named-parameter lambda
u:h_yp := { a b -> (a^2 + b^2)^0.5 }
3 u:h_yp 4                       # 5.0
u:n_ow := { @ -> s_ysclock @ }   # niladic
u:n_ow @
count! := 0                      # mutable variable
count! := count! + 1
```

The birds, in named-parameter form:

```
u:I_ := { x -> x }
u:K_ := { x y -> x }
u:B_ := { f_ g_ x -> f_ g_ x }       # f (g x)
u:C_ := { f_ x y -> y f_ x }         # f y x
u:S_ := { f_ g_ x -> x f_ g_ x }     # f x (g x)
u:W_ := { f_ x -> x f_ x }           # f x x
u:V_ := { x y f_ -> x f_ y }         # f x y
u:T_ := { x f_ -> f_ x }             # f x
```

Conway's Life (rule from design.md 6.2: S is the 3 by 3 sum including
the cell, next = (S = 3) + cell * (S = 4)):

```
u:l_ife := { ('+ r_/_12 -1 0 1 o_-_12 _r) { (_l = 3) + _r * _l = 4 } _r }
u:l_ife board
```
