# X_eTaL Spatial Equations — Coding Agent Implementation Plan

**Status:** Proposed proof of concept; verify all syntax against current repositories.

## Objective

Build a macro-based X_eTaL library that turns typed, deferred symbolic tuples into (1) recognizable mathematical notation, (2) inspectable expanded X_eTaL array code, and (3) executable/debuggable CNN computation. Discover and document any limitations requiring changes to the base language. Do not introduce imperative iteration or change existing `+/` semantics.

Target equation (valid 2D CNN cross-correlation):

$$
Y_{o,h,w}=\sum_{c=0}^{C_{in}-1}\sum_{r=0}^{K_h-1}\sum_{s=0}^{K_w-1}X_{c,h+r,w+s}W_{o,c,r,s}.
$$

## Decisions

| Topic | Decision |
|---|---|
| Entry point | Namespaced `e:q<...>` macro |
| Regions | Optional `e:u<...>`, `e:c<...>`, `e:l<...>` macros |
| Syntax | Named letters first; defer punctuation aliases |
| Structure | Typed heterogeneous tuples of deferred symbolic expressions |
| Repetition | Homogeneous arrays of normalized domain descriptors |
| Transformations | Compile-time array operations wherever available |
| Semantics | Preserve ordinary X_eTaL array, map, and reduction behavior |
| Rendering | Same symbolic representation drives math output and lowering |
| Core compiler | Patch only after a minimal failing test proves need |
| AST | Do not assume conventional AST manipulation; investigate existing string-to-symbolic pipeline |

All source examples below are **proposals**, not verified current X_eTaL syntax.

## Repositories and deliverables

Research `softwarewrighter/X_eTaL`, existing X_eTaL library and ML/demo repositories, `tuple-rs`, DiscoveryOne, and `sw-cor24-macrolisp`. Locate real paths before writing code. Prefer adding a `spatial`/`equations` library to the existing library repo, a CNN example to the existing ML repo, and only narrow parser/renderer fixes to the core repo.

Proposed files, adapted to actual conventions:

```text
libraries/spatial/equations.xtlm      # e:q, e:u, e:c, e:l
libraries/spatial/symbolic.xtl       # symbolic capture/inspection helpers
libraries/spatial/domains.xtl        # pairing, validation, shape operations
libraries/spatial/lowering.xtl       # executable array-expression generation
libraries/spatial/render.xtl         # math layout / LaTeX adapter
ml/cnn-spatial/reference.xtl         # independent array CNN oracle
ml/cnn-spatial/spatial.xtl           # macro-based implementation
ml/cnn-spatial/fixtures.xtl          # deterministic test tensors
docs/spatial-equations/capabilities.md
docs/spatial-equations/decisions.md
docs/spatial-equations/limitations.md
docs/spatial-equations/README.md
```

## M0 — Research and capability probes

Produce `capabilities.md` with observed behavior, source paths/lines, runnable probes, and limitations for:

1. `.xtlm` macro definitions, namespaces, naming and underscore conventions.
2. Macro arguments: raw strings, token sequences, typed symbolic expressions, or evaluated values.
3. Nested macro expansion order and ability to defer inner expansion.
4. `~` evaluation suppression: opaque thunk versus inspectable symbolic quotation.
5. Tuple construction, heterogeneity, laziness, and per-element access.
6. Compile-time array operators: `each`, `over`, `table`, zip, indexing, reshape, shape checks.
7. Existing symbolic-expression parsing, substitution, type checking, and regeneration.
8. Macro expansion inspection, diagnostics, source ranges, and debugging.
9. Existing mathematical rendering, pretty-printing, org-mode/LaTeX/SVG/WASM support.
10. Exact ordinary `+/` reduction axis/shape semantics.

**Gate:** Do not modify compiler grammar until minimal tests demonstrate the actual blocker. Preserve failed examples as regression tests.

## M1 — Independent CNN reference

Use valid cross-correlation (kernel NOT flipped), stride 1, padding 0, dilation 1, no batch or bias:

