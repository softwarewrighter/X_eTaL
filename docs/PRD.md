# X_eTaL -- Product Requirements

X_eTaL -- the eXperimental
eXtensible Typed Array Language.

> LaTeX uses text to produce typography. X_eTaL uses typography to
> express computation.

Sources: `docs/research.txt` and `docs/research2.txt` (archival design
conversations, not normative). This document, `docs/design.md`,
`docs/architecture.md` and `docs/plan.md` are the normative planning
docs; the executable test suite is the normative language spec.

## 1. Problem

APL-family languages are extremely terse and expressive, but they
depend on a large non-ASCII glyph alphabet (for example rho, iota,
rotate, reduce, scan) that needs special keyboards, fonts and memorized
symbols. ASCII descendants (J, K) trade glyphs for digraphs that are
hard to read. None of them offer a regular way to move between a
readable long form and a terse form, and none are statically typed.

## 2. Vision

A terse, statically typed, functional array language whose source is
plain ASCII typed on a US keyboard, where **typographic decoration**
(underline, subscript, superscript) changes the grammatical role of an
ordinary Latin name:

| Raw ASCII   | Presentation              | Meaning                            |
| ----------- | ------------------------- | ---------------------------------- |
| `r`         | r                         | noun (variable) named r            |
| `t_`        | t underlined              | function t (rotate)                |
| `t_2`       | t underlined, sub 2       | rotate specialized to axis 2       |
| `t_12`      | t underlined, sub 12      | rotate lifted over axes 1 and 2    |
| `+^r`       | + with superscript r      | derived function: reduce by +      |
| `+^s_2`     | + sup s, sub 2            | scan by + along axis 2             |
| `_l`, `_r`  | _l, _r                    | left / right lambda argument       |
| `@`         | @                         | the Unit value                     |
| `now_@`     | now underlined, sub @     | sugar for `now_ @` (niladic call)  |

The long and terse spellings are the same grammar: `rotate_2` and
`t_2` are the same token class and the same function, so teaching code
can be written in words and then progressively abbreviated without
changing paradigm.

The raw ASCII text is the stored source (git diffs, terminals, compiler
diagnostics stay plain). The Unicode decorated form is a lossless
presentation layer, never the storage format.

## 3. Goals

- G1. Terse array programming using only ASCII input.
- G2. Decoration determines lexical/grammatical class; the grammar is
  unambiguous and never resolved by heuristics.
- G3. A small core calculus: every executable thing is application of
  one-argument functions; niladic/monadic/dyadic are surface sugar
  over currying.
- G4. Static, inferred (Hindley-Milner style) types; terse code needs
  no annotations; composition errors are caught before evaluation.
- G5. APL-style arrays: shape, reshape, range, index, rotate, reduce,
  scan, scalar extension / broadcasting, axis specialization.
- G6. First-class functions, derived functions, combinators and trains
  that desugar into the core calculus (nothing train-specific in the
  evaluator).
- G7. Tests are the specification: every syntax and semantic rule is
  pinned by positive, negative and ambiguity tests.
- G8. A browser playground (Rust -> WASM) with Raw / Decorated /
  Canonical / Expanded / Core display modes, role-aware hover tooltips,
  and an AST-driven right-to-left explainer showing value, type and
  shape of every sub-expression.
- G9. Capstone: one-line Conway's Life passes golden generations:
  `life = { (+^r -1 0 1 t_12 _r) = 3 + _r }` (exact spelling to be
  pinned by the M5/M8 tests; see `docs/design.md`).

## 4. Non-goals (for now)

- Performance, compilation to native code, GPU backends.
- Shape-indexed (dependent) types; shape is runtime metadata in v0.
- Type classes beyond a built-in `Num` constraint in v0.
- A package manager or module system beyond dotted namespaces.
- Unicode input as source (accepted only as presentation).

## 5. Users

- The author and array-language enthusiasts exploring notation design.
- Learners: progressive disclosure from long names to terse stems,
  with the explainer showing what a one-liner does.
- AI coding agents implementing the language under a strict TDD
  process (see `CLAUDE.md`).

## 6. Functional requirements

| ID  | Requirement                                                           |
| --- | --------------------------------------------------------------------- |
| F1  | Lexer classifies plain names, decorated names, lambda args, Unit, numbers (incl. negative literals), symbols, delimiters, with spans. |
| F2  | Raw <-> decorated presentation rendering is lossless in both directions. |
| F3  | Parser yields 0, 1 or many parses; many = `AmbiguousExpression` error listing the alternatives. |
| F4  | Canonical formatter emits a fully disambiguated form; `parse(fmt(parse(x))) == parse(x)`. |
| F5  | Desugaring to a Core IR (Lit, Unit, Var, Lam, App, Let); sugar forms normalize identically. |
| F6  | Type inference over the Core IR; invalid valence (e.g. `now_ 42`) is a type error, never an ignored argument. |
| F7  | Evaluator over Core IR only; never inspects source spelling. |
| F8  | Array primitives: range, shape/reshape, index, rotate (axis, multi-axis), reduce, scan, each, table/outer, with scalar extension. |
| F9  | Combinators I K S B C W (and Y/Z per the evaluation-strategy decision) expressible and type-inferred. |
| F10 | Trains `[f g]` (hook/atop) and `[f g h]` (fork) desugar to Core. |
| F11 | CLI `xetal` with subcommands `lex`, `render`, `parse`, `fmt`, `core`, `type`, `eval`, `run`, `repl` producing stable text output. |
| F12 | Execution trace tree: NodeId, span, value, type, shape, children. |
| F13 | WASM playground (display modes, tooltips, explainer, "why this parse"). |

## 7. Quality requirements

- Parser, lexer and evaluator never panic on any input (fuzzed).
- Every accepted grammar rule has at least one rejection/ambiguity test.
- Every sugar form has a normalization-equivalence test.
- Zero clippy warnings, `cargo fmt` clean, ASCII-only markdown.
- CLI goldens are held by `reg-rs` baselines committed in the repo.

## 8. Milestones (demo-driven)

| M  | Demo                                                        |
| -- | ----------------------------------------------------------- |
| M0 | raw keyboard source renders decorated and back, losslessly  |
| M1 | `square = { _r * _r }; square_ 7` -> 49                     |
| M2 | `now_@` works; `now_ 42` visibly rejected by the type checker |
| M3 | reshape / index / range matrix demo                         |
| M4 | `+^r`, `+^s`; `sum = +^r; sum_ 1 2 3 4` -> 10               |
| M5 | 2-D rotate, multi-axis `-1 0 1 t_12 A`                      |
| M6 | SKI / BCKW (+ Y or Z) combinator notebook with inferred types |
| M7 | hook / fork / train expansion                               |
| M8 | one-line Life (block, blinker, glider goldens)              |
| M9 | WASM playground with right-to-left visual explainer         |

Life is the capstone integration test, present (as pending) from the
first saga, never the source of special-case semantics.

## 9. Success criteria

- M8 Life acceptance test passes with no Life-specific code paths.
- Every example in user docs is executed by a test.
- The ambiguity corpus is a first-class, growing part of the spec.
