# Private names: the `h:` namespace

Status: decided with the user on 2026-10-07; to be implemented by a
saga of its own. Its first step records the rules below in
`docs/lang-choices.md` (section 14, beside MC8 and MC9) and the
decisions register in `docs/design.md`; until then this document is
the plan.

## Summary

- `h:` ("hidden", "helper") is a new namespace for file-private
  definitions. It is legal at the top level of apps and libraries
  alike, and it is never part of what a file offers: a library never
  exports it, and an app's interface (what `xetal type` lists and
  `xetal doc` documents) leaves it out. An app's `u:` names are its
  interface.
- A bare (unprefixed) top-level function in a library is deprecated:
  a warning now, an error after one release. Its replacement is the
  same name under `h:`.
- Importing an `.xtl` file that defines no `l:` name is an error: a
  library must export something.
- When the deprecation has run its course, every bare top-level
  function name refers to a built-in, so adding a built-in can never
  be shadowed by a definition at the top level of a file. Only lambda
  locals can shadow one, and those already warn (L7).

## How `.xtl` files are handled today

A file has no declared role. Running it treats it as an app (unless it
names an `l:`, in which case `xetal run` prints the types of its
exports); importing it with `u_se<` treats it as a library. Checked
with the binary on 2026-10-07:

| File | `xetal run FILE` | `"m:" u_se< "FILE"` |
| ---- | ---------------- | ------------------- |
| app: `u:` functions and expressions | runs | error: a top-level expression in a library (MC8 row 16) |
| app plus a bare top-level `h_alf` | error `bad-binding`: "write u:h_alf" | as above |
| bare top-level functions only | error `bad-binding`: "write u:h_alf" | imports, and exports nothing; `m:h_alf` is `not-exported` ("it defines" and an empty list) |
| `l:`, `u:` and bare functions | error `user-name-in-library` (MC8 row 8) | the same |
| library: `l:` and bare functions | prints the exports' types | works; the bare names stay private (MC9) |

## The problems

1. The same spelling means opposite things. A bare top-level `h_alf`
   is private in a library and illegal in an app, and the app's error
   ("write `u:h_alf`") is the wrong advice for someone writing a
   library.
2. Bare names are the one place a definition at the top level of a
   file can collide with a built-in. An app cannot have them; a
   library can, and its definition wins inside the file. L7 promises
   a warning there, but none is given for a library's private
   functions today. `lib/Svg.xtl` defines a private `j_oin`; a `j_oin`
   built-in (a plausible one: the Saga 30 measurements point at
   joining boxed texts) would be quietly hidden inside Svg.
3. A file of bare functions only is an error when run and a library
   that exports nothing when imported: the mistake is silent on one
   side.
4. An app has no way to write a top-level helper; every top-level
   function must be `u:`, so "the program's" and "a helper" look the
   same.

## The rules

Identifiers for `docs/lang-choices.md`: PN1 to PN7 ("private names").

| #   | Rule |
| --- | ---- |
| PN1 | `h:` is a namespace for file-private definitions, functions and variables: `h:j_oin := { b -> ... }`, `h:limit := 8`. It is legal at the top level of an app and of a library. An `h:` name is visible in its own file only: it is never exported, an importer cannot name it, and two files' `h:` names never meet. `h:` joins `u:` and `l:` as a reserved alias (MC8 row 5). It is a namespace like any other (MC13), so the lexer needs no change; the display decorates it the way it decorates every prefix. |
| PN2 | A bare top-level function in a library is deprecated: it still works and is private, with a warning `deprecated-private` ("write `h:j_oin`; bare top-level functions in a library are deprecated"). The repository's gate treats the warning as an error, so its own libraries migrate at once. After one release the warning becomes an error (a new MC8 row). |
| PN3 | A bare top-level function in an app stays an error; its message names both choices: "write `u:h_alf` (the program's) or `h:h_alf` (a helper)". |
| PN4 | Importing an `.xtl` file that defines no `l:` name, function or variable, is an error (a new MC8 row): "library Name exports nothing; mark its exports with `l:`". A library's own imports do not count; they are private to it (MC6). |
| PN5 | A bare top-level variable in a library stays private, with no warning: built-ins are functions or quad names, so a variable cannot collide with one. `h:` is allowed for variables too, and recommended where it makes a library's private state easier to see. |
| PN6 | Unchanged: local functions inside a lambda are bare and scoped to it (`c_ap := { ... }`), and one named like a built-in shadows it with the L7 warning. |
| PN7 | What a file offers, its interface, is its exports for a library (`l:` names) and its `u:` names and bare variables for an app; `h:` names are never part of it. The interface is what `xetal type FILE` lists, what `xetal doc` documents, and what anything that later loads a program (a notebook, a REPL session started from the file) exposes. Inside the file, `u:`, `l:` and `h:` names are all visible alike. So in an app `h:` has a testable effect today: an `h:` helper runs exactly as a `u:` function would, and does not appear in `xetal type app.xtl`. An app with no `u:` names (only `h:` helpers, or only expressions) is legitimate: a script with no interface. |

