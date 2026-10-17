# Type-checking macro inputs

A macro's arguments are strings (MC10, MC17), so it looks as if
everything inside them escapes the type checker. That is only partly
true. This page describes what is checked today, what slips through to
run time or to a wrong answer, and how each kind of error can be
caught earlier: at macro time (while the call expands) or at compile
time (when the expansion is type-checked). It ends with a plan.

## What happens to a macro call today

```
source -> macro phase (expand) -> lex, parse -> lower -> type check -> lint -> run
```

1. The macro runs at expansion with the two strings and returns source
   text. It may reject the call (`[]R_EJECT`), reported at the macro's
   name or at an argument.
2. The returned text is lexed, parsed and **type-checked like any other
   code, before the program runs**. A type error in a macro's
   argument is a compile-time error, not a run-time one.
3. Lints run on the expanded program too.

So the expansion is not untyped. The hole is in three places:

- **Locations.** Text the macro copies from an argument verbatim is
  reported where it was written, inside the string; text it transforms
  or adds is reported at the whole call (MC17). An error is caught, but
  the user is pointed at a 100-character call instead of at `X`.
- **The macro's own notation.** A notation inside the string (an
  equation's `X[c,h+r]`, a pipeline's `| relu |`) has rules the X_eTaL
  checker never sees: how many indices an array takes, which names are
  indices, how ranges are written. Unless the macro checks them, a
  mistake is lowered into well-typed code that computes the wrong
  thing.
- **Shapes and ranks are not types.** In X_eTaL `2 3 r_eshape 1` has
  type `Int`: types say an array's element type, not its rank or its
  axis lengths. That is true of all X_eTaL code, macros or not, so
  shape and index errors are run-time errors everywhere.

## What slips through: probes

Run against the spatial proof of concept (`docs/spatial-poc/cnn.xtl`
with one change each, on the current binary; each is a file in
`docs/spatial-poc/`, run from there with `xetal run probe-....xtl`):

| Change (file) | Kind of error | Caught | Reported at |
| ------------- | ------------- | ------ | ----------- |
| `X := "abcd"` (`probe-text-array.xtl`) | element type (Char, not number) | compile time | the whole call (`at 276..372`) |
| summand `X[c,h+q,w+s]` (`probe-unknown-index.xtl`) | unknown name `q` | compile time | the whole call |
| `X` of shape `8 4` (rank 2, three indices used; `probe-rank-2-array.xtl`) | rank | run time, `error[index]` | the whole call |
| summand `X[c,h+r]` (two indices for rank 3; `probe-too-few-indices.xtl`) | rank, in the notation | **never: a wrong answer** (`162 162 162 ...`) | nowhere |

And in plain X_eTaL, for comparison:

```
$ xetal type -e '2 3 r_eshape 1'
Int
$ xetal eval -e '(2 3 r_eshape 1) + 1 2 3 4'
error[shape-mismatch]: shapes differ: 2 3 and 4 at 1..26
$ xetal eval -e '3 s_elect 1 2'
error[index]: index 3 is outside 1..2 at 0..13
```

## The kinds of error

| Kind | Example | Today | Earliest it can be caught |
| ---- | ------- | ----- | ------------------------- |
| Notation syntax | `sum c=0..C`, a missing region bar | only if the macro checks | macro time |
| Notation names | an index used but never bound; bound twice | only if the macro checks; else an undefined name at compile time, located at the call | macro time |
| Notation counts | a free index without an extent; two indices for a rank-3 array | only if the macro checks; else a wrong answer | macro time (counts it can see), run time (ranks it cannot) |
| X_eTaL syntax in an argument | `C-1` copied into the expansion | compile time | compile time, located in the string if copied verbatim |
| Element types | `X` is text | compile time, located at the call | compile time, located at the name |
| Undefined caller names | `Kh` misspelled | compile time, located at the call | macro time, if the macro could ask what is defined |
| Rank | a rank-2 array indexed by three indices | run time, or a wrong answer through a ravel | macro time if ranks were known statically; else the first line of the expansion at run time |
| Axis lengths and bounds | `h+r` past the height | run time | run time (a value), early in the expansion, with a precise message |
| Macro bugs | the macro writes ill-typed code for a valid call | compile time, at the call | the macro's own tests |

## Ways to catch them

### 1. Validate in the macro (macro time; available now)

The macro parses its notation and rejects what is wrong with
`[]R_EJECT`, at the argument: syntax, names bound and used, counts,
duplicate indices, ranges. This is where the notation's own rules
belong; no checker can know them.

- Feasibility: easy, no compiler change; every library macro should do
  it, and System.xtlm's macros already do (`bad-macro-argument`).
- Limit: the macro sees text, not the caller's values or types, so it
  cannot know that `X` has rank 3.

### 2. Emit checks into the expansion (compile time and early run time; available now)

