# Review: spatial equations (research8 and the PoC plan)

Inputs reviewed: `docs/research8.txt` (archival, not normative) and
`docs/xetal-spatial-equations-plan.md` (the proposed plan). Checked
against `docs/lang-choices.md`, `builtins.toml`, `docs/asks.md`,
`docs/plan.md`, `lib/System.xtlm`, `lib/Macros.xtlm`,
`lib/Combinators.xtlm` and X_eTaL-ML's `libs/Conv`, with
`./target/release/xetal` (built from main at 8002548c). Every probe
below was run; the proof of concept's files are in `docs/spatial-poc/`; the other
probes stayed in the review's scratch area.

## Summary

The research conversation starts from a punctuation syntax for
two-dimensional math (`|~ upper | center | lower _|`), generalizes it
to "one center, ordered upper and lower regions, meaning given by the
consuming macro", and settles on prototyping with namespaced macros
(`e:q<` outer, `e:u<`, `e:c<`, `e:l<` inner) whose arguments would be
typed tuples of deferred symbolic expressions, transformed at compile
time with array operations into ordinary X_eTaL array code, with the
same structure also rendered as LaTeX. The plan turns that into a
proof of concept on one target, the 2-D CNN cross-correlation
`Y[o,h,w] = sum_c sum_r sum_s X[c,h+r,w+s] W[o,c,r,s]`: a capabilities
report (M0), an independent oracle (M1), macro capture (M2), domain
pairing (M3), lowering (M4), LaTeX (M5), an expansion report (M6),
tests and a README (M7) and a limitations list (M8), with the rule
that the compiler changes only after a minimal failing test.

## Viability

Verdict: **viable as a library, with no compiler change.** A
142-line proof of concept (`docs/spatial-poc/Spatial.xtlm`) written during this
review runs the CNN case end to end on the current binary: one macro
call gives both the tensor and its LaTeX, the expansion is ordinary
X_eTaL built from built-ins only, hygiene renames the macro's own
binders, and the result matches an independent oracle and Python on
two configurations. What does not survive is the plan's *surface*:
none of its X_eTaL snippets parse, macros take two strings rather
than typed tuples, and the symbolic layer is text processed by X_eTaL
array code (which is what the user proposed at research8 line 1617
and what `lib/System.xtlm` already does).

```
$ cat docs/spatial-poc/both.xtl     # (setup lines: X, W, C, H, Wd, O, Kh, Kw, Ho, Wo)
(Y, tex) := "Y[o,h,w] | O Ho Wo" e:q_< "C - 1, Kh - 1, Kw - 1 | X[c,h+r,w+s] * W[o,c,r,s] | c:=0, r:=0, s:=0"
$ xetal run docs/spatial-poc/both.xtl
560 596 632
704 740 776
848 884 920
Y_{o,h,w} = \sum_{c=0}^{C - 1} \sum_{r=0}^{Kh - 1} \sum_{s=0}^{Kw - 1} X_{c,h+r,w+s} \, W_{o,c,r,s}
$ xetal type docs/spatial-poc/Spatial.xtlm
m:s_um< : Char -> Char -> Char
m:t_ex< : Char -> Char -> Char
m:q_< : Char -> Char -> Char
```

`xetal expand docs/spatial-poc/cnn.xtl` shows the generated code (one line,
wrapped here):

```
Y := ({ @ -> g1:grid := (1 + (C - 1) - (0)) c_at (1 + (Kh - 1) - (0)) c_at (1 + (Kw - 1) - (0))
  c_at (O) c_at (Ho) c_at (Wo);
  g2:g_rid := { k -> g1:grid r_eshape (o_ffsets k s_elect g1:grid) 'l_eft t_able ('* r_/ k d_rop g1:grid) r_eshape 0 };
  g3:a_t := { A i -> (1 + i) s_elect r_avel A };
  '+ r_/_1 '+ r_/_2 '+ r_/_3
    (X g3:a_t ((g2:g_rid 6)+((0) + g2:g_rid 3)) + (3 s_elect s_hape X) * ((g2:g_rid 5)+((0) + g2:g_rid 2)) + (2 s_elect s_hape X) * (((0) + g2:g_rid 1)))
  * (W g3:a_t (((0) + g2:g_rid 3)) + (4 s_elect s_hape W) * (((0) + g2:g_rid 2)) + (3 s_elect s_hape W) * (((0) + g2:g_rid 1)) + (2 s_elect s_hape W) * ((g2:g_rid 4))) } @)
```