- Input `X`: `[2,4,4]` (channel, height, width).
- Weights `W`: `[1,2,2,2]` (output channel, input channel, kernel row, kernel column).
- Output `Y`: `[1,3,3]`.
- Reduction domain: `[2,2,2]` = eight products per output element.

Fixture A: all inputs and weights 1; every output must be 8.

Fixture B:

```text
X[0] = [[ 1, 2, 3, 4], [ 5, 6, 7, 8], [ 9,10,11,12], [13,14,15,16]]
X[1] = [[17,18,19,20], [21,22,23,24], [25,26,27,28], [29,30,31,32]]
W[0,0] = [[1,2],[3,4]]
W[0,1] = [[5,6],[7,8]]
```

Expected `Y[0]` (independently verify in test harness):

```text
[[ 852, 888, 924],
 [ 996,1032,1068],
 [1140,1176,1212]]
```

Implement the oracle using ordinary X_eTaL array operations without the new macros. Add a second output-channel/kernel configuration later to catch hard-coded assumptions.

## M2 — Macro API and symbolic capture

Try explicit nested form:

```text
e:q<
    e:u< C-1, Kh-1, Kw-1 >,
    e:c< +/ (X[c,h+r,w+s] * W[o,c,r,s]) >,
    e:l< c:=0, r:=0, s:=0 >
>
```

Compare a positional tuple form:

```text
e:q<
    (C-1, Kh-1, Kw-1),
    +/ (X[c,h+r,w+s] * W[o,c,r,s]),
    (c:=0, r:=0, s:=0)
>
```

Prefer whichever is valid with minimal language changes. Inner macros label roles, not perform evaluation. Outer macro receives the complete structure before lowering. If the current macro engine expands inner macros first, test quoting/deferred expansion and document the smallest viable workaround.

Represent conceptually:

```text
Spatial(
  upper: Tuple<SymbolicExpr...>,
  center: SymbolicExpr,
  lower: Tuple<SymbolicExpr...>
)
```

**Important:** Tilde laziness is not automatically syntax quotation. Test `~(a,b)` versus `(~a,~b)` and determine whether individual expressions can be inspected without forcing evaluation. If only strings are available, split them with balanced delimiters and quote-aware logic; do not confuse string arrays with structured semantics.

## M3 — Typed domain normalization

For CNN, pair upper and lower components by position, first outermost:

```text
upper = (C-1, Kh-1, Kw-1)
lower = (c:=0, r:=0, s:=0)

paired = [
  Domain(c, 0, C-1),
  Domain(r, 0, Kh-1),
  Domain(s, 0, Kw-1)
]
```

A domain descriptor may be a typed tuple because its fields have different roles; the collection of descriptors should be a homogeneous array. Use compile-time array idioms to zip/pair, map/normalize, check shapes, and collect symbols. Keep bounds symbolic. Validate domain counts, integer-compatible bounds, binding scope, and lambda/free-variable references. Avoid unintended name capture; test hygiene explicitly.

## M4 — Lower to existing array semantics

For fixed output `(o,h,w)`:

```text
D = [0,C) × [0,Kh) × [0,Kw)
f(c,r,s) = X[c,h+r,w+s] * W[o,c,r,s]
result = reduce(+, map(f,D))
```

This is *semantic pseudocode*, not guaranteed X_eTaL syntax. The macro must produce actual valid X_eTaL source, printed in expansion output. Build output coordinates using existing array generators/table/broadcasting; do not introduce imperative loop syntax.

**Critical:** `+/` must keep its current meaning. If it reduces one axis, either flatten the rank-3 summand array in a documented order then use `+/` once, or use successive ordinary axis reductions. Do not reinterpret `+/` as context-sensitive all-axis reduction and do not accidentally reduce twice. Record reduction order and floating-point rounding implications.

## M5 — Mathematical rendering

Render recognizable nested sums from the *same normalized symbolic representation* used by lowering:

```latex
Y_{o,h,w} =
\sum_{c=0}^{C_{in}-1}
\sum_{r=0}^{K_h-1}
\sum_{s=0}^{K_w-1}
X_{c,h+r,w+s}W_{o,c,r,s}
```

