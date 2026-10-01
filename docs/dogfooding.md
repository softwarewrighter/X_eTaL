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
| `r_eplicate`, APL's replicate and compress: counts or a mask on the left, over major cells, any axis by subscript | feature | Roman numerals and run-length decoding (and every "keep where" written as `(w_here m) s_elect v`) | lang-choices B11; design D35 |
| `e_ncode` and `d_ecode`, APL's encode and decode, in their own component (`radix`) | feature | truth tables, base conversion, Hanoi's moves from the bits of k (written as a remainder table), Mastermind's 1296 codes (written with `d_iv` and `m_od` by place values) | lang-choices B12; design D36 |
| `c_at_k`, catenate along any axis: both arguments' axis k moves (it had been refused, since the axis rule moves only the right argument) | feature | the swimming ducks, ducks and waves joined frame by frame (written as a recursion before) | lang-choices B15; design D40 |

## Planned, because a demo needs it

| Feature | Needed by | Status |
| ------- | --------- | ------ |
| Nested arrays, with static depth (`Box a`) and boxed printing | word frequency, N-Queens, ragged Pascal, the APL subset interpreter | decided (A7, B14, D37): string-literal strands, `m_ap` to box each result, `p_artition` with APL2's keys, strict `Box a`; classics lane, nested-core and nested-builtins |
| Transpose (`o_\`) | `[]P_ATH` taking n by 2 points, any program wanting columns | planned (plan.md, cross-cutting; confirmed by the user) |
| Matrix inverse and division (APL's domino) | ADVANCEDEX's INV and INVP, regression in STATS | to decide with the user, when the ADVANCEDEX ports reach it |
| Execute (`[]V_ALUE`, read and evaluate) | the APL subset interpreter, the COURSE and DRILL ports | reserved (lang-choices QD3); to decide |
| Complex numbers | Mandelbrot (z is two Float arrays today) | deferred (lang-choices section 15) |
| Evaluator speed: a primitive operand applied as a vector kernel | TTTML's training time; Mandelbrot is kept small (60 by 90 frames) for it; Mastermind's player over all 1296 secrets takes 25 s in a release build, so the demo plays 35 of them | planned (plan.md, cross-cutting) |

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
| Numbers in a string are a segmented computation (one Horner per run of digits), and there is no key or segmented reduce, so the runs are summed through a table of run numbers | LeetCode 1805 | `'+ r_/_2 ((r_ange k) '= t_able g) * ...`, k by n in size; APL's key operator or `p_artition` with `m_ap` (B14) would say it directly |
| Numbers longer than an Int: LeetCode 1805 allows 1000 digits, and compares the digit strings without leading zeros | LeetCode 1805 | Ints (to 18 digits) or `n_umbers` Floats; the exact answer needs the runs as strings, which waits for nested arrays (`p_artition`) |
| No "merge" (APL's `@`, or `(mask) choose`) to replace some items: blanking the non-digits picks from `" " c_at s` by index | LeetCode 1805 | `(1 + m * r_ange t_ally s) s_elect " " c_at s` |
