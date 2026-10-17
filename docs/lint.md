# xetal lint: a clippy-style linter

`xetal lint` checks a program or library against a catalog of named
lints, as `cargo clippy` checks a Rust crate. This document is the
design: what it does, how it is built, why, and where it goes next.
User-facing usage is in the README; decisions are D137 to D145 in
`docs/design.md` and S12 in `docs/lang-choices.md`.

## 1. What it does

```
xetal lint FILE                     # report findings (stderr); exit 0 unless one is denied
xetal lint -e 'Count! := 0'         # lint text given inline
xetal lint -D style FILE            # style findings are errors: exit 1
xetal lint -W pedantic -A shadows-builtin FILE
xetal lint --list                   # every lint: group, default level, summary
xetal lint --explain mutable-constant
xetal lint --fix FILE               # rewrite FILE with the sure fixes, report what is left
xetal lint --fix --allow-dirty FILE # the same for a file with uncommitted changes
xetal lint --format json FILE       # one JSON object a line on stdout, for editors
```

A finding is printed as the rest of the toolchain prints diagnostics,
with its place in the file as written, and each lint's first finding
carries a note naming its group and how to read more:

```
warning[mutable-constant]: `Count!` is mutable but named like a constant: write `count!` at style.xtl:4:11
  note: `mutable-constant` is in group `style`; xetal lint --explain mutable-constant
1 warning(s), 0 error(s)
```

A library is loaded as it is when imported (its bare top-level
functions hidden in its private namespace), so the library-only lints
fire when the library itself is linted. Findings in imported libraries
are not reported: lint a library by naming it, as clippy does not lint
dependencies.

### Fixes and JSON

A lint may give a fix, as rustc gives a machine-applicable suggestion:
a span and the text to write there (`Diagnostic::with_fix`). The driver
makes it an edit to the file as written only when that is sure: the
span lies wholly in the linted file, as written (not a macro's
expansion, not an import), and holds there what the pass saw (the
same text the lowered program was made from). `redundant-lambda` gives
one: the lambda becomes the name, its quote stays (`'{ x -> n_eg x }` is
`'n_eg`). `unused-binding` gives one for a local `:=`: its name
becomes `_` (`shown := p_rint! v` is `_ := p_rint! v`, the value kept
for its effect); an unused parameter has none, since a caller passes
it. A rename is one fix of several edits, made together or not at
all: `mutable-constant` renames the variable (`Count!` to `count!`),
`missing-bang` and `needless-bang` the function (`u:s_ay` to
`u:s_ay!`), at every binding and use in the file. There is none when
the new name is already in use (it would be captured), when a binding
or use is not written as the name (a macro made it), or for a
library's exported function, whose callers are in other files (only
`u:` and `h:` functions are renamed).

`--fix` makes the edits of the reported findings (an allowed finding
is not fixed), earliest first, leaving out a fix with an edit that
overlaps one already kept, writes the file only when the result still loads, prints
`fixed N finding(s) in FILE` on stderr, and reports what is left as
usual. It needs a file: with `-e` it is `error[fix-needs-file]`. It
rewrites only a file git could give back as it was (tracked and
committed as it is, as `cargo fix` asks): a file changed since its
commit, untracked, ignored or outside a repository is
`error[fix-dirty]` and left alone, unless `--allow-dirty` is given.

`--format json` writes each finding as one JSON object a line on
stdout (as cargo's `--message-format=json`), nothing on stderr, with
the same exit status:

```
{"lint":"redundant-lambda","group":"complexity","level":"warn","file":"t.xtl","line":1,"col":17,"end_line":1,"end_col":32,"message":"...","fix":[{"start":16,"end":31,"replacement":"n_eg"}]}
```

`line` and `col` count from 1 (columns in characters); `end_line` and
`end_col` are just past the finding's last character (as an editor
marks a range), its start when it ends outside the file as written;
`fix` is the list of a fix's edits (empty without one), each with
`start` and `end` as byte offsets in the file.
The live demo needs no JSON: it calls the same Rust functions.

