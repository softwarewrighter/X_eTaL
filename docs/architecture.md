# X_eTaL -- Architecture

## 1. Pipeline

```
raw ASCII source
    |  xetal-lex        tokens + spans (decoration -> token class)
    v
tokens
    |  xetal-syntax     deterministic parse: 0 or 1 tree; boundary
    |                   shapes rejected with specific errors
    v
surface AST          (many parses => AmbiguousExpression error)
    |  xetal-core       desugar: lambdas, niladic sugar, derivations,
    v                   long/terse names, trains -> Core IR
Core IR
    |  xetal-types      Hindley-Milner inference (+ Num constraint)
    v
typed Core IR
    |  xetal-eval       strict evaluator over Core only; trace tree
    v                   (uses xetal-array primitives)
value + trace
```

Side paths:

- `xetal-render`: raw <-> decorated Unicode presentation (lossless),
  canonical (fully parenthesized) and expanded (long names) printers.
- `xetal-cli`: the `xetal` binary exposing every stage as text.
- `xetal-web` (later): Rust -> WASM playground consuming the public
  library APIs only.

## 2. Workspace layout

Single cargo workspace (split into component workspaces only if it
grows large).

```
Cargo.toml                 workspace
crates/
  xetal-base/              spans, NodeId, Diagnostic, LANG_NAME
  xetal-lex/               lexer + token types
  xetal-render/            raw <-> decorated, canonical, expanded
  xetal-syntax/            parser, surface AST, ambiguity detection
  xetal-core/              Core IR + desugaring
  xetal-types/             type inference
  xetal-array/             dense arrays + primitive kernels
  xetal-eval/              evaluator + trace tree
  xetal-cli/               `xetal` binary (+ tests/spec.rs harness)
  xetal-spec/              spec-case file parser, checker, blesser
  xetal-web/               (later) WASM playground
spec/                      language spec corpus (*.case files)
reg/                       reg-rs baselines (*.rgt, *.out) - committed
scripts/                   quality gate, reg-rs helpers
docs/                      PRD, design, architecture, plan, research
```

The display-name constant `xetal_base::LANG_NAME` lives in
`xetal-base` (kept tiny, depended on by all).

## 3. Dependency rules

```
base -> lex -> syntax -> core -> types -> eval -> cli / web
base -> render (depends on lex + syntax + core for printers)
base -> array  (depends on nothing else; peer of the front end)
spec           (no deps; dev-dependency of cli for the spec harness)
eval -> array
```

- `xetal-array` knows nothing about syntax; the evaluator maps
  `PrimId` to kernels.
- The evaluator never sees surface syntax or source spelling.
- Each crate owns its own error type; all convert to
  `xetal_base::Diagnostic` (code, message, primary span, notes).
- No crate may panic on user input; errors are values.

## 4. Test architecture (tests are the spec)

Four layers, all run by `cargo test` except reg-rs:

1. **Unit tests** inside each crate (TDD inner loop).
2. **Spec corpus** `spec/<area>/*.case`, driven by a harness in
   `xetal-cli/tests/spec.rs` (case parsing and checking live in the
   `xetal-spec` crate). A case file has sections, each optional
   except SOURCE:

   ```
   == SOURCE
   square = { _r * _r }; square_ 7
   == TOKENS
   ...
   == RENDER
   ...
   == CORE
   ...
   == TYPE
   Int
   == RESULT
   49
   == ERROR
   (expected diagnostic code + message, instead of RESULT)
   == STATUS
   pending   (acceptance test not yet expected to pass)
   ```

   Areas: `lex/ render/ syntax/ ambiguity/ core/ types/ arrays/
   combinators/ trains/ errors/ integration/`. A `pending` case must
   still FAIL; the harness errors if a pending case unexpectedly
   passes, so flipping it to active is a deliberate commit. The Life
   acceptance case exists from the first saga as `pending`.
   `XETAL_BLESS=1 cargo test` rewrites expected sections (reviewed as
   diffs; never bless blindly).
3. **Property tests** (proptest): render round-trip, fmt/parse
   round-trip, rotate inverse, shape/reshape, reduce/concat, scan
   last == reduce, currying, sugar normalization, fork law.
4. **CLI goldens** with `reg-rs`: each demo command
   (`xetal eval ...`, `xetal run demos/*.xtl`) is a reg-rs test whose
   baseline lives in `reg/` (set `REG_RS_DATA_DIR=reg`). Only `.rgt`,
   `.out` and `.err` files are committed; `*.tdb*` are gitignored (reg-rs
   regenerates them from `.rgt`). Helper: `scripts/reg.sh`.

Unit tests live in each crate's `tests/` directory (public API only)
so source modules stay small: `sw-checklist` (run by
`scripts/gate.sh`) fails a module over 7 functions, a crate over 7
modules or a function over 50 lines.

Later: `cargo-fuzz` targets for lexer, parser, `fmt`, and evaluator
(never panic; `parse(fmt(parse x)) == parse x`).

## 5. CLI surface

```
xetal lex    <src|-e expr>     token dump
xetal render <src|-e expr>     decorated Unicode form (--raw back,
                               --latex for LaTeX math)
xetal parse  <src|-e expr>     surface AST or ambiguity report
xetal fmt    <src|-e expr>     canonical form
xetal core   <src|-e expr>     Core IR
xetal type   <src|-e expr>     inferred type
xetal eval   -e expr           result
xetal run    file.xtl          run a program
xetal repl                     interactive
```

All outputs are deterministic text so they can be reg-rs baselines.

## 6. Web playground (later saga)

Rust -> WASM (same style as the COR24 APL web demo): an editor on raw
text with a decorated overlay, display-mode switcher, role-based
semantic highlighting (value / function / derived / lambda arg /
namespace / axis / unit), hover tooltips (kind, inferred type, long
form, axes, namespace), "why this parse" rule explanation, and a
right-to-left explainer driven by the trace tree showing value, type
and shape of each node (Life shows the nine shifted boards).
