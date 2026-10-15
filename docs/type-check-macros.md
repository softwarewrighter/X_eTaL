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
with one change each, on the current binary):

| Change | Kind of error | Caught | Reported at |
| ------ | ------------- | ------ | ----------- |
| `X := "abcd"` | element type (Char, not number) | compile time | the whole call (`at 276..372`) |
| summand `X[c,h+q,w+s]` | unknown name `q` | compile time | the whole call |
| `X` of shape `8 4` (rank 2, three indices used) | rank | run time, `error[index]` | the whole call |
| summand `X[c,h+r]` (two indices for rank 3) | rank, in the notation | **never: a wrong answer** (`162 162 162 ...`) | nowhere |

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
4. **Research, not scheduled:** approaches 5 and 6. Ranks in types
   would close the biggest remaining hole (rank and shape errors are
   run-time errors in all X_eTaL, not only in macros) and deserves its
   own review before any decision.

Nothing here changes the language until it is decided with the user
and recorded in `docs/lang-choices.md`.
