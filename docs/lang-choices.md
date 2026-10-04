# X_eTaL -- Language Choices

Status: DECISIONS COMPLETE FOR SAGA 1, CONSISTENCY-REVIEWED. Each
entry was chosen by the user, one question at a time (Q1-Q50, sub-
questions, and review items R1-R6). Section 15 lists the optional
items deliberately left for later. The lexer, renderer, parser,
formatter, Core desugaring, a scalar evaluator (saga calculus), type
inference with type-checked evaluation (saga types-and-unit), dense
arrays with the structural built-ins and a REPL (saga arrays), and
the higher-order built-ins of B6 and the search, order and random
built-ins of B7 (saga higher-order), and rotate, reverse and axis
subscripts with their multi-axis forms (saga rotate-and-axes; the Life
one-liner runs) implement it; the rest follows the sagas in `docs/plan.md`.

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
| F4 | A quote passes a function without applying it: a function name (`'r_/`, `'+`, `'u:s_quare`), a lambda literal (`'{ x -> x * 2 }`) or a train (`'['+ r_/ / t_ally]`). Quoting a variable is an error (a variable is already a value). |
| F5 | A function held in a value is applied by an underline after a lambda argument or a closing parenthesis: `_l_ x`, `(expr)_ x`, and `(f)_ x` for a variable `f`. |
| F6 | Functions that take functions put them first, data last: `'+ r_/ A` is `reduce '+ A`; rotate takes the amount first: `1 o_- A`. Swapping order is done with a flip combinator (C, APL's commute), e.g. `u:o_ver := s_wap 'r_/` then `A u:o_ver '+`. |
| F7 | There is no separate operator class. Reduce, scan, each and table are ordinary curried functions; the function-first convention gives the APL look (`'+ r_/ A` for APL `+/A`). |
| F8 | A quoted function directly left of a function name binds to it as its operand, and the pair acts as one function: `A '* t_able B` (APL `A jot.times B`), `A '= e_ach B`, `'+ r_/ A`. This is the only reading the grammar gives; it produces the same Core as the curried dyadic reading (`App(App(reduce, plus), A)`), and it works for any function taking a function first, user functions included. |
| F9 | Quoted operands chain: in `'+ '* i_nner` the operand nearest the function binds first, so a two-operand function takes its nearest operand first and reads in APL order at the use site: `A '+ '* i_nner B` (APL `A +.times B`), `'f_ 'g_ c_ompose x`. Its definition lists parameters nearest-first (`{ g_ f_ a b -> ... }`). |

## 3. Names

| #  | Decision |
| -- | -------- |
| N1 | A name starts with a letter. A variable name is letters and digits only (`x`, `n`, `board2`). |
| N2 | A function name contains exactly one `_`, directly after a letter, which underlines that letter (`r_ev`, `s_quare`, `o_-`). |
| N3 | A function name may end in one punctuation character from `\| - / \ + * < > ~ ! ? % $ &` (`r_/`, `o_-`, `e_mpty?`). A trailing `<` marks a macro (MC2). |
| N4 | A mutable variable name ends in `!` (see M2). A `!` touching a variable name and followed by `=` (`x!=3`) is an error asking for a space: `x != 3` (not equal) or `x! = 3` (compare the mutable variable), like the ambiguous-minus rule (review R2). |
| N5 | Namespaces are a leading prefix: `u:` for the user's definitions, `c:` for the combinator library; no prefix means the system (built-ins). The display shows the prefix as a leading superscript. Plain variables (`x`, `n`) are in the user namespace implicitly. |
| N6 | User functions are defined and called with the same full name: `u:l_ife := { ... }`, `u:l_ife board`. |
| N7 | Libraries are brought in by the `u_se<` macro (section 14); the required left argument names the library's namespace in this file: `"k:" u_se< "Combinators"`. |

## 4. Decorations

| #  | Decision |
| -- | -------- |
| D-1 | Trailing superscript key is `^`, so `^` never appears inside a name. |
| D-2 | An exponent may follow a value token (number, variable, lambda argument) or a parenthesized expression: `x^2`, `_r^2`, `(a + b)^2`. |
| D-3 | It binds to the token it touches: `1 2 3^2` is `1 2 9`; `(1 2 3)^2` squares all three. |
| D-4 | Any number literal may be an exponent, including negative and decimal (`x^-1`, `x^0.5`). An Int base with a whole non-negative exponent gives Int; otherwise Float. A negative base with a fractional exponent is an error, as is integer overflow. LaTeX renders what Unicode cannot. |
| D-5 | For now exponents are literal numbers only (phased); `x^n` is an error with a hint to use the power function. Variable or expression exponents may be added later. |
| D-6 | Power with a computed exponent is the spaced symbol `^`: `x ^ n`, `2 ^ (k - 1)`. Touching `^` is the literal superscript; spaced `^` is the function (like `-1` versus `- 1`). |
| D-10 | The spaced power function with a computed exponent: Int `^` Int gives Int, and a negative exponent at run time is an error with the hint "use a Float base: `2.0 ^ n`"; any Float operand gives Float. Integer overflow is an error (`2 ^ 100` on Ints; use `2.0 ^ 100`). (Review R5.) |
| D-7 | A superscript on a function is function power (decided with the user after the tui saga): `f_^3 x` applies f three times (`u:l_ife^4 board`, displayed with a superscript 4), with the same literal-number rule as D-4/D-5; a computed count uses the built-in `n 'f_ p_ower x`, as spaced `^` does for numbers (D-6). `^-1` (the inverse) stays reserved. The count is a whole number, 0 or more (`f_^0` is the identity); `f_^n` with a name, `f_^1.5` and a power on a symbol (`+^2`) are `error[bad-power]`, and a power on a parenthesized function (`(r_ev)^2`) is an error; `p_ower` with a negative count is `error[domain]`. `f_^3` means `3 'f_ p_ower` waiting for its argument, so a function repeated must return what it takes (`(a -> a) -> Int -> a -> a`): a dyadic function's power is a type error. The count is drawn in the function's color. |
| D-8 | An axis subscript is `_` followed by digits after a function name: `r_/_2`, `o_-_12`. Because a function name has exactly one underline, a later `_` always starts the subscript (`r_2` is the function `r2`; the function `r` on axis 2 is `r__2`). |
| D-9 | One digit per axis, axes 1 to 9: `_12` means axes 1 and 2. A spelling for axes above 9 is reserved for later. |

## 5. Arrays and axes

| #  | Decision |
| -- | -------- |
| A1 | Leading-axis theory: with no subscript, every axis-taking function acts on the first axis (as in J and BQN). |
| A2 | Rotate is `o_-` (like APL's circle-minus, first-axis rotate), amount on the left; a positive amount moves items toward the front as in APL, J and BQN (`1 o_- 1 2 3` is `2 3 1`), and amounts wrap around. `o_\|` is reserved; `o_\` (transpose) and `o_/` are future candidates for the mirror family. |
| A3 | Reverse is `r_ev` (`r_ev_2` along axis 2). |
| A4 | A list of amounts with a multi-axis subscript means every combination, with one leading result axis per subscripted axis: `-1 0 1 o_-_12 B` on an n by m board has shape 3 3 n m. The same holds with one axis or none: `-1 0 1 o_- V` is three rotated copies, shape 3 n (decided with the user in the rotate-and-axes saga). |
| A5 | Index origin is 1: `r_ange 5` is `1 2 3 4 5` and index 1 is the first item, consistent with 1-based axes. Not configurable (no APL-style index-origin setting). |
| A6 | An axis subscript works on any function by one rule: `f_k X` moves axis k to the front, applies f (which works on the leading axis) and moves it back. Built-ins and user functions alike: `u:n_ormalize_2 M`, and `'+ r_/_2 M` is reduce by this rule. Multi-digit subscripts (`_12`) mean something only where a function defines them (rotate, A4); on a user function they are an error for now. Moving an axis can be an index view, not a copy. On a dyadic function `_k` moves axis k of the right (data) argument only, since control arguments sit on the left (F6, B10); `A c_at_2 B` is an error for now. After f runs, a result of the same rank has its axis 1 moved back to k, a result one rank lower (f consumed the leading axis, as reduce does) is left as it is, and any other change of rank is `error[axis]` (decided with the user in the rotate-and-axes saga). |
| A7 | Nested arrays follow APL2 / BQN: any element may itself be an array (`"ab" "cde"` is a 2-element vector of strings), with enclose / disclose built-ins. Depth is static: an enclosed item has type `Box a`, so `"ab" "cde"` is `Box Char`, `e_nclose : a -> Box a` and `d_isclose` takes it back; a nested array prints boxed, each nested item framed as APL2's DISPLAY does (decided with the user for the classics lane, replacing "no explicit box type"). v0 arrays are flat (a flat array is a nested array of scalars), and v0 rules such as "`e_ach` returns scalars" are written so nesting can be added without breaking programs; the remaining questions were settled with the user in the classics lane's nested-design step (B14). |

## 6. Lambdas

| #  | Decision |
| -- | -------- |
| L1 | Shorthand arguments: `_r` alone makes a monadic lambda; `_l` and `_r` make a dyadic (curried) one. `_l` without `_r` is an error: use named parameters. |
| L2 | `_l` and `_r` refer to the innermost enclosing lambda. |
| L3 | An inline lambda is an ordinary function, so `X { ... } Y` is a dyadic application. |
| L4 | Named parameters, decorated like names, separated from the body by `->`: `{ f_ g_ x -> f_ g_ x }`. A function parameter (`f_`) is called directly. They desugar to nested one-argument lambdas. |
| L5 | Shorthand and named parameters may not be mixed in one lambda. |
| L7 | Names resolve lexically: the innermost enclosing lambda's parameter, then outer lambdas, then the program (variables) or the system (unqualified functions). A parameter or local variable may shadow a system name; the parameter wins and a non-fatal warning is reported ("parameter `r_ev` shadows the built-in reverse"). Warnings go to stderr, do not change the exit code, and a linter may treat them as errors. New built-ins therefore never break existing programs. (Review R6.) |
| L6 | A niladic function is defined with `@` as its only parameter: `u:n_ow! := { @ -> ... }`, and called as `u:n_ow! @` (the `!` marks the effect, by the style guide). There is no `_@` call sugar; the subscript slot is for axes only. |

## 6a. Trains

| #  | Decision |
| -- | -------- |
| TR1 | Trains are written in square brackets. A fork `[F G H] x` is `(F x) G (H x)`; dyadically `x [F G H] y` is `(x F y) G (x H y)`. Elements are function expressions: names, symbols, or quoted-operand derived functions (`'+ r_/`). |
| TR2 | A two-element train is atop: `[F G] x` is `F (G x)` (BQN / Dyalog, not J's hook). |
| TR4 | A train's arity comes from where it is written: `[F G H] x` is monadic, `x [F G H] y` dyadic. A train bound to a name or quoted is monadic (`u:a_vg := ['+ r_/ / t_ally]`); a named dyadic train is written as a lambda, `{ a b -> (a F b) G (a H b) }`. Decided with the user during the core-desugar step. |
| TR3 | Longer trains group from the right into forks: `[A B C D E]` is `[A B [C D E]]`. Trains desugar to ordinary application; nothing train-specific reaches the evaluator. |

```
u:a_vg := ['+ r_/ / t_ally]      # fork: sum divided by count
u:a_vg 1 2 3 4                   # 2.5
[n_eg a_bs] x                    # atop: negate the absolute value
```

## 6b. Conditionals

| #  | Decision |
| -- | -------- |
| G1 | Branching uses APL dfn-style guards inside lambdas: a statement `condition ? result` returns `result` from the lambda when the condition is true; otherwise evaluation continues with the next statement. Only the chosen result is evaluated, so recursion terminates. The guard is a spaced `?`; a touching `?` ends a predicate name (`e_mpty?`). |
| G2 | A guard's condition must be a scalar Bool (or Int 1 / 0, per T1); an array condition is an error. If no guard fires and no plain statement follows, it is an error. Guards exist only inside `{ }`. |

```
u:f_act := { n ->
  n <= 1 ? 1
  n * u:f_act n - 1
}
u:s_ign := { x -> x < 0 ? -1; x = 0 ? 0; 1 }
```

## 7. Statements, bindings and comments

| #  | Decision |
| -- | -------- |
| S1 | Binding is `:=` (`x := 3`); `=` is always equality. The display may render `:=` as APL's left arrow. Accepting the arrow as input is a far-future idea. |
| S2 | A newline separates statements at the top level and inside `{ }`; inside `( )` and `[ ]` it is whitespace. |
| S3 | `;` separates statements on one line (APL's diamond), at the top level and inside `{ }`; it is an error inside `( )` and `[ ]`. Empty statements are ignored. The display may render `;` as the diamond. |
| S4 | `#` starts a comment anywhere on a line and runs to the end of the line; the newline still separates statements. The display renders `#` as APL's lamp. |
| S5 | Scripts use the extension `.xtl` and start with `#!/usr/bin/env xetal` (a plain comment). `xetal FILE` runs FILE. |
| S8 | A Float literal may have an exponent: `e` or `E` right after the digits, then an optional `-` and digits, all touching, is one number token (`1.5e-7`, `6.02e23`, `2E3`), always a Float. The `-` inside an exponent is part of the number and never subtraction; a spaced `e` is a name as before (`1.5 e3` is a number and a name). Printed results stay as today. Asked for by X_eTaL-demos (ask D4); decided with the user, 2026-10-02. |
| S7 | The command is `xetal` (easy to type, matches the crate slug). `x_etal` is installed as an alias (symlink), so `#!/usr/bin/env x_etal` also works. The display name stays `X_eTaL`, which decorates as X underlined, matching the logo. |
| S9 | Comments by count of `#` (drawn as APL's lamp, one per `#`; the Emacs Lisp convention): `#` lines and end-of-line remarks are ordinary comments, ignored by `xetal doc`; `##` is documentation (a block directly above a definition documents it, a block at the top of a file, after any `#!`, documents the file); `###` is a section heading (it groups the definitions after it, an anchor and a table-of-contents entry). Inside `##`, backquoted code is drawn decorated, and an example is a transcript: `## >> expression` and the expected output on the following `##` lines (`## error[code]` for an expected error), shown by `xetal doc` and run by its doctests. Decided with the user, 2026-10-04; not yet implemented (Saga 32). |

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
(remainder, maths order: `7 m_od 3`), `d_iv`, `m_ax`, `m_in` (B7).

## 9. Types and values

| #  | Decision |
| -- | -------- |
| T1 | A real `Bool` type; `=` and the comparisons return Bool. Bool converts to Int implicitly in arithmetic (true 1, false 0). Int converts to Bool where a Bool is required: 1 is true, 0 is false, anything else is an error (at run time when only known then). |
| T2 | `/` is true division and always returns a Float (`7 / 2` is `3.5`, `6 / 2` is `3.0`); integer quotient and remainder are the named functions `d_iv` and `m_od`. Division by zero is an error (`error[division-by-zero]` with the span), including `d_iv` and `m_od` by 0 and element-wise inside arrays. There is no `inf` / `nan` result from division. |
| T3 | `=` is exact (IEEE) equality and transitive; `(0.1 + 0.2) = 0.3` is 0. Tolerant equality is the named function `e_q~` with a fixed relative tolerance (about 1e-14): `(0.1 + 0.2) e_q~ 0.3` is 1. There is no comparison-tolerance setting. Int versus Float comparison is exact numeric comparison (`3 = 3.0` is 1); `<` `<=` `>` `>=` are exact too (tolerant versions may be added later with the `~` suffix). |
| T5 | Numeric typing is Haskell-style: arithmetic is `Num a => a -> a -> a` (Num = Int, Float); number literals are polymorphic (`3 + 2.5` and `2 ^ 0.5` type-check); comparisons return a `Truthy` type (Bool, Int) that defaults to Bool but is Int when used in arithmetic, so `(S = 3) + c * (S = 4)` is Int (T1); conditions and `& \| n_ot` take `Truthy`. An Int variable mixed with a Float needs `f_loat` (B8). A top-level binding that is not a function has its numbers defaulted when it is defined (Int), so `n := 3; n + 2.5` is a type error; a top-level condition stays a condition, so `a := 1 2 > 0` is an Int in arithmetic (`1 * a`, `f_loat a`) and a Bool in a guard, as written inline (T1; shown as Bool; fixed 2026-10-03 for X_eTaL-demos). Decided with the user in the types saga. |
| T6 | Programs are type-checked before they run: `xetal eval` and `xetal run` refuse an ill-typed program with a spanned error, and `--untyped` skips the checker for experiments. Y in its textbook shape (E3) has no finite type (`infinite-type`), so it runs with `--untyped`. Checking follows Haskell: mutually recursive `u:` definitions are generalized together once no forward reference is pending (a use before that is monomorphic); an integer literal whose type resolves to Float is a Float value (`c ? 42; 1 / 0` gives `42.0`), also inside a polymorphic function used at Float (`u:k_ := { @ -> 1 }` used as a Float gives `1.0`). Decided with the user in the types saga. |
| T7 | Array types are rank-erased, as in APL: every value is an array and a type names only the element type. `1 2 3` and `7` are both Int, `"abc"` is Char, so a scalar function such as `+ : Num a => a -> a -> a` applies to arrays with no extra rule: `1 2 3 + 1` is `2 3 4` and `u:s_quare 1 2 3` is `1 4 9`. A scalar extends to every item of an array; two arrays combined item by item must have the same shape, checked at run time (`1 2 + 1 2 3` is `error[shape-mismatch]`). A condition must be a single value (`error[not-a-scalar]`). Decided with the user in the arrays saga. |
| T8 | Comparisons: `=` and `!=` work on two values of any one scalar type (numbers compare exactly across Int and Float, T3; Char with Char; Bool with Bool), typed with a class `Eq`; `<` `>` `<=` `>=` work on numbers and on Char in ASCII order, class `Ord`; `e_q~` stays numbers only. They apply item by item: `"abc" = "abd"` is `1 1 0`. Mixing kinds (`"a" = 1`) is a type error. A class implied by another is not printed: `Num a` implies `Eq a` and `Ord a`. Decided with the user in the arrays saga. |
| T9 | An array remembers the kind of its items (character, number, box) even when it is empty, as APL2's prototype does: `""` is an empty character vector and `0 r_eshape 1` an empty number vector, and the display marks each by its kind (`d_isplay ""` has the plain bottom line, not `~`). Every primitive that can make an empty array keeps the kind of its argument. Decided with the user, 2026-10-02 (asked for by X_eTaL-libraries, ask X5); not yet implemented. |
| T4 | Type annotations: none in v0 (types are inferred). `::` is reserved for later signature lines (`u:s_quare :: Int -> Int`), which will be checked against inference so they cannot drift, and shown by hover and doc tools. Until then, a comment `# :: Int -> Int` above a definition is an unchecked documentation convention. |
| M1 | Values are immutable. Rebinding a variable creates a new binding (shadowing); a lambda keeps the value it captured. A top-level function name is defined once per file (MC8 row 12). There is no indexed assignment; updates return new arrays. |
| M2 | Mutation is an explicit escape hatch: only variables named with a trailing `!` may be reassigned in place (`count! := count! + 1`), so every read and write shows it. |

## 9a. Evaluation

| #  | Decision |
| -- | -------- |
| E1 | Evaluation is strict by default. A lambda parameter may be declared lazy with a `~` prefix in the parameter list (`{ ~s_elf n -> ... }`): its argument is not evaluated at the call but on first use in the body, then remembered (call-by-need); unused, it is never evaluated. The body uses the parameter normally (`s_elf`, not `~s_elf`). |
| E2 | A function is evaluated before its argument, so the evaluator knows whether the parameter is lazy. Laziness is carried by the function value at run time and is not part of the static type. |
| E4 | Evaluation order is the function first, then its arguments right to left (APL order): in `x f y`, `f`, then `y`, then `x`. `(p_rint! 1) + p_rint! 2` prints 2 then 1; in a fork `(F x) G (H x)`, H's result is computed before F's. Lazy (`~`) arguments are not evaluated in this sequence, only on first use. The trace explainer steps in the same order. (Review R4.) |
| E3 | The Y combinator works in its textbook shape when its functional marks its self parameter lazy; Z also works. Y runs with `--untyped` (T6). Lazy parameters also let users write their own control structures (`u:w_hen := { c ~a ~b -> c ? a; b }`). |

Implementation note (no language change): because values are
immutable, arrays may be kept virtual. `r_ange n` and `o_ffsets n` can
be arithmetic progressions; rotate, reverse, take, drop and transpose
can be index transformations over the original (Abrams' "beating");
chains of element-wise operations can be fused (Abrams' "drag-along");
reductions can stream over virtual arrays, so `'+ r_/ r_ange 1000000000`
never builds the array. Results are exactly those of full evaluation.
Arrays have finite shape; infinite sequences would be a separate
stream type later.

## 10. Input and display

| #  | Decision |
| -- | -------- |
| I1 | Source is ASCII, except inside string literals and comments, which may hold any Unicode (revised with the user, so a library can greet with the drawn logo, `"hello X_ e:T a:L"` drawn with its spaces removed). Decorated Unicode input is not accepted as code: a Unicode character outside a string or comment is an error (revisit much later). |
| I2 | The display is Unicode where it can be, plus a LaTeX subset for anything Unicode lacks (for example superscript `q`, subscript `@`). A decimal exponent is raised with a middle dot (U+00B7) as its point, since Unicode has no superscript full stop: `x^0.5` shows as a superscript 0, a dot and a 5 (decided with the user; a raised fraction was set aside, since only some decimals are nice fractions). |
| I3 | The display may render a multi-character token as one glyph: `:=` as the left arrow, `->` as an arrow, `;` as the diamond, `#` as the lamp, `!=` `<=` `>=` `*` `/` `&` `\|` as their mathematical glyphs. Ligatures apply to standalone tokens only, never to punctuation inside a function name (`r_/` keeps its slash). The lambda arguments `_l` and `_r` display as APL's alpha and omega, the names of a dfn's left and right arguments (decided with the user in the tui saga; subscript l and r read poorly in terminal fonts). |

## 10a. Printed results

Plain ASCII, as printed by `xetal eval`, the REPL and the goldens;
printed values are valid input where possible.

| Value | Printed as |
| ----- | ---------- |
| Int | `42`, `-3` (the rich display may show APL's high minus) |
| Float | shortest form that reads back exactly, always with a `.`: `2.5`, `0.1`, `5.0` |
| Bool | `1` / `0` |
| Unit | `@` |
| vector | space-separated: `1 2 3` |
| matrix | one row per line, columns right-aligned |
| rank 3 and up | matrices separated by blank lines (one more blank line per further rank) |
| empty vector | an empty line |
| string | the characters without quotes: `hello` |
| function | `<function>` (or its source text when known) |

## 11. Built-in names

| #  | Decision |
| -- | -------- |
| B1 | Built-ins are named with full words or standard short forms, first letter underlined: `r_eshape`, `r_ange`, `e_ach`, `t_able`, `t_ally`, `f_irst`, `r_ev`, `n_eg`. |
| B2 | A punctuation-suffixed name is used only where the mark carries APL meaning: `r_/` reduce, `s_\` scan, `o_-` rotate (and `o_\` transpose, later). |
| B3 | One name per built-in; no aliases (`r_ev` exists, `r_everse` does not). |
| B4 | Structural built-ins (one arity each, leading axis default, `_digits` for other axes, 1-origin): `s_hape` (shape vector), `r_eshape` (dyadic, reuses elements cyclically), `r_ange` (1..n), `t_ally` (items along the leading axis), `f_irst` (first major cell), `t_ake` / `d_rop` (first n / all but first n; negative counts from the end), `s_elect` (items at indices; "index of" would be a separate `i_ndexOf`), `r_avel` (all elements as a vector), `c_at` (join along the leading axis). |
| B5 | `o_ffsets n` gives `0 1 ... n-1`: a distinct built-in for offset arithmetic (place values, wrap-around), so 1-origin stays fixed with no index-origin setting: `10 ^ r_ev o_ffsets 3` is `100 10 1`. |
| B6 | Higher-order built-ins: `r_/` reduce and `s_\` scan (leading axis; empty reduce gives the operand's identity, or an error if it has none), `e_ach` (apply to each element; monadic or dyadic; results must be scalars until nested arrays exist), `t_able` (outer product: `1 2 3 '* t_able 1 2 3`), `i_nner` (inner product: `A '+ '* i_nner B`, pairing the last axis of A with the first axis of B as in APL and J), `c_ompose` (`'n_eg 'a_bs c_ompose x` is `n_eg a_bs x`), `s_wap` (APL commute, the C combinator: `A '/ s_wap B` is `B / A`). Reduce and scan with a multi-digit subscript work over each listed axis in turn: `'+ r_/_12 X` reduces axis 1 and then the new axis 1 (as used by Life), `s_\_12` likewise (review R1; arrives with axis subscripts). Reduce is a right fold, as in J, BQN and APL: `'- r_/ 1 2 3` is `1 - (2 - 3)`, which is 2; item k of a scan is the reduce of the first k items, so the last item of a scan is the reduce (`'- s_\ 1 2 3` is `1 -1 2`). Empty-reduce identities, shaped like one major cell and of the element type: `+ -` 0, `* /` 1 (a Float for `/`), `&` 1, `\|` 0, `=` 1, `!=` 0; any other operand (`m_ax`, `m_in`, lambdas, user functions) is `error[no-identity]`. `e_ach` is one built-in, `(a -> b) -> a -> b`; its dyadic use is currying: when f applied to an item returns a function, `A '= e_ach B` applies those item by item to B (a single value on either side extends, T7). `t_able`'s result has the shape of A followed by the shape of B. Decided with the user in the higher-order saga. A quoted function is written exactly as its name is spelled (`'u:s_quare`, `'n_eg`, `'f_`). |
| B7 | Arithmetic, search and effect built-ins: `n_eg`, `a_bs`, `f_loor`, `c_eiling`, `m_ax` / `m_in` (dyadic; `'m_ax r_/ v` is the maximum), `d_iv` and `m_od` in maths order (`7 d_iv 2` is `3`, `7 m_od 3` is `1`), `n_ot`, `e_q~` (T3), `e_xp`, `l_og` (natural), `i_ndexOf` (`5 6 7 i_ndexOf 7 9` is `3 4`; not found gives tally + 1), `m_ember?` (`2 9 m_ember? 1 2 3` is `1 0`), `m_atch` (1 when both sides have the same shape and equal items, APL's match: one result for the whole arrays, added at the user's request), `u_nique` (first-seen order), `s_ort` (ascending; descending is `r_ev s_ort v`), `g_rade` (sorting indices), `w_here` (indices of 1s), `r_oll!` (random 1..n for every item of n: `r_oll! 6 6` rolls two dice; truly random, with a seed only for tests that need one: `xetal eval --seed N` or `XETAL_SEED`; in the REPL a rolled value keeps its value), `p_rint!` (print and return the value). `s_ort` and `g_rade` are stable and ascending on `Ord` types (numbers, Char), ordering major cells lexicographically; `g_rade` gives 1-origin indices. `i_ndexOf` searches the major cells of its left argument (a matrix is searched row by row); `m_ember?` is item by item; `w_here` takes a vector (indices of a matrix would need nested arrays). Decided with the user in the higher-order saga. |
| B8 | `f_loat` converts a number to Float (T5): `f_loat 3` is `3.0`. |
| B9 | Identity and the tacks (APL's sideways T symbols) are three built-ins, because dyadic use is currying and one name cannot be both the monadic identity and the dyadic tack: `i_d x` is `x` (identity, the I combinator), `x l_eft y` is `x` (left tack, the K combinator), `x r_ight y` is `y` (right tack). In a train they fill slots by position (TR4): the hook `[i_d F G]` is `x F (G x)` (the S combinator), and the monadic fork `[F G H]` is `(F x) G (H x)` (S', Phi). Decided with the user in the arrays saga. |
| B10 | Structural details, decided with the user in the arrays saga: the count, shape or indices go on the left (`2 3 r_eshape X`, `2 t_ake X`, `2 3 s_elect X`). `s_elect` takes 1-origin indices and returns major cells; an index outside 1..n is `error[index]`. Overtake pads with a fill taken from the items, 0 for numbers and a space for characters (`5 t_ake 1 2 3` is `1 2 3 0 0`); overtaking an empty array, `f_irst` of an empty array and `r_eshape` of no items into a non-empty shape are `error[empty]`, since an empty array has no run-time fill. `c_at` joins along the leading axis: a scalar extends to one major cell, an argument one rank lower is a single major cell (a matrix `c_at` a vector appends a row), otherwise the cell shapes must match. A scalar acts as a 1-item vector for `t_ake`, `d_rop`, `s_elect`, `t_ally` and `f_irst`; `s_hape` of a scalar is the empty vector. |
| B11 | `r_eplicate` is APL's replicate (`/` with counts): counts on the left (B10), one per major cell of the right argument, a scalar count extends to every cell, a count of 0 drops the cell: `1 0 2 r_eplicate "abc"` is `"acc"`, and a 0/1 mask compresses. A negative count is `error[domain]` (no APL fill items). Decided with the user for the classics lane. |
| B12 | `e_ncode` and `d_ecode` are APL's encode and decode (the up and down tacks), radix on the left: `2 2 2 e_ncode 5` is `1 0 1` and `2 d_ecode 1 0 1` is `5`; a scalar radix extends in `d_ecode` (`10 d_ecode 1 2 3` is `123`), and `e_ncode` of a vector gives one column per item, shape radix-length by items, as in APL. Decided with the user for the classics lane. |
| B13 | Trigonometry, decided with the user for turtle graphics: `s_in`, `c_os` and `a_tan` in radians, each `Num a => a -> Float`, and `p_i @`, niladic. |
| B14 | Nested arrays, the remaining questions (A7), decided with the user for the classics lane. Strands: a strand of string literals is a nested vector, as a strand of number literals is a flat one: `"ab" "cde"` is a 2-item `Box Char`; names and expressions side by side still do not form a strand (they are joined with `c_at` and `e_nclose`). Each: `e_ach` stays scalar-only, so programs that use it are unchanged; a new built-in `m_ap : (a -> b) -> a -> Box b` applies a function to each item and boxes every result, so a function returning arrays is mapped with `m_ap`. Partition: `p_artition` is APL2's partition, Int keys on the left, one per item: a new piece starts where the key increases, and a 0 key drops the item, so `(s != " ") p_artition s` is the words of `s`; the result is a `Box` vector. Types: `Box a` unifies only with `Box a`, never boxing or unboxing a value implicitly, so depth is part of the type; it prints as `Box Char`, `Box (Box Int)`. |
| B15 | `c_at_k` is APL's catenate along axis k (decided with the user, from the swimming-ducks demo): `c_at` defines its own axis rule, moving axis k of both arguments to the front, joining, and moving it back (A6 moves only the right argument, which would join mismatched arrays). The axes other than k must match; a scalar extends to one cell along k, and an argument one rank lower is one cell, so `M c_at_2 v` appends a column. `c_at_1` is `c_at`; several axes (`c_at_12`) are `error[axis]`. Laminate (a new axis) is not part of this. |
| B16 | Nested arrays print as APL2's DISPLAY draws them (decided with the user for the classics lane): every array in a frame, an arrow along the top (a crossed circle for an empty last axis), a down arrow on the left for each leading axis, and a mark at the bottom for what it holds (`~` numbers, a plain line for characters, a membership sign, epsilon, for boxes); a box of a simple scalar is a frame without arrows; the items of a nested array sit side by side, a space apart, centred in their row. A flat array prints as before. `e_nclose : a -> Box a` makes a box (of anything, so `e_nclose 5` is a `Box Int`) and `d_isclose : Box a -> a` opens one box; opening an array of boxes into one array (APL2's disclose of a vector) is `error[rank]` until decided. Boxes compare by what they hold in `=`, `m_atch` and the search built-ins (`Box a` is in `Eq` when `a` is) but are not ordered (not `Ord`), so `s_ort` of boxes is a type error. `xetal --ascii` draws the same pictures in plain ASCII, as APL2 did on terminals without box characters (`.` and `'` corners, `-` and `|` sides, `>` and `v` arrows, `e` for boxes), one character for one, so a picture keeps its shape; the generated reference uses it (approved by the user). |
| B17 | Transpose, decided with the user for the transpose lane (asked for by X_eTaL-demos, ADVANCEDEX and the classics): `o_\ A` (reserved in A2, B2) reverses the order of all axes, as in APL and J, so a 2 3 4 array becomes 4 3 2 and a vector or a scalar is unchanged; it is monadic, since a name has one arity (B9). `p t_ranspose A` permutes the axes: p is a permutation of 1 .. the rank of A, and axis i of A becomes axis p[i] of the result (APL's dyadic transpose without diagonals); a repeated or out-of-range axis is `error[domain]`, a p of another length `error[length]`. `o_\_jk A` swaps axes j and k (transpose's own axis rule); one digit or three are `error[axis]`. `[]P_ATH` keeps taking 2 rows, x over y. |
| B18 | `d_ecode` takes any numbers, as APL's decode does: `Num a => a -> a -> a`, Horner's rule with the radix on the left, so `x d_ecode r_ev c` evaluates the polynomial with coefficients c (lowest first) at x, and `2.0 d_ecode 3.0 -2.0 1.0` is `9.0`. Radix and digits share one number type (Int and Float do not mix, T5), so Int programs are unchanged (`10 d_ecode 1 2 3` is `123`). `e_ncode` stays Int-only: a fractional or negative radix and floating residues are left undecided. Decided with the user, 2026-10-03 (asked for by X_eTaL-libraries, ask X9, for Polynomials); not yet implemented. |

The Life one-liner is unaffected (it uses only `r_/` and `o_-`);
words appear in the surrounding code, e.g.
`board := 5 5 r_eshape 0 0 0 0 0 0 0 1 0 0 ...`.

## 12. Sections

| #  | Decision |
| -- | -------- |
| SC1 | A symbol function applied to one argument is an error (`- 3`, `/ 2`, `2 +`): under currying it would fill the *left* argument, so `- 3` would mean "3 minus ...". Named functions still curry (`r_/ '+`, `o_- 1`), and a quoted symbol may be partially applied explicitly (`('-)_ 3`). |

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

## 13. Strings

| #  | Decision |
| -- | -------- |
| ST1 | A string is written `"..."`. Escapes: `\"`, `\\`, `\n`, `\t`; any other escape is an error. `#` inside a string is an ordinary character. |
| ST2 | A string may not span lines (use `\n`) and may hold any Unicode (I1 as revised); a string is a vector of characters, one per Unicode scalar value, so a combining mark counts as a character of its own (`t_ally` counts code points, as APL does). A name touching `"` is an error, reserving `r"..."` for later. |
| ST3 | A string is a 1-D array of characters: `"abc"` is a 3-element Char vector and every array function applies (`r_ev "abc"` is `"cba"`). `"a"` is a 1-element vector (no APL length-1 scalar wart). There is no scalar character literal for now. |

Planned enhancements (after the MVP milestones):

- Rust-style raw strings `r"..."` that may span lines.
- Unicode text as a library-supplied type extension, once the
  language has a way for libraries to add types.

## 13a. System names (quads)

Decided with the user after the libraries saga; implemented by the
quads saga (docs/plan.md).

| #  | Decision |
| -- | -------- |
| QD1 | `[]` written touching a name is the system namespace, APL's quad: `[]A`, `[]D_L 0.5`. It is one token and displays as the quad glyph (U+2395) before the name. An empty `[]` on its own stays an error (an empty train). Class comes from the tokens as everywhere else: a name without an underline is a value, one with an underlined letter a function. |
| QD2 | System values, read-only: `[]A` the alphabet `"ABCDEFGHIJKLMNOPQRSTUVWXYZ"`, `[]D` the digits `"0123456789"`, `[]AV` the atomic vector (every character, ASCII 0 to 127, since source is ASCII), `[]TS` the time stamp (year, month, day, hour, minute, second, millisecond; read afresh each time), `[]IO` the index origin, always 1 (there is no index-origin setting, A5). |
| QD3 | System functions live in the quad namespace and are named like any function: `[]D_L s` waits s seconds (APL's DL), `[]U_CS` converts between characters and their codes (`[]U_CS "A"` is 65, `[]U_CS 65` is `"A"`), `[]R_EAD @` reads a line typed by the user as a Char vector (APL's quote-quad) and `[]V_ALUE @` reads a line and evaluates it (APL's quad input). Quad names are uppercase, as in APL, so they stand out, and take no `!` even when they have an effect: the quad already marks a system facility (APL writes the delay as quad DL); a quad function still has its first letter underlined (`[]D_L` displays as the quad glyph, D underlined, L). Ordinary built-ins stay plain lowercase words. |
| QD4 | Files, the keyboard and numbers as text (decided with the user for TTTML's saved model and its moves): `t []N_PUT path` writes text t to a file (made with its directories, or replaced) and gives how many characters; `[]N_GET path` reads a file's text; `[]R_EAD @` reads a line typed at the keyboard (without its newline). `f_ormat v` is v as the text it prints as (APL's format), and `n_umbers t` the numbers in text t, separated by spaces or newlines, as Floats (Dyalog's VFI), an error for anything that is not a number. Saving an array is `(f_ormat m) []N_PUT path` and reading it back `n_umbers []N_GET path`, reshaped. APL's shared variables (`[]S_VO`) are not used for files; that name stays reserved (section 15). |
| QD6 | A terminal for interactive programs (asked for by X_eTaL-games): screen control is named, typed system functions, not escape sequences or words in text. Settled with the user (2026-10-03), choices typed, never strings: two built-in enumerated types, `Color` (black, red, green, yellow, blue, magenta, cyan, white) and `Key` (up, down, left, right, enter, escape, backspace, tab, delete, home, end, and a printing key carrying its character), are nominal types in the checker; their values are named in the standard Terminal library (`"t:" u_se< "Terminal"`, then `t:RED`, `t:UP`), which builds them with typed constructors meant for libraries (`[]C_OLOR : Int -> Color`, `[]K_NAMED : Int -> Key`), so a misspelled name is an undefined name and a Key where a Color is expected a type error when the program is checked. The screen functions are pure, typed text builders whose results a program prints: `c []F_G t` and `c []B_G t` (`Color -> Char -> Char`, a foreground or background colour), `[]B_OLD t`, `r_c []A_T t` (`Int -> Char -> Char`, text placed at row and column, 1-origin), `[]C_LS @` (the text that clears the screen), so `p_rint! t:RED []F_G []B_OLD "Game over"` composes. `[]K_EY @ : Unit -> Key` waits for one key, no Enter, and `[]K_CHAR k : Key -> Char` gives a printing key's character (empty for a named one); keys compare with `=`. `[]E_RR t` writes t as a line to standard error (red in the live demo) and gives t; `[]T_E @` gives four Ints: rows, columns, 1 when output is a terminal, 1 when it does screen control. The CLI writes the screen text as ANSI sequences; the live demo's terminal interprets the same sequences into its grid. The enum machinery is general: Saga 29 (algebraic data) makes Color and Key ordinary declarations. |
| QD5 | Graphics, decided with the user for the classics lane: programs compute what to draw as ordinary arrays, and pure system functions turn them into one self-contained SVG, returned as a Char vector: `[]G_RID m` draws a Bool, number or Char matrix as a grid of cells (a rank-3 array is frames along the leading axis, animated), `[]P_ATH xy` draws points given as 2 rows, x over y, as a polyline (rows rather than the n by 2 columns first proposed, because there is no transpose yet and a turtle's running sums give the rows directly; a rank-3 array is frames of paths; confirmed by the user after merging, with transpose planned, after which n by 2 may also be accepted). `[]S_HOW svg` is the one effect: the command line writes numbered `.svg` files, the web and a future desktop app show the picture in a Draw pane. Large grids are embedded as an image inside the SVG: more than 4096 cells a frame (64 by 64), confirmed by the user. Turtle graphics is a library written in the language (`lib/Turtle.xtl`). |

## 14. Macros and libraries

| #  | Decision |
| -- | -------- |
| MC1 | A macro phase runs between the lexer and the parser. It works on the token stream (not raw text), and every token keeps its original file and span, so errors inside an inlined library point at the library's source. |
| MC2 | A macro is a function-shaped name ending in `<` ("slurp in"): `u_se<`. Macros are system-provided. No keywords are introduced. The `<` suffix on function names is reserved for macros. |
| MC3 | `u_se<` is applied like a dyadic function: `"c:" u_se< "Combinators"` inlines the library with its namespace rewritten to `c:` (Python's `as`). The alias is always required; monadic `u_se< "X"` is an error, so every namespace is visible where it is introduced. |
| MC4 | The library argument is a string: an installed library name (`"Combinators"`) or a file path (`"../lib/life.xtl"`). The alias string must be a namespace (lowercase letters followed by `:`); otherwise it is a macro-phase error. A string with a `/` or ending in `.xtl` is a path relative to the importing file; a name `Name` is `Name.xtl` looked for in the importing file's directory, then in `userlibs/` in the current directory (libraries of your own; added with the user), then in each directory of `XETAL_PATH`, then among the standard libraries built into `xetal` (the repo's `lib/`). Libraries are not executable files and programs are; `xetal run` on a library lists its exports' types. Library names are capitalized by convention. Decided with the user in the libraries saga. |
| MC5 | A library defines its own names under `l:` ("this library"); a program defines under `u:`. The macro rewrites a library's `l:` to the importer's alias. There is no namespace-declaration macro. |
| MC9 | In a library, the `l:` prefix is the export list for variables and functions alike (`l:pi := 3.14...`, `l:c_ircle := ...`); top-level names without `l:` are private to the file. Importers see exported names under their alias (`m:pi`); a private name is MC8 row 11 (not defined by the library). In a library a top-level function without a namespace is private (`h_elper := { ... }`, used as `h_elper x`; one named like a built-in shadows it with the L7 warning), and a library holds only definitions, bindings and its own imports: a top-level expression there is MC8 row 16, so importing never prints (decided with the user in the libraries saga). In a program, plain top-level variables are the user's own and are the only spelling (`u:x` is an error); `u:` remains required for user functions (N6). (Review R3.) |
| MC6 | Aliases are per file. A library's own imports are private to it. The macro phase renames every library instance to a hidden, globally unique internal namespace and rewrites each file's letters through that file's alias table, so letters in different files never collide. Error messages and the display use the letter written in the file being read. |
| MC7 | Each library (identified by its resolved path) is instantiated once and shared by every file that imports it; values are immutable, so sharing is safe. |
| MC10 | Macro libraries are `.xtlm` files. A macro library defines its exported macros under `m:` ("this macro library"), as a library defines its exports under `l:` (MC5), and each definition carries the macro mark: `m:u_nless< := { cond body -> ... }`. A macro is an ordinary XeTaL function of type `(String, String) -> String`: it takes the source text written at the call as two strings and returns source, which replaces the call and goes back through the lexer (each token keeps a span pointing at the call). The importer picks the alias as for any library (MC3), and the macro phase rewrites the file's `m:` to it, so `"x:" u_se< "Control"` then `"n = 0" x:u_nless< "p_rint! 100 / n"`. `m:` is a placeholder only inside a `.xtlm` (reserved there, as `l:` is in a library); in programs and `.xtl` libraries `m:` stays an ordinary alias (`"m:" u_se< "Maybe"`). Top-level names without `m:` in a `.xtlm` are private helpers. A macro library is compiled and run before the importing file is expanded, so a file never uses a macro it defines itself. The built-in `u_` prefix stays for system macros (`u_se<`). Expansion repeats on the result, to a depth limit (an error points at the call and the macro's definition). Asked for by X_eTaL-libraries (ask X1). Decided with the user, 2026-10-02; implemented in the macros lane (2026-10-03), how a macro runs in MC23. |
| MC11 | `"m:" u_se< "Name"` finds `Name.xtl` and `Name.xtlm` together: the search directories keep MC4's order, the first directory holding either file wins, and whichever of the two it holds load under the one alias (functions and values `m:f_`, macros `m:f_<`; the `<` keeps them apart). Files from different directories are never mixed. Neither file anywhere is MC8 row 1. An explicit path (`"../x.xtl"`, `"../x.xtlm"`) loads only that file. Decided with the user, 2026-10-02; implemented in the macros lane (2026-10-03). |
| MC12 | A macro call may stand as a top-level statement or inside an expression; its expansion parses as a block of statements or as a parenthesized expression, by where it stands. `u_se<` itself stays a top-level statement (MC8 row 13). Decided with the user, 2026-10-02; implemented in the macros lane (2026-10-03). |
| MC13 | A namespace prefix is a lowercase letter followed by lowercase letters or digits, of any length, then `:` (`c:`, `b2:`, `combinators:`); the lexer and the alias check apply the same rule. It is drawn as a superscript; when a character has no superscript form (such as `q`), the whole prefix is drawn as written (`quux:`). Revises N5's one-letter convention. Decided with the user, 2026-10-02; implemented in the macros lane (2026-10-03): the lexer and `valid_alias` read a prefix alike, an all-uppercase prefix is the macro phase's hidden namespace (written in a file, `hidden-namespace`), a digit has no superscript form in a prefix (`b2:` is drawn as written, so a raised digit is always an exponent). |
| MC14 | `"c" i_f< "a; b"` is a system macro (no import; defined in System.xtlm, MC18): the value of `a` when the condition `c` holds, else of `b`. It expands to a run-time guard, `{ @ -> (c) ? a; b } @`, so the condition is an ordinary expression evaluated when the program runs, both texts are type-checked and share one type, and only the chosen one is evaluated. Its right side is exactly two expressions separated by `;` (a newline counts as `;`), else `bad-macro-argument`. Chosen over a condition known when the program is compiled: that would need constant evaluation in the macro phase and could not test run-time values, while the guard needs nothing new and keeps the program fully typed; a compile-time choice can come later as a macro library's. Proposed in the macros lane (2026-10-03), to confirm with the user. |
| MC15 | `"c" u_nless< "b"` is a system macro: run the statements `b` unless `c` holds. It expands to `{ @ -> (c) ? @; b; @ } @`: a run-time guard (as MC14), `b` run for its effects (one or more statements, written as in a lambda body), and the value Unit (`@`) either way, so the program stays typed whatever `b` is; at the top level the statement shows `@` as any Unit statement does. Proposed in the macros lane (2026-10-03), to confirm with the user (the alternative: a value, with a default for when `c` holds). |
| MC16 | `"w1 w2" e_ach< "template"` is a system macro: one copy of the template per word of its left (a word is a run of characters other than white space), `$w` in the template replaced by the word (`$w` is never valid code, so a template can build names: `"sum max" e_ach< "u:$w_s := ..."`). The copies are statements, one per line, so `e_ach<` stands as a statement of its own (inside an expression it is `misplaced-macro`); no words, or a template without `$w`, is `bad-macro-argument`. It is distinct from the function `e_ach` (the macro mark makes another token). Proposed in the macros lane (2026-10-03), to confirm with the user. |
| MC17 | System macros are expanded in each file before its imports and names are read, again on each expansion (a macro written in an argument is expanded in turn) up to 32 levels (`macro-depth`). A call is `"left" n_ame< "right"`: a string, or `@` for none (MC22), on each side, no further strings or calls beside them (`bad-macro-call`); where it stands decides its form (MC12): a statement of its own (at the top level or in a lambda body) expands to statements, inside an expression to a parenthesized expression. Text a macro copies from an argument is reported where it was written, inside the string (runs of the macro's text found in an argument map back to it); text the macro adds is reported at the whole call. `xetal expand FILE` (and `-e`) prints the program after expansion, imports and names as written. Inside an argument of `i_f<` or `u_nless<`, `_l` and `_r` would name the macro's own niladic lambda (an error); use named parameters. Proposed in the macros lane (2026-10-03), to confirm with the user. |
| MC18 | The system macros live in `lib/System.xtlm`, written in X_eTaL like any macro library and built into the binary (so the browser needs no file). In it they are defined under `s:` (`s:i_f< := { cond both -> ... }`), as a macro library defines under `m:` and a library under `l:`, so a search for definitions tells the three apart; `s:` is that placeholder only inside System.xtlm, and everywhere else an ordinary alias (`"s:" u_se< "Stats"` then `s:m_ean x`). A system macro is never called with `s:`: `s:i_f<` is `i_f<` under whatever `s:` names (an error when that has no such macro). Decided with the user, 2026-10-03; implemented in the macros lane. |
| MC19 | System.xtlm is loaded before every file, with no import and no alias: its `s:` macros are called unprefixed (`"c" i_f< "a; b"`). A macro library that defines one of those names is an error (`system-macro-redefined`), not shadowing; a program or library cannot define a macro at all (MC10). Decided with the user, 2026-10-03; implemented in the macros lane. |
| MC20 | What only the compiler knows comes from a few hooks, quad built-ins usable only in a macro body while a call is expanded (anywhere else `hook-outside-macro`); System.xtlm wraps them as macros, and everything else in it is plain X_eTaL. Hooks: `"code place" []R_EJECT "message"` (the call fails with that code and message, reported at the macro's name, `place` `call`, or at its `left` or `right` argument), `[]S_TATEMENT @` (whether the call stands as a statement of its own), `[]F_ILE @` and `[]L_INE @` (where the call is written; inside another macro's expansion, that macro's call), `[]I_NCLUDE path` (a file's text) and `[]C_FG name` (a configuration fact). Decided with the user, 2026-10-03 (the spellings proposed in the macros lane). |
| MC21 | `u_se<` stays built into the compiler and is not declared as an `s:` macro in System.xtlm: what it does (finding, loading and binding a library's names under an alias) is not text, and a macro can only give source text, never create bindings. System.xtlm carries a comment showing how `u_se<` would be declared were it a macro (its call shape, `"alias" u_se< "Name"`, and the two texts it would receive), as a model for users, saying plainly why it is built in. Decided with the user, 2026-10-03 (replacing the proposal to declare it with a built-in body). |
| MC22 | A side of a call that takes no argument is written `@`: `@ i_nclude< "data.csv"`, `@ l_ine< @`, `p_rint! @ f_ormat< "x = {x}"`. Every call stays dyadic (MC3). `@` and `""` are different arguments: `@` is no argument (the macro receives Unit), `""` is an empty text. A macro declares which sides it takes by its parameters' types: a side whose parameter is Unit takes only `@` (text there is `bad-macro-argument`: "takes @ on its left, not text"), a side that takes text rejects `@`. Decided with the user, 2026-10-03; how a `.xtlm` macro writes a Unit side is proposed: today by using the parameter as Unit (`{ u r -> ({ @ -> r })_ u }`); a parameter written `@` among others (`{ @ r -> ... }`, as niladic lambdas take `@`) would need L6 extended. |
| MC24 | Which macros are system macros: a macro is a system macro (in `lib/System.xtlm` under `s:`, called unprefixed) when it needs what only the compiler knows, or nearly any program could use it (the test Rust's std uses). Every domain-specific macro is a library macro, under `m:`, called under an alias, in the libraries repository or a user's own `.xtlm`. The set stays small; adding a system macro is its own decision. The set: compiler-only `u_se<` (declared, built-in body), `i_nclude<`, `c_fg<`, `f_ile<`, `l_ine<`, `e_rror<`; general-purpose `f_ormat<` (interpolation, Rust's `format!`), `d_bg<` (Rust's `dbg!`), `a_ssert<` (Rust's `assert!`), and `i_f<`, `u_nless<`, `e_ach<`. Decided with the user, 2026-10-03; `i_f<`, `u_nless<`, `e_ach<` implemented, the rest planned in the macros lane. |
| MC25 | The compiler-only system macros, each with `@` for a side that takes nothing: `@ l_ine< @` (the call's line, a number), `@ f_ile< @` (its file, a string; `-e` for command-line text), `@ i_nclude< "path"` (a file's text as a string literal, Rust's `include_str!`: from the disk at the command line, from the store in the browser), `@ c_fg< "name"` (1 or 0: the platform, `cli` or `web`, or a flag set with `xetal --cfg NAME`), `"code" e_rror< "message"` (a compile error at the call, Rust's `compile_error!`; a macro writes it into its text to refuse a call). Proposed in the macros lane (2026-10-03), to confirm with the user: an include path is relative to the including file (`-e` text: the current directory), `..` allowed, an absolute path refused (`bad-include`); a missing file is `missing-file`. |
| MC26 | `@ d_bg< "expr"` (Rust's `dbg!`) is a system macro: the value of `expr`, after writing `[file:line] expr = value` to standard error (`[]E_RR`), the expression as written; it stands in any expression. Its value is bound to a local `dbgValue` in the niladic lambda it writes, so an expression naming `dbgValue` itself sees that local (proposed). Decided with the user (MC24's set), 2026-10-03; implemented in the macros lane. |
| MC23 | How a macro of a macro library runs: the `.xtlm` is loaded on its own when imported (its own imports first, its `m:` names in a hidden namespace; a `.xtlm` importing its own name finds itself and is an import cycle, so it imports its `.xtl` by path), and each call appends `"left" m:n_ame< "right"` to it, type-checks the whole (the call must give text, else `macro-not-text`) and runs it; what the call prints is the macro's text (a macro should not print). Text the macro gives is code at the call: errors in it are reported at the call, errors inside the macro library at its `FILE:LINE:COLUMN`. A macro's expansion may not import a library (`macro-import`). The standard macro library `Macros` (`lib/Macros.xtlm`) is the example: `m:u_nless<`, `m:d_ef<`, `m:c_heck<`. `xetal type X.xtlm` lists its macros' types. Proposed in the macros lane (2026-10-03), to confirm with the user. |

Example:

```
# Combinators.xtl  (a library)
l:K_ := { x y -> x }
l:B_ := { f_ g_ x -> f_ g_ x }
```

```
# life.xtl  (a program)
"c:" u_se< "Combinators"     # Combinators' l: is c: in this file
u:l_ife := { ... }
c:K_ 1 2
```

Macro-phase errors (MC8), reported before anything runs with the
file, span and the letters as written in that file:

| #  | Condition | Example |
| -- | --------- | ------- |
| 1  | library not found (installed name or path) | `"c:" u_se< "Combinatorz"` |
| 2  | import cycle (the chain is shown) | A uses B, B uses A |
| 3  | missing alias | `u_se< "Combinators"` |
| 4  | malformed alias | `"c" u_se< "A"`, `"3:" u_se< "A"` |
| 5  | reserved alias | `"u:" u_se< "A"`, `"l:" u_se< "A"` |
| 6  | one letter for two libraries in one file | `"c:"` used for A and for B |
| 7  | one library under two letters in one file | A imported as `c:` and as `k:` |
| 8  | a library defines `u:` names | `u:f_ := ...` in a used file |
| 9  | a program defines `l:` names | `l:f_ := ...` in the main file (standalone library runs may be allowed later) |
| 10 | unknown namespace letter in a file | `s:u_nion` without an `"s:"` import |
| 11 | name not defined by the library (near matches listed) | `c:Z_` |
| 12 | the same function name defined twice at top level of a file | `l:f_ := ...` twice |
| 13 | `u_se<` anywhere but a top-level statement | inside a lambda or parentheses |
| 14 | unknown macro | `x_yz< "a"` |
| 15 | wrong argument shapes (both must be strings) | `c u_se< "A"`, `"c:" u_se< 3` |
| 16 | a top-level expression in a library (it would print on import) | `1 + 2` in a used file |
| 17 | a system macro's argument malformed (MC14, MC16) | `"c" i_f< "1"`, `" " e_ach< "$w"` |
| 18 | a macro defined outside a macro library's `m:` exports, an `m:` export without `<`, or `l:` names in a `.xtlm` (MC10) | `u:f_< := ...`, `m:f_ := ...` |
| 19 | a macro that does not give text, or fails when run (MC23) | `m:b_ad< := { a b -> 3 }` |
| 20 | a macro's expansion imports a library (MC23) | a macro giving `"c:" u_se< "A"` |

Row 12 narrows M1: rebinding stays allowed for variables, but a
top-level function name is defined once per file, so a library's
interface is unambiguous.

## 14a. Combinators

| #  | Decision |
| -- | -------- |
| CB1 | The standard library `Combinators` holds every bird Raymond Smullyan names in *To Mock a Mockingbird* that type-checks, each spelled as its letter (`l:K_`, `l:S_`); a digit follows the letter (`l:B_1`, the function B1 by D-8), a star is `s` (`l:C_s` for C*, `l:C_ss` for C**), the Bald Eagle's hat is `h` (`l:E_h`). Each bird's inferred type is pinned by a test. The list is `docs/birds.md`. Decided with the user in the combinators saga (spellings for the variants proposed in its first step). |
| CB2 | Y is in the library by recursion, `l:Y_ := { f_ -> f_ l:Y_ 'f_ }`, typed `(a -> a) -> a`, which works because a function's parameter is lazy. The birds that apply an argument to itself (L, M, M2, U) and the textbook Y have no finite type; they are in an untyped demo, run with `--untyped`. |
| CB3 | A second standard library, `Maybe`, Church-encoded (`n_othing`, `j_ust`, `b_ind` and helpers), shows a monad in the language as it is. Its higher-order functions take the function first, as a quoted operand, like `'+ r_/ A`, so a chain reads right to left: `-1 m:o_r 'g m:b_ind 'f m:b_ind m`. Church encoding types a maybe only by use: a `j_ust` built on its own does not fix the empty case's type, so a mismatched default is caught once the maybe can be either case (from a guard, say); named types would close that gap. |
| CB4 | `xetal type FILE` on a library (a file that names `l:`) checks it on its own, as it is loaded when imported, and prints the type of each export, written `l:` as in the file; private names and imported libraries are not listed. |

## 15. Queue of open questions

Optional, later: a spelling for axes above 9; an explicit `_`
wildcard parameter (`{ x _ -> x }`); count-from-the-end axes; raw strings `r"..."`; Unicode text as a
library type; nested arrays (A7); checked `::` signatures (T4);
complex numbers via the same type-extension mechanism (literal
`3j4`, currently a lex error, reserved for them); system I/O (see
below).

System I/O (discussed with the user, names to be decided): modern APLs
use a few named system functions for whole files (Dyalog's quad NGET /
NPUT, BQN's FChars / FLines), an FFI for native libraries (Dyalog's
quad NA, BQN's FFI) and networking as a library on top; shared
variables (quad SVO) are the older mechanism. For X_eTaL: file
functions in the quad namespace (text and bytes, read and write), and
`[]S_VO` reserved for shared-variable-style channels to special
facilities (graphics, a Rust dynamic library), the escape hatch a
networking library would build on; in the web demo, files map to the
browser's local storage.

## 16. Style guide (conventions; a linter may check them later)

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
| library (file `Name.xtl`) | UpperCamel | `Stats`, `Combinators` |
| library alias | short lowercase | `"s:" u_se< "Stats"` |

Recommended function-name suffixes:

| Suffix | Meaning | Example |
| ------ | ------- | ------- |
| `?` | predicate, returns Bool (instead of an `is` prefix) | `e_mpty?` |
| `!` | has a side effect (I/O, clock, randomness) | `p_rint!` |
| `/` | reduce-like, collapses an axis | `r_/`, `m_ax/` |
| `\` | scan-like, running results | `s_\` |
| `~` | approximate or tolerant | `e_q~` |

No `$` suffix for functions that produce text, for now (decided with
the user): they are named plainly, like `f_ormat`.

In a comment, backquotes mark X_eTaL code, drawn decorated
(`` `'+ r_/ v` ``); a shell command, a path or other text that is not
X_eTaL goes in double quotes, drawn as written: `# run with "just show
demos/tour.xtl"`.

## 17. Examples

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
u:n_ow! := { @ -> s_ysclock! @ } # niladic, an effect (!)
u:n_ow! @
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
u:Y_ := { f_ -> { x_ -> f_ x_ 'x_ } '{ x_ -> f_ x_ 'x_ } }
u:F_ := { ~s_elf n -> n <= 1 ? 1; n * s_elf n - 1 }
(u:Y_ 'u:F_)_ 5                      # 120: Y works because s_elf is lazy (E1)
```

Conway's Life (rule from design.md 6.2: S is the 3 by 3 sum including
the cell, next = (S = 3) + cell * (S = 4)):

```
u:l_ife := { ('+ r_/_12 -1 0 1 o_-_12 _r) { (_l = 3) + _r * _l = 4 } _r }
u:l_ife board
```
