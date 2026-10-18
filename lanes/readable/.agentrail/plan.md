readable lane: Saga 27 in docs/plan.md, readable type errors (the user, 2026-10-03), with ask E5 (an error in a library also names the program line that called it). Main-saga steps 123 and 126-130 done here, in a cloud sandbox without agentrail (the files were written by hand; lanes/readable/HANDOFF.md says what to record locally). One PR from branch feat/errors-fceu5f; it rebases many goldens and spec cases, so it merges last.

Steps
1. located: every diagnostic with FILE:LINE:COLUMN and the line underlined (CLI, REPL, editor, live demo); goldens and spec cases rebased on purpose.
2. explained: a type mismatch says where each side's type came from (a binding, a literal, a built-in's signature, a lambda's parameter).
3. hints: f_loat for Int against Float, e_nclose / d_isclose for a Box, arity, a function where a value is expected, a ! variable assigned without !.
4. type-errors-doc: docs/literate/type-errors.org, each mistake, its error as recorded, the fix; goldens.
5. library-calls (ask E5): an error raised inside a library also names the program line that called into it.
6. errors-release: README, reference (error codes), design register, CHANGES, pages; the lane archived.