## 2. Vocabulary (borrowed from clippy and rustc)

| Term | Meaning |
| ---- | ------- |
| lint | A named check with a stable kebab-case name, which is also the diagnostic's code (`warning[function-name-case]`). |
| group | What a lint is about: `correctness`, `suspicious`, `complexity`, `style`, `pedantic`, `deprecated`. A group name selects all its lints. |
| level | `allow` (not reported), `warn` (reported, exit 0) or `deny` (reported as an error; the command fails with `error[lint-denied]`). |
| default level | The level a lint has unless a rule changes it; `pedantic` lints default to `allow`. |
| rule | A level set for a lint, a group or `all`, from the command line or a `# lint:` comment, over some lines. |

## 3. The catalog

| Lint | Group | Default | Checks |
| ---- | ----- | ------- | ------ |
| `shadows-builtin` | suspicious | warn | a parameter or local binding shadows a built-in (L7; also printed by `xetal run`) |
| `deprecated-private` | deprecated | warn | a library's bare top-level function (PN2; also printed by `xetal run`) |
| `unknown-lint` | suspicious | warn | a `# lint:` comment with a bad level, no names, or an unknown name |
| `unused-binding` | suspicious | warn | a lambda parameter or local `:=` nothing uses; `_ :=`, `_r`/`_l` and top-level names are exempt |
| `redundant-lambda` | complexity | warn | a lambda that only passes its arguments to a named function (`'{ x -> n_eg x }` is `'n_eg`) or returns its argument (`'i_d`); a definition's whole value is exempt, and so is power (`{ a b -> a ^ b }`: `'^` is an exponent, so power has no quoted form) |
| `mutable-constant` | style | warn | a mutable variable named like a constant (`Count!`) |
| `missing-bang` | style | warn | a function calling an effect (`p_rint!`, a `!` function) directly, without `!` in its name |
| `needless-bang` | pedantic | allow | a `!` function that could not have an effect |
| `function-name-case` | pedantic | allow | a function named UpperCamel (`u:N_extGen`); a capital start is for combinators |
| `int-condition` | suspicious | warn | a guard whose condition type-checks as an Int (`t_ally v ? ...`), which fails at run time unless it is 1 or 0; runs after type checking |
| `upper-case-name` | pedantic | allow | a variable of two or more letters, all capitals (`MAXSIZE`) |
| `gensym-unused` | suspicious | warn | a `[]G_ENSYM` result thrown away or bound to a name nothing reads; reported instead of `unused-binding` there |
| `gensym-not-name` | correctness | warn | `[]G_ENSYM "..."` whose literal is not a plain name, which fails at every call (`error[bad-gensym]`) |
| `reject-bad-place` | suspicious | warn | a literal `"code place" []R_EJECT ...` whose place is not `left`, `right` or `call` (ignored: the error lands on the macro's name) |
| `reject-undocumented` | pedantic | allow | a code a macro rejects its call with that its `##` block does not name, in its prose or examples |
| `reject-unplaced` | pedantic | allow | a literal `"code" []R_EJECT ...` with no place |
| `missing-example` | pedantic | allow | a macro (`m:n_ame<`) whose doc block has no `## >>` example |

