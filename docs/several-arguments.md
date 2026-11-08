# Calling a function of several arguments

Every X_eTaL function takes one argument. Two-argument use is currying,
`x f y` is `(f x) y`, so a function of more arguments is called one
argument at a time, each further argument passed to the function the
call so far gives. There are three ways to write such a call;
`demos/apply.xtl` runs all three (each prints 4321), and the golden
`run-apply` pins its output and its expansion. The function used here:

```
u:f_ := { a b c d -> a + (10 * b) + (100 * c) + 1000 * d }
```

## 1. Written out

```
((1 u:f_ 2)_ 3)_ 4
```

`(1 u:f_ 2)` is the function still waiting for `c`; the `_` touching
its `)` applies it (rule F5), and so on for `d`. This is the core
syntax, and it shows exactly what happens. It nests one level for each
argument after the second, which gets hard to read past three or four
arguments. Stopping early is a partial application:
`u:f_irstThree := (1 u:f_ 2)_ 3` is the function waiting for `d`.

## 2. A macro writes the chain

```
"ap:" u_se< "Apply"
c"u:f_" ap:a_pply< c"1; 2; 3; 4"
```

`userlibs/Apply.xtlm` is a macro library of your own (in the live demo,
`Apply.xtlm` among your files). Its `ap:a_pply<` takes the function on
its left and the arguments on its right, separated by `;` (a space
between values would make them one array), and writes the written-out
chain when the program is expanded; `xetal expand` shows it:

```
(((1) u:f_ (2))_ (3))_ (4)
```

Both of its sides are code (`## takes: code code`), so a plain string
there is the `string-kind` warning and a code string's errors are
placed inside it before it expands. An argument may be any expression;
one argument is a plain call; none is refused at the call. It is a
demonstration of what a macro can write: the program it leaves behind
is the written-out form.

## 3. A tuple: the idiom

```
u:g_ := { (a, b, c, d) -> a + (10 * b) + (100 * c) + 1000 * d }
u:g_ (1, 2, 3, 4)
```

The function takes one tuple and names its parts in a pattern (TU3).
One call and no nesting; a tuple of the wrong size is a type error
before the program runs (TU5).

## Which to use

| | Written out | The macro | A tuple |
| - | ----------- | --------- | ------- |
| Nesting | one level per argument after the second | none in the source; the expansion nests | none |
| Partial application | yes: stop early | yes: fewer arguments | no: a tuple comes whole |
| Checked by the types | yes | yes, after expansion | yes, with the size |
| Needs | nothing | an import | the function written to take a tuple |
| Best for | two arguments, or currying on purpose | showing what macros can do | three or more arguments |

The convention (docs/lang-choices.md, section 16): two arguments are
written `x f y`; a function of three or more takes a tuple.
