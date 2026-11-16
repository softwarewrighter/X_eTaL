# X_eTaL for VS Code

Debug X_eTaL programs: breakpoints (with conditions and hit counts),
the call stack, variables (an array a page at a time), watches, and
stepping back in time (Step Back, Reverse Continue in the debug
toolbar). The adapter is `xetal dap`; see
[docs/debugging.md](../../docs/debugging.md).

## Install from this folder

```bash
cd editors/vscode
npx @vscode/vsce package        # makes xetal-0.1.0.vsix
code --install-extension xetal-0.1.0.vsix
```

`xetal` must be on the PATH, or set `xetal.path` in the settings.
Open an `.xtl` file, then Run > Start Debugging (F5).
