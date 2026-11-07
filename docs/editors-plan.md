# Editors and IDEs for X_eTaL (decision record)

The user (2026-11-07) asked how to demo a language server (LSP) and a
debug adapter (DAP) in Emacs and in a JetBrains IDE, whether RustRover
could host a full-stack X_eTaL environment, and which base a "native"
X_eTaL IDE should start from. This records the analysis and the
decisions; the steps are Saga 44 in docs/plan.md, after the debugger
(Saga 43, docs/debugger-plan.md).

## 1. Two servers, every editor

X_eTaL ships two servers, each a thin crate speaking JSON on standard
input and output, and every editor is a client of them:

- `xetal dap`, the Debug Adapter Protocol over the debugger's core
  (`xetal-debug`): breakpoints, stepping (backward too), the stack,
  variables, watches, the array viewer (step debug-dap).
- `xetal lsp`, the Language Server Protocol over what already exists:

| LSP feature | Built from |
| ----------- | ---------- |
| Diagnostics as you type | loading, type checking and the linter, placed through the source map |
| Quick fixes | the linter's fixes (`xetal lint --fix`) |
| Hover: types and docs | the program's types and the `xetal doc` model (doc comments) |
| Completion | `xetal-complete` (the editor's and the live demo's) |
| Go to definition, references | the `xetal doc` model's definitions and uses |
| Formatting | the canonical printer (`xetal fmt`) |
| Semantic highlighting | the view model's token classes |
| The drawn form | the decorated renderer, as inlay hints or a preview |

The debugger does not serve the language server: one answers questions
about a running program, the other about the text being edited. They
share the JSON-RPC plumbing and the editor packages.

## 2. Emacs

Emacs 30 (on this machine, 30.2) has eglot built in. The repository
already has `docs/emacs/xetal-mode.el`.

- LSP: eglot with one `eglot-server-programs` entry mapping
  `xetal-mode` to `xetal lsp`.
- DAP: dape, a small DAP client from GNU ELPA (one package to install),
  configured to start `xetal dap` on the file: breakpoints in the
  margin, the usual stepping keys, locals and watches in a window.
- The configurations live in `docs/emacs/` beside the mode, so a
  recorded demo can be reproduced.

## 3. JetBrains

- **JetBrains MPS** (Meta Programming System) is the open-source,
  AST-oriented language workbench. It is a poor fit and is not used:
  MPS edits the syntax tree directly (a projectional editor), so
  X_eTaL would be redefined inside it (structure, editor, type system,
  generator), duplicating the Rust implementation and giving up plain
  ASCII text drawn decorated.
- **WebStorm and RustRover** have been free for non-commercial use but
  are not open source: they can be extended with plugins, never forked
  or rebranded. RustRover has the web-language support of the
  JetBrains IDEs beside Rust, so one window holds the Rust extensions,
  the Yew front ends and, with a plugin, X_eTaL: a full-stack
  environment, under its license (commercial use needs a paid one).
- **LSP4IJ**, Red Hat's open-source plugin, is the client to start with:
  it runs in the free IntelliJ IDEA and in the other JetBrains IDEs and
  speaks both LSP and DAP, so pointing it at `xetal lsp` and `xetal
  dap` gives diagnostics, hover, completion and a debugger with no
  plugin code of ours. A dedicated plugin can follow: an X_eTaL file
  type and icon, run configurations, the array viewer as a tool window.
- JetBrains has been changing its editions and licenses (IntelliJ IDEA
  Community folding into one distribution); check the current terms
  before a document names an edition.

## 4. A "native" X_eTaL IDE: the bases compared

| Base | What it is | For X_eTaL |
| ---- | ---------- | ---------- |
| Code - OSS | VS Code's open-source core (MIT); forks such as Posit's Positron, a data-science IDE | The best precedent: Positron's variables pane, plots pane, data viewer and console are what an array language wants (our REPL, array viewer and debugger). TypeScript, and keeping up with VS Code. |
| Eclipse Theia | An Eclipse Foundation framework for building branded IDEs, running VS Code extensions, desktop and browser | Made for this (the Arduino IDE 2 is built on it); LSP and DAP built in. TypeScript. |
| Zed | An open-source editor in Rust; extensions in Rust compiled to WASM; LSP, and a debugger more recently | The one Rust option, as this project is Rust throughout. An extension is cheap; a fork carries GPL and AGPL obligations and a fast-moving codebase. |
| Eclipse IDE | The classic Java and SWT IDE; LSP and DAP through LSP4E | Works, but heavy and dated: not a starting point. |
| Other JavaScript IDEs | Smaller VS Code look-alikes, or Monaco or CodeMirror embedded in a page | Little gain over Code - OSS or Theia. |

## 5. Decisions

Taken with the user, 2026-11-07:

1. **Extensions first, no fork.** The VS Code extension (step
   debug-dap) and a Zed extension in Rust, both thin over the two
   servers; Emacs and JetBrains through their standard clients (eglot
   and dape; LSP4IJ).
2. **A branded IDE later, if wanted, forks Code - OSS** as Positron
   did, an array language wanting a data-science IDE's panes; Eclipse
   Theia if the browser matters as much as the desktop. Not before the
   extensions have proven the servers.
3. **The live demo stays the zero-install IDE**: Rust, Yew and WASM,
   the three panes, the terminal shell, and the debugger page (step
   debug-web).
4. **MPS and the Eclipse IDE are not used.**

## 6. Steps (Saga 44)

1. **lsp-server**: `xetal lsp`, the table in section 1, with golden
   transcripts (requests in, responses out) as `xetal dap` has.
2. **editor-emacs**: eglot and dape configurations in `docs/emacs/`, a
   walkthrough and a recording: editing with diagnostics, hover and
   completion, then debugging with breakpoints, stepping back and the
   array viewer.
3. **editor-jetbrains**: LSP4IJ set up for `xetal lsp` and `xetal dap`
   in IntelliJ IDEA (and RustRover for the full-stack case),
   documented with screenshots; then a dedicated plugin's design.
4. **editor-zed**: a Zed extension in Rust (the language, the server,
   the debugger), published to Zed's extension registry by the user.
5. **editor-vscode-lsp**: the VS Code extension of step debug-dap gains
   the language server, so one install edits and debugs.