Reuse X_eTaL's existing glyph conventions and pretty printer. Provide LaTeX text plus a rendered preview through existing docs/tooling. Do not independently hand-maintain the math and executable representations. Generic spatial layout should support upper/center/lower positions; the math consumer supplies sum/product notation. Capture surrounding assignment metadata for the left-hand side, or make it an explicit rendering parameter if unavailable.

## M6 — Expansion inspection and debugging

Provide a reproducible command or REPL workflow displaying:

```text
source macro
  -> captured upper / center / lower symbolic expressions
  -> typed paired domain descriptors
  -> generated executable X_eTaL array code
  -> intermediate shapes and reduction axes
  -> final tensor
```

Show exact generated source, not only pseudocode. Preserve source positions where supported; highlight corresponding `c`, `r`, `s` regions in math, macro input, and expansion. If the debugger cannot step through generated expressions, implement a trace/report mode first and document debugger integration as a separate issue.

## M7 — Tests and educational demo

Tests must cover:

- Correct outputs for both fixtures and output shape `[1,3,3]`.
- One, two, and three paired dimensions; positional order and mismatched counts.
- Nested macros, nested tuples, lambdas, quoted delimiters, and malformed regions.
- No premature evaluation of bounds/bindings; distinction between tilde and quotation.
- Typed domain construction, symbol scope, and capture avoidance.
- Exact generated array expression and valid compilation.
- Existing `+/` behavior unchanged; explicit reduction axes and order.
- Golden LaTeX rendering and correspondence to source regions.
- Error diagnostics identifying the responsible region.
- Baseline parser and macro regression suite passes.
- Float case uses documented tolerance; don't require bitwise equality under reassociation.

Educational README: one page showing paper equation, named-macro source, normalized domains, macro expansion, tensor shapes, and numeric result. Optional WASM view may highlight matching sections; keep it out of the critical path until the CLI/literate demo works.

## Milestones / exit gates

| ID | Deliverable | Exit gate |
|---|---|---|
| M0 | Capabilities report | Claims grounded in source/tests |
| M1 | Independent CNN oracle | Fixtures pass |
| M2 | Lazy symbolic tuple capture | Each region inspectable, unevaluated |
| M3 | Domain pairing library | Typed descriptors, diagnostics |
| M4 | Array lowering | Expanded X_eTaL executes, matches oracle |
| M5 | Mathematical renderer | Triple sum generated from same symbolic structure |
| M6 | Expansion/debug report | Generated source and shapes visible |
| M7 | Tests and README | Clean checkout reproduces all results |
| M8 | Limitations and core patches | Each proposed change has failing test and minimal patch |

## Core-language change policy

For each blocker create a small issue/entry with: reproduction, observed behavior, expected behavior, why a library workaround fails, proposed minimal change, compatibility impact, and regression tests. Possible blockers include raw-token macro capture, typed symbolic tuple arguments, quotation versus tilde thunks, compile-time array primitives, hygienic binding, source maps, and math-layout rendering. Do not implement speculative generalized AST facilities or punctuation aliases in this PoC.

## Agent execution instructions

1. Inspect repositories and tests before coding; never assert an API exists without verifying it.
2. Implement in small, testable commits with library-first changes.
3. At each milestone run the smallest tests, then relevant integration/regression tests.
4. Update `capabilities.md`, `decisions.md`, and `limitations.md` with discoveries.
5. Report exact commands, actual output, generated source, and failing tests.
6. If blocked, preserve a minimal reproducer and propose alternatives ranked by invasiveness.
7. Do not change established operator semantics or introduce explicit runtime loops to make the demo pass.
8. Stop after a passing CNN PoC and documented limitations; treat symbolic punctuation aliases, generalized spatial consumers, and UI tooling as follow-on work.

## Follow-on experiments (not MVP)

- Product, tensor contraction, and matrix multiplication consumers.
- Dependent bounds and nonrectangular domains.
- Alternate punctuation macro aliases (`|<`, `|*<`, etc.) only after grammar collision tests.
- Bidirectional math/source highlighting and single-step visualization in Yew/WASM.
- GPU/FPGA lowering of the already-expanded array computation.
- General compile-time array-based symbolic rewriting as an X_eTaL capability.