The naming lints check lang-choices section 16 (style guide). Case
carries no meaning to the language (decoration decides what a name is),
so only the naming lint that misleads a reader, `mutable-constant`, is
on by default; the case checks are `pedantic`, on request. A parameter
that must be there but is not used (a combinator such as `K_`, an
operand ignoring an argument) has no spelling of its own: silence it
with `# lint: allow unused-binding` (decided with the user, D140). The
catalog is data (`xetal-lint`'s `catalog.rs`): each entry has its
name, group, default level, one-line summary and the text
`--explain` prints (what it does, why, an example and the fix). A test
fails if a pass reports a code that is not in the catalog, or an entry
lacks its example, so a lint cannot ship undocumented.

The macro library lints (the last six) check what a macro author can
get wrong that no call shows until a user meets it
(docs/type-check-macros.md approach 7). They read Core and the file's
`##` blocks (xetal-doccom), so they need no expansion: a fresh name
from `[]G_ENSYM` and a reject code are found where the macro body
calls them with a literal (a computed name or code is not checked),
and a macro's doc block is the `##` block directly above its
definition, as `xetal doc` reads it. The reject-code documentation is
checked in macros only (a helper's codes belong to the macros that
call it); the places in any definition. `gensym-unused` covers a fresh
name the text never writes, the usual way a macro captures its
caller's name by mistake; `unused-binding` would report the same name
with a fix to `_`, which would only throw the fresh name away, so the
more specific finding replaces it (D162).

## 4. Setting levels

### Command line

`-A`, `-W` and `-D` (long: `--allow`, `--warn`, `--deny`) take a lint,
a group or `all`, and repeat. An unknown name is `error[unknown-lint]`
and stops the command: a typo never silently changes nothing.

### Comments in the source (S12)

```
# lint-file: warn pedantic            whole file
# lint: allow shadows-builtin         on its own line: the next statement, all of its lines
x := 1  # lint: allow upper-case-name at a line's end: that line
```

A directive is an ordinary `#` comment (S4), so it needs no grammar,
the formatter and the decorated display keep it, and every other tool
ignores it. Only a single `#` counts: `##` documentation and `###`
headings never hold directives, and a `#` inside a string is not a
comment. Names may be separated by spaces or commas. Blank and comment
lines between an own-line directive and its statement are skipped; the
statement's extent comes from its span in Core, so a definition over
several lines is covered whole. A misspelled level or name is itself a
finding (`unknown-lint`) and silences nothing.

### What `xetal run` does with them

The warnings `xetal run` prints (`shadows-builtin`, `deprecated-private`)
honor the same comments, as do the REPL, the `--echo` notebook and the
live demo: a warning a comment allows is not printed. Each warning is
judged by the comments of the file it is in, so a library can write
`# lint-file: allow deprecated-private` for its own warnings and they
are not printed where it is imported. Run takes no command-line levels,
and a warning stays a warning whatever its level: `deny` fails
`xetal lint`, never a run (L7: warnings do not change the exit code).
One function does this for every front end:
`xetal_linter::honored(sources, program, warnings)`.

### Resolution

For each finding, of the rules that cover its line and select its lint:

1. the narrowest scope wins: statement or line, over file, over the
   command line (as `#[allow]` in Rust overrides `-W` on the command
   line);
2. then the most specific target: a lint, over its group, over `all`
   (so `-D style -A function-name-case` denies the style lints but
   that one, whatever the order of the flags);
3. then the last one written (on the command line: allow, then warn,
   then deny).

No rule: the lint's default level. The resolver is one pure function
(`xetal_linter::resolve`) over a list of rules, which is what makes
these rules testable one by one.

## 5. Architecture

```
          xetal lint FILE  (xetal-cli: args.rs parses, stages.rs dispatches)
                 |
                 v
  components/lint: xetal-linter  (the driver, as clippy-driver)
     command.rs   --list, --explain, report (text or JSON lines), exit status
     driver.rs    load (program or library), run the passes, place, level
     fix.rs       a pass's fix made an Edit to the file as written; --fix
     rule.rs      Rule, Scope, command_line(), resolve()
     directive.rs # lint: comments as written (no lexer needed)
     scope.rs     what each directive covers (statement extents from Core)
                 |
                 v
  components/core: xetal-lint  (the lints, as clippy_lints)
     registry.rs  Group, Level, Lint, lookup
     catalog.rs   the lints as data, with their explanations
     warn.rs      warnings() for xetal run; lints() = all passes
     naming.rs    the style-guide naming lints
     unused.rs    unused-binding (scope-aware uses of a name)
  components/core: xetal-bang  (missing-bang, needless-bang; a crate of
                   its own: xetal-lint is at seven modules)
  components/core: xetal-complexity  (redundant-lambda; the home of
                   the complexity group)
  components/lint: xetal-typed  (int-condition: lints after type
                   checking, over xetal_types::guard_types)
  components/lint: xetal-macrolint  (the macro library lints: fresh
                   names, reject codes, examples; Core and ## blocks)
  components/core: xetal-ir    Expr::children(), the walk every pass uses
     walk.rs      Core children and binding sites
```

Why two crates. The passes stay in `xetal-lint`, beside Core, where
`xetal run`, the REPL, the live demo and the notebook already call
`warnings()`; many components depend on it, so it gains no new
dependency. The driver needs the loader (`xetal-program`) and the
source map (`xetal-sources`) to place findings and read comments, which
would be a cycle in `core`, so it is a component of its own, as
clippy-driver is apart from clippy_lints. "Expand up and out, never
merge" (CLAUDE.md).

Why Core. Lints run on the lowered program (after macros and
desugaring), as the existing warnings do: every surface form has one
Core shape (rule 5), so a lint is written once, not once per sugar.
Core nodes carry the span of a whole construct, so lowering also
records where each binding's name is written (`Program::name_span` for
a `:=`, `Program::param_span` for a lambda parameter): `unused-binding`
and the naming and effect lints are placed at the name, and the
renames are made there (`Program::renamed` finds every place a name
is written).

