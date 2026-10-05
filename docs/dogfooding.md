# Dogfooding: what the demos asked of the language

The classic programs, the APL ports and the other demos are forcing
functions. A demo is written as X_eTaL should say it; when it cannot
be written cleanly, that names a missing feature or a bug. A feature
is decided with the user, recorded in `docs/lang-choices.md` and the
design register (`docs/design.md`), and added test-first; it is never
worked around in the demo. This page tracks what the demos have added,
what they are waiting for, and the friction they ran into that the
language keeps on purpose for now.

Add a row whenever a demo changes the language, the tools or the
plan, and move a row from "Planned" to "Added" when it lands.

## Added because of a demo

| Feature or fix | Kind | Asked for by | Recorded in |
| -------------- | ---- | ------------ | ----------- |
| Rotate and reduce over several axes at once (`-1 0 1 o_-_12`, `'+ r_/_12`) | feature | Conway's Life in one line, the acceptance test | lang-choices A4, R1; design D26, D27 |
| Elaborator: a typed built-in under an axis subscript is wrapped whole, so the axis rule still sees reduce | fix | Life inside a polymorphic function | design section 7 |
| Files, the keyboard and numbers as text (`[]N_PUT`, `[]N_GET`, `[]R_EAD`, `f_ormat`, `n_umbers`), and system names (quads) to hold them | feature | TTTML, saving its model and playing a human | lang-choices QD1-QD4; design D30, D32 |
| Notebooks run in one pass and stream their output (a training program had rerun its training for every later line, and showed nothing until the end) | fix | TTTML as a notebook | plan.md, Saga 9 retrospective |
| Streaming output in the live demo: programs run in a worker, output streams, with a Stop button | feature | `tttml-train` showing its progress in the browser | main's saga, streaming-output |
| A hint for a name used without its underline (`s:mean` suggests `s:m_ean`) | fix | the user, in the live demo | xetal-macro tests |
| `m_atch`, whole-array match | feature | the user's Dyalog examples | lang-choices B7; design D34 |
| A closed stdout ends a run quietly (exit 141, nothing on stderr), instead of a panic | fix | Pascal's triangle piped into `head` | golden cli-pipe-closed; architecture.md |
| Trigonometry: `s_in`, `c_os`, `a_tan`, `p_i @` | feature | turtle graphics | lang-choices B13; design D39 |
| Pictures: `[]G_RID` (grids, rank 3 as animated frames), `[]S_HOW`, `xetal run --draw DIR`, `just draw` | feature | watching Life, and the classics that are best seen | lang-choices QD5; design D38, 8.1h |
| `[]P_ATH` (points as a path) and the Turtle library, written in X_eTaL | feature | Koch's snowflake, Sierpinski's arrowhead | QD5; design 8.1h |
| Large grids drawn as one image a frame, not a rectangle a cell | feature | the Mandelbrot zoom and fly-over | design 8.1h (RASTER_CELLS, confirmed by the user) |
| The Draw pane in the live demo | feature | the drawing classics in the browser | main's saga, draw-pane |
| A session shows each picture once (a replayed line had shown its picture again), and a `--context` run shows none | fix | literate walkthroughs with pictures | xetal-store and xetal-repl tests |
| `ob-xetal` saves a block's picture with `:results file :file PATH`; the literate check compares pictures | tooling | the literate classics walkthrough | design 8.1g |
| `scripts/literate.sh --check` uses a mktemp both GNU and BSD accept | fix | running the literate check on Linux | scripts/literate.sh |
| `--ascii`: nested arrays drawn in plain ASCII, APL2 style | tooling | the ASCII-only reference page, which could not show a nested value | lang-choices B16; design D43 |
| `r_eplicate`, APL's replicate and compress: counts or a mask on the left, over major cells, any axis by subscript | feature | Roman numerals and run-length decoding (and every "keep where" written as `(w_here m) s_elect v`) | lang-choices B11; design D35 |
| `e_ncode` and `d_ecode`, APL's encode and decode, in their own component (`radix`) | feature | truth tables, base conversion, Hanoi's moves from the bits of k (written as a remainder table), Mastermind's 1296 codes (written with `d_iv` and `m_od` by place values) | lang-choices B12; design D36 |
| `c_at_k`, catenate along any axis: both arguments' axis k moves (it had been refused, since the axis rule moves only the right argument) | feature | the swimming ducks, ducks and waves joined frame by frame (written as a recursion before) | lang-choices B15; design D40 |
| Nested arrays with static depth: `Box a`, strands of strings, `e_nclose`, `d_isclose`, `p_artition`, `m_ap`, and APL2 DISPLAY printing | feature | word frequency, N-Queens, ragged Pascal, the APL subset interpreter; LeetCode 1805 exactly (digit runs as strings) | lang-choices A7, B14, B16; design D37, D41, D42 |
| Transpose: `o_\` reverses the axes, `p t_ranspose A` permutes them (axis i to p[i]), `o_\_jk` swaps two | feature | X_eTaL-demos (attention, PCA), Mastermind's codes as rows, `[]P_ATH` points written as rows, ADVANCEDEX | lang-choices B17; design D48 |

## Planned, because a demo needs it

| Feature | Needed by | Status |
| ------- | --------- | ------ |
| Matrix inverse and division (APL's domino) | ADVANCEDEX's INV and INVP, regression in STATS | to decide with the user, when the ADVANCEDEX ports reach it |
| Execute (`[]V_ALUE`, read and evaluate) | the APL subset interpreter, the X_eTaL course's answer checking (COURSE and DRILL translated to teach X_eTaL, Saga 28) | reserved (lang-choices QD3); to decide |
| Complex numbers | Mandelbrot (z is two Float arrays today) | deferred (lang-choices section 15) |
| Evaluator speed: a primitive operand applied as a vector kernel | TTTML's training time; Mandelbrot is kept small (60 by 90 frames) for it; Mastermind's player over all 1296 secrets takes 25 s in a release build, so the demo plays 35 of them | measuring: `just bench`, baseline and profile in docs/speed.md (speed lane) |

## Friction found, kept for now

What the demos ran into that the language does on purpose today. None
is a bug; each could become a decision with the user if it keeps
coming up.

| Friction | Seen in | Written instead |
| -------- | ------- | --------------- |
| A strand holds only number literals: `1 w r_eshape x` is two values side by side | automaton, shortest paths | `(1 c_at w) r_eshape x`; a literal 999 for "no edge" |
| A one-character string is a 1-item vector, not a character | histogram | `f_irst " "` |
| Haskell-style numbers: Int and Float do not mix, and a Bool table does not multiply Ints | Mandelbrot, sequences, matmul | `f_loat`; an identity as `3 3 r_eshape 1 0 0 0` |
| A call takes one value on each side, so a curried function of four arguments is called through `(expr)_` | Hanoi, the curried version | the pegs as one vector |
| Overtaking an empty array is an error (there is no fill) | Hanoi, drawing the pegs | a 0 put in front before taking |
| A lambda that uses `_l` must use `_r` too | Mandelbrot | the tacks as operands, `'l_eft` and `'r_ight` |
| `u:` functions are defined only at the top level | Mandelbrot | a local function name (`s_tep := ...`) or a top-level definition |
| `m_od` reads in maths order (`a m_od n` is a mod n), the reverse of APL's residue | automaton | written as maths reads it |
| `e_ach` goes over items (scalars), not rows, so a function of a row cannot be applied to each row of a matrix | Mastermind, every secret | each row by its index: `'{ ... _r s_elect m } e_ach r_ange n`; `m_ap` (B14) will box each row's result, though a matrix still goes item by item: a rank operator is not planned |
| A guard is a statement, so a choice in the middle of an expression needs a function | Mastermind, showing "none" for no pegs | a small function (`u:p_egs`) whose first line is the guard |
| A comparison named at the top level is a Bool, and a Bool is not a number: `t c_at f` and `1 * f` fail for a named Bool `f`, though the same comparison written in place is an Int (T5) | truth tables, Gray code | the binding written `f := 0 + ...`, so it is named as an Int |
| Numbers in a string are a segmented computation (one Horner per run of digits), and there is no key or segmented reduce, so the runs are summed through a table of run numbers | LeetCode 1805 | `'+ r_/_2 ((r_ange k) '= t_able g) * ...`, k by n in size; `p_artition` with `m_ap` now says it directly for strings; a key operator for numbers is not planned |
| Numbers longer than an Int: LeetCode 1805 allows 1000 digits, and compares the digit strings without leading zeros | LeetCode 1805 | solved: the runs as strings by `p_artition`, leading zeros dropped by `m_ap`, compared by `u_nique` on boxes |
| No "merge" (APL's `@`, or `(mask) choose`) to replace some items: blanking the non-digits picks from `" " c_at s` by index | LeetCode 1805 | `(1 + m * r_ange t_ally s) s_elect " " c_at s` |
| No raze (enlist): the boxes of a nested vector cannot be joined into one array | the mini APL interpreter, reading a strand of number tokens | each token read by `n_umbers` on its own, through `e_ach` |
| `s_elect` takes no index from the end | the mini APL interpreter, the last token | `(t_ally t) s_elect t` |
| No table of functions to dispatch on | the mini APL interpreter, picking a function by its name | a chain of guards, one per name |
| A train's type is fixed by its elements, and a mismatch is reported where it is applied, over the whole call, without the train notes: the mean `['+ r_/ / t_ally]` takes Ints only (`t_ally` gives an Int, `/` one number type) | the trains document | `[[f_loat '+ r_/] / [f_loat t_ally]]` for Floats |

## Retrofit audit (for Saga 13a)

A survey of the demos, libraries, literate documents and help for code
written before a feature existed, and for features never shown where
an older form is. Each rewrite marked "checked" was run and gives the
same result. The milestone tours (`docs/tour-m*.md`) are snapshots and
stay as they are; later features get new pages instead.

### Workarounds a newer feature replaces

| Feature | Where | Today | With the feature |
| ------- | ----- | ----- | ---------------- |
| `r_eplicate` | classics gcd:20, primes:14,19, quicksort:9-11, sieve:12,19, sorting:26,30, histogram:23, pascal:37, rle:31, mastermind:50; lib/TTTML:88; idioms.md:36; live demo help.rs:119; classics.org (sieve, primes, quicksort, histogram) | `(w_here m) s_elect v` | `m r_eplicate v` (checked) |
| `r_eplicate` | mastermind-play:14 | stars and circles by two reshapes | `r r_eplicate "*o"` (checked) |
| `e_ncode` | automaton:9 (and classics.org:474) | `(rule d_iv 2 ^ o_ffsets 8) m_od 2` | `r_ev (8 r_eshape 2) e_ncode rule` (checked) |
| `d_ecode` | automaton:15 | `1 + (4 * ...) + (2 * row) + ...` | `1 + 2 d_ecode -1 0 1 o_- row` (checked) |
| `e_ncode` | hanoi:29 (and hanoi.org:224) | trailing zero bits by a remainder table | `1 + '+ r_/ '& s_\ r_ev 0 = (n r_eshape 2) e_ncode k` (checked); keep the old one beside it, it teaches |
| `d_ecode` | sequences:11 | Horner by a reduce | `x d_ecode r_ev c` (Int x only) |
| `d_ecode`, `o_\` | lib/TTTML:30 | base 3 by `'+ '* i_nner 6561 ... 1` | done: `3 d_ecode o_\ ...` (transpose lane) |
| `d_ecode`, `p_artition` | leetcode numbers-in-string:24-32 | Horner per run through a table | `'{ 10 d_ecode ... d_isclose _r } e_ach (...) p_artition s` (checked); keep the flat way as the comparison |
| `o_\` | hanoi:34 (and hanoi.org) | two tables put side by side | done: `o_\ (2 c_at t_ally from) r_eshape from c_at to` (transpose lane) |
| `c_at_2` | lib/TTTML:89-90, lib/Turtle:16-18 | columns joined by reshaping and rebuilding | `m c_at_2 ...` |
| `c_at`, lower rank | closure:40, hanoi:15, mandelbrot:39, bases:40, lib/TTTML:61,136 | a plane or row made by reshape before joining | join the lower-rank argument directly (checked) |
| nested strands | sorting:11-12 | names padded into a 4 by 5 matrix | `"Alice" "bob" "carol" "Dave"` (checked) |
| `m_ap` and boxed display | lib/TTTML:142-148 | boards laid side by side by index arithmetic | a boxed vector of boards (output changes to frames) |
| `m_atch` | magmas:21, life:22 | `'& r_/ a = b`, a table of 1s | `a m_atch b` |
| `i_d`, `l_eft` | lib/Maybe:19, birds-untyped:13 | `{ x -> x }` | `'i_d` |
| pictures | life.xtl, rotate.xtl | frames printed as text | `[]S_HOW []G_RID frames` (low priority) |

Done with transpose: Mastermind's codes as rows (`o_\ 1 + 6 6 6 6 e_ncode
o_ffsets 1296`). Still needed, so kept:
Mandelbrot's two Float planes (no complex numbers), `0 +` for a named
Bool, `(t_ally g) s_elect g` (no index from the end), `e_ach` over row
indices (no rank operator).

### Features not shown where an older form is

| Document | Gap |
| -------- | --- |
| docs/literate/tour.org | `c_at_2` (line 362) sits two blocks after `c_at` (326), out of step with demos/tour.xtl, which the tour mirrors; `m_atch`, `e_nclose`, `--ascii`, `f_ormat`, `n_umbers`, `[]N_PUT`, `[]N_GET`, `[]R_EAD` are not shown though the tour claims every feature; `r_eplicate` not beside `w_here v > 4` (959); trains lack a dyadic fork of plain functions, a long train, a train as an operand, the lambda beside its train, and an error with its notes |
| README.md | Status (152-172) lists trains as future work and names none of the newer array features; decoration and APL tables have no train or nested-array rows; Quick Start shows nothing newer; duck.org not in the document list |
| docs/idioms.md | no rows for replicate, encode/decode, catenate along an axis, enclose/disclose, partition, tacks, dyadic fork, DISPLAY (spec/integration/idioms.case pins it) |
| docs/literate/classics.org | none of truth tables, bases, Roman numerals, word frequency, N-Queens, ragged Pascal |
| docs/literate/birds.org, docs/birds.md | no birds-as-trains: Bluebird is atop, Blackbird a dyadic atop, Starling the hook `[i_d F G]`, Warbler `[i_d F i_d]` (all checked) |
| docs/literate/libraries.org:12, hello.org:62 | say four built-in libraries; Turtle makes five |
| live demo help.rs (94-110) | reference table lacks `c_at_2`, nested arrays, trains, trig, `[]G_RID` |
| docs/wish-list.md:20 | lists trains, nested arrays and `d_isplay` as planned |
| docs/reference/builtins.ref | no trains section; tacks shown without trains |
| syntax poster (scripts/poster) | no trains panel; nothing for `c_at_2`, nested arrays, `r_eplicate`, encode/decode, tacks; brackets render undecorated; `scripts/poster/build.sh` is a stale stub (poster.py builds it) |

### Trains, once Saga 15 lands

Done (trains lane, retrofit step): `lib/Stats.xtl` as trains
(`l:m_ean := [[f_loat '+ r_/] / [f_loat t_ally]]`, `d_eviations`,
`l:v_ariance := [l:m_ean [s_quare d_eviations]]`, `l:r_ange`);
`u:f_act := ['* r_/ r_ange]` (factorial, pascal); `'[t_ally
d_isclose] e_ach` (wordfreq, the reference) and `'['+ r_/ d_isclose]
e_ach` (pascal); `'[u:m_agic u:s_iamese] e_ach` (magic). The spec map
and partition cases keep their lambdas, which they test.
Best as side-by-side comparisons: the Float-safe mean
`[[f_loat '+ r_/] / [f_loat t_ally]]` (the tour's `['+ r_/ / t_ally]`
takes Ints only), argmax `[i_d i_ndexOf 'm_ax r_/]` (histogram,
collatz, TTTML), sort `[g_rade s_elect i_d]`, the nub sieve
`[[i_d i_ndexOf i_d] = [r_ange t_ally]]`, `c_ompose` and `c:B_` beside
atop, and `[l:m_ean [s_quare d_eviations]]` with its flat look-alike
(a fork) as the pitfall. Lambdas with a constant (`{ _r * 2 }`) and
named dyadic functions stay lambdas (TR1, TR4).
