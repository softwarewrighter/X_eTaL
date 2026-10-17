# Code strings: X_eTaL inside a string

Decided with the user 2026-10-17 (lang-choices ST5; steps 111 and 112
of the main saga). This document says what a code string is, what it
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
c"it nn:d_ense W1"            a code string, the usual escapes
cr"it c_at "label: " c_at x"  a raw code string (no escapes, ST4's r"")
cr#"..."#                     a raw code string that may hold "#
"sum i:=0..n - 1 | W[i,j]"    a plain string: text, not code
```

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
| Renderer | the characters as typed | the inside decorated as the code around it (the underlined letter, the superscript alias); the `c` and the quotes kept |
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

## The order of the work

Step 111 (code-strings), the literal and the view:

1. Lexer: `c"..."`, `cr"..."`, `cr#"..."#`; the token kind records
   that the string is code and whether it was raw; rejections for an
   unterminated code string and a name touching `c"`.
2. Renderer: the inside decorated with the rules of the code around
   it, escapes unescaped first; a code string whose inside does not
   lex drawn as a plain string (the lexer reports it separately).
3. Formatter: round trip (rule 10) with the inside kept as written in
   this step (canonical inside is step 112).
4. Expander: `inside()` reads the three forms as it reads raw strings.
5. Emacs mode, the web editor's decorated pane, the LaTeX gallery,
   the literate exporter, the syntax poster.
6. Spec cases, goldens, README syntax row, reference, D163.

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
