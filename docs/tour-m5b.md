# A tour of M5b

Part of the X_eTaL milestone tours ([index](tour.md), [README](../README.md)); previous: [A tour of M5](tour-m5.md).
Commands run from the repository root after `scripts/build-all.sh --release`
(or use `./target/debug/xetal` after `just build`).

**M5b -- seeing the decorated form.** Source stays plain ASCII; every
display draws it decorated and highlighted by one view model: system
functions blue (symbols light blue), your `u:` functions green,
library functions cyan, macros bold yellow, the lambda arguments `_l`
and `_r` as APL's alpha and omega, and a quote in the color of the
function it quotes. Comments keep their column, and code in backquotes
inside a comment is drawn too.

```bash
./target/release/xetal render --color -e 'u:s_quare := { _r * _r } # sq'   # like cat: just pp FILE
./target/release/xetal run --echo demos/factorial.xtl                     # each statement, then its output
```

`xetal run --echo` (`just show FILE`, and `just tour` for
`demos/tour.xtl`, a commented tour of every feature) prints each
statement decorated with its output indented below it; an error shows
under its statement and the run continues.

`xetal edit FILE` (`just edit FILE`) is a full-screen editor: the
ASCII you type on the left, its decorated form on the right, the types
of the top-level items (or the first error, marked in both panes)
below as you type, and on Ctrl-R the results, laid out with their type
and shape (matrices as grids). Keys: Ctrl-S save, Ctrl-R run, Ctrl-Q
quit (asks again with unsaved changes); Emacs and arrow motions
(Ctrl-A/E line start and end, Ctrl-B/F, Ctrl-P/N); Tab and Shift-Tab
move between the ASCII, Rendered and Output panes, where the motion
keys scroll that pane.

On a terminal, `xetal repl` (`just repl`) draws the line decorated as
you type it; Up and Down recall earlier lines and Ctrl-D ends the
session. Piped input is read plainly, as before.
