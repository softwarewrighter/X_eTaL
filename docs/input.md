# Typing X_eTaL

X_eTaL source is plain ASCII typed on a US keyboard. No input method,
Espanso snippet or special font is needed. Decoration that the
renderer draws as an underline, a subscript or a superscript is typed
as a short ASCII suffix on a name, so a few characters in become one
decorated glyph out: `t_2` (three keys) displays as t underlined with
a subscript 2.

Espanso or editor snippets still work if you like them, but they must
produce the ASCII spelling: the language does not accept the decorated
Unicode form as input. Use `xetal render --raw` to turn decorated text
back into source.

## Names and decoration

A *stem* is a letter followed by letters or digits (`r`, `square2`),
or one of the symbols `+ - * / = < > |`. Decoration follows the stem
in this fixed order:

```
stem ^word _sub
```

| Type       | Means                                   | Displayed as                    |
| ---------- | --------------------------------------- | ------------------------------- |
| `r`        | the value `r` (a noun)                  | r                               |
| `r_`       | `r` as a function (trailing underline)  | r underlined                    |
| `t_2`      | function `t` along axis 2               | t underlined, subscript 2       |
| `t_12`     | function `t` along axes 1 and 2         | t underlined, subscript 12      |
| `+^r`      | derived function: reduce by `+`         | +, superscript r                |
| `+^s_2`    | derived function on axis 2              | +, superscript s, subscript 2   |
| `now_@`    | apply `now` to the Unit value `@`       | now underlined, touching @      |
| `m.f_`     | function `f` from namespace `m`         | m. then f underlined            |

Rules that follow from the table:

- Only one underline, and it ends the name: `my_name` is an error
  (stems cannot contain `_`), and so is `r__`.
- Axes are the digits 1 to 9, each used once: `r_0` and `t_11` are
  errors.
- The superscript comes first: write `+^s_2`, not `+_2^s`.
- A symbol is already a function, so a bare underline on it is an
  error (`+_`); a subscript is fine (`+_1`). The same holds after a
  superscript: `f^r` is already a function, `f^r_` is an error.
- A superscript word is letters only (`+^r`, `+^reduce`).
- A namespace prefix goes on a function name only, one level deep
  (`m.f_`, not `m.x` or `a.b.f_`).

## Lambda arguments and Unit

Inside `{ ... }`, type `_l` for the left argument and `_r` for the
right one. They take no decoration. `@` is the Unit value.

```
square = { _r * _r }; square_ 7
```

## Numbers and minus

Numbers are integers or decimals: `42`, `2.5`. There is no high minus
key, so a negative literal is `-` directly before a digit, at the
start of the input or after a space, a newline, `(`, `{`, `[` or `;`:

| Type      | Means                                       |
| --------- | ------------------------------------------- |
| `-1 0 1`  | the three numbers -1, 0, 1                  |
| `3 - 1`   | 3 minus 1                                   |
| `3 -1`    | the two numbers 3 and -1                    |
| `3-1`     | error: add a space to say which you mean    |

## Spacing

Spaces separate tokens. An undecorated symbol may touch its neighbours
(`1+2`), but a decorated name must end at a space, a bracket or a
symbol. Newlines are kept as separate tokens.

## Seeing what you typed

```bash
xetal lex -e 't_12 +^r'       # tokens with byte spans
xetal render -e 't_12 +^r'    # decorated Unicode
xetal render --latex -e 't_12 +^r'   # LaTeX math for a post-processor
```

Unicode has no superscript `q` and no reliable uppercase superscripts,
so a name using them in a superscript word is displayed exactly as
typed (`+^q`). The LaTeX output has no such gaps.

Errors name the rule and the byte span, for example
`xetal lex -e '3-1'` prints `error[ambiguous-minus]` at `1..2`.

Every example on this page is checked by the test suite
(`spec/lex/`, `spec/render/`, `reg/`).
