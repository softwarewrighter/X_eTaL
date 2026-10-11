# Records and choices, array first: a proposal

A proposal for review (2026-10-10), replacing Saga 29's plan of
records, sum types and pattern matching in the ML style. Nothing here
is decided; the questions at the end are for the user. Position A
below is built as a macro library to try (X_eTaL-libraries' Tags). Where this note
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

Four questions, across statically typed languages and array
languages: how choices are written, how two values of one shape are
kept apart, how parts are named, and how many records are stored.

### Choices (enumerations)

Three tiers:

1. Named numbers, no type safety: C's `enum` (ints, converted
   silently); APL, J, K and BQN use integer codes with named
   constants, or text. Go sits between: `type Color int` with
   `const (Red Color = iota ...)` keeps `Color` apart from `int`, but
   the set of values is open (`Color(42)`) and nothing checks that
   every case is handled.
2. A distinct type with a closed list of values, stored as small
   integers: Pascal and Ada (`type Color is (Red, Green, Blue)`, its
   own ordered type, usable as an array's index type: `array (Color)
   of RGB`, `for C in Color loop`); q's enumerations (`` `color$v ``,
   integer indices into a domain list, the type recording which
   domain: how q stores categorical columns and foreign keys); pandas'
   categorical columns (codes and a list of categories); Julia's
   `@enum Color red green blue`, a macro writing a distinct
   integer-backed type; Dex, a typed array language, where an
   enumeration is an index set and an array is typed by its index set
   (`Color=>Float` is a table with one entry per color).
