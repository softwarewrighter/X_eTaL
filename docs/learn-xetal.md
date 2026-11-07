# Learn X_eTaL in five minutes

`docs/learn-xetal.xtl` is a quick tour of X_eTaL in the style of
[Learn X in Y minutes](https://learnxinyminutes.com/): one program,
mostly code, explained by its comments and read top to bottom in about
five minutes. Each line's trailing comment says what it prints. It
covers numbers and arrays, right-to-left evaluation, binding, shapes
and indexing, functions and lambdas, currying and partial application,
niladic functions and composition, reduce, scan, each, the outer and
inner products, function power, trains, guards and recursion, strings,
search, tuples (and what they do not do), a library import and the Game of Life in one line.

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

### Binding and mutability
x := 5                   # `:=` binds (APL's left arrow); `=` is equality
x * x                    # 25
# A plain name is bound once: values are immutable. Binding it again,
#   x := 6               # error[rebind]: x is already bound in this scope
# is refused when the program is checked, before anything runs.
## u:t_wice y: a lambda's own x, which leaves the outer x alone.
u:t_wice := { y -> x := y * 2; x }   # a new scope may bind its own x
u:t_wice 10              # 20
x                        # 5: the outer x is unchanged
# A name ending in ! is a variable: it may be bound again, but its
# type is fixed by its first binding.
count! := 0
count! := count! + 1
count!                   # 1
#   count! := "one"      # error[type-mismatch]: expected Int, found Char
# At the REPL only, []E_X "x" unbinds a name, so it may be bound again,
# with a new type too; in a file that is error[erase-outside-session]:
# a program binds a new name instead.

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

### Every function takes one argument
# APL functions are niladic, monadic or dyadic, and many are both
# monadic and dyadic (ambivalent). Here a function takes exactly one
# argument: no-argument use passes the unit value @ (below), and
# two-argument use is currying: `x f y` is `(f x) y`, f applied to x
# giving a function that is then applied to y: u:p_ow below has type
# Num a => a -> a -> a.
## b u:p_ow e: b to the power e.
u:p_ow := { b e -> b ^ e }
2 u:p_ow 10              # 1024: dyadic use
## u:t_woTo e: 2 to the power e, u:p_ow with its left argument given.
u:t_woTo := u:p_ow 2     # partial application: give the left argument only
u:t_woTo 10              # 1024
(u:p_ow 3)_ 2            # 9: `_` makes a parenthesized value a function
# A symbol needs both sides (`- 3` is error[symbol-needs-left]), so
# a symbol is partially applied quoted: ('-)_ 10 is "10 minus ...".
## u:t_enMinus x: 10 - x.
u:t_enMinus := ('-)_ 10
u:t_enMinus 3            # 7
## u:m_inusTen x: x - 10, the arguments swapped first.
u:m_inusTen := (s_wap '-)_ 10
u:m_inusTen 3            # -7
(('*)_ 2) e_ach 1 2 3    # 2 4 6: a partial application as an operand
1 2 3 '= e_ach 1 5 3     # 1 0 1: dyadic each is currying too
# A function with no argument (niladic) takes the unit value @, so
# that it is still a function of one argument: call it with @.
p_i @                    # 3.141592653589793
## u:a_nswer: 42, computed when called.
u:a_nswer := { @ -> 42 }
u:a_nswer @              # 42; u:a_nswer alone is the function itself
# Composing functions: c_ompose (the nearest operand applies first),
# an atop train, or s_wap to exchange the arguments (APL's commute).
'n_eg 'a_bs c_ompose -4  # -4: n_eg of a_bs
[n_eg a_bs] -4           # -4: the same as a train
10 '- s_wap 3            # -7: 3 - 10

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

### Tuples
# A tuple holds values of different types side by side, in parentheses
# with commas; each part is an array of its own type and shape.
(1 2 3, 4.5, "abc")      # (1 2 3, 4.5, abc): its type is (Int, Float, Char)
## pt: a pair to take apart.
pt := (7, "seven")
(num, word) := pt        # a pattern takes it apart by position
word                     # seven
((p1, p2), p3) := ((1, 2.5), "z")   # patterns nest
(lo, _, hi) := (1, 2, 3) # _ skips a part
lo + hi                  # 4
## u:s_wap p: the pair p the other way around.
u:s_wap := { (a, b) -> (b, a) }     # a pattern as a parameter
u:s_wap (1, "x")         # (x, 1)
## u:s_tep st: halve the weight, count the steps: a state of two parts.
u:s_tep := { (w, k) -> (w * 0.5, k + 1) }
3 'u:s_tep p_ower (8.0, 0)   # (1.0, 3): a tuple as p_ower's state
(1, 2.5) m_atch (1, 2.5)     # 1: m_atch compares part by part
'{ k -> (k, f_loat k) } m_ap 1 2   # a vector of boxed tuples
# What tuples do not do (each is refused before the program runs):
#   (5,)                 # error[empty-tuple-part]: no one-part tuple;
#                        #   (5) is just 5 in parentheses
#   1 s_elect (10, 20)   # error[type-mismatch]: a tuple is not an
#                        #   array, so no indexing; use a pattern
#   (1, 2) + 1           # error[type-mismatch]: no arithmetic, and
#   (1, 2) = (1, 2)      #   no `=`, inside a tuple; m_atch compares
#   r_ev (1, 2)          # error[type-mismatch]: array functions take
#                        #   arrays only
#   (a, b) := (1, 2, 3)  # error[type-mismatch]: expected a tuple of 2
#   (w: 1, k: 2)         # error[record-field]: named fields are for
#                        #   records, which come later
# A tuple in an array must be boxed (e_nclose, or m_ap as above).

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
