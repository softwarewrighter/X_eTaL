xasks lane: asks from X_eTaL-libraries (docs/xetal-asks.md there) fixed
in X_eTaL, each a small step and its own PR, beside the lanes other
agents work (main's saga, Saga 40 lint). Rules: one PR per step from
the latest origin/main, branch pr/xasks-<slug>; TDD; the gate (just gate
before each commit, just gate --affected before complete);
docs/design.md register and CHANGES with the behavior.

Steps
1. x20-power-lint: redundant-lambda never suggests quoting power
   (`'^` is an exponent, D-6), so `--fix` cannot write code that does
   not lex.
2. x19-hygiene: hygiene renames a binder but not a same-spelled
   reference copied from a taken-apart argument (X19); the analysis and
   the choice put to the user before anything changes.
