# A time-traveling debugger (plan)

The user (2026-11-04): a debugger that steps through a program, the
X_eTaL-ML tensor pipelines of equations included, in their expanded
code, showing intermediate results; with breakpoints, step and
continue, watches, variables to introspect, 2D and 3D views of arrays
(truncated or paged), stepping backward in time, and, if it helps, the
Debug Adapter Protocol (DAP). This document is the plan; the steps are
in `.agentrail` (Saga 43 in docs/plan.md). It supersedes the stepper
sketched in Saga 17, whose trace tree it builds on.

## 1. What exists to build on

- **The steppable evaluator (D50, `components/step`, `xetal-step`).**
  Core is evaluated by an explicit machine whose state is data: the
  control in hand (`Control::Eval(expr, env)` or `Return(value)`), a
  stack of pending frames (`Kont`), the top-level environment and the
  random number generator. `Machine::run(budget)` takes a slice of any
  size and stops between any two transitions. The CLI, the REPL, the
  live demo's worker and its Step button all run on it.
- **Values are immutable and shared** (`Rc` arrays, boxes and
  closures; a `!` variable is rebound, never mutated in place). So a
  copy of the machine's state is cheap: it shares every array with the
  live state. A checkpoint is a clone of a few vectors of pointers.
- **Spans everywhere.** Every Core node carries the span of the text
  it came from, and `xetal-sources` maps a span in macro-expanded text
  back to the call that wrote it (MC1, the diagnostics' places). The
  expansion itself is available as text (`xetal expand`).
- **Drawing arrays.** `xetal-grid` lays out a value with its type and
  shape (matrices boxed, higher ranks as slices); APL2's DISPLAY form
  is `d_isplay`; X_eTaL-ML's microscope draws rank-3 values as blocks
  of cubes. The terminal widgets (`xetal-panes`), the editor, the shell
  and the screen dump (`xetal edit --screen`) are host-free state
  machines drawn with ratatui, in the terminal and in the browser.

## 2. The design in one paragraph

One debugger core, `xetal-debug`, wraps the stepping machine: it
pauses at **stops** (the evaluation of a Core node whose span starts a
statement, or any node at the finer granularity), keeps **breakpoints**
(a line, or a span inside an expansion, with an optional condition and
hit count), answers questions about the paused state (the **call
stack**, each frame's **variables**, any expression as a **watch**),
and moves through time: step over, in and out, continue, and the same
backward. Every front end is a thin client of that core: a **DAP
adapter** (`xetal dap`), so VS Code, Emacs (dap-mode), Neovim
(nvim-dap) and JetBrains debug X_eTaL with their own windows; a
**terminal debugger** (`xetal debug FILE`); and a **debugger page** in
the live demo. Large values are paged, and an **array viewer** shows a
value as a 2D grid or a 3D block of slices, in all three.

## 3. Time travel: checkpoints and replay

A run is deterministic once its seed and its inputs are fixed (the
seed is recorded, and the lines and keys a program reads are logged as
it reads them). So the debugger numbers every transition (the step
counter) and keeps a **checkpoint**, a clone of the machine state,
every N transitions (N grows with the run, so memory stays bounded:
1,000 checkpoints cover any run). Going back to step k restores the
nearest checkpoint at or before k and runs forward silently to k, with
output and pictures muted up to the point already shown (as the REPL's
replay does today). Step back, reverse continue (back to the previous
breakpoint hit) and "run to step k" are all that one operation. A
value's **history** (every value a name or a watch had, with the step
that set it) is collected on the way.

Effects are the boundary: output already shown is not shown twice; a
file written by `[]N_PUT` is not unwritten (the debugger says so when
stepping back over one); `[]R_EAD` and `[]K_EY` replay their logged
input. This is what rr and Pernosco do for native code; here it is
cheap because the state is data and the values are immutable.

## 4. Stops, granularity and expanded code

- **Statement stops** (the default): before each top-level statement
  and before each statement in a lambda body.
- **Expression stops** (step in): before each application, so a step
  goes right to left through `a + b * c` as the language reads it,
  showing each intermediate value, the trace tree of Saga 17.
- **Inside the higher-order built-ins**: `r_/`, `e_ach`, `t_able`,
  `i_nner`, `p_ower` call their operand through kernels (D50); a step
  in enters the operand's call for each item, with the item's index
  shown ("e_ach: item 3 of 10"). Arithmetic on whole arrays is one
  step, not one per element.
- **Expanded code.** A macro call (`e:e_q<`, `flow:p_ipe<`) is
  replaced by its expansion before checking. The debugger shows the
  expansion as a source of its own (DAP's source reference, the
  terminal's and the page's "expanded" tab), with breakpoints and
  stops there; a stop inside it is also shown at the call in the
  written file. A pipeline's stages (`|>` ... `$q`) are statements of
  the expansion, so stepping a pipeline of equations stops at each
  stage with its value (the queries, the keys, the scores, the
  softmax), which is the X_eTaL-ML case the user asked for.

## 5. Variables, watches and big values

- **Scopes**: the frame's locals (a lambda's parameters and bindings),
  then the top level, then the libraries' names by alias.
- **A value's summary**: its type, shape and the first items
  (`Float 4 3: 0.42 0.45 0.07 ...`), as the Types pane shows it.
