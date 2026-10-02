# trains lane: Saga 15 (M7), trains

A lane (branch per PR, from main) whose saga lives in lanes/trains/.agentrail;
run agentrail with `--saga lanes/trains`.

Goal: finish M7. Trains are already decided (lang-choices TR1-TR4) and
already parse and desugar (spec/syntax/train-*.case): `[F G H] x` is
`(F x) G (H x)`, `x [F G H] y` is `(x F y) G (x H y)`, `[F G] x` is
`F (G x)`, longer trains group from the right, a train holds functions
only, and nothing train-specific reaches the evaluator. What M7 still
lacks: proof that the desugaring is the law (property tests), errors
that point into an ill-typed train, and documentation.

Rules: TDD; the full gate; one PR per step from the newest main; ask the
user before resolving anything TR1-TR4 leaves open.

Steps
1. fork-law: property tests that every train equals its desugared
   expression (fork, atop, dyadic fork, long trains) over random arrays
   and a pool of functions; the cases that fail become bugs to fix.
2. train-errors: an ill-typed train names the element at fault and
   points at it (a symbol used monadically in a monadic train, an
   element whose result the next cannot take), test-first spec cases.
3. trains-docs: the reference and README describe trains; a literate
   document, trains beside their Combinators-library birds (Bluebird,
   Phoenix), checked equal, answering how trains differ from birds.
4. trains-release: plan.md retrospective, M7 marked, lane archived.
