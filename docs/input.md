# Typing X_eTaL

X_eTaL source is plain ASCII typed on a US keyboard. No input method,
Espanso snippet or special font is needed. What the display draws as
an underline, a subscript or a superscript is typed as a short ASCII
mark on a name, so a few keys in become one decorated glyph out:
`o_-_2` (five keys) displays as o underlined, a minus, and a
subscript 2.

Espanso or editor snippets still work if you like them, but they must
produce the ASCII spelling: decorated Unicode is not accepted as
input. `xetal render --raw` turns decorated text back into source.

## Variables and function names

A name starts with a letter and continues with letters and digits.

- A **variable** has no underline: `x`, `n`, `board2`.
- A **function name** has exactly one underline, typed as `_`
  directly after the letter it underlines: `r_ev` (reverse, r
  underlined), `s_quare`, `self_` (f underlined).
- A function name may end in one **mark**:
  `| - / \ + * < > ~ ! ? % $ &`. Built-ins use marks only where APL
  gives them meaning (`r_/` reduce, `s_\` scan, `o_-` rotate); by
  convention `?` ends a predicate (`e_mpty?`) and `!` a function with
  a side effect (`p_rint!`). A name ending in `<` is a macro
  (`u_se<`).
- A trailing `!` on a variable makes it mutable: `count!`.

| Type        | Means                                       | Displayed as                    |
| ----------- | ------------------------------------------- | ------------------------------- |
| `x`         | the variable x                              | x                               |
| `r_ev`      | the built-in function reverse               | rev, r underlined               |
| `r_/`       | reduce                                      | r underlined, then /            |
| `o_-_12`    | rotate along axes 1 and 2                   | o underlined, -, subscript 12   |
| `u:s_quare` | the user function square                    | superscript u, square           |
| `c:K_`      | K from the library imported as c            | superscript c, K underlined     |
| `m:pi`      | the variable pi from the library m          | superscript m, pi               |
| `x^2`       | x squared                                   | x, superscript 2                |
| `_r` `_l`   | a lambda's right and left arguments         | subscript r, subscript l        |

Rules that follow (each is rejected with an error and a hint):

- The underline must directly follow a letter (`a1_` is an error) and
  appears once (`a_b_c` is an error).
- The name ends after its mark: `f_-1` is an error; write `f_- 1`.
- Axes come after the function name as `_` and digits, one digit per
  axis, 1 to 9, each once: `o_-_0` and `o_-_11` are errors. The
  function `r_` on axis 2 is written `r__2`.
- `x!=3` is ambiguous: write `x != 3` (not equal) or `x! = 3`.

## Namespaces

`u:` marks your own functions, `l:` a library's own names, and other
letters name imported libraries: `u:s_quare`, `c:K_`, `m:pi`. Plain
variables like `x` need no prefix. A prefix on its own (`u:`) is an
error.

## Exponents and power

A `^` touching a value, followed by a number, is an exponent: `x^2`,
`x^-1`, `x^0.5`, `(a + b)^2`. For a computed power, use the spaced
power function: `x ^ n`. `x^n` is an error with that hint, and
superscripts on functions (`r_ev^2`) are reserved.

## Symbols

`+ - * / ^ = != < > <= >= & |` are functions of two arguments:
`1+2`, `x+y`, `3 - 1`. They may touch names and numbers, except
where that is ambiguous: `3-1` is an error (write `3 - 1` to subtract
or `3 -1` for the numbers 3 and -1).

## Numbers and minus

Numbers are integers or decimals: `42`, `2.5`. A negative literal is
`-` directly before a digit, at the start or after a space, a newline,
`(`, `{`, `[` or `;`:

| Type      | Means                                       |
| --------- | ------------------------------------------- |
| `-1 0 1`  | the three numbers -1, 0, 1                  |
| `3 - 1`   | 3 minus 1                                   |
| `3 -1`    | the two numbers 3 and -1                    |
| `x - -3`  | x minus negative 3                          |

## Lambdas, quotes and statements

```
u:s_quare := { _r * _r }; u:s_quare 7
```

- `:=` binds a name; `=` is always equality.
- In `{ ... }`, `_l` and `_r` are the left and right arguments; named
  parameters come before `->`: `{ ~s_elf n -> n <= 1 ? 1 }` (the `~`
  makes a parameter lazy, and a spaced `?` is a guard).
- `'` passes a function as a value: `'+ r_/ v` reduces v by plus.
- A newline or `;` separates statements; `#` starts a comment that
  runs to the end of the line.
- Strings are `"..."` with the escapes `\"`, `\\`, `\n`, `\t`, on one
  line: `"c:" u_se< "Combinators"`.

## Seeing what you typed

```bash
xetal lex -e 'u:s_quare := { _r * _r }; u:s_quare 7'   # tokens with byte spans
xetal render -e 'x r_ev o_-_2 u:s_quare c:K_ x^2 _l _r @'  # decorated Unicode
xetal render --latex -e 'o_-_12 x^2'                    # LaTeX math
```

The decorated form also draws `:=` as an arrow, `;` as a diamond,
`#` as APL's lamp and `*` `/` `!=` `<=` `>=` `&` `|` as their
mathematical signs. Unicode has no superscript `q` and no superscript
decimal point, so `q:x` and `x^0.5` are displayed as typed; the LaTeX
output has no such gaps.

Every example on this page is checked by the test suite
(`spec/lex/`, `spec/render/`, `reg/`).
