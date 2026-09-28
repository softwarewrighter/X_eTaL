# types-and-unit

Saga 3 of X_eTaL (see docs/plan.md). Milestone M2 from docs/PRD.md:
static types. `u:a_nswer := { @ -> 42 }; u:a_nswer @` works and
`u:a_nswer 42` is rejected by the type checker before anything runs.

Specification: docs/lang-choices.md (governs). Relevant decisions:
T1 (Bool type, implicit Bool -> Int in arithmetic, Int -> Bool only
from 1/0), T2, T3, D-4, D-10, T4 (no annotations; `::` reserved), L6
(niladic `{ @ -> ... }`), E1-E4 (laziness is not part of the type),
M1/M2 (immutability, `!` variables), MC/N rules for names. Where the
exact typing rule is not decided (e.g. how T1's implicit coercion is
typed), ask the user before implementing.

Rules for every step: strict TDD (failing test first, rejection tests
for every accepted rule), tests are the spec (spec/**/*.case with TYPE
sections, crate tests, proptest, reg-rs goldens), no panics on any
input, run scripts/gate.sh before committing, commit (including
.agentrail/ and Cargo.lock) before agentrail complete, push to
origin/main.

## Steps

1. nesting-limit -- a nesting-depth limit for parser, desugarer and
   printers with a clear error (no stack overflow on pathological
   input); proptest with deep nesting.
2. type-core -- xetal-types: types (Unit Bool Int Float Char,
   functions, type variables, Array<T> stub), unification with occurs
   check, the Num constraint, errors with spans.
3. infer -- Algorithm W over Core with let-polymorphism, letrec,
   late-bound module definitions (inferred as a group), guards,
   mutable variables, built-in signatures; the T1 typing rule decided
   with the user first.
4. type-cli -- `xetal type`, TYPE sections in the spec corpus, eval
   refuses ill-typed programs, goldens; the M2 demo.
5. m2-docs-release -- README tour for M2, docs sync, retrospective.
