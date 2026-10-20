# Code strings: X_eTaL inside a string

Decided with the user 2026-10-17 (lang-choices ST5; steps 111 and 112
of the main saga; the raw form `cr"..."` was proposed and dropped). This document says what a code string is, what it
lets the tools check that a plain string cannot, what it does not give,
and the order the tools are updated in.

## The literal

A macro's argument is a string (MC17), and that string is often X_eTaL:
a pipeline stage `"it nn:d_ense W1"`, a condition `"n = 0"`, a body
`"p_rint! n"`. To every tool it is one Char vector, so the renderer
leaves it alone, the formatter cannot format it, the linter cannot
look inside it, and a mistake in it is found only when the macro has
run and the expansion is checked, with the error placed back into the
string by the expander's byte map. An equation string
(`"sum i:=0..n - 1 | W[i,j] * x[j]"`) is not X_eTaL at all, so no tool
should read it as code.

A code string says which is which, in the source, as a token:

```
c"it nn:d_ense W1"              a code string, the usual escapes
c"it c_at \"label: \" c_at x"   a string literal inside it is escaped
c#"it c_at "label: ""#          verbatim (ST4's # rule): no escapes inside
c"it nn:d_ense W1
  |> nn:r_elu"                  either may span lines; newlines are literal
r"sum i:=0..n - 1 | W[i,j]"     a raw string: verbatim text, not code (ST4)
"sum i:=0..n - 1 | W[i,j]"      a plain string: text, not code
```

An unescaped `"` inside plain `c"..."` cannot be allowed: at
`c"it c_at "label: "`, the second `"` is either the closer or an inner
string's opener, and choosing by looking ahead is a heuristic (rule 3).
So inner strings are escaped in `c"..."` or written as they are in
`c#"..."#`, whose closer is `"#`. Code strings nest one level only:
the renderer decorates the inside, and a code string inside the inside
is drawn as a string (a macro call written inside a stage takes
ordinary strings anyway). Decided with the user 2026-10-17; `cr"..."`
was proposed and dropped, `r"..."` staying the form for verbatim text
that is not code.

The value is the same Char vector as the plain string of the same
text; a macro receives it unchanged, and no macro has to change. The
`c` is a declaration by the writer: this text is X_eTaL. Grammatical
class comes from tokens only (rule 4), so the tools need no knowledge
of the macro to know it.

## What the tools can then check

Each check below is impossible on a plain string, because a plain
string might be anything; each becomes possible on a code string
because its inside is X_eTaL by declaration.

| Tool | On a plain string | On a code string |
| ---- | ----------------- | ---------------- |
| Renderer | the characters as typed | the inside, escapes resolved, decorated as the code around it (the underlined letter, the superscript alias); the `c` and the quotes kept |
| Lexer | nothing | the inside lexed when the file is lexed: a bad character, an unterminated inner string, a malformed name is `error[...]` at its place inside the string, whether or not a macro ever runs |
| Parser | nothing | the inside parsed as an expression or statements: an ambiguity (rule 3), an unbalanced bracket, a stray `->` is found before expansion, and placed inside the string |
| Formatter | the string kept as typed | the inside in canonical form (`c"it nn:d_ense W1"` and `c"it  nn:d_ense  W1"` format alike), rule 10 holding for the inside too |
| Linter | nothing | the lints of ordinary code run on the inside: an unused binding, a shadowed name, a style lint, a plain name where a function is needed; and two lints of its own, below |
| Type checker | nothing until the expansion is checked | the inside checked on its own as a lambda over the names the macro binds for it (`## binds: it`): an unknown name, a Char where a number is needed, a function where a value is needed, wrong arity, found before expansion and reported inside the string. The lambda's parameters have free types, so this is a partial check: what holds for every `it` |
| Macro contracts (MC31) | `## takes: text` | `## takes: code` on a side says the macro expects X_eTaL there. A plain string passed to a `code` side, or a code string to a `text` side (an equation), is a lint, `string-kind`, with the fix (add or drop the `c`) |
| Expander | copies the inside, maps bytes back | the same; the map is unchanged. A `copied` macro's unused-argument check (MC31) already works on tokens, so it needs nothing new |
| Debugger, REPL | the expansion is the only readable view | the call itself is readable; the expansion stays the debugging view |

