# X_eTaL -- Architecture

## 1. Pipeline

```
raw ASCII source
    |  xetal-lex        tokens + spans (decoration -> token class)
    v
tokens
    |  (macro phase)    macro calls expanded (xetal-expand; the system
    |                   macros from lib/System.xtlm); u_se< libraries,
    |                   per-file namespaces
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
                           catalog, generated from builtins.toml),
                           xetal-store (where files live: the disk,
                           memory, or a store the host installs)
  expand/                  xetal-mapped (a text with a map back to
                           where each byte was written), xetal-expand
                           (macro calls found and replaced by what
                           their macros give, to a depth limit)
  lookup/                  xetal-lookup (where libraries and macro
                           libraries come from: disk, the store,
                           built in; found together, MC11)
  macro/                   xetal-sources (several files as one combined
                           text with a source map back to file, line
                           and column), xetal-names (one file: its
                           imports, definitions and renamed names),
                           xetal-macro (loading the files into one
                           program; macro libraries loaded and their
                           macros called), xetal-program (a program
                           with its libraries, lowered, errors
                           located; or a library file on its own;
                           macros run),
                           xetal-libs (the standard libraries, lib/,
                           built in)
  syntax/                  xetal-token (tokens, lexer errors),
                           xetal-lex (lexer), xetal-ast
                           (surface AST, printer), xetal-syntax (parser)
  core/                    xetal-ir (Core IR, printer), xetal-core
                           (desugaring), xetal-explain (notes for
                           errors in trains), xetal-lint (warnings)
  render/                  xetal-render: decorated, LaTeX, canonical
  view/                    xetal-view: the front-end-agnostic view
                           model (styled segments, classes, raw <->
                           rendered map, ANSI output); xetal-grid:
                           values laid out for display; xetal-diagram:
                           annotated SVG diagrams of a line of source
  types/                   xetal-ty (types, unifier, schemes),
                           xetal-prim-types (built-in types, read from
                           catalog signatures),
                           xetal-elab (number-type elaboration),
                           xetal-types (inference, checking)
  eval/                    xetal-array (dense arrays, layout),
                           xetal-value (runtime values, printing),
                           xetal-arith (scalar rules, scalar extension),
                           xetal-struct (structural built-ins),
                           xetal-eval (running a program: the API the
                           CLI, REPL and live demo call)
  step/                    the steppable evaluator (D50): xetal-step
                           (Core run by an explicit machine whose
                           state is data, in slices), xetal-kernel (the
                           higher-order built-ins as kernels: each call
                           of an operand is a step of the machine),
                           xetal-prim (the first-order built-ins
                           called on values)
  hof/                     xetal-hof (higher-order built-ins: reduce,
                           scan, power, dispatch; each a kernel whose
                           operand calls the evaluator makes), xetal-map
                           (item by item: each, table, inner)
  search/                  xetal-search (search and order: index-of,
                           member, unique, sort, grade, where)
  radix/                   xetal-radix (encode and decode in a mixed
                           radix: e_ncode, d_ecode)
  draw/                    xetal-svg (the pieces: shapes and cells,
                           colors, SVG elements, frames in turn),
                           xetal-draw (grids, large grids as images,
                           paths; no knowledge of the language)
  system/                  xetal-system (files, the keyboard, numbers as
                           text: []N_PUT, []N_GET, []R_EAD, f_ormat,
                           n_umbers; graphics: []G_RID, []S_HOW)
  axes/                    xetal-rotate (rotate and reverse along the
                           leading axis), xetal-axes (axis subscripts:
                           the move-to-front rule)
  console/                 the terminal interactive programs run in
                           (Saga 25, after web-sw-tos): xetal-screen
                           (styled character cells, wrapping,
                           scrollback), xetal-lineedit (browser keys
                           translated, a line editor with history);
                           plain Rust, tested natively
  tui/                     terminal front end (ratatui): xetal-buffer
                           (text and cursor), xetal-keys (nano-like
                           keymap as data), xetal-panes (ASCII and
                           rendered panes), xetal-term (terminal
                           guard), xetal-edit (`xetal edit`)
  line/                    xetal-line: the REPL's live line editor
  doc/                     `xetal doc` (Saga 32): xetal-doccom (doc
                           comments, S9: the `##` block above a
                           definition, the file's block, `###`
                           sections, `## >>` examples), xetal-doc (the
                           model: every item of a program, of the
                           libraries and macro libraries it imports
                           and of System.xtlm when it calls one, with
                           type, doc, source and resolved uses; JSON),
                           xetal-doclink (every name in a source linked
                           to what it names: an item, an import's
                           export, a system macro, a built-in; locals
                           left alone; where each item is used),
                           xetal-dochtml (source drawn decorated with
                           links, numbered lines, doc prose, examples,
                           anchors, the light and dark stylesheet),
                           xetal-docsite (the static site: index, a
                           page and a source page per file, built-ins)
  web/                     xetal-play: the live demo's engine
                           (decorate, check, run; libraries and files
                           from the installed store, then the standard
                           libraries); builds for wasm32
  cli/                     xetal-cli (`xetal` binary + tests/spec.rs
                           harness), xetal-spec (case files),
                           xetal-repl (interactive session)