3. Sum types, each case with data of its own, taken apart by
   exhaustive matching: Rust, Haskell, OCaml, Swift, F#, Elm, Zig; and
   Futhark, a typed array language (`#circle f32 | #rect f32 f32`, a
   constructor marked by `#`, `match` checking every case). OCaml's
   polymorphic variants (`` `Red ``) need no declaration.

Every array-friendly design (q, pandas, Julia, Ada) stores a choice
as a code and a list of names. The tag type of this note is tier 2;
Pascal, Ada and Dex suggest indexing a table directly by a choice.

### Distinct types for one shape (brands)

| Language | Mechanism | Cost when it runs |
| -------- | --------- | ----------------- |
| Ada | derived types, `type Meters is new Float` | none |
| Go | defined types, `type UserID int` | none |
| Haskell | `newtype`; phantom type parameters | none |
| Rust | newtype structs, `struct Meters(f64)` | none |
| F# | units of measure, `float<m>`, checked through arithmetic | none (erased) |
| TypeScript | branded types, `string & { __brand: "UserId" }`: a tag field that exists for the checker | none |
| OCaml, ML | abstract types behind a module signature | none |

TypeScript is closest to X_eTaL: its types are structural, like
X_eTaL's tuples, so the way to keep two values of one shape apart is a
tag field, which is what a tag as the first part does.

### Named parts (records)

- Nominal: C, Go, Rust, Swift and Haskell structs and records.
  Haskell shows a cost: field names are ordinary functions in the
  module's one name space, so two records cannot both have a field `w`
  without an extension -- the clash generated accessors meet in
  X_eTaL (hence `u:s_tateW`, not `u:w_`).
- Structural: Elm, TypeScript, Futhark, Dex; in Futhark and Dex a tuple
  is a record with numbered fields.
- None: APL (namespaces such as Dyalog's `[]NS` as objects; inverted
  tables for data), J (locales), K (dictionaries, `` `w`k!(1.0;0) ``),
  BQN (namespaces, `ns.w`); no static types.
- Accessors written from a declaration: Uiua's data definitions write
  a constructor and a getter per field, and its variants are boxed
  arrays whose first item is a tag (as far as is known here; to be
  verified); Julia's StructArrays.jl and others by macro; Zig by
  compile-time code.

### A tag as the first element, by convention

Erlang and Elixir: `{ok, Value}`, `{error, Reason}`, a tuple whose
first part is an atom, the language's central idiom (Gleam, statically
typed on the same machine, replaced it with declared custom types to
check it). C: a struct of an enum tag and a union. TypeScript:
discriminated unions, objects sharing a `kind` field of a literal
type, narrowed by the checker on that field: the statically typed form
of the idea.

### Many records: rows or columns

Columns: q (a table is a dictionary of columns), APL inverted tables,
Futhark (an array of tuples or records stored as one array per field,
automatically), Julia's StructArrays.jl, Zig's `MultiArrayList`,
pandas. Rows: NumPy structured arrays, C arrays of structs, Rust's
`Vec<Struct>`. Array-oriented systems choose columns, so operations
stay on whole arrays.

### Where X_eTaL could sit

| Position | Precedents | New in X_eTaL | Checked before running |
| -------- | ---------- | ------------- | ---------------------- |
| A. Codes, constants, columns, macros | APL, J, K, BQN, C enums, Erlang | nothing | no |
| B. One closed tag type, macros, columns | Ada and Pascal enumerations, q enumerations, Julia `@enum`, TypeScript brands and discriminated unions | `[]T_AG` | choices and tagged tuples |
| C. Records and sums built in, with matching | Futhark, Dex, Elm, Rust, Haskell | record and sum syntax, declarations, constructors, matching, exhaustiveness | everything |

Futhark and Dex, the typed array languages closest to X_eTaL, chose C:
it works with arrays, at the most language. The classic array
languages chose A and gave up checking for these cases. B is where
Ada, q, Julia and TypeScript arrived separately: one nominal type with
a closed list of values, stored as codes, the rest by convention or
macro. What B cannot do is a case carrying data of its own kind (a
circle's radius beside a rectangle's sides); in B those are columns, a
vector of kinds and an array per kind of data, as q or APL would write
them.

## Built: position A, by macro (X_eTaL-libraries' Tags)

Position A is built, with no change to X_eTaL, as the macro-only
library Tags in X_eTaL-libraries (`libs/Tags`, its page
`libs/Tags/docs/README.md`), so the idioms can be tried on real
programs before a tag type is decided:

```
"tg:" u_se< "Tags"
@ tg:e_num< "Heading: north east south west"
@ tg:p_arts< "Turtle: at heading steps"
```

The first writes `NORTH := 1` ... `WEST := 4` and `HEADING := "north"
"east" "south" "west"`; the second writes `TURTLE := "Turtle"`, the
constructor `u:t_urtle (at, heading, steps)`, the test `u:t_urtle?`,
a getter per part (`u:t_urtleAt`, `u:t_urtleHeading`, ...), a setter
per part keeping its type (`x u:s_etTurtleAt s`), and `u:t_urtleRows`
for a table of columns. Its turtle demo turns and moves by tables
indexed by the heading:

```
RIGHT := EAST c_at SOUTH c_at WEST c_at NORTH        # the heading after a right turn, by heading
MOVE := 4 2 r_eshape 0 1 1 0 0 -1 -1 0             # a step's x and y, by heading
u:r_ight := { s -> ((u:t_urtleHeading s) s_elect RIGHT) u:s_etTurtleHeading s }
start := u:t_urtle (0 0, NORTH, 0)
```

What it checks, and when: a tuple of another shape, and a setter given
a value of another type, are type errors before the program runs;
another kind of the same shape stops with `error[wrong-kind]` when it
runs (the tag is text); a choice of one enumeration where another is
expected is not caught (both are Int). Those last two are exactly what
the tag type of position B would move to check time. Building it found
an X_eTaL bug, X_eTaL-libraries' ask X19: hygiene (MC30) can rename a
binder a macro wrote but not its reference when the macro took its
argument apart; Tags works around it by naming the parameters of the
functions it writes by position.

## Questions for the user

1. The three idioms (array and named positions, tuple and accessors,
   columns) as the style guide's rule for when a tuple is justified
   (lang-choices section 16)?
2. The accessor macro, built as X_eTaL-libraries' Tags: keep it
   there or move it to X_eTaL's `lib/`; its names (`e_num<`,
   `p_arts<`) and the names it writes (getter `u:s_tateW`, setter
   `u:s_etStateW`, rows `u:s_tateRows`, constants in capitals).
3. Type identity: a run-time tag first (option 1), the tag type
   (option 3) now, or neither until a program needs it?
4. Choices: the tag type as enumerations, with `Color` and `Key`
   moved onto it, and text results such as `[]E_KIND` given enum
   types? Before deciding, survey the repositories for choices
   spelled as text or codes (as M1 was decided on a survey).
5. Saga 29: replace its plan with this one (the accessor macro and
   the style rule; then the tag type and enumerations if chosen),
   and drop records, sums and matching from the roadmap?
