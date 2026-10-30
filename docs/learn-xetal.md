# Learn X_eTaL in five minutes

`docs/learn-xetal.xtl` is a quick tour of X_eTaL in the style of
[Learn X in Y minutes](https://learnxinyminutes.com/): one program,
mostly code, explained by its comments and read top to bottom in about
five minutes. Each line's trailing comment says what it prints. It
covers numbers and arrays, right-to-left evaluation, binding, shapes
and indexing, functions and lambdas, reduce, scan, each, the outer and
inner products, function power, trains, guards and recursion, strings,
search, a library import and the Game of Life in one line.

The file is a runnable program, and a golden (`reg/learn-xetal.rgt`)
checks that its output stays the same.

## Running it

```bash
xetal run docs/learn-xetal.xtl          # every top-level value, one after another
xetal run --echo docs/learn-xetal.xtl   # each statement drawn decorated, then its output
xetal render docs/learn-xetal.xtl       # the source drawn decorated
```

Or paste it into the [live demo](https://softwarewrighter.github.io/X_eTaL/),
which needs nothing installed.

## The file

```
#!/usr/bin/env xetal
## Learn X_eTaL in five minutes.
##
## X_eTaL (the eXperimental Extensible Typed Array Language) is a terse,
## statically typed, functional array language of the APL family. You
## type it in plain ASCII; the display draws it decorated: `r_ev` (r
## underlined) is a function, `x^2` a superscript, `o_-_2` a subscript
## axis. Run this file with "xetal run docs/learn-xetal.xtl": every
## top-level expression prints its value, shown here after the `#`.

# Comments start with #. A `##` line documents the definition below it,
# and a `###` line is a section heading.

### Numbers, arrays and strands
42                       # 42
-3 2.5                   # -3.0 2.5: a strand is an array; one type (Float)
1 2 3 4                  # 1 2 3 4: no brackets, just space between items

### Right to left, no precedence
# Every function takes everything to its right: there is no order of
# operations, just as in APL.
2 * 3 + 1                # 8: that is 2 * (3 + 1)
(2 * 3) + 1              # 7: parenthesize the left to go first
10 - 4 - 1               # 7: 10 - (4 - 1)
7 / 2                    # 3.5: `/` always gives a Float
7 d_iv 2; 7 m_od 3       # 3 then 1; `;` separates statements
2 ^ 10                   # 1024: spaced `^` is power
3^2                      # 9: a `^` touching a value is an exponent

### Whole arrays at once
1 2 3 4 * 10             # 10 20 30 40: a scalar extends to every item
1 2 3 + 10 20 30         # 11 22 33: item by item
1 2 3 = 1 5 3            # 1 0 1: comparisons give Bools, 1 true, 0 false
(3 < 4) & 2 != 2         # 0

### Binding
x := 5                   # `:=` binds (APL's left arrow); `=` is equality
x * x                    # 25
count! := 0              # only a name ending in ! may be bound again
count! := count! + 1
count!                   # 1

### Shapes, ranges and indexing
r_ange 5                 # 1 2 3 4 5: 1-origin, like APL's iota
## A 2 by 3 matrix of 1 to 6.
m := 2 3 r_eshape r_ange 6       # APL: m <- 2 3 rho iota 6
m                        # 1 2 3
                         # 4 5 6
s_hape m                 # 2 3
t_ally m                 # 2: the number of rows (major cells)
t_ally r_ange 5          # 5
2 s_elect m              # 4 5 6: the 2nd row
3 s_elect 10 20 30 40    # 30
2 t_ake 10 20 30         # 10 20
1 d_rop 10 20 30         # 20 30
r_ev 1 2 3               # 3 2 1
1 2 c_at 3 4             # 1 2 3 4: join

### Functions are names with an underline
# A name with one underlined letter is a function: `r_ev` is typed
# r, underscore, ev and drawn as rev with the r underlined. Built-ins
# have plain names; your own live in the `u:` namespace.
n_eg 5                   # -5
a_bs -3 4 -5             # 3 4 5
## u:s_quare x: x times x, item by item.
u:s_quare := { _r * _r }         # `_r` is the right argument (APL's omega)
u:s_quare 1 2 3                  # 1 4 9
## a u:s_ub b: a minus b.
u:s_ub := { _l - _r }            # `_l` is the left argument (APL's alpha)
10 u:s_ub 3                      # 7
## a u:h_yp b: the hypotenuse of a right triangle with legs a and b.
u:h_yp := { a b -> ((a * a) + b * b) ^ 0.5 }   # named arguments before ->
3 u:h_yp 4                       # 5.0

### Operators take quoted functions
# A quote `'` passes a function as a value: it is the operand of the
# function on its right. APL's operators are plain functions here.
'+ r_/ 1 2 3 4           # 10: reduce (APL: +/1 2 3 4)
'* r_/ 1 2 3 4           # 24
'- r_/ 1 2 3             # 2: folds from the right, 1 - (2 - 3)
'+ s_\ 1 2 3 4           # 1 3 6 10: scan, the running sums
'+ r_/ m                 # 5 7 9: the leading axis, column sums
'+ r_/_2 m               # 6 15: a subscript names the axis, row sums
'u:s_quare e_ach 1 2 3   # 1 4 9: each
1 2 3 '* t_able 1 2 3    # the outer product (APL: 1 2 3 o.x 1 2 3)
                         # 1 2 3
                         # 2 4 6
                         # 3 6 9
1 2 3 '+ '* i_nner 4 5 6 # 32: inner product, the dot product (APL: +.x)
'{ _r + 10 } e_ach 1 2   # 11 12: a quoted lambda is an operand too
n_eg^3 5                 # -5: a superscript repeats a function 3 times
3 'u:s_quare p_ower 2    # 256: p_ower for a computed count

### Trains and forks
## u:m_ean x: the mean of x, a fork of sum, divide and tally.
u:m_ean := ['+ r_/ / t_ally]     # (sum x) / (tally x); APL: (+/ div tally)
u:m_ean 1 2 3 4                  # 2.5
[n_eg a_bs] -5 7                 # -5 -7: atop, n_eg of a_bs
1 2 [+ * -] 3 4                  # -8 -12: (x + y) * (x - y)

### Guards and recursion
## u:s_ign x: -1, 0 or 1 as x is negative, zero or positive.
u:s_ign := { x -> x < 0 ? -1; x = 0 ? 0; 1 }   # condition ? result
'u:s_ign e_ach -5 0 5            # -1 0 1
## u:f_act n: the factorial of n, by recursion.
u:f_act := { n ->
  n <= 1 ? 1
  n * u:f_act n - 1
}
u:f_act 10                       # 3628800

### Strings are vectors of characters
"hello"                  # hello
t_ally "hello"           # 5
r_ev "stressed"          # desserts
"abc" = "abd"            # 1 1 0
"ab" c_at "cd"           # abcd
f_ormat 42               # 42, now the string "42"
n_umbers "3 4.5"         # 3.0 4.5
## s: a sentence to split into words.
s := "to be or not"
(s != f_irst " ") p_artition s   # the words, each in its own box

### Search and order
## v: a vector to sort and search.
v := 3 1 4 1 5 9
s_ort v                  # 1 1 3 4 5 9
g_rade v                 # 2 4 1 3 5 6: the indices that sort it
v i_ndexOf 5             # 5
w_here v > 3             # 3 5 6: the indices of the 1s
(w_here v > 3) s_elect v # 4 5 9: a filter

### Libraries
"c:" u_se< "Combinators" # import a library under an alias you choose
10 '- c:C_ 3             # -7: C swaps the arguments, 3 - 10
"s:" u_se< "Stats"
s:s_d 2 4 4 4 5 5 7 9    # 2.0: the standard deviation

### Effects end in !
p_rint! "hi"             # prints hi, then the value hi is printed too

### The Game of Life in one line
## u:l_ife b: the next generation of Life on board b (0 dead, 1 alive),
## wrapping at the edges: a cell lives when its 3 by 3 neighborhood
## total is 3, or 4 and it is alive.
u:l_ife := { ('+ r_/_12 -1 0 1 o_-_12 _r) { (_l = 3) + _r * _l = 4 } _r }
u:l_ife 5 5 r_eshape 0 0 0 0 0 0 0 1 0 0 0 0 1 0 0 0 0 1 0 0 0 0 0 0 0
                         # a blinker turns on its side:
                         # 0 0 0 0 0
                         # 0 0 0 0 0
                         # 0 1 1 1 0
                         # 0 0 0 0 0
                         # 0 0 0 0 0

### Where to go next
# README.md: the language at a glance, and how to build and run it.
# The live demo, nothing to install: https://softwarewrighter.github.io/X_eTaL/
# docs/reference.md: every built-in, with examples.
# docs/idioms.md: everyday idioms beside Python, Rust, APL, J, BQN and K.
# docs/input.md: what to type for every decoration.
# demos/tour.xtl: a longer tour touching every feature.
```

## Where to go next

- [README](../README.md): the language at a glance
- [docs/reference.md](reference.md): every built-in, with examples
- [docs/idioms.md](idioms.md): everyday idioms beside other languages
- [docs/input.md](input.md): what to type for every decoration
