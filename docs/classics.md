# Classic APL programs in X_eTaL

The programs APL has been taught and shown off with for decades,
written as X_eTaL says them. Each is a commented notebook file in
`demos/classics/`: run one with `just show demos/classics/NAME.xtl`
to see every line beside its output, or `just run FILE` for the output
alone. Each is also in the live demo's Open list, and its output is
pinned by a golden (`reg/run-classics-NAME`), so the programs keep
working as the language grows.

Some programs also draw. `[]G_RID` turns an array into a picture (an
SVG document: a matrix as a grid of cells, a rank-3 array as frames
shown in turn) and `[]S_HOW` shows it: `just draw FILE` runs the
program and opens its first picture in the browser, and `xetal run
--draw DIR FILE` writes every picture to DIR as `NAME-1.svg`,
`NAME-2.svg`, and so on. Several of the programs are also walked
through, with their pictures, in the literate document
[classics.org](literate/classics.org).

| Program | Why it is a classic | Concepts | File |
| ------- | ------------------- | -------- | ---- |
| Pascal's triangle | The quintessential array construction: a whole row at a time, no loop over items | shift and add by rotate, function power, stacking rows with `c_at`, binomials by `t_able`, text layout with `t_ake`, Sierpinski's triangle modulo 2, both drawn with `[]G_RID` | [pascal.xtl](../demos/classics/pascal.xtl) |
| Sieve of Eratosthenes | The classic demonstration of eliminating explicit loops | remainders with `m_od`, masks, `w_here` and `s_elect` to keep what a prime does not divide, recursion that stops at the square root | [sieve.xtl](../demos/classics/sieve.xtl) |
| Primes by divisors | Very compact and recognizable: every question asked of a table at once | the remainder table (`'m_od t_able`), counting divisors by reduce along axis 2, perfect numbers | [primes.xtl](../demos/classics/primes.xtl) |
| Euclid's GCD | Historically present in APL examples since APL\\360 | a guard and a recursion, a dyadic function as the operand of reduce and each, the least common multiple | [gcd.xtl](../demos/classics/gcd.xtl) |
| Fibonacci | A simple recurrence: the contrast between scalar and array thinking | the recursive definition run by each, a vector grown from its last two items, powers of a matrix by inner product | [fibonacci.xtl](../demos/classics/fibonacci.xtl) |
| Factorial and combinations | Tiny but mathematically recognizable | factorial as a product reduction, combinations and permutations, e as a sum of 1/k! | [factorial.xtl](../demos/classics/factorial.xtl) |
| Collatz sequence | A small iterative problem, then asked of many starts at once | a predicate in a guard, a recursion building the sequence, steps for 1 to 30 by each, `i_ndexOf` the longest | [collatz.xtl](../demos/classics/collatz.xtl) |
| Tower of Hanoi | Where scalar, recursive thinking stays natural, even in APL, and where the array way is a surprise | the moves by recursion, pegs permuted by `s_elect`; every move at once from the trailing zeros of the move number (a remainder table), checked with `m_atch`; the state moved by `i_ndexOf` and a mask and drawn as frames with `[]G_RID`; three ways in [hanoi.org](literate/hanoi.org) | [hanoi.xtl](../demos/classics/hanoi.xtl) |
| Quicksort | The famous recursive APL definition | partitions as masks (`v < p`), selection by `w_here` and `s_elect`, recursion on each side; checked against `s_ort`, with `g_rade` beside it | [quicksort.xtl](../demos/classics/quicksort.xtl) |
| Conway's Life | The best-known modern APL demo, in one line | rotations over two axes, reduce over two axes, Boolean arithmetic | [life.xtl](../demos/life.xtl) |
| Turtle graphics | Logo's turtle, without the turtle: the classic fractals as running sums | a walk as a vector of turns, positions by scans of `c_os` and `s_in` (the Turtle library), Koch's snowflake and Sierpinski's arrowhead by recursion, drawing a curve as it goes by selecting a growing prefix with `s_elect_2` | [turtle.xtl](../demos/classics/turtle.xtl), [lib/Turtle.xtl](../lib/Turtle.xtl) |
| Mandelbrot set | Small but visually rewarding: every point of the plane iterated at once | z as two Float arrays (real and imaginary), the state as planes of a rank-3 array, iteration by `p_ower`, escaped points frozen by a mask; a still, a zoom and a fly-over drawn as images | [mandelbrot.xtl](../demos/classics/mandelbrot.xtl) |
| Life, drawn | Life is best watched | generations stacked as frames (rank 3), animated with `[]G_RID`; a torus by rotation, a box by a mask of dead border cells | [life-drawn.xtl](../demos/classics/life-drawn.xtl) |
| Tic-tac-toe | A complete application rather than an expression puzzle: a machine that learns to play | boards as vectors, symmetry by indexing, inner product over lines, files and the keyboard | [tttml.xtl](../demos/tttml.xtl), [lib/TTTML.xtl](../lib/TTTML.xtl) |
