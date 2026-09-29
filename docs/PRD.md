# X_eTaL -- Product Requirements

X_eTaL -- the eXperimental
eXtensible Typed Array Language.

> LaTeX uses text to produce typography. X_eTaL uses typography to
> express computation.

Sources: `docs/research.txt` and `docs/research2.txt` (archival design
conversations, not normative). The language decisions are recorded in
`docs/lang-choices.md`, which governs where other documents disagree.
This document, `docs/design.md`, `docs/architecture.md` and
`docs/plan.md` are the planning docs; the executable test suite is the
normative language spec.

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
changes the role of an ordinary name: an underlined letter makes a
name a function, a leading superscript names its namespace, and a
subscript gives its axes.

| Raw ASCII     | Presentation                          | Meaning                              |
| ------------- | ------------------------------------- | ------------------------------------ |
| `r`           | r                                     | a variable named r                   |
| `r_ev`        | rev, r underlined                     | built-in function: reverse           |
| `o_-`         | o-, o underlined                      | built-in function: rotate            |
| `o_-_2`       | o-, o underlined, subscript 2         | rotate along axis 2                  |
| `'+ r_/ A`    | quote, +, r/ with r underlined        | reduce A by plus (APL `+/A`)         |
| `u:s_quare`   | superscript u, square, s underlined   | a user function                      |
| `c:K_`        | superscript c, K underlined           | K from the combinator library        |
| `x^2`         | x squared                             | exponent on a value                  |
| `_l`, `_r`    | _l, _r                                | left / right lambda argument         |
| `@`           | @                                     | the Unit value                       |

Built-ins are named with words (`r_eshape`, `t_ally`), so code stays
recognizable; a punctuation mark appears only where it carries APL
meaning (`r_/` reduce, `s_\` scan, `o_-` rotate).

The raw ASCII text is the stored source (git diffs, terminals, compiler
diagnostics stay plain). The decorated Unicode and LaTeX forms are a
presentation layer, never the storage format.

## 3. Goals

- G1. Terse array programming using only ASCII input.
- G2. Decoration determines lexical/grammatical class; the grammar is
  unambiguous and never resolved by heuristics.
- G3. A small core calculus: every executable thing is application of
  one-argument functions; each function has one arity and dyadic use
  is currying; niladic functions take the Unit value `@`.
- G4. Static, inferred (Hindley-Milner style) types; terse code needs
  no annotations; composition errors are caught before evaluation.
- G5. APL-style arrays with leading-axis defaults: shape, reshape,
  range, select, rotate, reduce, scan, scalar extension, axis
  subscripts on any function.
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
  `u:l_ife := { ('+ r_/_12 -1 0 1 o_-_12 _r) { (_l = 3) + _r * _l = 4 } _r }`
  (Conway's rule, design.md 6.2; syntax per `docs/lang-choices.md`).

## 4. Non-goals (for now)

- Performance, compilation to native code, GPU backends.
- Shape-indexed (dependent) types; shape is runtime metadata in v0.
- Type classes beyond a built-in `Num` constraint in v0.
- A package manager; libraries are brought in by the `u_se<` macro
  with per-file namespaces, nothing more.
- Unicode input as source (presentation only).

## 5. Users

- The author and array-language enthusiasts exploring notation design.
- Learners: progressive disclosure from long names to terse stems,
  with the explainer showing what a one-liner does.
- AI coding agents implementing the language under a strict TDD
  process (see `CLAUDE.md`).

## 6. Functional requirements

| ID  | Requirement                                                           |
| --- | --------------------------------------------------------------------- |
| F1  | Lexer classifies variables, function names (one underline, optional trailing mark), namespace prefixes, subscripts, exponents, lambda args, Unit, numbers (incl. negative literals), strings, symbols (incl. digraphs), delimiters, comments, with spans. |
| F2  | Raw <-> decorated Unicode rendering is lossless in both directions; LaTeX output covers what Unicode cannot. |
| F3  | The parser never accepts an ambiguous expression. The grammar is decided by tokens so every input has at most one parse, and each shape near a rule boundary is rejected with a specific error (the `spec/ambiguity/` corpus); an `AmbiguousExpression` error listing alternatives is reserved for any future rule that could admit two parses. |
| F4  | Canonical formatter emits a fully disambiguated form; `parse(fmt(parse(x))) == parse(x)`. |
| F5  | Desugaring to a Core IR (Lit, Unit, Var, Lam, App, Let); sugar forms normalize identically. |
| F6  | Type inference over the Core IR; invalid valence (e.g. `u:n_ow! 42` for a niladic function) is a type error, never an ignored argument. |
| F7  | Evaluator over Core IR only; never inspects source spelling. |
| F8  | Array built-ins: range, shape/reshape, select, rotate (axis, multi-axis), reduce, scan, each, table, inner product, with scalar extension. |
| F9  | Combinators I K S B C W V T expressible and type-inferred; Y works through a lazy (`~`) parameter, Z under strict evaluation. |
| F10 | Trains `[F G]` (atop) and `[F G H]` (fork) desugar to Core. |
| F11 | CLI `xetal` (alias `x_etal`) with subcommands `lex`, `render`, `parse`, `fmt`, `core`, `type`, `eval`, `run`, `repl`, and `xetal FILE` for `.xtl` scripts, producing stable text output. |
| F12 | Execution trace tree: NodeId, span, value, type, shape, children. |
| F13 | WASM playground (display modes, tooltips, explainer, "why this parse"). |
| F14 | Macro phase between lexer and parser: `u_se<` libraries with required aliases and per-file namespaces. |
| F15 | Diagnostics: errors with codes and spans; non-fatal warnings (e.g. a parameter shadowing a built-in). |

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
| M1 | `u:s_quare := { _r * _r }; u:s_quare 7` gives 49; `10 u:s_ub 3`; guarded factorial |
| M2 | `u:n_ow! @` works; `u:n_ow! 42` visibly rejected by the type checker |
| M3 | `2 3 r_eshape r_ange 6` and select / take / drop matrix demo |
| M4 | `'+ r_/ 1 2 3 4` gives 10; `u:s_um := r_/ '+`; scan, each, table |
| M5 | 2-D rotate, multi-axis `-1 0 1 o_-_12 B`                    |
| M5b| `xetal edit`: ASCII left, live decorated view right; live-rendered REPL |
| M6 | combinator notebook with inferred types; Y via a lazy parameter |
| M6b| the combinators as a library: `"c:" u_se< "Combinators"`    |
| M7 | fork / atop train expansion                                 |
| M8 | one-line Life (block, blinker, glider goldens)              |
| M9 | WASM playground with right-to-left visual explainer         |

Life is the capstone integration test, present (as pending) from the
first saga, never the source of special-case semantics.

## 9. Success criteria

- M8 Life acceptance test passes with no Life-specific code paths.
- Every example in user docs is executed by a test.
- The ambiguity corpus is a first-class, growing part of the spec.
