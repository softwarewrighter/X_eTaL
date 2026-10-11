# Records and choices, array first: a proposal

A proposal for review (2026-10-10), replacing Saga 29's plan of
records, sum types and pattern matching in the ML style. Nothing here
is decided; the questions at the end are for the user. Where this note
and `lang-choices.md` differ, `lang-choices.md` governs.

## The aim

X_eTaL is an array-first language that is statically typed and has
macros; tuples (section 9d, TU1-TU12) are a convenience, used
idiomatically in rare, justified cases. The language should stay
small: as much as possible is done with macros and naming conventions,
and a new language feature is added only where neither can do the job.

So the question is not "how do records look in X_eTaL" but "what
problem do records solve, and how is it solved array first, type safe,
with what the language already has".

## What records are for

1. Readability: a part is read by name (`s.k`), not by position.
2. Robustness: adding a part to a positional tuple breaks every
   pattern that takes it apart.
3. Updating one part without writing the whole value again.
4. Type identity: a `State` cannot be passed where a `Config` of the
   same shape is expected.

All four are about a single value with named slots. Array languages
mostly avoid such values: data is kept as whole arrays and named by
convention. APL has inverted tables (one array per field), k has
dictionaries, q has tables of named columns; none has a record type.

## Three idioms, by the shape of the data

### Parts of one type: an array and named positions

A point, a bounding box, a five-number summary is a vector, not a
tuple. Its positions are named by constants:

```
X := 1
Y := 2
p := 3.0 4.0
X s_elect p
```

No feature is needed, and a matrix of points gives a whole column at
once. This is the existing rule of X_eTaL-libraries (its plan A13): a
same-typed grouping that is computed on, joined or vectorized stays an
array.

### One state of parts of different types: a tuple and accessors

This is where a tuple is justified. The names come from functions:

```
u:w_ := { (w, _, _, _) -> w }
u:k_ := { (_, _, _, k) -> k }
u:s_etK := { x (w, m, v, _) -> (w, m, v, x) }
s := (8.0 4.0, 0.0 0.0, 0.0 0.0, 0)
u:w_ s                     # 8.0 4.0
(1 + u:k_ s) u:s_etK s     # (8.0 4.0, 0.0 0.0, 0.0 0.0, 1)
```

