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
| `macro-capture` | suspicious | warn | the text of a macro call reads a name the macro wrote and bound itself, neither from `[]G_ENSYM` nor declared in `## binds:` (section 8) |
| `unused-anaphor` | pedantic | allow | a call's text never reads a name the macro declares it binds for it (`## binds: it`) |
| `macro-internals` | pedantic | allow | another lint's finding in code a macro wrote, not text copied from the call (section 8) |
| `shape-mismatch` | correctness | warn | shapes the shape lint proves differ where they must agree: an item-by-item function on two arrays of different shapes, an inner product whose paired axes differ, an `r_equireShape` check that cannot pass |
| `shape-check-proven` | pedantic | allow | an `r_equireShape` check the shape lint proves always passes, so it can go |
| `needless-raw-hashes` | style | warn | a raw string with more `#` than it needs (`r##"a"##`); the fewest is none without a `"` inside, else one more than the most `#` after a `"`; fix: the delimiters |
| `needless-raw-string` | pedantic | allow | a raw string with no backslash, `"` or newline, which plain spells the same (`r"abc"` is `"abc"`); fix: drop the `r` and `#` |
| `escapes-could-be-raw` | pedantic | allow | a plain string with two or more `\"` or `\\` and no `\n` or `\t`, clearer raw; fix: `r"..."`, or `r#"..."#` with the fewest `#` when it holds a `"` |
| `string-holds-directive` | suspicious | warn | a line inside a string (any kind) that starts `# lint:` or `# lint-file:`: text there, not a directive; no fix |
| `gensym-unused` | suspicious | warn | a `[]G_ENSYM` result thrown away or bound to a name nothing reads; reported instead of `unused-binding` there |
| `gensym-not-name` | correctness | warn | `[]G_ENSYM "..."` whose literal is not a plain name, which fails at every call (`error[bad-gensym]`) |
| `reject-bad-place` | suspicious | warn | a literal `"code place" []R_EJECT ...` whose place is not `left`, `right` or `call` (ignored: the error lands on the macro's name) |
| `reject-undocumented` | pedantic | allow | a code a macro rejects its call with that its `##` block does not name, in its prose or examples |
| `reject-unplaced` | pedantic | allow | a literal `"code" []R_EJECT ...` with no place |
| `missing-example` | pedantic | allow | a macro (`m:n_ame<`) whose doc block has no `## >>` example |
| `takes-mismatch` | suspicious | warn | a `## takes:` side declared text whose parameter is `@`, or declared `@` whose parameter is named |
| `ignored-argument` | correctness | warn | a copying macro (`## uses: copied`, the default) that never reads a named parameter, so every call fails `unused-argument`; replaces `unused-binding` there |
| `binds-unwritten` | suspicious | warn | a `## binds:` name no string literal in the macro's body has as a word |

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

The macro library lints (the last nine) check what a macro author can
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
more specific finding replaces it (D162). The contract lints read the
`## takes:`, `## uses:` and `## binds:` lines (MC31, xetal-contract's
`contracts_of`, as the runner does) and check them against the
macro's two parameters and the text its body writes, so a contract
the body contradicts is found before any call meets it.

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
headings never hold directives, and a `#` inside a string of any
kind is not a comment: comments are the gaps between the lexer's
tokens, so a raw string's `#` and `"`, and its lines, are text. Names may be separated by spaces or commas. Blank and comment
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
  components/lint: xetal-origin  (macro-capture, unused-anaphor,
                   macro-internals: where Core came from, over the
                   source map and the macro calls it records)
  components/lint: xetal-shape  (shape-mismatch, shape-check-proven:
                   symbolic shapes by the catalog's shape rules)
  components/lint: xetal-strings  (the string lints, over the file's
                   own tokens, not Core)
     form.rs      Form (plain, raw with n #), delimiters, fewest_hashes
     literal.rs   each literal's form, span, body, value; holds()
     spelling.rs  needless-raw-hashes, needless-raw-string,
                  escapes-could-be-raw
     directive.rs string-holds-directive
     scan.rs      strings() runs them all; line_col() places them
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

### The shape lint

`xetal-shape` carries shapes through Core symbolically (type-check-macros
approach 8). What it knows of a value is its shape, one extent per
axis, and, for a short vector of extents (`2 3`, `N c_at 3`, `s_hape
X`), its items. An extent is a number or a name bound to a single
number the lint cannot see (`N := t_ally V`), so `N 3` and `N 3` are
the same shape and `N 3` and `N 4` differ. How each built-in's result
follows from its arguments is the `shape` (or `shapes`) field of its
group in builtins.toml, beside its type, so the catalog stays the one
list: `scalar` (item by item, a single value extends), `reshape`,
`cat`, `shape`, `count`, `reduce` (`r_/` without a subscript), `inner`
and `check`. Anything else (a parameter, a file read, a function call,
a mutable `!` variable) is unknown, and an unknown shape never makes a
finding: the lint warns only where the program surely fails.

The contract with macros is `r_equireShape` (`2 3 r_equireShape X` gives X when
its shape is 2 3, else `error[shape-mismatch]`). The linter cannot
learn the notations of macros in other repositories (Equations, the
pipeline), so a macro emits the check after a step whose shape it
promises (the pipeline's ` [N O Ho Wo]`): at run time it is the
backstop, and to the lint it is a fact, from which later mismatches
are found before running, and a check the lint can already prove shows
as `shape-check-proven`. The pipeline examples' hand-written
equivalents (docs/spatial-poc/pipes-oracle.xtl), with their
annotations written as checks, are its tests.
Why the string lints read the text. How a string is spelled (plain
`"..."` or raw `r#"..."#`, ST4) is gone once it is lexed: Core holds
only its value. So `xetal-strings` lexes the program's file as written
and reads each string token's own text for its form and `#` count;
its spans are offsets in that file, placed by line and column the way
the source map places the Core findings (a file that does not lex has
none: loading reports why). The form is an enum (`Form::Plain`,
`Form::Raw(n)`), so another kind of string (a code string `c"..."`,
docs/code-strings.md) is one more variant, and may bring lints of its
own. Every fix keeps the string's value byte for byte: a plain
argument is unescaped before a macro sees it and a raw one is not
(D156), so the fix is offered only when its new spelling lexes to
exactly the same value (`holds()`), and the tests check that lexing
the fixed text gives every string its old value.

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
- The shape lint through user functions (a function's result shape
  from its body, per call) and lambdas given known arguments; an
  `index-out-of-range` lint for a gather whose index it can bound.
- More lints: more typed lints (an array where a scalar truth
  belongs); more of the complexity group (`'{ a b -> b f_ a }` is
  `'f_ s_wap`).
- Renames across files (a library's exported function and its
  callers), and a fix for `function-name-case` and `upper-case-name`
  (off by default; a constant's new name needs a choice).
- The Emacs mode reading the JSON, and a configuration file for
  project-wide levels.

## 8. Code a macro wrote

A macro's expansion is a mix of the call's own text (copied from its
arguments) and text the macro wrote. The macro phase already keeps a
map from every expanded byte back to where it was written (MC17): a
copied byte maps to its place inside the call's string, a byte the
macro wrote maps to the whole call. It now also records each call
(`xetal_sources::Site`): where it is written, the macro as the call
writes it (`x:w_ith<`) and the names the macro declares it binds for
the call's text (`## binds:`). A call inside another's expansion is
recorded at the outermost call, its binds added to that call's. The
lints in `xetal-origin` read these two facts.

`macro-capture` (suspicious, warn). A binding whose name the macro
wrote (not copied from the call), that is not a fresh name from
`[]G_ENSYM` (`g1:t`) and is not declared in the macro's `## binds:`,
read by text copied from that call. The caller wrote the name meaning
their own binding and got the macro's:

```
x := 10
p_rint! @ c:c_ap< "x + 1"     # c:c_ap< writes `x := 3; ` around its text
```

prints 4. It is reported at each read in the call's text, naming the
macro, with the two fixes open to the macro's author: `[]G_ENSYM` for
a name of its own, or `## binds: x` when the capture is meant (then it
is an anaphor, and the call's text may not rebind it:
`shadows-anaphor`). A name written nowhere (the shorthand's `_r` in a
lambda the macro opened, as `m:d_ef<` does) is not a capture. The
caller's own binding of the same name inside the text hides the
macro's, as scope says, and is not reported.

Origin-aware reporting. Every other pass runs on the expansion too, so
a finding can start in code the macro wrote, which is placed at the
whole call. The caller cannot change that code, so the driver
(`xetal_origin::placed`, before levels) reports it under another lint:

- an `unused-binding` of a name the macro declares it binds for the
  call's text is `unused-anaphor` (pedantic): "x:w_ith< binds `it` for
  your text, which never reads it", with no `_ :=` advice and no fix;
- any other is `macro-internals` (pedantic), its message naming the
  macro and the lint that found it ("in code x:w_aste< wrote:
  unused-binding: ..."), with no fix. `xetal lint -W macro-internals`
  shows them, for a macro's author testing a call.

A finding that starts in text copied from the call is the caller's, and
is reported as before, at its place inside the string.

Limits. Copied text is found by matching the macro's output against
the call's arguments (whole arguments first, then runs of three bytes
or more), so a macro's own text that happens to equal an argument (a
one-letter argument `"x"` next to the macro's own `x`) counts as
copied: such a capture is missed, never reported falsely. Two
neighbors are not done yet: a statement macro that binds a top-level
name later statements read (`macro-binds-statement`, pedantic), and
`xetal run`'s own warnings (`shadows-builtin`) over code a macro wrote.
When step 110's contracts (`## takes:`, `## uses:`) land, the lints
read them beside `## binds:`.
