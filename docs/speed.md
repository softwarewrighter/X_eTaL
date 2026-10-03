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
| `demos/classics/mandelbrot.xtl` | the Mandelbrot zoom (Float planes, many steps) |
| `demos/classics/mastermind.xtl` | Mastermind's scoring against all 1296 codes |
| `demos/tttml-train.xtl` | TTTML learning tic-tac-toe by playing itself |

Each benchmark program prints one small value (a count, a sum, a
shape), so printing does not count.

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
