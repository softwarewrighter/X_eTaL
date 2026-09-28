# X_eTaL -- Language Choices

Status: DECISIONS COMPLETE FOR SAGA 1, CONSISTENCY-REVIEWED. Each
entry was chosen by the user, one question at a time (Q1-Q50, sub-
questions, and review items R1-R6). Section 15 lists the optional
items deliberately left for later. The lexer, renderer, parser,
formatter, Core desugaring and a scalar evaluator implement it (saga
calculus); types, arrays and the rest follow the sagas in
`docs/plan.md`.

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
| A5 | Index origin is 1: `r_ange 5` is `1 2 3 4 5` and index 1 is the first item, consistent with 1-based axes. Not configurable (no APL-style index-origin setting). |
| A6 | An axis subscript works on any function by one rule: `f_k X` moves axis k to the front, applies f (which works on the leading axis) and moves it back. Built-ins and user functions alike: `u:n_ormalize_2 M`, and `'+ r_/_2 M` is reduce by this rule. Multi-digit subscripts (`_12`) mean something only where a function defines them (rotate, A4); on a user function they are an error for now. Moving an axis can be an index view, not a copy. |
| A7 | Nested arrays follow APL2 / BQN: any element may itself be an array (`"ab" "cde"` is a 2-element vector of strings), with enclose / disclose built-ins and no explicit box type. Planned for after the Life milestone; v0 arrays are flat (a flat array is a nested array of scalars), and v0 rules such as "`e_ach` returns scalars" are written so nesting can be added without breaking programs. |

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
| S7 | The command is `xetal` (easy to type, matches the crate slug). `x_etal` is installed as an alias (symlink), so `#!/usr/bin/env x_etal` also works. The display name stays `X_eTaL`, which decorates as X underlined, matching the logo. |

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
| T5 | Numeric typing is Haskell-style: arithmetic is `Num a => a -> a -> a` (Num = Int, Float); number literals are polymorphic (`3 + 2.5` and `2 ^ 0.5` type-check); comparisons return a `Truthy` type (Bool, Int) that defaults to Bool but is Int when used in arithmetic, so `(S = 3) + c * (S = 4)` is Int (T1); conditions and `& \| n_ot` take `Truthy`. An Int variable mixed with a Float needs `f_loat` (B8). A top-level binding that is not a function has its numbers defaulted when it is defined (Int; a condition Bool), so `n := 3; n + 2.5` is a type error. Decided with the user in the types saga. |
| T4 | Type annotations: none in v0 (types are inferred). `::` is reserved for later signature lines (`u:s_quare :: Int -> Int`), which will be checked against inference so they cannot drift, and shown by hover and doc tools. Until then, a comment `# :: Int -> Int` above a definition is an unchecked documentation convention. |
| M1 | Values are immutable. Rebinding a variable creates a new binding (shadowing); a lambda keeps the value it captured. A top-level function name is defined once per file (MC8 row 12). There is no indexed assignment; updates return new arrays. |
| M2 | Mutation is an explicit escape hatch: only variables named with a trailing `!` may be reassigned in place (`count! := count! + 1`), so every read and write shows it. |

## 9a. Evaluation

