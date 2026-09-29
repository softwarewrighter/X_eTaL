# A tour of M6b

Part of the X_eTaL milestone tours ([index](tour.md), [README](../README.md)); previous: [A tour of M5b](tour-m5b.md).
Commands run from the repository root after `scripts/build-all.sh --release`
(or use `./target/debug/xetal` after `just build`).

**M6b -- libraries.** A library is an ordinary X_eTaL file of
definitions. Its exported names start with `l:`; its other top-level
names (functions written without a namespace, plain variables) are
private to it. A program imports a library with the `u_se<` macro,
giving it an alias of its choosing, and uses the exports through that
alias:

```
# demos/stats.xtl
"s:" u_se< "Stats"

v := 2 4 4 4 5 5 7 9
s:m_ean v                # 5.0
s:v_ariance v            # the mean squared deviation: 4.0
s:s_d v                  # the standard deviation: 2.0
s:r_ange v               # largest minus smallest: 7
s:m_ean 1.5 2.5          # the same functions work on Floats
```

```bash
./target/release/xetal run demos/stats.xtl
```

prints `5.0`, `4.0`, `2.0`, `7` and `2.0`. `Stats` is a standard
library, built into `xetal` from `lib/Stats.xtl`:

```
l:m_ean := { (f_loat '+ r_/ _r) / f_loat t_ally _r }  # Int or Float numbers

s_quare := { _r * _r }                  # private: not visible outside
d_eviations := { v -> (f_loat v) - l:m_ean v }

l:v_ariance := { v -> l:m_ean s_quare d_eviations v }
l:s_d := { (l:v_ariance _r) ^ 0.5 }     # the standard deviation
l:r_ange := { v -> ('m_ax r_/ v) - 'm_in r_/ v }
```

A name `"Name"` is found as `Name.xtl` beside the importing file, then
in each directory of `XETAL_PATH`, then among the standard libraries;
a string with a `/` or ending in `.xtl` is a path relative to the
importing file. A library is loaded once however many files import
it, each file may use its own alias for it, and a library's own
imports stay private to it.

`xetal type` lists the program's own items; a private name is not
visible, and the error lists what the library does export:

```bash
./target/release/xetal type demos/stats.xtl
./target/release/xetal eval -e '"s:" u_se< "Stats"; s:s_quare 3'
```

The second reports `error[not-exported]: s:s_quare is not defined by
that library; it defines m_ean, v_ariance, s_d, r_ange`. An error
inside a library is reported in that file, at its line and column:

```bash
./target/release/xetal run reg/fixtures/use-oops.xtl
```

reports `error[missing-argument]: ... at reg/fixtures/Oops.xtl:3:20`.
Every other mistake an import can make (a missing, malformed or
reserved alias, a library not found, an import cycle, one alias for
two libraries, a top-level expression in a library, and so on) has an
error of its own; `docs/lang-choices.md` section 14 lists them.

In the pretty views (`just pp`, `just show`, the editor) an import is
drawn as its alias bound to the macro: `"s:" u_se< "Stats"` shows as a
superscript s and a superscript equals before `u_se<`, and the
library's names as a superscript s before the function.
