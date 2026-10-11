adt lane: Saga 29 in docs/plan.md, algebraic data on Saga 39's tuples:
records as named tuples, sum types and enums, matching. Asked for by
the user (2026-10-03: no records or sums; choices spelled as strings
are a code smell in a typed language), with X_eTaL-demos' game states
(D7) and the Rosetta stone's (axes, pointer, clock) state (D134) as the
test cases. The terminal's Color and Key (QD6) become ordinary
declarations. The design is docs/adt.md; the decisions are made with
the user at step 1 and recorded in docs/lang-choices.md before anything
is built.

Rules for every step: one PR per step from the latest origin/main,
branch pr/adt-<slug>, never rewritten once pushed; TDD (a failing spec
case or test first); the language rules of CLAUDE.md (no heuristic
parses, class from tokens only, surface desugars to Core, every sugar
has a normalization test, every grammar rule a rejection test,
parse(fmt(parse(s))) == parse(s), no panics); the gate before every
commit (`just gate`; `just gate --affected` before complete);
docs/design.md register and CHANGES in the same commit as the behavior.

Steps (refined after step 1)
1. adt-decisions: with the user, the questions of docs/adt.md.
2. adt-records: record literal, type, access by name, update.
3. adt-sums: declarations, constructors, evaluation, printing.
4. adt-matching: patterns on constructors, exhaustiveness.
5. adt-library: Maybe and Result; Color and Key as declarations.
6. adt-tools: renderers, live demo, Emacs, poster, reference, xetal doc.
7. adt-retrofit: D7 game states and the Rosetta state named; siblings told.
8. adt-release: tour, literate document, register, CHANGES; lane archived.