The macro writes code that makes the checker or the program check what
the macro cannot:

- Type witnesses: an expression that only type-checks if `X` is
  numeric (`X + 0`), placed first, so a type error appears on a short
  piece of generated code rather than the whole computation.
- Rank and bounds guards at the top of the expansion, before any
  gather: `3 = t_ally s_hape X` or a `"index" []S_IGNAL` naming the
  array, the axis and the expression. Still run time, but at once, with
  a message in the notation's terms, and never a silent wrong answer.

- Feasibility: easy; the spatial library will do this (its run-time
  index error was decided with it, D154). It closes the wrong-answer row.

### 3. Better locations for transformed text (compile time; a compiler change)

Let a macro say where a piece of its output came from, so an error in
transformed text is reported at the argument's characters (`X` inside
the string) instead of the whole call. Two shapes:

- A hook `[]A_T` (`"right 12..13" []A_T "text"`) wrapping generated text
  with the span it stands for; the macro phase records the mapping as
  it does for verbatim copies.
- The macro returns a list of (text, origin) pieces instead of one
  string (changes MC10's type).

- Feasibility: medium; the span-mapping machinery for copied text
  exists (MC17), so this extends it. The hook keeps MC10's type and is
  the smaller change.

### 4. Ask the compiler about the caller's names (macro time; a compiler change)

A hook giving the macro the type of a name in scope at the call,
`[]T_YPE "X"` (as text: `Int`, `Char`, a tuple, or `undefined`), lets
the macro reject a misspelled name or a text array at the argument,
in its own words.

- Feasibility: medium to hard. The macro phase runs before the file is
  type-checked (MC17: macros expand before names are read), so it
  needs the types of what is defined above the call: an incremental
  check up to the call, or a second pass that expands, checks,
  then re-runs macros that asked. Types still give no ranks, so this
  catches element types and names, not rank.

### 5. Typed macro arguments (compile time; a language change, L6)

A macro declares that an argument is X_eTaL code of a type
(`m:s_um< :: Code Int -> ...`), so the argument is parsed and
type-checked in the caller's scope before the macro runs.

- Feasibility: medium for arguments that are X_eTaL; it does not help a
  notation that is not X_eTaL (equations, pipelines), which is the
  point of those macros.

### 6. Ranks in types (compile time; a language change)

Types that carry rank (and optionally axis lengths), as in Remora,
Futhark's size types or Dex's index sets, would make rank errors and
many shape errors compile-time errors in all X_eTaL, and a macro's
expansion would inherit them.

- Feasibility: hard. A change to the type system, inference, every
  built-in's signature (the catalog) and error messages; rank
  polymorphism (APL's leading-axis rules) makes inference much harder.
  Research, not a step.

### 7. Lint and test macro libraries (before shipping)

- A macro's `## >>` doc tests and goldens cover valid and invalid
  calls, including the rejections.
- A lint over a macro library: every `[]R_EJECT` code is documented,
  every macro has a test of a rejection.
- Fuzzing a macro with mutated calls: every call either is rejected at
  macro time or expands to code that type-checks (a macro whose valid
  output fails to type-check is a macro bug).

- Feasibility: easy to medium; no language change.

### 8. A shape lint over the expansion (before running; a lint, no language change)

Lints run on the expanded program, so a lint can carry shapes through
it symbolically: `X` of shape `N C H Wd` (from an annotation or a
literal `r_eshape`) and a kernel `O C Kh Kw` give a valid
cross-correlation of shape `N O (H-Kh+1) (Wd-Kw+1)`; `+` needs equal
shapes; `i_nner` needs the inner extents equal. Where it can prove a
mismatch, it warns before the program runs; where a size depends on
data read at run time, it says nothing and the run-time check
(approach 2) stays the backstop. The pipeline's optional shape
annotations (` : conv [N O Ho Wo]`, docs/spatial.md) are the facts it
starts from, as declared types are in gradual typing.

- Feasibility: medium. A table of shape rules per built-in (beside
  their types in the catalog), symbolic extents with `+ - *` and
  equality, a pass in `xetal lint`. Rank polymorphism is handled per
  call, not in the type system, so it needs no language change; it is
  sound only where it speaks (it never proves a program right).

### 9. Total parsing for macro authors (macro time; a standard library)

A reading library for `.xtlm` files (combinators or a small grammar
form) whose results carry spans and must end at the end of the input:
a parse that stops short is an error, so macro libraries stop writing
their own cut-and-drop readers. Equations, Tags, and the coming
pipeline and integral macros would use it.

- Feasibility: medium; a library, no compiler change. It is the
  positive form of the rule the Equations reader now follows by hand.

### 10. An unused-argument check in the macro runner (compile time; a compiler change)

**Decided and done (2026-10-17, MC31, step 110):** as a contract the
macro declares. `## uses: copied` (the default) makes the runner
check that every name and number of each argument is a token of the
expansion (inside its strings too), else `unused-argument` at the
argument naming the text ignored; `## uses: parsed` says the macro
reads a notation and answers for it (through the Parsing library,
approach 9). `## takes:` states the shape of call and is checked
before the macro runs. The design below is what was weighed.

The runner maps a macro's output back to its call. If a macro received
its arguments as spanned text (as the expander already holds them),
the runner could refuse an expansion that leaves part of an argument
unaccounted for, or require the macro to mark text it ignores on
purpose: "this macro ignored `^2`". That would make the whole class a
compile-time error for every macro, however its reader is written.

- Feasibility: medium to hard: what counts as "used" needs a design
  (copied text is tracked today, transformed text is not; approach 3
  shares the question), and an opt-out for macros that read only part
  of a text by design (a comment convention, a label).

### 11. A lint for macro libraries (before shipping)

`xetal lint` on a `.xtlm`: a reader that drops characters it has not
matched (`1 d_rop -1 d_rop t` on an unchecked text) or a parse never
checked for leftover input.

- Feasibility: medium; heuristic by nature, so a warning, never an
  error.

### The case that taught the class (2026-10-17)

Writing about the Equations library turned up three silent wrong
answers, with `y = 1 2 3 4` and `p = 1.5 2 2 4`:

| Summand | Gave | Correct |
| ------- | ---- | ------- |
| `(y[i] - p[i])^2` | 0.5, the plain sum of `y - p` | 1.25 |
| `y[i]2` | 10, the sum of `y` | an error |
| `y[i] % 2` | 10; the `% 2` was dropped | an error |

No error anywhere, and the LaTeX printed what was written, so the
typeset math and the value disagreed. The reader took a term starting
with `(` as a group by dropping its first and last characters without
checking that the last one closed it, and a reference from the first
`[` to the last `]` without checking that the `]` ended the term;
nothing checked that every character was used. Fixed in the library
(every rule uses all of its text or refuses it; a closed alphabet; six
rejection tests), it is the clearest case for approaches 9 to 11:
type and compile-time errors are good, run-time errors worse, silent
wrong answers the worst.

## Feasibility in brief

| Approach | Catches | When | Cost |
| -------- | ------- | ---- | ---- |
| 1 Validate in the macro | notation syntax, names, counts | macro time | easy, library |
| 2 Emit checks | element types early; rank and bounds, no silent answers | compile / early run | easy, library |
| 3 Locations | (same errors) pointed at the right characters | compile time | medium, compiler |
| 4 `[]T_YPE` hook | caller's names and element types | macro time | medium to hard, compiler |
| 5 Typed arguments | X_eTaL-code arguments only | compile time | medium, language |
| 6 Ranks in types | rank and shape everywhere | compile time | hard, language |
| 7 Lint and tests | macro bugs | before release | easy to medium |
| 8 Shape lint | rank and shape mismatches it can prove | before running | medium, lint |
| 9 Total parsing library | readers that stop short | macro time | medium, library |
| 10 Unused-argument check | any text a macro ignores | compile time | medium to hard, compiler |
| 11 .xtlm lint | cut-and-drop readers | before release | medium, lint |

## Plan

1. **Now, in the spatial library (Saga 41):** approaches 1, 2 and 7.
   Every notation rule is checked at macro time with `[]R_EJECT` at the
   argument; the expansion opens with rank checks and a bounds check
   per axis, so a wrong rank or index is an error in the notation's
   terms, never a wrong answer; the rejection tests and goldens pin it.
   The probe above that gives `162 162 162` becomes a pinned
   rejection test.
2. **Next, decided with the user:** approach 3, located errors for
   transformed text (a hook in the macro phase), as a candidate in the
   spatial limitations review: it improves every macro library at
   once.
3. **Then, if a consumer needs it:** approach 4, the `[]T_YPE` hook,
   with the two-pass design worked out first.
4. **Planned with the pipelines (decided with the user):** approach 8,
   the shape lint, starting from the pipeline's shape annotations, with
   the run-time check after each annotated stage as the backstop:
   preventable run-time errors should be prevented.
5. **Done (2026-10-17):** approach 10 as macro contracts (MC31,
   `## takes:` and `## uses:`), and approach 9 as the Parsing library
   (`pa:`) in X_eTaL-libraries; approach 11, a lint for `.xtlm`
   readers, stays with the lint lane and the spatial limitations
   review.
6. **Research, not scheduled:** approaches 5 and 6. Ranks in types
   would close the biggest remaining hole (rank and shape errors are
   run-time errors in all X_eTaL, not only in macros) and deserves its
   own review before any decision.

Nothing here changes the language until it is decided with the user
and recorded in `docs/lang-choices.md`.
