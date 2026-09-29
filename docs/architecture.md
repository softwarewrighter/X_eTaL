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

The repository is a set of components. Each `components/<name>/` is
its own Cargo workspace holding a few small crates; components depend
on each other by path, and every build lands in one shared `target/`
directory (`.cargo/config.toml`). There is no root workspace:
`scripts/build-all.sh` builds every component and `scripts/gate.sh`
checks every component.

```
.cargo/config.toml         shared target dir for all components
components/
  base/                    xetal-base (spans, NodeId, Diagnostic,
                           LANG_NAME), xetal-catalog (the built-in
                           catalog, generated from builtins.toml)
  syntax/                  xetal-lex (lexer, tokens), xetal-ast
                           (surface AST, printer), xetal-syntax (parser)
  core/                    xetal-ir (Core IR, printer), xetal-core
                           (desugaring), xetal-lint (warnings)
  render/                  xetal-render: decorated, LaTeX, canonical
  view/                    xetal-view: the front-end-agnostic view
                           model (styled segments, classes, raw <->
                           rendered map, ANSI output)
  types/                   xetal-ty (types, unifier, schemes),
                           xetal-prim-types (built-in types, read from
                           catalog signatures),
                           xetal-elab (number-type elaboration),
                           xetal-types (inference, checking)
  eval/                    xetal-array (dense arrays, layout),
                           xetal-value (runtime values, printing),
                           xetal-arith (scalar rules, scalar extension),
                           xetal-struct (structural built-ins),
                           xetal-eval (evaluator; implements the
                           callback that applies function values)
  hof/                     xetal-hof (higher-order built-ins: reduce,
                           scan, dispatch; operands applied through
                           the evaluator's callback), xetal-map
                           (item by item: each, table, inner)
  search/                  xetal-search (search and order: index-of,
                           member, unique, sort, grade, where)
  axes/                    xetal-rotate (rotate and reverse along the
                           leading axis), xetal-axes (axis subscripts:
                           the move-to-front rule)
  tui/                     terminal front end (ratatui): xetal-buffer
                           (text and cursor), xetal-keys (nano-like
                           keymap as data), xetal-panes (ASCII and
                           rendered panes), xetal-term (terminal
                           guard)
  cli/                     xetal-cli (`xetal` binary + tests/spec.rs
                           harness), xetal-spec (case files),
                           xetal-repl (interactive session)
spec/                      language spec corpus (*.case files)
reg/                       reg-rs baselines (*.rgt, *.out, *.err) - committed
demos/                     executable .xtl demo scripts (reg-rs goldens)
scripts/                   build-all, gate, check-locks, reg-rs helper
docs/                      PRD, design, architecture, plan, research
```

Growth follows the sw-checklist limits by expanding up and out, never
by merging: a module with too many functions gets a sibling module, a
crate with too many modules a sibling crate, a component with too many
crates a sibling component (target gates: 25 lines per function, 5
functions per module, 5 modules per crate, 5 crates per component;
`lib.rs` holds only `mod` and `use`). Lists live in data files turned
into Rust by `build.rs` (the built-in catalog: `builtins.toml` ->
`OUT_DIR` -> `include!`), dispatch is by table or one-line arms, and
CLI behavior is pinned by reg-rs goldens rather than hand-written test
code. Each component has its own `Cargo.lock`;
`scripts/check-locks.sh --fix` refreshes locks after a manifest
change.

The display-name constant `xetal_base::LANG_NAME` lives in
`xetal-base` (kept tiny, depended on by all).

## 3. Dependency rules

```
base -> lex -> syntax -> core -> ty -> types -> eval -> cli / web
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

Four layers, all run by `cargo test` in each component (or
`scripts/gate.sh`) except reg-rs:

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
xetal repl                     interactive session (lines from stdin)
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
