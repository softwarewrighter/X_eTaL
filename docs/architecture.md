# X_eTaL -- Architecture

## 1. Pipeline

```
raw ASCII source
    |  xetal-lex        tokens + spans (decoration -> token class)
    v
tokens
    |  (macro phase)    u_se< libraries, per-file namespaces (libraries saga)
    v
tokens
    |  xetal-syntax     deterministic parse: 0 or 1 tree; boundary
    v                   shapes rejected with specific errors
surface AST
    |  xetal-core       desugar: currying, lambdas, quotes and operands,
    v                   trains by position, guards, statements -> Core IR
Core IR
    |  xetal-types      Hindley-Milner inference (types saga)
    v
typed Core IR
    |  xetal-eval       strict evaluator over Core only (lazy ~ params);
    v                   trace tree later; array kernels from xetal-array
value (+ trace)
```

Side paths:

- `xetal-render`: raw <-> decorated Unicode presentation (lossless),
  LaTeX output, and the canonical (fully parenthesized) printer; an
  expanded (long names) printer comes later.
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
reg/                       reg-rs baselines (*.rgt, *.out, *.err) - committed
demos/                     executable .xtl demo scripts (reg-rs goldens)
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
   u:s_quare := { _r * _r }; u:s_quare 7
   == TOKENS
   ...
   == RENDER
   ...
   == SURFACE
   ...
   == CANONICAL
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

   Areas so far: `lex/ render/ syntax/ ambiguity/ eval/ integration/`
   (later: `types/ arrays/ combinators/ trains/`). A `pending` case must
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
xetal lex    <FILE|-e EXPR>    token dump with byte spans
xetal render <FILE|-e EXPR>    decorated Unicode (--raw back, --latex)
xetal parse  <FILE|-e EXPR>    surface tree as S-expressions
xetal fmt    <FILE|-e EXPR>    canonical form
xetal core   <FILE|-e EXPR>    Core IR (built-ins marked #)
xetal type   <FILE|-e EXPR>    inferred type of each top-level item
xetal eval   <FILE|-e EXPR>    type-check, evaluate; print each value
                               (--untyped skips the checker)
xetal run    FILE.xtl          type-check and run a script (--untyped)
xetal FILE.xtl                 the same (for #!/usr/bin/env xetal)
xetal repl                     interactive (arrays saga)
```

Each command runs the earlier stages first, so an early error is
reported by any later command. All outputs are deterministic text so
they can be reg-rs baselines; errors and warnings go to stderr as
`error[CODE]` / `warning[CODE]` with byte spans.

## 6. Web playground (later saga)

Rust -> WASM (same style as the COR24 APL web demo): an editor on raw
text with a decorated overlay, display-mode switcher, role-based
semantic highlighting (value / function / derived / lambda arg /
namespace / axis / unit), hover tooltips (kind, inferred type, long
form, axes, namespace), "why this parse" rule explanation, and a
right-to-left explainer driven by the trace tree showing value, type
and shape of each node (Life shows the nine shifted boards).