docs/emacs/                xetal-mode.el, ob-xetal.el (Org Babel), ERT tests
docs/literate/             literate Org documents (tour.org), results recorded
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

Build provenance (the build host, the short commit and the build time,
in `xetal --version` and the live demo's footer) comes from the
`build.rs` of `xetal-cli` and `xetal-chrome`: `hostname`, `git
rev-parse` and the clock, each overridden by `XETAL_BUILD_HOST`,
`XETAL_BUILD_SHA` or `XETAL_BUILD_TIMESTAMP` when set (for a repository
that vendors X_eTaL, where `git` would report its own commit).

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

The gate also runs every `just` recipe (`scripts/just-smoke.sh`): the
file recipes (`show`, `pp`, `run`) on every demo, the others on a
sample, the rest skipped with a reason; a new recipe fails the gate
until it is added there.

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
                               (of a library file: each export)
xetal eval   <FILE|-e EXPR>    type-check, evaluate; print each value
                               (--untyped skips the checker)
xetal run    FILE.xtl          type-check and run a script (--untyped)
xetal FILE.xtl                 the same (for #!/usr/bin/env xetal)
xetal repl                     interactive session (lines from stdin)
xetal diagram NOTES            annotated SVG of a line of source (callouts
                               anchored to its tokens)
xetal doc --json <FILE|-e EXPR>
                               the cross-reference model as JSON: every
                               item of the file, its imports and the
                               system macros it calls
xetal doc --out DIR FILE       the same as a static site in DIR
```

Each command runs the earlier stages first, so an early error is
reported by any later command. All outputs are deterministic text so
they can be reg-rs baselines; errors and warnings go to stderr as
`error[CODE]` / `warning[CODE]` with byte spans. When the reader of
stdout goes away (`xetal run FILE | head`), the process ends on
SIGPIPE, exit status 141, with nothing on stderr, as cat and grep do
(`main` restores the default disposition that Rust's runtime
ignores; golden cli-pipe-closed). Pictures shown with `[]S_HOW` are
written as numbered SVG files named after the program
(`life-drawn-1.svg`, ...; `eval-1.svg` for `-e` text) into `--draw
DIR`, else `XETAL_DRAW`, else the current directory, and each path is
reported on stderr as `drawn PATH`; the command line installs this
`xetal_store::Drawing` store, and other hosts install their own
(golden cli-draw). The `just` recipes and the goldens default
`XETAL_DRAW` to `work/draw` (gitignored).

## 6. Web playground (later saga)

Rust -> WASM (same style as the COR24 APL web demo): an editor on raw
text with a decorated overlay, display-mode switcher, role-based
semantic highlighting (value / function / derived / lambda arg /
namespace / axis / unit), hover tooltips (kind, inferred type, long
form, axes, namespace), "why this parse" rule explanation, and a
right-to-left explainer driven by the trace tree showing value, type
and shape of each node (Life shows the nine shifted boards).