Why the type check is apart. The Core passes need only Core, so
`xetal lint` works on a program that does not type-check yet. The
typed lints (`int-condition`) need what inference gives:
`xetal_types::guard_types` infers the program (without elaborating
it) and returns each guard condition's span and type. They run only
when the program type-checks; otherwise they find nothing, and the
type error is reported where the program is run. They live in their
own crate, `xetal-typed` in the lint component, because the checker
depends on Core and xetal-lint must not depend on the checker; their
catalog entries are in xetal-lint with the rest.

## 6. Qualities

- Dogfooding: every library, macro library and demo here lints clean
  at the default levels, and a test (xetal-linter tests/repository.rs) fails on a new
  finding, so the gate keeps it that way.
- Reliability: no panics (rule 11); a program that does not load is
  reported as `xetal run` reports it, and `xetal lint` exits 1.
- Determinism and traceability: findings sorted by line, column and
  lint; each carries its stable code, the same name `--explain` and the
  directives use; goldens pin the output (`reg/lint-*.rgt`).
- Usability and discoverability: `--list`, `--explain`, a note on each
  lint's first finding, suggestions in the message (`write n_extGen`),
  unknown names are errors or findings, never ignored.
- Maintainability: a lint is one catalog entry plus one pass function;
  the resolver, directive parser and scoping are separate, pure and
  unit-tested (`components/lint/crates/xetal-linter/tests`).
- Upgradability: new lints default to a level by group, so a new
  `pedantic` lint never breaks a `-D warnings`-style workflow; lint
  names are stable codes; renaming one means keeping the old name as an
  alias (not needed yet).
- Performance: one walk of Core per pass; directives are a single scan
  of the text; negligible next to loading.
- Serviceability: exit status 0 (clean or warnings), 1 (denied, unknown
  command-line name, or load error); stdout is empty unless `--list`
  or `--explain`, so scripts can rely on it.

## 7. Not yet (next steps, in docs/plan.md Saga 40)

- Effects of system functions: the catalog does not say which `[]`
  functions have an effect (`[]S_HOW` does, `[]P_ATH` does not), so
  `missing-bang` ignores them; an effect flag in builtins.toml would let
  it count them.
- More lints: more typed lints (an array where a scalar truth
  belongs); more of the complexity group (`'{ a b -> b f_ a }` is
  `'f_ s_wap`).
- Renames across files (a library's exported function and its
  callers), and a fix for `function-name-case` and `upper-case-name`
  (off by default; a constant's new name needs a choice).
- The Emacs mode reading the JSON, and a configuration file for
  project-wide levels.

