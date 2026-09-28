# X_eTaL -- Syntax Proposal (raw ASCII input)

Status: SUPERSEDED by `docs/lang-choices.md` (decisions made one at a
time with the user). Kept for history. Originally: PROPOSAL, awaiting
the user's review. Nothing here is
implemented. Where this proposal and `docs/design.md` sections 2-3 /
`docs/input.md` disagree, those documents describe what is currently
implemented, not what has been agreed.

## 1. Where the current design diverged from the research

The research (`docs/research.txt`, `docs/research2.txt`) converged on
this, in its later sections:

| Research (converged)                          | Where                                 |
| --------------------------------------------- | ------------------------------------- |
| plain name = noun                             | research.txt section 1, 7             |
| underline = callable (`r_` displays r underlined) | research.txt section 7            |
| subscript digits = axis (`t_12` = rotate, axes 1 and 2) | research.txt Life discussion |
| superscript letter = provenance / namespace: `r^u` user r, `r^m` module m, `w_^c` W from the combinator library | research.txt sections 1, 3, "Self-table combinator"; research2 `f^m` |
| reduce is a named function `r_` written after its operand: `+ r_ A`, with "reduce binds to the function immediately to its left" | research.txt Life target, M4, "Why this parse?" rule 31 |
| rotate is `t_` (turn), `e_` each, `b_` table  | research.txt Life discussion          |
| Life: `life = { (+ r_ -1 0 1 t_12 _r) = 3 + _r }` | research.txt, raw display mode     |

What `docs/design.md` (bootstrap commit b1a8faf) did instead, and
what was then implemented (lexer, renderer, `docs/input.md`):

1. **Superscript became derivation** (`+^r` reduce, `+^s` scan) and
   namespaces moved to a dotted prefix (`m.f_`). This reverses the
   research, where superscript letters are namespaces. Its stated
   reason: under right-to-left application `+ r_ A` parses as
   `+ (r_ A)`, so reduce needed "a separate operator class". That
   problem is real and is addressed in section 5 below.
2. **`_` serves two roles in raw input**: underline and the start of a
   subscript (`t_12`). The research did the same (LaTeX-like `_`),
   but `_` also spells the underline in `r_` (reduce), so one key has
   two meanings.
3. **Digits were never allowed in superscripts** in either document;
   that is kept (a superscript digit reads as a power).
4. The research's Life rule `= 3 + _r` was not Conway's rule; that is
   already corrected (design.md 6.2).
5. Superscripts on symbols (`*` with a superscript) were never
   discussed; section 4 below proposes them.

## 2. Principles

- Each decoration has its own key; one key never means two things.
- Raw input is ASCII; several keys in, one decorated glyph out.
- Decoration decides grammatical class; spelling does not (except the
  fixed built-in dictionary, section 5).
- Digits only in subscripts (axes); letters only in superscripts
  (namespaces).
- One spelling per meaning (no redundant decorations).

## 3. Decoration keys

Proposed (recommended option first):

| Decoration       | Raw key            | Example raw  | Displayed as               |
| ---------------- | ------------------ | ------------ | -------------------------- |
| underline        | `_` after the stem | `r_`         | r underlined               |
| superscript      | `:` then letters   | `square:u`   | square, superscript u      |
| subscript        | `,` then digits    | `t,12`       | t, subscript 12            |

Alternatives considered for the superscript key: `.` (`K.c`) reads
like the familiar dotted namespace, but `.` is also the decimal
point and today's prefix namespace (`m.f`), so `K.c` and `c.K` would
both look plausible; `^` (LaTeX, current) is fine to type but invites
reading the letter as an exponent. `:` has no other use.