| #  | Decision |
| -- | -------- |
| E1 | Evaluation is strict by default. A lambda parameter may be declared lazy with a `~` prefix in the parameter list (`{ ~s_elf n -> ... }`): its argument is not evaluated at the call but on first use in the body, then remembered (call-by-need); unused, it is never evaluated. The body uses the parameter normally (`s_elf`, not `~s_elf`). |
| E2 | A function is evaluated before its argument, so the evaluator knows whether the parameter is lazy. Laziness is carried by the function value at run time and is not part of the static type. |
| E4 | Evaluation order is the function first, then its arguments right to left (APL order): in `x f y`, `f`, then `y`, then `x`. `(p_rint! 1) + p_rint! 2` prints 2 then 1; in a fork `(F x) G (H x)`, H's result is computed before F's. Lazy (`~`) arguments are not evaluated in this sequence, only on first use. The trace explainer steps in the same order. (Review R4.) |
| E3 | The Y combinator works in its textbook shape when its functional marks its self parameter lazy; Z also works. Lazy parameters also let users write their own control structures (`u:w_hen := { c ~a ~b -> c ? a; b }`). |

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
| I1 | Source is ASCII only. Decorated Unicode input is not accepted (revisit much later). |
| I2 | The display is Unicode where it can be, plus a LaTeX subset for anything Unicode lacks (for example superscript `q`, subscript `@`, superscript decimal point). |
| I3 | The display may render a multi-character token as one glyph: `:=` as the left arrow, `->` as an arrow, `;` as the diamond, `#` as the lamp, `!=` `<=` `>=` `*` `/` `&` `\|` as their mathematical glyphs. Ligatures apply to standalone tokens only, never to punctuation inside a function name (`r_/` keeps its slash). |

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
| B6 | Higher-order built-ins: `r_/` reduce and `s_\` scan (leading axis; empty reduce gives the operand's identity, or an error if it has none), `e_ach` (apply to each element; monadic or dyadic; results must be scalars until nested arrays exist), `t_able` (outer product: `1 2 3 '* t_able 1 2 3`), `i_nner` (inner product: `A '+ '* i_nner B`), `c_ompose` (`'n_eg 'a_bs c_ompose x` is `n_eg a_bs x`), `s_wap` (APL commute, the C combinator: `A '/ s_wap B` is `B / A`). Reduce and scan with a multi-digit subscript work over each listed axis in turn: `'+ r_/_12 X` reduces axis 1 and then the new axis 1 (as used by Life), `s_\_12` likewise (review R1). A quoted function is written exactly as its name is spelled (`'u:s_quare`, `'n_eg`, `'f_`). |
| B7 | Arithmetic, search and effect built-ins: `n_eg`, `a_bs`, `f_loor`, `c_eiling`, `m_ax` / `m_in` (dyadic; `'m_ax r_/ v` is the maximum), `d_iv` and `m_od` in maths order (`7 d_iv 2` is `3`, `7 m_od 3` is `1`), `n_ot`, `e_q~` (T3), `e_xp`, `l_og` (natural), `i_ndexOf` (`5 6 7 i_ndexOf 7 9` is `3 4`; not found gives tally + 1), `m_ember?` (`2 9 m_ember? 1 2 3` is `1 0`), `u_nique` (first-seen order), `s_ort` (ascending; descending is `r_ev s_ort v`), `g_rade` (sorting indices), `w_here` (indices of 1s), `r_oll!` (random 1..n), `p_rint!` (print and return the value). |
| B8 | `f_loat` converts a number to Float (T5): `f_loat 3` is `3.0`. |

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
| ST2 | A string may not span lines (use `\n`) and is ASCII, like all source. A name touching `"` is an error, reserving `r"..."` for later. |
| ST3 | A string is a 1-D array of characters: `"abc"` is a 3-element Char vector and every array function applies (`r_ev "abc"` is `"cba"`). `"a"` is a 1-element vector (no APL length-1 scalar wart). There is no scalar character literal for now. |

Planned enhancements (after the MVP milestones):

- Rust-style raw strings `r"..."` that may span lines.
- Unicode text as a library-supplied type extension, once the
  language has a way for libraries to add types.

## 14. Macros and libraries

| #  | Decision |
| -- | -------- |
| MC1 | A macro phase runs between the lexer and the parser. It works on the token stream (not raw text), and every token keeps its original file and span, so errors inside an inlined library point at the library's source. |
| MC2 | A macro is a function-shaped name ending in `<` ("slurp in"): `u_se<`. Macros are system-provided. No keywords are introduced. The `<` suffix on function names is reserved for macros. |
| MC3 | `u_se<` is applied like a dyadic function: `"c:" u_se< "Combinators"` inlines the library with its namespace rewritten to `c:` (Python's `as`). The alias is always required; monadic `u_se< "X"` is an error, so every namespace is visible where it is introduced. |
| MC4 | The library argument is a string: an installed library name (`"Combinators"`) or a file path (`"../lib/life.xtl"`). The alias string must be a namespace (letters followed by `:`); otherwise it is a macro-phase error. |
| MC5 | A library defines its own names under `l:` ("this library"); a program defines under `u:`. The macro rewrites a library's `l:` to the importer's alias. There is no namespace-declaration macro. |
| MC9 | In a library, the `l:` prefix is the export list for variables and functions alike (`l:pi := 3.14...`, `l:c_ircle := ...`); top-level names without `l:` are private to the file. Importers see exported names under their alias (`m:pi`); a private name is MC8 row 11 (not defined by the library). In a program, plain top-level variables are the user's own and are the only spelling (`u:x` is an error); `u:` remains required for user functions (N6). (Review R3.) |
| MC6 | Aliases are per file. A library's own imports are private to it. The macro phase renames every library instance to a hidden, globally unique internal namespace and rewrites each file's letters through that file's alias table, so letters in different files never collide. Error messages and the display use the letter written in the file being read. |
| MC7 | Each library (identified by its resolved path) is instantiated once and shared by every file that imports it; values are immutable, so sharing is safe. |

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

Row 12 narrows M1: rebinding stays allowed for variables, but a
top-level function name is defined once per file, so a library's
interface is unambiguous.

## 15. Queue of open questions

Optional, later: a spelling for axes above 9; function power on
functions; an explicit `_` wildcard parameter (`{ x _ -> x }`);
count-from-the-end axes; raw strings `r"..."`; Unicode text as a
library type; nested arrays (A7); checked `::` signatures (T4);
complex numbers via the same type-extension mechanism (literal
`3j4`, currently a lex error, reserved for them).

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

Recommended function-name suffixes:

| Suffix | Meaning | Example |
| ------ | ------- | ------- |
| `?` | predicate, returns Bool (instead of an `is` prefix) | `e_mpty?` |
| `!` | has a side effect (I/O, clock, randomness) | `p_rint!` |
| `/` | reduce-like, collapses an axis | `r_/`, `m_ax/` |
| `\` | scan-like, running results | `s_\` |
| `~` | approximate or tolerant | `e_q~` |
| `$` | produces text | `f_mt$` |

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
