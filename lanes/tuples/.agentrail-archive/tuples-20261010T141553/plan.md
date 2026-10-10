tuples lane: Saga 39 in docs/plan.md, typed tuples with destructuring,
asked for by X_eTaL-ML (ask M13: train-live packs 99 weights and Adam's
two averages into one vector of 298 numbers because p_ower iterates a
single value) and X_eTaL-demos (ask D7: a game state of several arrays
for p_ower). The first part of Saga 29 (algebraic data): tuples here,
records, sums and matching stay in Saga 29 and build on these types.
The design is docs/tuples.md; the decisions are made with the user at
step 1 and recorded in docs/lang-choices.md before anything is built.

Rules for every step: one PR per step from the latest origin/main,
branch pr/tuples-<slug>, never rewritten once pushed; TDD (a failing
spec case or test first); the language rules of CLAUDE.md (no
heuristic parses, class from tokens only, surface desugars to Core,
every sugar has a normalization test, every grammar rule a rejection
test, parse(fmt(parse(s))) == parse(s), no panics); the gate before
every commit (`just gate`; `just gate --affected` before complete);
docs/design.md register and CHANGES in the same commit as the behavior.

Steps
1. tuples-decisions: with the user, the questions of docs/tuples.md.
2. tuples-values: syntax, Core, evaluation, printing, formatter.
3. tuples-types: product types in the checker; p_ower and friends.
4. tuples-destructure: patterns in bindings and lambda parameters.
5. tuples-in-arrays: tuples inside arrays and boxes, as decided.
6. tuples-tools: renderers, live demo, Emacs, poster, reference, xetal doc.
7. tuples-retrofit: programs that pack state rewritten (the Rosetta
   stone's Comparison state first); the siblings told (M13, D7).
8. tuples-release: README tour, literate document, register, CHANGES,
   pages; Saga 29 replanned on these types; lane archived.
