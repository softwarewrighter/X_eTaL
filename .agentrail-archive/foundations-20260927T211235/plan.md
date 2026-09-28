# foundations

Saga 1 of X_eTaL (see docs/plan.md). Milestones M0 + M1 from docs/PRD.md.
Normative docs: docs/PRD.md, docs/design.md, docs/architecture.md,
docs/plan.md. docs/research*.txt are archival only.

Goal: raw keyboard source renders decorated and back losslessly (M0);
the scalar functional calculus evaluates (M1):
`1 + 2` -> 3, `square = { _r * _r }; square_ 7` -> 49,
`sub = { _l - _r }; 10 sub_ 3` -> 7.

Rules for every step: strict TDD (failing test first), tests are the
spec (spec/*.case + unit + proptest + reg-rs goldens in reg/), never
resolve ambiguity heuristically, evaluator consumes Core only, update
docs/design.md when a test pins or changes a rule, run
scripts/gate.sh before committing, commit before agentrail complete.

## Steps

1. workspace-scaffold -- cargo workspace with crates per
   docs/architecture.md (xetal-base, -lex, -render, -syntax, -core,
   -types, -array, -eval, -cli) as stubs; xetal-base holds LANG_NAME
   ("X_eTaL"). `xetal --version` prints it.
   Spec-case harness (xetal-cli/tests/spec.rs) parsing `== SECTION`
   files under spec/, with STATUS pending support (pending must fail;
   unexpected pass is an error) and XETAL_BLESS=1. Add the pending
   Life acceptance case spec/integration/life-blinker.case. reg/ dir
   with scripts/reg.sh (REG_RS_DATA_DIR=reg) and one baseline for
   `xetal --version`; .gitignore for target/ and reg/*.tdb*.
   scripts/gate.sh: fmt --check, clippy -D warnings, cargo test,
   reg-rs run, sw-markdown-checker.
2. lexer-tokens -- implement the design.md section 2 token set with
   spans: stems, `_` underline, axis subscripts, `_@`, `^` derivation
   superscripts, dotted namespaces, symbols, `_l`/`_r`, `@`, `;`,
   brackets, numbers with the negative-literal rule. Rejection tests
   for every accepted form (r__, r_0, _x, r^, r_2^r, 3-1...). `xetal
   lex`. Lexer never panics (proptest on random ASCII).
3. decorated-render -- raw <-> Unicode decorated presentation
   (U+0332 underline, subscript digits, superscript modifier letters,
   `@` subscript form chosen and documented). Lossless round-trip
   proptest over lexable sources. `xetal render` and `--raw`. reg-rs
   goldens. This is the M0 demo.
4. parser-surface-ast -- noun/function classes from tokens only,
   right-to-left long-right-scope application, monadic/dyadic,
   numeric strands, parens, lambdas, trains syntax (parse only),
   statements with `;`, bindings. ParseResult with alternatives;
   >1 => AmbiguousExpression listing parses. spec/ambiguity corpus
   (a f_ b g_ c, a b, `x = 3` at statement start, etc.). Decide and
   record D1 (binding token), D2 (newlines), D4 (sections).
   `xetal parse`.
5. canonical-fmt -- canonical fully parenthesized printer; `xetal
   fmt`; proptest parse(fmt(parse x)) == parse x over generated ASTs.
6. core-desugar -- Core IR (Lit, Unit, Var, Prim, Lam, App, Let) with
   NodeId + span; desugar lambdas (_l/_r -> curried Lam), dyadic ->
   curried App, `now_@` == `now_ @`, long/terse names -> same Prim.
   Normalization-equivalence tests. Decide D3. `xetal core`.
7. scalar-eval -- strict evaluator over Core: Int/Float/Bool scalars,
   `+ - * / =`, bindings, lambdas, closures, currying/partial
   application; runtime errors as Diagnostics (division by zero
   decided by test). `xetal eval -e`, `xetal run file.xtl`. M1 demos
   in demos/ and as reg-rs goldens.
8. m1-docs-release -- README tour for M0/M1 with commands that are
   all reg-rs baselines, docs/design.md decisions table updated with
   test references, saga retrospective in docs/plan.md.