The checker infers `u:w_ : (a, b, c, d) -> a`, so `u:w_ (1, 2)` is a
type error before the program runs ("expected a tuple of 4, found a
tuple of 2"). Problems 1 to 3 are solved: functions use names, adding
a part changes only the definitions, a setter changes one part. A
setter can keep the part's type by comparing the new value with the
old one (`_ := x m_atch k`), since `m_atch` takes two values of one
type.

Writing these by hand is boilerplate, so a macro writes them. That is
the user's rule for macros (creating named definitions), and it has a
precedent: X_eTaL-libraries' Bits macro `f_ields<` writes a getter and
a setter per bit field of a packed number. The tuple version:

```
@ p:a_rts< "w m v k"
```

defines `u:w_`, `u:m_`, `u:v_`, `u:k_` and `u:s_etW` ... `u:s_etK`
(the names are a question below).

### Many values: columns (an inverted table)

A list of people is not an array of records but one array per field,
side by side in a tuple:

```
t := ("ann" "bob", 31 42, 1.5 2.5)                # (Box Char, Int, Float)
u:r_ows := { i (a, b, c) -> (i s_elect a, i s_elect b, i s_elect c) }
2 1 u:r_ows t                                     # rows 2 and 1 of every column
```

Each column keeps its own type and every operation is vectorized. The
same macro can write `u:r_ows` and a column accessor per name. This is
the most array-oriented of the three, and a clear case for a tuple.

## Type identity: a tag as the first part

The macro can also add a tag as the first part, so `"w m v k"` makes
a 5-tuple whose part 1 names the kind of value, with a constructor
that adds it and accessors that check it. The catch is the type of the
tag: a macro writes text, and text spells only values of existing
types, so every tag is a Char (or an Int) and two tagged states of the
same shape still have the same type. Verified on X_eTaL main:

```
u:w_ := { (t, w, _) ->
  n_ot t m_atch "State" ? @ p_anic< "not a State"
  w
}
u:w_ ("State", 1.5, 0)     # 1.5
u:w_ ("Config", 1.5, 0)    # error[panic]: not a State, at run time
```

The checker infers `(Char, a, b) -> a`; the mix-up is found only when
it runs. Three ways from here:

1. A run-time tag, no new feature: as above. A mix-up is caught when
   the bad call runs; each access pays one comparison.
2. The name encoded in a type, no new feature: the macro spells the
   name out of existing types (nested `Box` and the unit). Checked
   before the program runs, but types and errors print as gibberish
   and names from different libraries may collide. Not recommended.
3. One small feature, a tag type: a literal tag whose type is its own
   name, `[]T_AG "State" : Tag State`. The argument must be a literal,
   settled when the program is checked (as `[]E_X`'s is, M4); two tag
   types unify only when their names are equal; at run time a tag is
   just its name. The macro still writes everything else; an accessor
   pins the tag's type with `t m_atch []T_AG "State"` (both sides of
   `m_atch` have one type), so a `Config` is a type error before the
   program runs, printed readably as `(Tag State, Float, Int)`. Once
   signature lines (`::`, T4) exist, the macro can write signatures
   instead.

Recommended: start with 1, which needs nothing from X_eTaL and shows
whether states are mixed up in practice; 3 can follow without breaking
those programs (only the tag literal in the macro's output changes).

## Typed choices from the same feature

Choices are spelled as text today (`[]E_KIND` gives `"tick"`,
`"down"`, ...) or as numbers, and the terminal's `Color` and `Key`
(QD6) are nominal types special-cased in the checker. The tag type of
option 3, with a value under a shared type name, is an enumeration:
`"Color" []T_AG "red"` has the type `Tag Color`. Its values are
scalars, so they sit in arrays, compare with `=` and are found with
`i_ndexOf`; a choice dispatches by selecting from a table, as arrays
do, not by matching. A macro names the values (`@ e:n_um< "Color:
black red green"` defining `e:BLACK` ... as Terminal's `t:RED` is
today). `Color` and `Key` would become ordinary uses of it, and their
constructor quads (`[]C_OLOR`, `[]K_NAMED`) retire.

## Not proposed

Record syntax (`(w: 1, k: 0)`, `r.w`, `(k: 2, ..r)`), constructors
as a new kind of name, type declarations, pattern matching on
constructors, exhaustiveness checks, `Maybe` and `Result` (errors have
`[]S_IGNAL` and `t_ry`; absence in arrays is an empty array or a fill
value). TU12 keeps `(w: ...)` an error, so none of these is ruled out
for later; each would need a program that cannot be written well
without it.

## How others do it

| Language | Named parts | Distinct types | Choices |
| -------- | ----------- | -------------- | ------- |
| APL, J | inverted tables, named indices | none | integer codes |
| k, q | dictionaries, tables of columns | none | symbols |
| Futhark | records, arrays of them stored as columns | structural | sum types |
| Haskell | records | `newtype`, phantom types | sum types |
| TypeScript | object types | branded types (a tag field) | literal types |

Option 3 is close to TypeScript's branded types and Haskell's phantom
types: a name that exists for the checker, carried by a value that
costs nothing.

## Questions for the user

1. The three idioms (array and named positions, tuple and accessors,
   columns) as the style guide's rule for when a tuple is justified
   (lang-choices section 16)?
2. The accessor macro: where it lives (X_eTaL's `lib/`, beside
   Combinators, or an X_eTaL-libraries library), its name, and the
   names it writes (getter `u:w_`, setter `u:s_etW`, rows `u:r_ows`;
   whether a setter keeps the part's type).
3. Type identity: a run-time tag first (option 1), the tag type
   (option 3) now, or neither until a program needs it?
4. Choices: the tag type as enumerations, with `Color` and `Key`
   moved onto it, and text results such as `[]E_KIND` given enum
   types? Before deciding, survey the repositories for choices
   spelled as text or codes (as M1 was decided on a survey).
5. Saga 29: replace its plan with this one (the accessor macro and
   the style rule; then the tag type and enumerations if chosen),
   and drop records, sums and matching from the roadmap?
