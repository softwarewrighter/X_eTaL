# Speed

How fast X_eTaL runs, measured with the benchmarks in `bench/` and three
demos. `just bench` (or `scripts/bench.sh [RUNS]`) builds the release
binary, runs each program three times and prints the best wall time as
a table. Timings depend on the machine, so compare runs made on one
machine; the benchmarks are not part of the gate.

## The benchmarks

| Program | What it times |
| ------- | ------------- |
| `bench/int-add.xtl` | three elementwise operations on 1M Ints |
| `bench/float-mul.xtl` | three elementwise operations on 1M Floats |
| `bench/reduce.xtl` | five reductions with a primitive operand, 1M Ints each |
| `bench/scan.xtl` | a scan with a primitive operand, 1M Ints |
| `bench/rotate.xtl` | three rotations of a 1000 by 1000 matrix |
| `bench/table.xtl` | an outer product of 1000 by 1000, reduced |
| `bench/life.xtl` | Conway's Life on a 200 by 200 torus, 20 generations |
| `bench/each.xtl` | each with a lambda on 300K Ints |
| `bench/table-lambda.xtl` | an outer product with a lambda, 600 by 600, reduced |
| `bench/table-right.xtl` | spreading with `t_able` and `'r_ight`, 512 by 512, four times |
| `bench/inner.xtl` | a matrix product with `i_nner`, 64 by 512 by 64, four times |
| `bench/matmul.xtl` | a small matrix product, 16 by 16, 300 times through `p_ower` |
| `bench/transpose.xtl` | five transposes of a 1500 by 1500 matrix |
| `bench/rosetta-update.xtl` | the Rosetta stone's update phase: 3000 ticks through `Comparison` |
| `bench/rosetta-scene.xtl` | the stone's scene phase: 300 frames of geometry, both halves turned and ordered |
| `bench/rosetta-svg.xtl` | the stone's SVG phase: 100 frames of the twelve panels as markup through `Svg` |
| `demos/classics/mandelbrot.xtl` | the Mandelbrot zoom (Float planes, many steps) |
| `demos/classics/mastermind.xtl` | Mastermind's scoring against all 1296 codes |
| `demos/tttml-train.xtl` | TTTML learning tic-tac-toe by playing itself |

Each benchmark program prints one small value (a count, a sum, a
shape), so printing does not count.

## Regression check

`just bench-check` times every program in `bench/` (the release
build, best of 5) and compares it with this machine's baseline,
`bench/baseline/HOST.tsv` (HOST is `hostname -s`). A program more than
15% slower than its baseline, and slower by more than 15 ms, fails
the check; it prints each program's baseline, time now and change.
`just bench-bless` records the times now as the baseline: run it on a
quiet machine (other builds running make the times swing by a third),
and blessing a slowdown needs the user's approval, said in the commit
message. `BENCH_LIMIT` and `BENCH_FLOOR` change the percentage and the
floor. The check is part of the release checklist; the gate keeps the
deterministic guard instead (`components/step/crates/xetal-step/tests/cost.rs`:
transitions and allocations per operand call, which do not depend on
the machine).

## Baseline

Release build, best of 3, in the cloud sandbox (2 cores, Intel Xeon
at 2.80 GHz), on 2026-10-02:

| Program | Best of 3 (s) |
| ------- | ------------- |
| `bench/float-mul.xtl` | 0.252 |
| `bench/int-add.xtl` | 0.198 |
| `bench/life.xtl` | 1.658 |
| `bench/reduce.xtl` | 1.628 |
| `bench/rotate.xtl` | 0.194 |
| `bench/scan.xtl` | 0.463 |
| `bench/table.xtl` | 0.299 |
| `demos/classics/mandelbrot.xtl` | 7.665 |
| `demos/classics/mastermind.xtl` | 0.786 |
| `demos/tttml-train.xtl` | 14.122 |

## The Rosetta stone's frame

The stone (`demos/rosetta/`, docs/rosetta.md) draws a frame per tick:
`update` (the state machine), `scene` (the geometry) and the SVG text
of twelve panels. Its budget is 30 frames a second, 33 ms, in the
browser's worker. Measured with the release build in the cloud
sandbox (2 cores) on 2026-10-06, the three phases from the benchmarks
above and the whole frame from the program itself under 300 scripted
ticks (`xetal run --events`):

| Phase | Per frame, before | Per frame, after |
| ----- | ----------------- | ---------------- |
| update (`cm:t_ick`) | 0.2 ms | 0.2 ms |
| scene (`st:r_ing` twice, `st:p_ainting`) | 0.5 ms | 0.5 ms |
| SVG text (twelve `st:p_anel`) | 34 ms | 9 ms |
| the whole frame, from the program | 37 ms | 16 ms |

The cost was all in `Svg`: `v:e_scape` ran a lambda over every
character of every attribute value and text (about 8 us a character
through the machine, 180 us for one font-family value) and joined the
pieces with a boxed reduce, and a frame escapes some hundreds of
texts. "After" is one change in X_eTaL, not Rust: text with none of
`&<>"` (found with one primitive `m_ember?`) is returned as it is, and
only text with a special character takes the slow path. The frame
meets its budget on the CLI with headroom of two; the worker's share
in the browser is for the page's own measurement (the browser's
performance panel, recording a few seconds of the stone turning), as
wasm32 is not built in this sandbox. What is left (about 6 ms of the
16 is the faces: `[]V_IEW`, the guessed classes, the escapes that do
have specials, `j_oin`) is the Saga 30 case: a lambda operand costs
about 8 us a call, and a reduce with a lambda over boxed texts is
quadratic in copies. Nothing moved into Rust.