A second configuration (2 output channels, 3 input channels, a 2 by
3 kernel on 5 by 6 pictures, `docs/spatial-poc/cnn2.xtl`) gives shape `2 4 4` and
the same numbers as Python.

Per milestone:

| Milestone | Implementable as X_eTaL is? | Notes |
| --------- | --------------------------- | ----- |
| M0 capabilities | Yes (a document plus probes) | Most answers are in this review; they belong in spec cases and goldens, not prose alone |
| M1 oracle | Yes | `docs/spatial-poc/oracle.xtl`: rotations, `t_ranspose`, one `i_nner`; X_eTaL-ML's `cv:c_onv` is the same idea for 3 by 3 single-channel batches |
| M2 capture | Yes, in a changed form | Two strings per call (`"left" e:q_< "right"`), the three regions inside the right string; no typed tuples, no `~`, no nested calls as arguments |
| M3 domains | Yes | Pairing, counts and diagnostics in X_eTaL string code (`p_artition`, `m_ap`, `e_ach`, `[]R_EJECT`); descriptors as boxed texts or tuples of them inside the macro |
| M4 lowering | Yes | Coordinate grids by `t_able` and `r_eshape`, gathers by Horner's rule and `s_elect`, one `'+ r_/_k` per sum; bounds checks still to add (see Gaps) |
| M5 LaTeX | Yes | `m:t_ex<` / `m:q_<` give a string literal from the same parse; KaTeX (`tools/katex`, `pages/latex`) and the org literate documents can show it |
| M6 inspection | Mostly | `xetal expand`, `d_bg<`, `xetal run --echo`, the notebook; no stepping debugger (Saga 17 not done) |
| M7 tests, README | Yes | spec cases (`spec/macros/data` holds macro libraries), reg-rs goldens, `## >>` doc tests in the `.xtlm`, a literate document |
| M8 limitations | Yes | the candidates below, each with a failing snippet |

## Corrections

Each entry: where, what it says, what is true now (command and
output), the corrected form. P = the plan, R = research8 (line
numbers are the file's).

### C1. Macro names `e:q<`, `e:u<`, `e:c<`, `e:l<`

Where: P lines 19, 20, 105-108, 115; R lines 1485, 1501-1503, 1520,
1569-1575, 1583-1585, 1605, 1963. Says: `e:q<` is a valid macro name
and "a macro invocation is already identifiable by its invocation
syntax ... `e:q<` is perfectly reasonable" (R 1428, 1575). True now: a
macro is a function-shaped name ending in `<` (MC2, N2, N3); every
macro is an ordinary function `Char -> Char -> Char` (MC10). Without
an underline it is a variable and a less-than:

```
$ xetal lex -e 'e:q< "a"'
0..3 Var(e:q)
3..4 Sym(<)
5..8 Str("a")
$ cat p1/Eq.xtlm      # m:q< := { a b -> a c_at " + " c_at b }
$ xetal run p1/t1.xtl
error[expression-in-library]: a library holds definitions only; this would print when imported at ./Eq.xtlm:2:1
$ xetal lex -e 'e:q_< "a"'
0..5 Func(e:q_<)
```

Corrected: `e:q_<`, `e:u_<`, `e:c_<`, `e:l_<`. This also answers the
user's question at R 1071 ("maybe all macros are functions?"): yes,
already decided. R's own spellings `e:u_pper<`, `e:c_enter<`,
`e:l_ower<`, `e:e_quation<`, `math:e_quation<` and
`spatial:e_stack<` are valid names (long prefixes, MC13).

### C2. Call shape `e:q< A, B, C >`

Where: P lines 105-119; R lines 641-648, 699-702, 878-881, 1218-1223,
1500-1522, 1582-1592, 2228-2231, 2821-2824. Says: a macro takes three
comma-separated arguments (or nested macro calls) between `<` and
`>`. True now: a call is `"left" n_ame< "right"`, a string or `@` on
each side, nothing else beside them (MC3, MC17, MC22); a call as an
argument is refused; text inside a string is passed as written and
expanded only if the result contains it (so inner "macros" are
deferred for free, but only as text):

```
$ xetal run p1/t2.xtl     # (@ e:u_< "C - 1") e:q_< "2"
error[bad-macro-call]: e:q_< takes a string, or @ for none, on each side: "left" e:q_< "right" at 64..69
$ xetal expand p1/t3.xtl  # "(@ e:u_< \"7\") c_at \"x\"" e:q_< "2"
(("U|7")) c_at "x" + 2
```

Corrected: three regions inside one string, for example
`"Y[o,h,w] | O Ho Wo" e:q_< "C - 1, Kh - 1, Kw - 1 | X[c,h+r,w+s] * W[o,c,r,s] | c:=0, r:=0, s:=0"`.

### C3. `+/` for reduce

Where: P lines 107, 117, 165, 260 (R 2261); R lines 59, 94, 1088,
1221, 1502, 1584, 2230, 2358. True now: `+/` is not reduce (F7: reduce
is the function `r_/` with a quoted operand):

```
$ xetal eval -e '+/ 1 2 3'
error[symbol-needs-left]: a symbol function needs a left argument ...
$ xetal eval -e "'+ r_/ 1 2 3"
6
```

Corrected: `'+ r_/`.

### C4. Indexing `X[c,h+r,w+s]`

Where: P lines 107, 117, 159; R lines 1221, 1584, 1745, 1868, 2230,
2834. True now: brackets are trains (TR1); there is no index syntax:

```
$ xetal eval -e 'X := 2 4 4 r_eshape r_ange 32; X[1,2,3]'
error[unexpected-token]: expected `]` at 34..35
$ xetal eval -e 'X := 2 3 r_eshape r_ange 6; (2, 3) s_elect X'
error[type-mismatch]: expected Int, found (a, b) at 28..44
```

