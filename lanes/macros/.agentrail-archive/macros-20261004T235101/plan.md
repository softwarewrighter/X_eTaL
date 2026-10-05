# macros lane: Saga 19, macros (system macros, `.xtlm` macro libraries)

A lane (branch per PR, from main) whose saga lives in lanes/macros/.agentrail;
run agentrail with `--saga lanes/macros`.

Goal: the macro core before the launch (asks X1 and X2 of
X_eTaL-libraries; research3.txt's P0 items): system macros beside
`u_se<` (`i_f<`, `u_nless<`, `e_ach<`), user macro libraries (`.xtlm`,
MC10-MC12) working end to end, and `xetal expand FILE` showing the
program after expansion. Long prefixes (MC13), Combinators.xtlm, the
retrofit and the release come after (reordered at the user's request,
2026-10-03).

Rules: TDD (spec cases with an EXPAND section pin expansions; crate
tests pin the rules and rejections), the full gate, one PR per step
from the newest main; design register numbers D60 and up; new
lang-choices rows from MC14, marked "proposed" until confirmed with
the user; pages/ is rebuilt by the merger.

Steps
1. system-macros: `i_f<`, `u_nless<`, `e_ach<` built into the macro
   phase (run-time guards, a family of statements), with `xetal expand`
   and the spec EXPAND section; expansions keep spans pointing into
   the macro's arguments.
2. lookup-xtlm: MC11, `.xtl` and `.xtlm` found together.
3. macro-calls: MC10, MC12: `m:name<` exports and `alias:name<` calls.
4. macro-engine: running a `.xtlm` macro at compile time, depth limit.
5. expand-tool: `xetal expand` for user macros, where a macro is defined.
6. macro-example: a small `.xtlm` (u_nless< as a user macro) with a golden.
7. long-prefixes: MC13.
8. macros-combinators: Combinators.xtlm.
9. user-macros: userlibs/ example; the live demo carries `.xtlm`.
10. retrofit-macros: older programs rewritten where macros read better.
11. macros-release: docs, register, retrospective; the lane archived.