- **Paging**: an array's children are its major cells, in pages (DAP's
  `variables` with `start` and `count`); a matrix opens to rows, a row
  to items; a tuple to its parts, a box to its content.
- **Watches** are expressions evaluated in the paused frame's
  environment (checked and run like a REPL line in that scope, effects
  refused); a conditional breakpoint's condition is one.
- **Set value** is not offered: values are immutable, and a `!`
  variable changes only by the program's own binding.

## 6. Views: 2D and 3D

- **2D**: a grid of the value (a matrix as it prints, a vector as one
  row), with a window that pans and pages over large arrays, the axis
  extents and the window's position shown; colors by magnitude as an
  option (a heat map), useful for weights and attention scores.
- **3D**: a rank-3 value as slices along a chosen axis, side by side
  (text) or as a block of cubes (SVG in the browser and in DAP
  clients, as X_eTaL-ML's microscope draws them), with one slice
  highlighted and the others outlined; higher ranks by fixing all but
  three axes.
- **Where**: the terminal debugger's viewer pane; the page's viewer;
  for DAP, a custom request (`xetal/visualize`) returning SVG and text,
  which the VS Code extension opens in a panel, and a plain fallback
  that writes the SVG and links it (any client).

## 7. The Debug Adapter Protocol

DAP fits: it is JSON over standard input and output, the client draws
the windows, and its requests map onto the core one for one:
`initialize` (with `supportsStepBack`, `supportsConditionalBreakpoints`,
`supportsHitConditionalBreakpoints`, `supportsEvaluateForHovers`,
`supportsLoadedSourcesRequest`), `launch` (the file, a seed, input),
`setBreakpoints`, `configurationDone`, `threads` (one), `stackTrace`,
`scopes`, `variables` (paged), `evaluate` (watch, hover, console),
`source` (an expansion), `next`, `stepIn`, `stepOut`, `continue`,
`pause`, `stepBack`, `reverseContinue`, `goto` (to a step), and
`disconnect`; events `stopped`, `output`, `terminated`. The adapter is
`xetal dap` (stdio), a small crate over the core, tested by golden
transcripts (a sequence of requests in, the responses and events out).
A minimal VS Code extension (a `package.json` and a launch
configuration, no TypeScript logic) registers it; docs show Emacs and
Neovim configurations. The browser page does not need DAP: it calls
the core directly, as the shell calls `xetal-command`.

## 8. Steps (Saga 43), in order

1. **debug-core**: `xetal-debug` (components/debug): a session over
   `xetal-step` with stops (statement and expression), breakpoints by
   line, step over, in, out, continue, pause; the call stack with
   spans. Hooks in the machine where a node starts and a frame is
   pushed or popped, with no cost when no debugger is attached (the
   speed goldens show none). Tests drive a session over small programs.
2. **debug-variables**: scopes and variables with summaries and paging,
   watches and conditional breakpoints evaluated in a frame, value
   history.
3. **debug-time-travel**: the step counter, checkpoints, restore and
   replay with output muted and input replayed; step back, reverse
   continue, run to step k. Tests: back then forward gives the same
   states; a breakpoint hit backward is the one hit forward.
4. **debug-expansion**: expansions as sources with breakpoints, stops
   mapped both ways, a pipeline's stages as stops; tests on Equations
   and a Flow-like pipeline from X_eTaL-libraries' collection.
5. **debug-dap**: `xetal dap`, golden transcripts, the VS Code
   extension's manifest, docs for VS Code, Emacs and Neovim.
6. **debug-views**: the array viewer, 2D paged and heat-colored, 3D
   slices and cubes (SVG), with `xetal/visualize` in DAP.
7. **debug-tui**: `xetal debug FILE`: the source (ASCII and rendered)
   with the stop highlighted, the stack, variables, watches,
   breakpoints, the viewer; F5 continue, F10 over, F11 in, Shift-F11
   out, Shift-F5 back, F9 breakpoint; screen goldens through
   `--screen`, as the editor's.
8. **debug-web**: the live demo's debugger, the same core in the
   worker, the views drawn on the page; the nightly UI check steps a
   program.
9. **debug-ml**: the acceptance case: X_eTaL-ML's attention head as a
   pipeline of equations, stepped stage by stage in VS Code, the
   terminal and the page, each intermediate tensor shown; a literate
   walk-through.

DAP comes before the terminal and page front ends because the clients
give breakpoints, watches, the stack and stepping their windows for
free, so the core is usable, and tested, early.

## 9. Decisions (made with the user, 2026-11-05)

The user took the recommendation on each:

- **Granularity**: statement stops by default; step in stops at each
  application (expression stops), and inside the higher-order
  built-ins at each operand call.
- **Time travel**: checkpoints and replay (section 3), not a full
  trace of every transition.
- **Front end order**: DAP first (`xetal dap`, step debug-dap), then
  the views, the terminal debugger and the live demo's page.
- **The VS Code extension**: published to the Visual Studio
  Marketplace. The repository holds the extension (`editors/vscode`)
  and a packaging recipe (`vsce package`); publishing uses the user's
  publisher account, so the user runs the publish step.
- **Effects when stepping back**: say so and go on. Output already
  shown is not shown again; a file written by `[]N_PUT` stays written,
  and the debugger names it when stepping back over it.
