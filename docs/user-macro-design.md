# Macros defined in the program itself: a design, deferred

Status: a design recorded with the user on 2026-10-21, not planned
(no saga step). Today a macro lives in a `.xtlm` file under `m:`
(MC10) and a program only calls what it imports; pasting `m:`
definitions into the editor turns the buffer into a macro library,
so nothing can import it (`error[unknown-macro] ... main.xtlm`). The
workaround is Save as `Name.xtlm` and `"n:" u_se< "Name"` in a second
buffer, in the live demo and the terminal alike.

## The proposal

A program may define its own macros beside its functions:

```
u:c_ond< := { @ arms -> "{ @ -> " c_at (h:g_uards arms) c_at " } @" }
h:g_uards := { t -> ... }
x! := 9
@ u:c_ond< "x! < 0: -1; x! = 0: 0; 1"
```

- Spelling: `u:name< := { ... }`, the user namespace with the macro
  mark, called as `u:name<`. `u:` keeps its one meaning (your names);
  the `<` tells a macro from a function, as it does for `m:` today.
  Helpers the macro uses are private `h:` bindings, as in a `.xtlm`.
- Order in a file: the definitions are collected first, so a call may
  precede its definition, as imports may. At the REPL a macro is
  defined before it is used, since lines arrive one at a time and the
  session keeps the definitions.
- Expansion: the program's `u:…<` definitions and the `h:` helpers
  they use form a macro library of their own for that file, expanded
  exactly as an imported `.xtlm` is (MC23: each call appended, the
  whole type-checked and run, the text given is code at the call).
  Those definitions are stripped from the program text so they do not
  run as statements; an `h:` helper no macro uses is `unused-binding`.
- Unchanged: hygiene (`[]G_ENSYM`, `## binds:`), the contract lines
  (`## takes:`, `## uses:`), the import-cycle rule, and that a system
  macro cannot be redefined. A macro whose expansion calls itself is
  caught by the existing expansion depth limit.

## What it touches

- xetal-macro (the expander, about 600 lines): collecting the
  definitions, the on-the-fly library, stripping; perhaps 300 lines
  with tests. The real work.
- The REPL session: keeps the definitions and re-expands each line
  against them.
- Tooling that reads macro definitions, each a small change: the
  contract check (xetal-contract), the macro lints (xetal-macrolint),
  `xetal type` and `xetal doc`, the Emacs mode, the syntax poster,
  completion (xetal-complete offers `u:…<`).
- Rules: a lang-choices MC entry and a design decision; a spec case
  per rule and a rejection case (a `u:` macro called before any
  definition at the REPL, a `u:` macro redefining a system one).

Estimate: one to two sessions. Risk low; the gain is prototyping a
macro in the REPL or the live demo without a second buffer, and
single-file demos.

## What macros can already do at expansion time

A `.xtlm` may import libraries (`"s:" u_se< "Stats"` at its top; its
own imports are private to it, MC6) and its macros are whole X_eTaL
programs run by the evaluator when the call is expanded: they may
compute (`f_ormat s:m_ean n_umbers t`), read the text as data (`##
uses: parsed`), and perform effects (`t []N_PUT "note.txt"` writes a
file on the machine compiling the program, while it is expanded). The only limits are those of MC23: the text
a macro gives is the code at the call, a macro's expansion may not
import (`macro-import`), what it prints is taken as its text (so a
macro should not print), and errors inside it are reported at the
library's own file and line. Compile-time work is therefore ordinary
X_eTaL, with the system hooks (`[]R_EJECT`, `[]S_TATEMENT`,
`[]G_ENSYM`, MC20) for what only the compiler knows.