`,` for subscripts gives up `,` as a future catenate symbol touching a
name (APL's `,`); a spaced `a , b` could still be catenate.

Order is fixed: `stem [_] [:ns] [,axes]`. A token may not mix the
orders (`t,12:u` is an error).

## 4. What each decoration means

**Named stems** (`[A-Za-z][A-Za-z0-9]*`):

| Raw          | Class    | Meaning                                         |
| ------------ | -------- | ----------------------------------------------- |
| `r`          | noun     | the value named `r`                             |
| `r_`         | function | built-in (core) function `r`: reduce            |
| `t,12`       | function | core `t` (rotate) specialized to axes 1 and 2   |
| `square:u`   | function | `square` from namespace `u` (user definitions)  |
| `K:c`        | function | `K` from namespace `c` (combinator library)     |
| `rot:u,2`    | function | user `rot`, axis 2                              |

- Any decoration makes a name a function. Underline is the marker for
  a core function with nothing else to say; `t_,12` and `K_:c` are
  redundant and rejected.
- The display underlines every core function, so `t,12` shows as t
  underlined with subscript 12 (as in the research), and a
  namespaced function shows its superscript instead. The underline is
  therefore always "core", the superscript always "from namespace".
- User functions must carry a namespace: `square:u 7`. A bare
  `square_` names a core function and is an error if the core has no
  `square`.

**Symbol stems** (`+ - * / = < > |`) are core functions already; a
superscript on a symbol selects a variant from a fixed table instead
of a namespace:

| Raw    | Meaning (to confirm)        |
| ------ | --------------------------- |
| `*`    | multiply                    |
| `*:*`  | power (exponentiation)      |
| `*:x`  | multiply, times-sign form?  |

(The user mentioned `*` superscript `*` for exponentiation and `*`
superscript `x` for multiply; what plain `*` then means needs
confirming.) Symbols may take an axis subscript: `+,1`.

**Other tokens** are unchanged: `_l` `_r` lambda arguments, `@` Unit,
numbers (`-1` negative-literal rule), `;`, `{ }`, `( )`, `[ ]`.
Niladic sugar: `now_@` (underlined `now` touching `@`) means `now @`.

## 5. Reduce, scan, each, table: operators

The research writes `+ r_ A` and states that reduce binds to the
function on its left. With plain right-to-left application that
string parses as `+ (r_ A)`. Proposal:

- A small, fixed dictionary of core **operators** (reduce `r_`, scan
  `s_`, each `e_`, table `b_`) binds the single function immediately
  to its left, as APL's `/` does: `+ r_ A` is `(reduce +) A`, and
  `+ r_` alone is a function (`sum = + r_; sum:u 1 2 3`).
- This is the "fixed built-in dictionary" exception to "class comes
  from decoration": the lexer still sees `r_` as a decorated name; the
  parser knows `r_` is an operator.
- User-defined operators need their own marker later (open).

With operators taken from the dictionary, the core names become:

| Terse | Long        | Kind     | Meaning                                  |
| ----- | ----------- | -------- | ---------------------------------------- |
| `r_`  | `reduce_`   | operator | reduce by the function on the left       |
| `s_`  | `scan_`     | operator | scan by the function on the left         |
| `e_`  | `each_`     | operator | apply to each item                       |
| `b_`  | `table_`    | operator | outer product / table                    |
| `t_`  | `rotate_`   | function | reverse (monadic), rotate (dyadic)       |
| `p_`  | `shape_`    | function | shape-of (monadic), reshape (dyadic)     |
| `i_`  | `range_`    | function | range 1..n                               |
| `x_`  | `index_`    | function | select / slice                           |

(`s` for scan leaves shape as `p_`, rho; the research used `s_` for
shape before scan became an operator.)

## 6. Namespaces and aliases

- `u` holds the current program's definitions: `square = { _r * _r }`
  binds `square`, and it is called as `square:u 7`.
- A library has a default one-letter namespace (`c` combinators).
- Aliasing for clashes, Python-style (spelling to be settled):
  `use combinators as k` then `K:k`.

## 7. Examples in the proposed syntax

```
1 + 2
square = { _r * _r }; square:u 7
sub = { _l - _r }; 10 sub:u 3
2 3 p_ 1 2 3 4 5 6
+ r_ 1 2 3 4
sum = + r_; sum:u 1 2 3 4
1 t,2 A
K:c 1 2
now_@
```

Life (Conway's rule from design.md 6.2; reduce over the two offset
axes if rotate returns a rank-4 array, D7):

```
life = { (+ r,12 -1 0 1 t,12 _r) { (_l = 3) + _r * _l = 4 } _r }
```

## 8. Questions for the user

1. Superscript key: `:` (recommended), `.`, or keep `^`?
2. Subscript key `,` (so `t,12`), and is giving up `,` touching names OK?
3. Does any decoration make a function (so `t,12`, `K:c` need no
   underline), with the display adding the core underline?
4. Symbol superscripts: what do `*`, `*:*`, `*:x` each mean?
5. Operators from a fixed dictionary (`+ r_ A`), user-defined
   operators later: agreed?
6. User functions always called with `:u`; is `u` also used at the
   definition (`square:u = ...`) or only at calls?
7. Keep `now_@` sugar, or require `now @`?

## 9. What changes if approved

- Lexer: `:` superscript (letters, namespace), `,` subscript (digits),
  `_` underline only; drop `^` derivations and the dotted prefix.
- Renderer: underline on core functions, superscript namespace
  letters, subscript digits; LaTeX mode likewise.
- Parser (next step): operator dictionary binding left.
- `docs/design.md` sections 2-3 and 8, `docs/input.md`, README table,
  the Life line everywhere, spec cases and goldens.