The type check deserves care. A stage `c"it nn:d_ense W1"` is checked
as `{ it -> it nn:d_ense W1 }`: `nn:d_ense` must exist and take two
arguments, `W1` must be bound in the program, and the result type is
whatever `nn:d_ense` gives for a free `it`. What is not checked here is
the join between stages: that the previous stage's value is what this
`it` needs. That is checked, as today, on the expansion, where `it` is
bound to a value of a known type; and it is checked earlier by the
shape lint (type-check-macros approach 8) where the pipeline is
annotated. So a code string moves the errors inside one stage from
after expansion to before it; it does not change what is found about
the whole.

## What it does not give

- Nothing about shape or rank; those are run-time in all X_eTaL
  (type-check-macros approach 6 is the research item).
- Nothing for a macro that parses a notation (Equations): its strings
  stay plain, and the Parsing library (`pa:`) with the whole-text
  check is what makes those safe.
- Nothing for a macro that takes apart X_eTaL by text (`e_ach<`'s
  `$w`, `Y_<`'s `~s_elf`): such a string is almost X_eTaL but not
  quite, so it stays plain, and the macro says `## uses: parsed`.
- No new type: Char is Char. A code string in a value position (`x :=
  c"1 + 2"`) is a string whose inside happens to lex; the lints of
  its own apply only where it is a macro's argument.

## Notations: drawn by the library (MC32)

A string that is not X_eTaL but a library's own notation (an equation,
a pipeline's `|>` joins) cannot be a code string, so the views cannot
decorate it from tokens. Decided with the user 2026-10-18: the macro's
contract says so and names who draws it.

```
## The value of a sum as a paper prints it.
## takes: text notation
## draws: e:t_ex<
m:e_q< := ...
```

Where a program is drawn (the pages, the live demo, the editors), the
notation side of a call is drawn from what the `draws:` macro gives
(Equations: its LaTeX, drawn by KaTeX); the text typed and the value
are unchanged.

### Why no notation string (`n"..."`)

Settled with the user, 2026-10-19, when step 154 began. A fourth
spelling, `n"..."` by analogy with `c"..."`, was proposed so that a
view with only the tokens could style a notation, and declined.

The string forms are one type, a `Char` vector, and each prefix tells
the tools something they can act on. `r"..."` says how the characters
are read (verbatim, across lines) and changes nothing about their
meaning. `c"..."` says the characters are X_eTaL, and the tools act on
that: the renderer draws them decorated, the editor highlights them,
`xetal lint` and the type checker reach into them, the macro runner
checks them before the macro runs. A notation is neither: it is text to
every tool but its own library (an equation's `..`, `X[c,h]` and `|`
mean nothing to X_eTaL, so a code string mis-draws it and the code
checks reject it), and no tool but that library's `draws:` macro can
draw it. So a call-site mark would carry one fact, "a library draws
this", which the contract already states once per macro, and it would
cost a lexer kind, a formatter case, a renderer case, a `string-kind`
lint and a reference section. The drawing needs the program phase
either way, since only the library can typeset its notation, and the
same program phase can tell the editors which strings are notations
and style them. If a token-only view ever needs the mark, `n"..."` can
be added then as another spelling of the same `Char` vector, as `r`
and `c` were, from a concrete need; `c"..."` itself arrived that way
(ask M17).

Graphical tools that generate X_eTaL fit this: they write plain
strings for equations and `c"..."` for pipeline stages, and the
contracts say which is which. Integrals and continuous equations
change nothing: an integral is an operator word inside the same
equation string, drawn by the same `e:t_ex<`.

### How the views draw a notation (step 154)

1. The program phase, which loads the libraries anyway, runs the
   `draws:` macro for each call whose contract has one, given the same
   call, and records the drawing (its text read as one string literal,
   LaTeX for Equations) on the call's site in the program's sources;
   the renderer never resolves imports itself.
2. A view takes that overlay. In HTML (the doc site, the pages, the
   live demo) the notation strings of a call become one math box,
   drawn by KaTeX, after the macro's name and the binding, which stay
   as code: `Y := e:e_q< [math]`, so a reader still sees which macro
   ran. The terminal and the TUI, having no math renderer, show the
   typed text styled as notation; a Unicode drawing may come later.
3. A library loads with `bad-contract` when a notation side has no
   `draws:`, a `draws:` names no notation side or a macro the library
   lacks, or a macro names itself.

As built (D178). The contract: `xetal-contract` reads `notation` as a
text side that is drawn and `## draws: m_acro<` (an alias before the
name is dropped: `e:t_ex<` and `t_ex<` name the same macro), and
checks the three rules when the library loads. The program phase: the
macro table (`xetal-macro`) runs the `draws:` macro with the call's
own sides right after the call's contract is looked up, reads its
output as one string literal (`xetal_contract::drawing`; anything
else, or a failing drawer, draws nothing and the call reports its own
errors) and records it on the call's site (`Site::drawn`, beside
`Site::notation`); `xetal_notations::notations(file, sources)` lists a
file's drawn calls with the spans of the strings they stand for. The
overlay: `xetal-notate` (view component) replaces those strings on a
file's segments, `drawn` with one segment of class `Notation` holding
the LaTeX in place of the last string and an empty one for the other
(so the box follows the macro's name) and `marked` with the class
only; `found(name, src, libs)` loads a file as `xetal run` does and
gives its notations, none when it does not load. The views: `xetal
render --html`, the doc site (`xetal doc --out`, its source pages and
each item's source) and the literate pages write the LaTeX in a
`c-notation` span and load KaTeX from jsDelivr (0.18.10, the release
the libraries' site uses) to typeset it in place; the live demo's
Rendered pane typesets through `xetal-katex` (its own component,
`katex.renderToString` from wasm; the LaTeX, escaped, where KaTeX is
missing); `xetal render --color` and `xetal edit` show the typed
strings in the notation style (ANSI italic bright yellow; the TUI
theme light yellow italic), the binding and the macro name as code.
The value and the text are unchanged: the `draws:` macro runs only
where a program is drawn, never when it runs. Tests: xetal-contract
tests/doc.rs, xetal-notations tests/notations.rs, xetal-notate
tests/overlay.rs, xetal-docsite tests/site.rs (fixtures math.xtl and
Eq.xtlm), xetal-play tests/notation.rs, xetal-panes and xetal-edit
tests; goldens render-color-notation, render-html-notation,
macros-notation-run, macros-notation-no-drawer,
macros-notation-drawer-missing (spec/macros/data/Notation.xtlm).

## The order of the work

Step 111 (code-strings), the literal and the view:

1. Lexer: `c"..."` (ST1's escapes) and `c#"..."#` (verbatim, ST4's
   `#` rule), both spanning lines; the token kind records that the
   string is code; rejections for an unterminated code string and a
   name touching `c"`.
2. Renderer: the inside decorated with the rules of the code around
   it, escapes unescaped first, one level deep (a code string inside
   is drawn as a string). Drawing is tolerant: what does not lex (a
   macro's own notation such as a pipeline's ` : name` labels, or a
   slip while typing) is drawn as typed, in string color, and the code
   around it decorated; the checks of step 112 report it. Pipelines'
   `|>` is a token (S13), drawn as a triangle.
3. Formatter: round trip (rule 10) with the inside kept as written in
   this step (canonical inside is step 112).
4. Expander: `inside()` reads the three forms as it reads raw strings.
5. The view model (`xetal-view`, which both the terminal editor and
   the live demo draw through): a code string's inside segmented as
   code, and an open code string while typing (`c"it nn:d_ense`)
   decorated too, not marked as an error to the end of the line as
   an unterminated string is; view tests and a web test.
6. Emacs mode, the LaTeX gallery, the literate exporter, the syntax
   poster.
7. Spec cases, goldens, README syntax row, reference, a design register entry.

As built (D175): items 1 to 5 in parts 1 and 2; item 6 in the Emacs
mode (the inside colored as code, `|>` and a touching `$` drawn), in
`render --latex` (the inside set token by token; a label as
`{\scriptstyle\triangleright}`, since KaTeX has no
`\smalltriangleright`) and the gallery (a code string spanning lines
is one entry), and on the poster (panel 10); the HTML and color
renderers and the literate exporter draw through the view model and
needed no change. Item 7: `spec/lex/code-strings.case`, the README
row; the reference lists built-in functions only, so it is unchanged.

Step 112 (code-strings-checks), what the tools check:

1. Lexer and parser: the inside lexed and parsed with the file;
   errors placed inside the string; a spec case per error kind.
2. `## takes: code` in the macro contract (MC31 extended); the
   `string-kind` lint in xetal-lint with its fix; System.xtlm and
   lib/Macros.xtlm say `code` where they take X_eTaL.
3. Linter: the ordinary lints run on the inside, placed inside.
4. Type checker: the inside checked as a lambda over the macro's
   `## binds:` names, before expansion; the error inside the string;
   what the partial check cannot see documented in the reference.
5. Formatter: the inside in canonical form.
6. Asks to X_eTaL-ML (pipeline stages written `c"..."`, `## takes: @
   code` on the stage macros) and X_eTaL-libraries (demos that pass
   code to macros).

Nothing here changes a macro's text or value; a program that never
writes `c"` behaves as before.

As built (D176):

1. Every code string's inside is lexed when the file is expanded,
   before any macro runs. A code string on a side its macro declares
   `code` is also parsed there, as a lambda's body (`parse_body`: a
   guard and `_r` may stand, since the macro puts the text in a
   lambda or a block). One on a `text` side, or on a macro without a
   `takes:` line, is lexed only: the contract, not the `c`, says the
   text will be read as code, so a pipeline's notation (`|>`, `$`)
   stays a code string drawn as code but is never parsed. Spec cases
   lex/reject-code-inside-char and -string, macros/reject-code-unclosed
   and -pipe, macros/code-body.
2. `## takes:` sides are `text`, `code` or `@`. System.xtlm and
   lib/Macros.xtlm say `code` where they take X_eTaL, and their
   examples pass code strings. `string-kind` (style, warn) has the fix
   that adds or drops the `c`, the value kept. A code string on a side
   the macro takes as text or as a notation whose inside is not X_eTaL
   (an equation's `0..2`) is `string-kind` from the compiler too, an
   error with the same fix, in place of the lexer's error about the
   inside (B2, the blog's ask, 2026-10-19; D181): the expander lexes
   the inside of a code string up front only where it is not a macro's
   argument, and checks an argument with the macro's contract in hand
   (parsed on a code side, lexed otherwise; a macro without a
   `takes:` line keeps the lexer's error). `xetal lint --fix` makes a
   load error's own fix as it makes a finding's, so the file then loads
   and is linted. Spec cases macros/reject-code-on-text-side and
   macros/code-on-text-side-lexes; golden macros-code-on-notation.
3. The ordinary lints run on the inside for a macro that parses its
   text; a copying macro's are found on its expansion, placed by the
   byte map.
4. `code-check` (correctness, deny) is the type check before
   expansion, done in the linter: each code string on a code side is a
   probe, `_ := { NAMES -> INSIDE }` after the program, NAMES the
   macro's `## binds:` and then every other plain name the inside reads
   and does not bind, each of a free type. What it finds: an unknown
   function, a wrong number of arguments, a type that cannot fit for
   any value of the names. What it cannot see: whether a name is bound
   where the call stands (every plain name is a parameter), the
   values' types at the call (the join between stages), and an inside
   with `_l`/`_r` beside named parameters or naming a mutable
   variable (not probed). The expansion is checked as always.
5. `xetal fmt` writes a code string on a macro's side with its inside
   in canonical form, in the form written; one in a value position, or
   whose inside does not parse, is kept as written.
6. The notice for the siblings is in docs/asks.md.
