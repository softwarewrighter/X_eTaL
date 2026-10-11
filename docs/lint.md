# xetal lint: a clippy-style linter

`xetal lint` checks a program or library against a catalog of named
lints, as `cargo clippy` checks a Rust crate. This document is the
design: what it does, how it is built, why, and where it goes next.
User-facing usage is in the README; decisions are D137 to D142 in
`docs/design.md` and S12 in `docs/lang-choices.md`.

## 1. What it does

```
xetal lint FILE                     # report findings (stderr); exit 0 unless one is denied
xetal lint -e 'Count! := 0'         # lint text given inline
xetal lint -D style FILE            # style findings are errors: exit 1
xetal lint -W pedantic -A shadows-builtin FILE
xetal lint --list                   # every lint: group, default level, summary
xetal lint --explain mutable-constant
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
| `mutable-constant` | style | warn | a mutable variable named like a constant (`Count!`) |
| `missing-bang` | style | warn | a function calling an effect (`p_rint!`, a `!` function) directly, without `!` in its name |
| `needless-bang` | pedantic | allow | a `!` function that could not have an effect |
| `function-name-case` | pedantic | allow | a function named UpperCamel (`u:N_extGen`); a capital start is for combinators |
| `upper-case-name` | pedantic | allow | a variable of two or more letters, all capitals (`MAXSIZE`) |

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
     command.rs   --list, --explain, report and exit status
     driver.rs    load (program or library), run the passes, place, level
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
The price is that Core names carry no spans of their own: a naming
finding is placed at the binding's value or lambda, on the right line
but not the name's column.

Why no type check. The passes need only Core, so `xetal lint` works
on a program that does not type-check yet. Lints that need types (a
`?` predicate that does not return Bool) will run after
`xetal_types::check_program` and only when it succeeds.

## 6. Qualities

- Dogfooding: every library and demo here lints clean at the default
  levels, and a test (xetal-linter tests/repository.rs) fails on a new
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
- More lints: redundant lambdas
  (`{ x -> f_ x }` is `'f_`, `{ x -> x }` is `I_`), typed lints (`?`
  must return Bool).
- Machine-applicable suggestions and `xetal lint --fix`.
- `--format json` for editors (the Emacs mode, the live demo's editor),
  and a configuration file for project-wide levels.
- Name spans in Core, to point at the name's column.