With PN2 at its error stage, a bare function name at the top level of
any file can only mean a built-in. "New built-ins never break existing
programs" (L7) then holds without depending on a warning being read.

In an app, `u:` and `h:` differ by PN7 only: both are visible
throughout the file and nothing imports an app, but only the `u:`
names are the app's interface. That gives the two prefixes one meaning
in both kinds of file (`h:` is never part of what a file offers) and
makes the difference something a test can pin: today `xetal type
app.xtl` lists every `u:` function, every top-level variable and the
type of the last expression; under PN7 it omits the `h:` names. The
alternatives were to accept `u:` and `h:` as synonyms in apps (two
spellings of one thing, a convention the gate cannot check) or to
allow `h:` in libraries only (one way per kind of file, but app
helpers stay indistinguishable from the app's own functions); the
user chose PN7 (2026-10-07).

## Errors and warnings

| Kind | Condition | Example | Message |
| ---- | --------- | ------- | ------- |
| MC8 row 5 (widened) | `h:` used as an alias | `"h:" u_se< "Svg"` | reserved alias |
| new MC8 row | `h:` name used from another file | `m:j_oin` where Svg's `j_oin` is `h:` | not defined by that library (as row 11) |
| new MC8 row (PN4) | a library that exports nothing | `u_se<` of a file with only `h:` definitions | library Name exports nothing; mark its exports with `l:` |
| warning (PN2), later an MC8 row | bare top-level function in a library | `j_oin := ...` in `lib/Svg.xtl` | write `h:j_oin`; bare top-level functions in a library are deprecated |
| `bad-binding` (PN3, message changed) | bare top-level function in an app | `h_alf := ...` in a program | write `u:h_alf` (the program's) or `h:h_alf` (a helper) |

Row 12 (a function defined twice at top level) applies across `u:`,
`l:` and `h:` alike: `h:f_` and `l:f_` in one file are two names,
not a redefinition, but a reader would be confused; the saga decides
whether to warn.

## Where else `h:` must be understood

- The macro phase's hidden renaming of library instances (MC6): an
  `h:` name is renamed with the library's other internals, as private
  names are today, and macro hygiene (MC30) treats it the same way.
- `xetal type FILE` and `xetal doc` (PN7): on a library (CB4) they
  list `l:` exports only; on an app they list `u:` names and bare
  variables and omit `h:` names. Today they already omit a library's
  bare private names, and list everything at the top of an app.
- The renderers (decorated, canonical, expanded), the Emacs mode
  (`docs/emacs/`), the syntax poster and the language reference show
  `h:` as they show the other prefixes.
- The REPL and notebooks: a cell should be able to define `h:` names,
  private to the session as `u:` names are.
- `scripts/check-modes.sh`, which tells libraries beside a demo from
  programs, keeps working (it goes by the capitalized file name).

## The migration

Bare top-level functions in libraries (2026-10-07; the sibling
repositories counted from their latest local clones):

| Where | Library files | Bare private functions |
| ----- | ------------- | ---------------------- |
| `lib/` (Geometry3D, Stats, Svg, TTTML, Turtle) | 5 | 13 |
| `userlibs/Greetings.xtl` | 1 | 1 |
| `demos/rosetta/Stone.xtl` | 1 | 1 |
| X_eTaL-libraries | 15 | 45 |
| X_eTaL-extensions | 5 | 21 |
| X_eTaL-games | 2 | 2 |
| X_eTaL-ML | 1 | 2 |
| X_eTaL-demos | 0 | 0 |

In this repository the names are: Geometry3D `r_adians`; Stats
`s_quare`, `d_eviations`; Svg `e_sc`, `j_oin`, `p_airs`; TTTML
`r_esult`, `t_arget`, `b_ack`, `m_eet`, `g_reedy`, `f_inal`; Turtle
`r_adians`; Greetings `n_ame`; Stone `j_oin`. (Comparison's `rows`,
`cols` and Stone's `zoom` are variables, untouched by PN2.)

The rewrite should be done by the compiler, not by text substitution:
a lambda may have a local of the same name as a top-level helper, and
only the references that resolve to the top-level binding change. A
subcommand such as `xetal migrate FILE` (or `xetal fmt --migrate`)
that parses, renames each bare top-level function `f_` to `h:f_` and
its resolved uses, and prints the file back through the formatter
(rule 10: the parse is unchanged apart from the renamed names) does
it for this repository and for the sibling repositories alike, which
vendor X_eTaL and can run the same command.

## Saga outline

Each step is its own PR from main, test-first, with the gate.

1. **Decisions.** PN1 to PN7 in `docs/lang-choices.md` section 14, the
   new and widened MC8 rows, the decisions register in
   `docs/design.md`; spec cases for each rule written as `STATUS
   pending`.
2. **The namespace.** `h:` definitions at the top level of apps and
   libraries; private to the file; reserved as an alias; renamed with
   the library instance; left out of every interface (PN7: `xetal
   type` and `xetal doc` on apps and libraries); shown by the
   renderers and the Emacs mode. The PN1 and PN7 spec cases and
   goldens flip to active.
3. **Empty libraries.** PN4's error, with a golden for its message.
4. **Deprecation and migration.** PN2's warning and PN3's new message;
   the migrate subcommand with goldens (a file with a lambda-local of
   the same name stays correct); `lib/`, `userlibs/` and `demos/`
   migrated; the gate treats `deprecated-private` as an error.
5. **Documents.** The README tour, the language reference,
   `docs/literate/libraries.org` and `hello.org` write helpers as
   `h:`; a note in `docs/asks.md` (or each sibling's HANDOFF) for the
   sibling repositories with the migrate command.
6. **Later, after one release.** PN2's warning becomes an MC8 error.

Tests to have by the end: an app with `h:` helpers runs, and
`xetal type` on it lists its `u:` names and not its `h:` names (a
golden), as `xetal doc --json` does; an app with only `h:` helpers
runs and has an empty interface; a library's
`h:` names are unreachable from an importer (row 11); `"h:" u_se<`
is rejected; a file with only `h:` names runs as an app and fails as
an import (PN4); a library's bare top-level function warns (and the
gate fails on it); an app's bare top-level function gives the new
message; a bare top-level variable in a library does not warn (PN5);
the migrate subcommand leaves a same-named lambda local alone; the
renderers round-trip `h:` (rule 10).

## Alternatives considered

- **Doing nothing**, or only the two cheap fixes (PN4, and a warning
  when a library's bare name matches a built-in). No migration and no
  new syntax, but problem 1 stays, apps still have no top-level
  helpers, and collisions with future built-ins are caught by a
  warning rather than made impossible.
- **A file type for libraries, `.xtll`**, beside `.xtlm` for macro
  libraries. The role would be declared by the file name, but `l:`
  already declares it, and the change does not touch problems 1 or 2:
  bare names would still shadow built-ins. It costs the most
  migration (every library renamed, `u_se<` taught a third extension,
  the sibling repositories' vendored copies) for the least gain.
- **`p:` for "private"**: rejected for the letter only, since `p:`
  reads as "public" about as often as "private".

## Open questions for the saga

- How long PN2 stays a warning. Proposed: one release.
- Whether `h:` should be required for a library's private variables
  as well (PN5 says no: there is nothing for them to collide with).
- Whether `h:f_` and `l:f_` (or `u:f_`) in one file should warn.