Corrected: indices on the left, 1-origin (B10, A5): `2 s_elect X` is a
major cell, `3 s_elect_2 X` a column; a pointwise item at several axes
is `(1 + i) s_elect r_avel X` with `i` the 0-origin index by Horner's
rule. Inside a macro's string `X[c,h+r,w+s]` is fine as notation the
macro rewrites (the PoC does).

### C5. `C-1`, `Kh-1`, `Kw-1`

Where: P lines 106, 116, 141, 147-149; R lines 1087, 1501, 1583,
1659, 1675-1677, 2229, 2357, 2388-2390, 2822, 2829-2831. True now:

```
$ xetal eval -e 'C := 2; C-1'
error[ambiguous-minus]: `-` touching the previous token and a digit is ambiguous: write `3 - 1` to subtract or `3 -1` for a negative literal at 9..10
$ xetal eval -e 'C := 2; C - 1'
1
```

Corrected: `C - 1`. A macro that copies region text into code must
either require X_eTaL spelling or space the minus itself.

### C6. Bindings inside a tuple `(c:=0, r:=0, s:=0)`

Where: P lines 108, 118, 142; R lines 1089, 1503, 1510, 1585, 1592,
1661, 2231, 2359, 2824. True now:

```
$ xetal eval -e '(c:=0, r:=0, s:=0)'
error[unexpected-token]: `:=` binds only at the start of a statement (top level or in braces) at 2..4
```

Corrected: inside a macro's string it is notation the macro parses
(the PoC reads `c:=0, r:=0, s:=0`); as X_eTaL it would be a tuple of
values or a tuple pattern `(c, r, s) := (0, 0, 0)` (TU3).

### C7. Tilde as quotation: `~(a,b)`, `(~a,~b)`, "tilde suppresses eager evaluation"

Where: P lines 60, 134, 203; R lines 1617, 1624, 1739-1789, 2288,
2446-2458. True now: `~` marks a lazy *lambda parameter* only (E1,
call-by-need, not inspectable); it is not an expression operator:

```
$ xetal eval -e '~(1, 2)'
error[bad-lazy]: `~` marks a lazy parameter and must touch its name at 0..1
$ xetal eval -e '(~1, ~2)'
error[bad-lazy]: `~` marks a lazy parameter and must touch its name at 1..2
```

Corrected: the quotation a macro needs is its string argument; nothing
in it is evaluated until the expansion runs. The plan's three-way
distinction (thunk, quotation, deferred code) is real, and the answer
is: strings are the quotation, generated code is the deferral, `~`
plays no part.

### C8. "Typed heterogeneous tuples of deferred symbolic expressions" as macro arguments

