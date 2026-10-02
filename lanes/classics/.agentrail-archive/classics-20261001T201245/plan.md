# classics lane: the classic APL programs in X_eTaL

A parallel lane (branch feat/classics) beside the web-playground saga
on main. Its saga lives in lanes/classics/.agentrail, so run agentrail
with `--saga lanes/classics`; the main saga in .agentrail is not
touched, and merging the lane is conflict-free.

Goal: the classic APL example programs, each written as X_eTaL should
say it, one demo per program in demos/classics/NAME.xtl (a notebook
file starting `#!/usr/bin/env xetal`, so the just smoke test runs it),
pinned by a reg-rs golden run-classics-NAME, listed in
components/web/crates/xetal-web/src/demos.rs so the live demo offers
it, and indexed in docs/classics.md (program, why it is a classic, the
concepts it shows, the file). Life and tic-tac-toe exist already and
are linked from the index.

Rules: the ports are forcing functions. A program that cannot be
written cleanly names a missing feature, which is added first,
test-first, with the user's decision recorded in docs/lang-choices.md
and docs/design.md (decisions register), never worked around.

Decided with the user (2026-09-30):
- Deliverable: demo + golden per program (not a library).
- Replicate: `r_eplicate`, counts on the left (B10), over major
  cells, a scalar count extends, a negative count is error[domain]:
  `1 0 2 r_eplicate "abc"` is "acc".
- Encode and decode: `e_ncode` / `d_ecode`, radix on the left,
  a scalar radix extends in d_ecode, a vector right argument to
  e_ncode gives one column per item as in APL:
  `2 2 2 e_ncode 5` is `1 0 1`, `2 d_ecode 1 0 1` is `5`.
- Nested arrays (A7): static depth. An enclosed item has type
  `Box a`; `"ab" "cde"` is `Box Char`; e_nclose : a -> Box a and
  d_isclose back. Nested arrays print boxed (APL2 DISPLAY style).

Steps
1. index: docs/classics.md, demos/classics/, the golden and web-list conventions; Pascal's triangle as the first program.
2. numbers: sieve, primes, gcd, fibonacci, factorial and combinations, collatz.
3. recursion: tower of hanoi, quicksort.
4. graphs: matrix multiplication, transitive closure, warshall, min-plus shortest paths.
5. sequences: polynomial evaluation, moving average, finite differences, 1-D cellular automaton.
6. data: histogram, duplicate removal, sorting and grade, run-length encoding.
7. puzzles: magic square, mastermind, mandelbrot (real and imaginary Float arrays).
8. replicate: the built-in, test-first.
9. encode-decode: the built-ins, test-first.
10. radix-programs: truth tables, base conversion, roman numerals, run-length decoding.
11. nested-design: the remaining A7 decisions with the user (strands of strings, e_ach with array results, partition), recorded.
12. nested-core: Box a in the checker, nested values, boxed printing.
13. nested-builtins: e_nclose, d_isclose, partition, each over boxes.
14. nested-programs: word frequency, N-Queens, Pascal's triangle ragged.
15. mini-apl: a small APL-subset interpreter in X_eTaL (tokenize, parse right to left, evaluate), as far as the language allows; gaps recorded.
16. classics-release: index complete, README link, retrospective, lane archived, merge.
