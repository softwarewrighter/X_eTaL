errors lane: Saga 21 in docs/plan.md, errors of one's own (ask X3), as decided with the user 2026-10-04 (lang-choices ER1-ER4). One PR per step from the latest origin/main; TDD; the gate before every commit.

Steps
1. signal: []S_IGNAL; uncaught, a non-zero exit status; []P_ANIC as a signal.
2. trap: the typed trap built-in (protected body, codes, Error value, recover / retry / halt, cleanup); names with the user.
3. continue: the typed continue for errors raised resumably (form designed with the user).
4. try-macros: t_ry< c_atch< f_inally< r_ecover< r_etry< h_alt< c_ontinue< in System.xtlm.
5. errors-retrofit: Check's workaround and others rewritten.
6. errors-release: docs, reference, literate section, pages; the lane archived.