Where: P lines 22, 28, 124-131, 151, 230; R lines 1253-1258,
1487-1558, 1611, 1622, 1630-1683, 2089, 2292-2298, 2394-2404. Says:
macros receive (or could receive) typed tuples of unevaluated
expressions; R 1622 says "macros transform source expressions into
typed symbolic expressions". True now: a macro maps the two texts to a
text, which is lexed and parsed in place of the call (MC10, MC17);
there is no symbolic-expression value; typing happens after expansion
on the whole program:

```
$ xetal type docs/spatial-poc/Spatial.xtlm
m:s_um< : Char -> Char -> Char
```

Corrected: inside a macro, regions are `Char` vectors, lists of them
are `Box Char`, and a descriptor can be a tuple of boxes
(`(Box Char, Char, Box Char)`); records (`Domain(c, 0, C-1)`, R 2388,
P 145) are Saga 29's proposal (docs/adt.md), not available.

### C9. Compile-time array operators: `each, over, table, zip, indexing, reshape`

Where: P line 62; R lines 1697-1733, 2406-2441. True now: a macro is
ordinary X_eTaL run at expansion (MC23), so every built-in is
available; `e_ach` (scalar results), `m_ap` (boxes), `t_able`,
`r_eshape`, `s_elect`, `p_artition`, `w_here` exist; `over` is only a
lang-choices example (`u:o_ver := s_wap 'r_/`), and there is no `zip`,
`grid`, laminate or mix (mix is undecided, asks G15, M10). Pairing is
`a '{ u l -> ... } e_ach b` or a tuple. Corrected: name the real
built-ins; a coordinate grid is

```
g_ := { k -> G r_eshape (o_ffsets k s_elect G) 'l_eft t_able ('* r_/ k d_rop G) r_eshape 0 }
```

### C10. Reduction axes of `+/`

Where: P lines 66, 165, 206; R lines 247, 2588-2615. Says: ordinary
`+/` "may reduce only one axis"; choose flatten or successive
reductions. True now: settled (B6, A6, R1): `'+ r_/` reduces the
leading axis (a right fold), `'+ r_/_k` axis k, and a multi-digit
subscript reduces each listed axis (original numbering):

```
$ xetal eval -e "T := 2 3 4 r_eshape r_ange 24; '+ r_/_23 T"
78 222
$ xetal eval -e "T := 2 3 4 r_eshape r_ange 24; '+ r_/_13 T"
68 100 132
$ xetal eval -e "'+ r_/_123 2 2 2 r_eshape r_ange 8"
36
```

Corrected: the lowering writes one `'+ r_/_k` per sum
(`'+ r_/_1 '+ r_/_2 '+ r_/_3`, innermost applied first) or
`'+ r_/_123`; both keep reduce's meaning. Axes are 1 to 9 (D-9), so
summed plus free indices are limited to 9.

### C11. The expected output of Fixture B

Where: P lines 90-96. Says `[[852,888,924],[996,1032,1068],[1140,1176,1212]]`.
True now: valid cross-correlation of those fixtures is

```
$ xetal run docs/spatial-poc/oracle.xtl
560 596 632
704 740 776
848 884 920
1 3 3
8 8 8
8 8 8
8 8 8
```

