# tui

Saga 7 of X_eTaL (see docs/plan.md), inserted at the user's request
before the combinators: see source as it is displayed while typing it
as ASCII (the .xtl syntax). Milestone M5b in docs/PRD.md.

Architecture (reuse is a goal): a front-end-agnostic view model in
components/view turns source, valid or not, into styled segments --
decorated Unicode text, a semantic class per token (built-in, user
function, variable, lambda argument, number, string, symbol, axis,
exponent, namespace, comment, punctuation, error), and a map between
raw byte offsets and rendered columns. Terminal widgets in
components/tui (ratatui + crossterm) draw it: an ASCII source pane, a
rendered pane, an output pane and an array viewer. Apps: `xetal edit
FILE` and the live `xetal repl`. The stepping debugger (Saga 12) and
the web playground (Saga 13) reuse the view model and widgets.

Decisions made with the user:
- The REPL renders the line live as it is typed when on a terminal
  (raw-mode line editor with history); piped input keeps today's plain
  behavior, so goldens are unchanged.
- `xetal edit FILE`: nano-like keys (Ctrl-S save, Ctrl-Q quit with an
  unsaved-changes confirmation, Ctrl-R run); left pane ASCII, right
  pane live decorated and highlighted, cursor line synced; bottom pane
  shows types and diagnostics live, results only on Ctrl-R (effects
  such as p_rint! and r_oll! never run on a keystroke).
- `_r` / `_l` render as subscript r and l (U+1D63, U+2097) and
  round-trip back.
- ratatui + crossterm; screens tested with TestBackend buffer
  snapshots and scripted key sequences; the terminal is always
  restored (also on panic).

Rules for every step: strict TDD, tests are the spec, no panics on
any input (rendering tolerates invalid source), stricter gates and
expand up and out, docs ASCII-only (name glyphs, do not paste them
into docs/*.md), gate before commit, commit .agentrail with the work,
push to origin/main.

## Steps

1. lambda-glyphs -- `_r` / `_l` decorate as subscript r / l and
   round-trip through `xetal render --raw`; render spec cases, goldens
   rebased on purpose; record the decision (lang-choices I3, design).
2. view-model -- components/view: tolerant, token-wise rendering into
   styled segments with classes and a raw <-> rendered span map (never
   fails; invalid tokens are kept raw and marked as errors); `xetal
   render --color` prints ANSI-highlighted decorated text (golden).
3. tui-foundation -- components/tui: text buffer with cursor and
   edits, the nano keymap as data, source and rendered pane widgets
   with highlighting and synced scrolling; TestBackend snapshot tests
   and scripted key tests; terminal guard.
4. editor -- `xetal edit FILE`: split panes, live types and
   diagnostics (error spans highlighted in both panes), Ctrl-R runs
   and shows output, save, quit confirmation, new file.
5. array-view -- an array viewer widget for results (matrices as
   grids, rank 3 as slices, strings, scalars, long arrays scrolled),
   used by the editor's output pane and designed for the debugger.
6. repl-live -- `xetal repl` on a terminal: a raw-mode line editor
   rendering as you type, history (Up/Down), multi-line continuation,
   Ctrl-D; piped input unchanged.
7. tui-docs-release -- README tour (editor and live REPL, with
   screenshots as text from snapshot tests), docs sync, debugger
   design notes in docs/design.md, Saga 7 retrospective.