## Where the time goes

Profiled with `perf record -g` on the release build:

- **Reduce and scan** (`bench/reduce.xtl`): about 330 ns per item. The
  operand is applied through the evaluator's `Caller` once per pair
  (`Caller::call` and `prim` lead the profile), with an allocation and
  a free for each intermediate value.
- **Elementwise arithmetic** (`bench/int-add.xtl`): `xetal_prim::call::
  scalar2` runs once per item on `Value` enums, collected through
  iterator adapters into a new vector; about 30 to 60 ns per item per
  operation.
- **Copies of whole arrays**: `Vec::clone` leads the Life and TTTML
  profiles (19.6% and 15.1%): arrays are copied where they could be
  shared or moved.
- **Axis moves**: `move_axis` (9.6% of Life) copies the array for
  every subscripted function.

## The higher-order regression, fixed

Making the higher-order built-ins steps of the machine (so a program
can stop inside them and wait for input) had made `t_able` about 2.7
times and `i_nner` about 1.5 times slower than before, as X_eTaL-demos
measured. Two changes undid that and more:

- a first-order built-in operand (`'*`, `'+`, `'r_ight`) is called at
  once, not through the machine, since it runs no code of the user's;
- an operand that is a function of the user's is driven by one small
  state machine for the whole built-in, with nothing allocated per
  element.

Per operand call, counted by the cost guard
(`components/step/crates/xetal-step/tests/cost.rs`), so the same on
every machine:

| Case | Transitions before | after | Allocations before | after |
| ---- | ------------------ | ----- | ------------------ | ----- |
| `t_able` with a built-in | 1.05 | 0 | 6.5 | 0 |
| reduce with a built-in | 2.1 | 0 | 7.1 | 1 |
| `i_nner` with built-ins | 3.9 | 0 | 16.4 | 0.25 |
| `t_able` with a lambda | 10.2 | 10.2 | 11.6 | 7.2 |
| `e_ach` with a lambda | 10.1 | 10.0 | 11.1 | 7.0 |

`bench/inner.xtl` (a 64 by 512 by 64 matrix product, four times) went
from 8.3 s to 0.81 s on the development machine (an Apple M-series
laptop). X_eTaL-demos, built against the fixed version, measured its
own benchmarks against its baseline from before the regression:
`t_able` 82% faster, `i_nner` 86%, its n-body demo 52%, its image
pipeline 55%, Langton's ant 41%, and nothing slower.

The benchmark times now, the baseline `just bench-check` compares
with (`bench/baseline/max.tsv`, the development machine, best of 5):

| Program | ms |
| ------- | -- |
| `bench/int-add.xtl` | 71 |
| `bench/float-mul.xtl` | 94 |
| `bench/reduce.xtl` | 488 |
| `bench/scan.xtl` | 377 |
| `bench/each.xtl` | 154 |
| `bench/table.xtl` | 95 |
| `bench/table-lambda.xtl` | 186 |
| `bench/table-right.xtl` | 62 |
| `bench/inner.xtl` | 811 |
| `bench/matmul.xtl` | 132 |
| `bench/rotate.xtl` | 68 |
| `bench/transpose.xtl` | 122 |
| `bench/life.xtl` | 546 |

## The gate's own speed

The gate is fast by default (`scripts/gate.sh`; `--full` runs
everything): it checks the components a change touches, tests the
ones that depend on them and skips the rest (`scripts/affected.py`).
What it would have run for some recent changes, of 25 components:

| Change | Checked | Tested | Skipped |
| ------ | ------- | ------ | ------- |
| a document only | 0 | 0 | 25 |
| a demo program (Mastermind) | 0 | 1 | 24 |
| the live demo's engine (macro, web) | 2 | 7 | 16 |
| the higher-order kernels (hof, step) | 2 | 14 | 9 |
| a new built-in (base, system) | 2 | 21 | 2 |

A component that merely depends on a change is only compiled (its
library code, `cargo check`); the spec cases and the goldens, which
always run, cover its behavior end to end, and the full gate compiles
and runs every test. With nothing changed the fast gate takes about
two minutes; a change to `base`, which everything depends on, about
eight (the probe of 2026-10-05: 494 s to check `base`, compile the
other 24 components, run the spec cases and the browser build, before
the goldens and the document checks). The full gate took 20 minutes
after such a merge, 40 on a loaded machine; each step of 5 seconds or
more prints its time, and the total.

Pages are built in parts (`scripts/build-pages.sh`), each only when
its inputs changed: the live demo, the literate HTML, the doc site,
the poster and the LaTeX gallery; `pages/INPUTS` records a hash per
part, `just pages --all` rebuilds everything. A literate document
alone rebuilds the literate HTML and the gallery; a library rebuilds
the live demo, the literate HTML, the doc site and the gallery; the
poster's template only the poster.