(Python agrees; the flipped kernel gives 524 560 596 / 668 704 740 /
812 848 884, so the plan's numbers are neither.) Corrected: the
values above; Fixture A (all ones) is 8 everywhere, as the plan says.

### C12. Paths and repository layout

Where: P lines 34-51; R lines 2121-2146. Says:
`libraries/spatial/equations.xtlm`, `symbolic.xtl`, `domains.xtl`,
`lowering.xtl`, `render.xtl`, `ml/cnn-spatial/*.xtl`. True now: no
`libraries/` or `ml/` directory. This repository has `lib/Name.xtl` and
`lib/Name.xtlm` (standard libraries built into the binary), `userlibs/`
and `demos/`; X_eTaL-libraries has `libs/<Name>/{src,docs,demos,tests}`;
X_eTaL-ML has `libs/Conv/` and `demos/<name>/`. Library names are
capitalized (MC4), `Name.xtl` and `Name.xtlm` load together under one
alias (MC11), and each separate `.xtl` would be a separate library
with its own alias. The repository roles in R 2122-2142 are right.
Corrected: one `Spatial.xtlm` (macros) beside `Spatial.xtl` (run-time
helpers), plus a demo directory.

### C13. Index origin

Where: P lines 141-149, 158; R 2486. Says lower bounds `c:=0`. True
now: arrays are 1-origin with no setting (A5); `o_ffsets n` is
`0 .. n-1` for offset arithmetic (B5). Not an error as math, but the
lowering must add 1 at the gather (the PoC does). State it in the
docs.

### C14. Typed domains with shape checks

Where: P lines 151, 204, 220; R 1556. Says the type system could
check bounds and shapes. True now: types are rank-erased (T7), a
shape-carrying type was declined (ask M12); shapes are checked when
the program runs. Corrected: validate counts and names in the macro
(compile time) and shapes with run-time checks in the generated code.

### C15. Surrounding assignment for the left-hand side

Where: P line 179; R 2758-2770. Says: capture `Y_{o,h,w}` from the
assignment if possible. True now: a macro sees its two strings, and
the hooks `[]S_TATEMENT @`, `[]F_ILE @`, `[]L_INE @` (MC20); not the
statement around it. Corrected: put the left-hand side in the left
argument (`"Y[o,h,w] | O Ho Wo"`), or let the macro stand as a
statement and write `Y := ...` itself, as `c:Y_<` writes a definition.

### C16. Debugger and stepping

Where: P lines 194, 212; R 2320-2322, 2857. True now: there is no
debugger or stepper (Saga 17, trace-and-explain, is not started).
Available: `xetal expand`, `@ d_bg< "expr"`, `xetal run --echo`, the
notebook (`docs/notebook.md`), the live demo's Step button. Corrected:
"trace/report mode first" as the plan says; the debugger is Saga 17.

### C17. Punctuation delimiters and names

Where: R lines 14, 59, 91-97, 111, 170, 188, 254, 417, 426, 534-591,
632, 892, 998-1042, 1071, 1264-1309, 1437-1449; P lines 247, 230.
True now (lexing `a TOKEN b`): `|~` and `~~` are `bad-lazy`; `|\` and
`\|` `unexpected-char`; `__` `bad-lambda-arg`; `_|` is the wildcard
`_` then `|`; `||`, `|/`, `/|`, `//`, `--`, `|<` are two symbol
tokens; `e:|<` and `math:\|*<` are `bad-namespace`. `|` is already
"or" and `~` the lazy mark (and a name suffix, `e_q~`), which answers
R 632. Corrected: inside a macro's string any punctuation can be a
region separator (the PoC uses `|`); as tokens, every one of these
needs a lexer change (candidate L5).

### C18. Call and math syntax inside research examples

Where: R lines 170, 188 (`sum f(i)`, `product f(i)`), 591
(`i mod 2 = 0`), 1246-1249 (`upper = (...)` named arguments), 699-702
(`[n, step]` lists). True now: `sum`/`product` are `'+ r_/` and
`'* r_/`; a function applies by juxtaposition (`u:f_ 3`, and
`u:f_(3)` is the same); `=` is equality (S1), macros have no named
arguments; `[ ]` is a train. The conditional sum of R 220-240 is
already one line, and right-to-left order matters in the predicate:

```
$ xetal eval -e "'+ r_/ (r_ange 10)^2"
385
$ xetal eval -e "i := r_ange 10; '+ r_/ (0 = i m_od 2) r_eplicate i^2"
220
$ xetal eval -e 'i := 1 2 3 4; i m_od 2 = 0'
error[division-by-zero]: division by zero at 14..26
```

This also answers R 247 and R 891 ("math sum vs plus reduction"):
the one idiomatic sum is `'+ r_/` over an array of terms; a spatial
macro should write exactly that, never a second meaning.

### C19. Hygiene "if supported"

Where: P lines 151, 204; R 2620-2624. True now: hygiene is automatic
(MC30, D77): binders the macro writes around the call's code are
renamed `g1:`, `g2:`, ...; `## binds:` declares anaphors. Its
documented limit bites this design (one-letter index names):

```
$ xetal expand p1/t5.xtl     # "c r s" e:l_et< "c + r + s" (binds each word)
{ @ -> g1:c := 3; r := 3; g2:s := 3; c + r + s } @
$ xetal run p1/t5.xtl
error[undefined-name]: c is not defined at 33..34
```

Corrected: say hygiene exists, test it, and lower by substitution so
the macro binds no index names (the PoC); see candidate L2.

### C20. Repositories to research

Where: P line 34; R 438, 2331-2333. `tuple-rs`, DiscoveryOne and
`sw-cor24-macrolisp` are not under `~/github` (`find /Users/mike/github
-maxdepth 3`); X_eTaL-libraries, X_eTaL-ML and X_eTaL-demos are
siblings of this repository. Corrected: name where those three live,
or drop them from M0.

### C21. Minor

- P line 12 and line 173 write `C_{in}`, the code `C`: pick one.
- P line 39 `equations.xtlm` lowercase: libraries are capitalized
  (MC4): `Spatial.xtlm`.
- R line 1086 `math:e_equation<` is a typo for `math:e_quation<`.
- R lines 1490-1494, 2402: `struct Equation<U, C, L>`, `Domain<T>`:
  no user-defined types yet (Saga 29).

## Already exists

- Macro libraries `.xtlm` with `m:` exports, imported under an alias
  beside the `.xtl` (MC10, MC11, MC23); `xetal type X.xtlm` lists
  them; `## >>` doc tests run by `xetal doc --test`.
- Raw-text capture: macro arguments are the source text as written
  (the "Approach 2" of R 658-676), and text inside a string is not
  expanded before the outer macro runs.
- Array processing at compile time: the macro body is X_eTaL; the
  system macros already do their string work with `p_artition`,
  scans and `m_ap` (D65). The user's "arrays instead of AST" idea
  (R 1617) is how macros work today.
- Diagnostics: `[]R_EJECT "code left|right|call" "message"` placed at
  an argument; `e_rror<`; errors in copied text map back into the
  string.
- Hygiene (MC30) with `## binds:` anaphors; deterministic `g1:`
  numbering for goldens.
- `xetal expand FILE` for the generated source; `d_bg<`, `--echo`,
  the notebook, the live demo's Step.
- Tuples and patterns (TU1-TU12), so one call can give
  `(value, latex)` and bind both: `(Y, tex) := ... e:q_< ...`.
- Reduction on any axis or set of axes (`'+ r_/_k`, `'+ r_/_123`);
  `t_able`, `i_nner`, `t_ranspose`, `o_ffsets`, `d_ecode`, `e_q~` for
  float tolerance (T3).
- LaTeX: `xetal render --latex`, the KaTeX gallery
  (`scripts/latex-gallery.sh`, `pages/latex/`), the org literate
  documents (`docs/literate/*.org`, `pages/literate/`), `[]S_HOW` for
  SVG.
- A convolution in X_eTaL-ML (`libs/Conv/src/Conv.xtl`, `cv:c_onv`,
  3 by 3, single channel per picture, by rotations and `i_nner`) and
  a bench (`bench/conv.xtl`); not the multi-channel oracle the plan
  needs, but the same method as `docs/spatial-poc/oracle.xtl`.
- Ask M11 / X14 already records the alias problem (candidate L1).

## Gaps

1. No pointwise multi-axis gather: the ravel-and-Horner gather reads a
   neighboring row instead of failing when an index is out of range on
   a non-leading axis (`docs/spatial-poc/bad4.xtl`, the sum of row 1
   with c in 0..4 on a 2 by 4 array, prints 15 instead of an error;
   `(1 + 3 + 3 * 0) s_elect r_avel 2 3 r_eshape r_ange 6` gives 4).
   The generated code needs a bounds check per axis.
2. Output extents: the math leaves the free indices' ranges implicit;
   the macro needs them (`O Ho Wo`) or must infer them (only easy
   cases: `h + r < H` gives `Ho = H - Kh + 1`). Explicit for the PoC.
3. Error spans inside rewritten text: an undefined name in the
   summand is reported over the whole rewritten span
   (`docs/spatial-poc/bad2.xtl`: `error[undefined-name]: q is not defined at
   56..91`), not at `q`. Fine for a PoC; ask X15 (Saga 27) is the
   general fix.
4. Region syntax collisions: `|` as the region separator collides
   with "or" in a summand; `,` with tuples; the PoC's splitter ignores
   separators inside brackets and parentheses only.
5. Index expressions follow X_eTaL's right-to-left order, not math
   precedence: `X[2*r+h]` means `2 * (r + h)`. Either document it or
   have the macro parenthesize by math rules (a precedence parser in
   X_eTaL).
6. Memory: the grid lowering materializes the full `C Kh Kw O Ho Wo`
   product; fine for teaching sizes, not for MNIST-scale batches. A
   contraction recognizer could lower to `i_nner` (the oracle's path)
   later.
7. The LaTeX is library-made text; nothing in `xetal render` draws
   the call itself as stacked math, and the source cannot be laid out
   on three lines (strings are single-line, ST2; candidate L4).
8. No debugger stepping through the generated code (Saga 17).
9. A macro cannot call its own library's run-time helpers in its
   expansion (candidate L1), so the PoC inlines `g_rid` and `a_t` in
   every expansion.

## Language-change candidates

None is needed for the proof of concept. Each is an OPEN QUESTION for
the user; none is assumed approved.

### L1. A macro's expansion cannot name its own library (ask M11 / X14). OPEN QUESTION

Cannot be written: an expansion calling a helper of the `.xtl` beside
the `.xtlm`, because the macro does not know the importer's alias.

```
$ cat p1/Eq.xtlm    # m:m_< := { @ b -> "m:t_wo " c_at b }   (Eq.xtl: l:t_wo)
$ xetal run p1/t8.xtl           # "e:" u_se< "Eq"; @ e:m_< "5"
error[unknown-namespace]: no library is imported as m: here; add "m:" u_se< "Library" at 16..27
$ xetal run p1/t9.xtl           # imported as z:, macro writes e:t_wo
error[unknown-namespace]: no library is imported as e: here; ...
```

Smallest change: the macro phase rewrites `l:` in a macro's returned
text to the alias the calling file gave the library (MC5 already
rewrites `l:` in a library; this extends it to expansions), so
`"l:t_wo " c_at b` works under any alias. Alternatives: a hook
`[]A_LIAS @` (the caller's alias as text; more general, more
ceremony); inline helpers in each expansion (works today, the PoC
does it, expansions are long); pass the alias as an argument (leaks
plumbing into every call); a fixed alias convention (breaks MC3's
"the importer chooses"). Recommendation: inline for the PoC; decide
M11 on its own merits, preferring the `l:` rewrite, because it would
make every expansion shorter and readable in `xetal expand`.

### L2. Hygiene for short binders named by the call. OPEN QUESTION

Cannot be written reliably: a macro binding the call's one- or
two-letter names (`c r s`) around the call's code (C19's probe:
`error[undefined-name]: c is not defined`). Copying the whole binding
text verbatim (three bytes or more) works but is fragile:

```
$ xetal expand p1/t6.xtl        # "c:=0,r:=0,s:=0" e:l_et< "c + r + s"
{ @ -> c:=0 + 10; r:=0 + 10; s:=0 + 10; c + r + s } @
$ xetal run p1/t6.xtl
30
```

Smallest change: a hook `[]B_INDS "c r s"` (a dynamic `## binds:`) by
which a macro says, while it expands, that these names in its text
are the call's. Alternatives: substitution (the PoC; no binders, no
change); longer index names in the DSL; changing the byte-run rule
(would undo the documented `"t"` case of MC30). Recommendation: no
change now; the substitution lowering is also the clearer expansion.
Revisit only if a consumer needs real binders (an iteration consumer,
R 254).

