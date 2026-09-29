# libraries

Saga 8 of X_eTaL (see docs/plan.md), milestone M6b: the macro phase
and libraries (lang-choices section 14, MC1-MC9). `"c:" u_se<
"Combinators"` inlines a library with its `l:` names seen as `c:`
names in the importing file. The combinators (Saga 9) are then written
directly as the first real library.

Decisions made with the user when the saga was planned (record them in
lang-choices section 14 and the design.md register in step 1):
- Resolution: a string with a `/` or ending in `.xtl` is a file path
  relative to the importing file; a name `Name` is `Name.xtl` looked
  for in the importing file's directory, then each directory of
  XETAL_PATH, then the standard libraries in the repo's `lib/`, which
  are compiled into the binary so an installed xetal has them.
- A library holds definitions and bindings (and its own imports)
  only: a top-level expression in a library is a macro-phase error, so
  importing never prints.
- In a library a top-level function without a namespace is private to
  the file (`h_elper := {...}`; shadowing a built-in warns, L7);
  exports are `l:f_` and `l:x`; plain top-level variables are private
  too. Programs keep requiring `u:` for their functions.
- Display (pretty views only; `xetal render` keeps the tokens):
  `"c:" u_se< "Name"` draws as superscript c and superscript equals
  (U+207C) in the library color, `u_se<` underlined in bold yellow,
  then the name in string color. Library names are capitalized by
  convention (style guide).

Architecture: a new component (components/macro) between the lexer
and the parser (MC1). Files keep their own spans: the expanded program
is one combined text with a source map, so every later stage works on
byte offsets as today and a diagnostic is reported in the file and at
the place it was written. Each library instance gets a hidden
uppercase namespace (lowercase aliases only, so users cannot write
one); names in messages are shown with the letters written in the file
being read (MC6).

Rules for every step: strict TDD, every macro-phase error row has a
rejection test, no panics, stricter gates, expand up and out, docs
ASCII-only, gate before commit, commit .agentrail with the work, push.

## Steps

1. sources -- a multi-file source map (files, combined text,
   locating an offset to file, line and column); diagnostics name the
   file when more than one is involved (single-file output unchanged);
   record the saga decisions.
2. imports -- the macro phase: find top-level `u_se<` statements,
   validate them (MC8 rows 3, 4, 5, 13, 14, 15), resolve libraries
   (directory, XETAL_PATH, bundled lib/), load them recursively with
   cycle detection (rows 1, 2), one instance per resolved path (MC7).
3. namespaces -- per-file alias tables to hidden namespaces; `l:`
   exports, private functions and variables (MC5, MC6, MC9, rows 6-12,
   and the new row for top-level expressions in a library); core
   accepts definitions in hidden namespaces; messages use the letters
   written in the file.
4. first-library -- a bundled standard library (lib/Stats.xtl) and a
   demo using it; the tour's libraries section; the import display in
   pp / show / the editor; goldens.
5. tools -- run, eval, --echo, the REPL and the editor all expand
   imports (paths relative to the file; -e and the REPL relative to
   the working directory).
6. libs-docs-release -- README M6b tour (every command a golden), docs
   sync, Saga 8 retrospective.
