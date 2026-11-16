# Debugging X_eTaL programs

`xetal dap` is a debugger for X_eTaL programs that any editor speaking
the Debug Adapter Protocol (DAP) can drive: VS Code, Emacs (dape or
dap-mode) and Neovim (nvim-dap). It stops at statements, keeps
breakpoints by file and line (with a condition, a hit count, or both),
shows the call stack, each scope's variables (an array opens a page of
rows at a time) and watches, and travels back in time: **Step Back**
goes to the statement before, **Reverse Continue** to the breakpoint
hit before, and stepping forward again reaches the same states.

The editor starts `xetal dap` and talks to it on its standard input and
output. A launch configuration names the file and, optionally:

| Field | Means |
| ----- | ----- |
| `program` | the `.xtl` file to debug |
| `seed` | the seed `r_oll!` draws from (the same seed, the same run) |
| `stopOnEntry` | stop before the first statement (otherwise run to the first breakpoint) |
| `input` | the lines the program reads with `[]R_EAD`, in order |

Output the program prints appears in the editor's debug console, once:
stepping back and forward again does not print it twice. A file the
program wrote stays written when you step back over the write; the
console says which.

## VS Code

The extension is in `editors/vscode`:

```bash
cd editors/vscode
npx @vscode/vsce package
code --install-extension xetal-0.1.0.vsix
```

With `xetal` on the PATH (or the setting `xetal.path`), open an `.xtl`
file and press F5. A `launch.json` entry:

```json
{
  "type": "xetal",
  "request": "launch",
  "name": "Debug this X_eTaL file",
  "program": "${file}",
  "seed": 1
}
```

Step Back and Reverse Continue appear in the debug toolbar.

## Emacs

With [dape](https://github.com/svaante/dape) (Emacs 29 or later) and
`xetal-mode` (`docs/emacs/`):

```elisp
(with-eval-after-load 'dape
  (add-to-list 'dape-configs
               '(xetal
                 modes (xetal-mode)
                 command "xetal"
                 command-args ("dap")
                 :type "xetal"
                 :request "launch"
                 :program dape-buffer-default
                 :seed 1)))
```

Then `M-x dape RET xetal RET`. dape's `dape-step-back` and
`dape-reverse-continue` travel back (it shows them because the adapter
says it supports stepping back).

With [dap-mode](https://github.com/emacs-lsp/dap-mode):

```elisp
(require 'dap-mode)
(dap-register-debug-provider
 "xetal"
 (lambda (conf)
   (plist-put conf :dap-server-path '("xetal" "dap"))
   conf))
(dap-register-debug-template
 "X_eTaL: this file"
 (list :type "xetal" :request "launch" :name "X_eTaL"
       :program nil :seed 1))
```

`:program nil` takes the current buffer's file in dap-mode's templates;
give a path to debug another.

## Neovim

With [nvim-dap](https://github.com/mfussenegger/nvim-dap):

```lua
local dap = require("dap")
dap.adapters.xetal = { type = "executable", command = "xetal", args = { "dap" } }
dap.configurations.xetal = {
  { type = "xetal", request = "launch", name = "Debug this file",
    program = "${file}", seed = 1 },
}
vim.filetype.add({ extension = { xtl = "xetal", xtlm = "xetal" } })
```

`:lua require("dap").step_back()` and `require("dap").reverse_continue()`
travel back.

## What it answers

`initialize`, `launch`, `setBreakpoints` (with `condition` and
`hitCondition`: `3` or `>= 3`, `== 3`, `> 3`, `% 3`), `configurationDone`,
`threads` (one), `stackTrace`, `scopes` (Locals, Top level, Libraries),
`variables` (paged with `start` and `count`), `evaluate` (watches and
hovers; a watch may not have effects), `next`, `stepIn`, `stepOut`,
`continue`, `stepBack`, `reverseContinue`, `pause` and `disconnect`; and
`xetal/runTo` with a `step` (the transition count), to go to any point
of the run. Events: `initialized`, `stopped`, `output`, `exited`,
`terminated`.

`xetal dap --lines` reads and writes one JSON message a line instead of
the protocol's `Content-Length` headers, which makes a session easy to
write down: the golden `dap-session` is one (its requests are
`reg/fixtures/dap/session.jsonl`).