### L3. Pointwise multi-axis indexing (choose indexing). OPEN QUESTION

Cannot be written: `X[i;j]`-style pointwise selection at several
axes; a tuple of index arrays is a type error, and the ravel workaround
wraps silently (Gap 1):

```
$ xetal eval -e 'X := 2 3 r_eshape r_ange 6; (2, 3) s_elect X'
error[type-mismatch]: expected Int, found (a, b) at 28..44
$ xetal eval -e 'X := 2 3 r_eshape r_ange 6; (1 + 3 + 3 * 0) s_elect r_avel X'
4
```

Smallest change: a built-in taking an index array whose leading axis
lists the coordinates (APL's choose indexing, BQN's pick on lists),
`Int -> a -> a`, with `error[index]` per axis; the name to choose with
the user (`a_t` is wish-list's amend). Alternatives: a library gather
with per-axis bounds checks (works today, one extra pass; the plan's
saga uses it); laminate/mix to stack index arrays (undecided, G15).
Recommendation: library first; decide the built-in together with
amend (ask G3), since amend at several axes needs the same index form.

### L4. Multi-line macro arguments (raw strings). OPEN QUESTION

Cannot be written: the three regions on three lines, as the research
pictures them (R 89-97, 644-648):

```
$ xetal run ml.xtl              # "1 < 2" x:w_hen< "p_rint!<newline> 7"
error[bad-string]: unterminated string (strings are single-line; use \n) at 37..45
$ xetal eval -e 'r"abc"'
error[bad-string]: a name touching `"` is reserved (raw strings) at 1..2
```

Smallest change: Rust-style raw strings `r"..."` that may span lines,
already a planned enhancement (lang-choices section 13, section 15).
Alternatives: one line with `|` separators (works today); `\n` escapes.
Recommendation: not needed for the PoC; bring raw strings forward only
if the user wants the stacked source layout.

### L5. Punctuation delimiters or punctuation macro names. OPEN QUESTION

Cannot be written: `|~ upper | center | lower _|`, `e:|<`, `math:\|*<`
(C17's lexer probes: `bad-lazy`, `unexpected-char`, `bad-namespace`).
Smallest change: new lexer tokens plus a parse rule (or letting a
macro name be punctuation). Alternatives: named macros (work today);
punctuation inside the string argument (works today). Recommendation:
decline for now, as the plan itself defers it: `|` is or and `~` is
lazy, so "one key, one job" argues against reuse; reconsider after the
PoC shows which layout readers want.

### L6. Macro arguments as typed values or parsed expressions. OPEN QUESTION

Cannot be written: `e:q_< (C - 1, Kh - 1)` (P line 22's typed tuple
arguments):

```
$ xetal run p1/t2.xtl
error[bad-macro-call]: e:q_< takes a string, or @ for none, on each side: "left" e:q_< "right" at 64..69
```

Smallest change: none small; it would mean macros over Core or a
quotation form, a redesign of MC10. Alternatives: strings plus X_eTaL
array code (works, the PoC). Recommendation: decline; the PoC shows
text is enough, and it keeps "the evaluator never sees surface
syntax" (rule 5) simple.

### L7. Drawing a spatial call as math in the renderers. OPEN QUESTION

Cannot be done: `xetal render`, the editor and the notebook draw the
call as a string literal; the LaTeX exists only as a run-time value.
Smallest change: a renderer rule keyed to a library's declaration
(for example a `## render: latex` doc tag naming a macro whose `t_ex`
form the renderer may call). Alternatives: show the LaTeX the program
prints through KaTeX in a literate document (works today).
Recommendation: not now; literate document first.

