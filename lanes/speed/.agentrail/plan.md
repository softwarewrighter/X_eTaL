# speed lane: Saga 22, speed (vector kernels)

A lane (branch per PR, from main) whose saga lives in lanes/speed/.agentrail;
run agentrail with `--saga lanes/speed`.

Goal: ask D2 from X_eTaL-demos (whole-array arithmetic at about 28 to
50 ns per item per operation), and the evaluator-speed friction in
docs/dogfooding.md (TTTML training, Mastermind's player over all 1296
secrets, Mandelbrot kept small). No language change: every result,
golden and spec case stays the same; only time changes.

Rules: measure before and after every change; TDD for any new kernel
(property tests that the fast path equals the general path); the full
gate; one PR per step from the newest main.

Steps
1. speed-bench: checked-in benchmarks (bench/*.xtl and `just bench`):
   elementwise arithmetic on 1M Ints and Floats, rotate, reduce and
   scan with a primitive operand, outer product, a Life generation,
   Mandelbrot steps, TTTML training, Mastermind's scoring; a profile of
   where the time goes; docs/speed.md with the baseline numbers.
2. speed-elementwise: elementwise kernels on whole Int and Float
   arrays (and a scalar extended), measured against the baseline.
3. speed-operands: a primitive operand of reduce, scan, table and
   inner product applied as a vector kernel.
4. speed-retrofit: the demos' speed workarounds (Mastermind's 35
   secrets, Mandelbrot's size, TTTML's rounds) revisited.
5. speed-release: numbers before and after, plan.md retrospective,
   the lane archived.