## Risks and open questions

- Where the library lives: `lib/Spatial.xtl(m)` makes it a standard
  library built into the binary (a decision of its own), `demos/`
  keeps it a demo, X_eTaL-libraries or X_eTaL-ML are other agents'
  repositories. OPEN QUESTION; the saga proposes a demo directory
  here, promotion decided later.
- Region syntax: `|` and `,` inside one string collide with "or" and
  tuples in a summand; labeled regions or another separator may read
  better. OPEN QUESTION (saga step 1).
- The left argument: `Y[o,h,w] | O Ho Wo` (explicit extents) versus
  inferring extents from bounds. OPEN QUESTION.
- The operator: summation fixed by the macro name (`e:s_um<`,
  `e:p_rod<`) versus written in the center (`'+`). The research's
  "the macro supplies the semantics" favors the name. OPEN QUESTION.
- Precedence: math readers will write `2*r+h` meaning `(2*r)+h`.
  Translate or document. OPEN QUESTION.
- Float summation order: the generated order (innermost sum first) is
  fixed and documented; compare floats with `e_q~` or a tolerance.
- Size: materialized grids are O(product of all extents); keep demo
  sizes small, consider an `i_nner` lowering for contractions later.
- Silent wraparound in the gather (Gap 1) is the one correctness risk;
  the saga makes the bounds check a test-first requirement.
- Hygiene's byte-run rule (L2) can also misread a user's own one-letter
  name next to a macro binder; the substitution lowering avoids
  binders named like the call's.
- M11 (L1) is already an open ask; deciding it changes how long every
  expansion is, so decide before goldens of expansions are pinned, or
  accept a rebase later.
- The plan's own expected values were wrong (C11): every number in
  the docs must come from a run, as the repository's rules already
  require.
