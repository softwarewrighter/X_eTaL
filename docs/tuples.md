# Tuples: what X_eTaL lacks, what it would take, and how others do it

A design note (asked for by the user, 2026-10-03), and since
2026-10-07 the plan of Saga 39 (lanes/tuples): tuples first, at the
asks of X_eTaL-ML (M13) and X_eTaL-demos (D7), as the first part of
Saga 29 (algebraic data). The questions below were settled with the
user at Saga 39's first step (2026-10-07): every one as recommended.
The rules are TU1-TU12 in `lang-choices.md` section 9d, which governs
where this note and they differ.

## Where X_eTaL is

X_eTaL has no tuples, records or sum types. Every array, nested ones
included, has one element type (T7), and a boxed item unifies only
with a box of the same type (`Box a`, B14), so a pair of an Int and a
text, `(e_nclose 3) c_at e_nclose "abc"`, is a type error. A state of
mixed types (an Int count with a Float grid; X_eTaL-demos' ask D7)
must be packed by hand into one type. Choices are not typed either: the
first built-in enumerated types (`Color`, `Key`, decision QD6) arrive
with the terminal, made general so Saga 29 can absorb them.

## Why now: the asks

`p_ower` repeats a function on one value: `n 'f_ p_ower x` has the type
`(a -> a) -> Int -> a -> a`. A step whose state is several arrays of
different shapes has no one value to carry them in, because every array
has one element type (T7, and boxes too, B14) and there is no product
type. The sibling
repositories write around it:

- X_eTaL-ML, ask M13: `train-live` packs a network's 99 weights and
  Adam's two running averages into one vector of 298 numbers and takes
  it apart with `t_ake` and `d_rop` at fixed offsets every step.
- X_eTaL-demos, ask D7: the wave tank and Langton's ant carry a game
  state of several arrays the same way.
- Here: the Rosetta stone's state machine (`demos/rosetta/Comparison.xtl`)
  is a 5 by 5 Float matrix whose rows are the three axes, the pointer
  and the attract clock, read and written through masks by row and
  column numbers; the Int counts in it are Floats because a matrix has
  one element type.

Boxes help with shapes, not types: a vector of boxed arrays can carry
W1, W2 and Adam's averages, each in its own box, but every box holds the
same element type (no Int step count beside Float weights), and the
parts are still taken apart by position (`d_isclose 3 s_elect s`).

Combinators cannot fix it. `p_ower` already iterates any one-argument
function; what is missing is a value that holds the parts. A Church
pair (a function handing its parts to whatever function it is given)
carries them, but under Hindley-Milner it types only when the parts
share one type, and it reads worse than the packing.

Product types are the standard, well-understood extension of
Hindley-Milner inference. They change no existing type, so nothing that
works today breaks; the cost is new syntax and a few type rules.

## What first-class tuples would take

Every layer, as in any language; the architecture keeps each change
local.

| Layer | Change |
| ----- | ------ |
| Syntax | A tuple literal. The comma is free (`(1, "abc")` does not lex today), so it cannot collide with anything; strands stay as they are (`1 2 3` is a vector) and the comma marks a tuple. Each new rule needs its rejections (rule 9): `(1,)`, `1, 2` without parentheses. |
| Core IR | A `Tuple` form and a projection (or pattern) form; the evaluator still never sees syntax. |
| Types | `Type::Tuple(Vec<Type>)` in the checker, unified part by part, printed `(Int, Char)`. The heart of the work. |
| Values | `Value::Tuple(Rc<[Value]>)`, printed `(1, abc)`, drawn with boxes by DISPLAY; `=` when every part has equality. |
| Taking apart | Pattern parameters, `{ (n, grid) -> ... }`, and projections with a literal position. A computed index cannot be typed: the type of the part depends on the index. |
| Arrays of tuples | Arrays have one element type (T7), so `(Int, Float)` can be it; whether a reshape or `e_ach` over tuples stores them as a tuple of arrays (as Futhark does) is a decision. |
| Tooling | The formatter, renderer, LaTeX, Emacs mode, live demo and reference. |

Saga 39 (the decisions, then tuples; below): roughly a saga of medium
size. The hard parts are decisions, not code: projection,
arrays of tuples, and how tuples relate to `Box` and to curried
functions (functions stay curried; tuples are data).

## What the saga delivers

The minimum that removes the workarounds: a tuple literal, a tuple
type, and taking a tuple apart by a pattern, in a binding and in a
lambda's parameters, accepted by `p_ower` (and by every built-in whose
type is polymorphic, without changing them). Named fields (records),
sum types and matching stay in Saga 29 and build on these types.

The train-live step, once tuples land, would read something like this
(a sketch in the recommended syntax, not a pinned example; `u:g_rad`,
`eps` and `rate` stand for the demo's own):

    u:s_tep := { (w, m, v, k) ->
      g := u:g_rad w
      m2 := (0.9 * m) + 0.1 * g
      v2 := (0.999 * v) + 0.001 * g * g
      (w - rate * m2 / eps + v2 ^ 0.5, m2, v2, k + 1)
    }
    (w, m, v, k) := 500 'u:s_tep p_ower (w0, 0.0 * w0, 0.0 * w0, 0)

and `xetal type` would show `u:s_tep : (Float, Float, Float, Int) ->
(Float, Float, Float, Int)`.

## The decisions (step 1, 2026-10-07)

Each was the recommendation below, chosen to keep the grammar
unambiguous (rule 3: two parses are an error, never a guess) and to
leave room for records; the user took every one. The rules as recorded
are TU1-TU12 in `lang-choices.md` section 9d.

| #    | Question | Decision (as recommended) |
| ---- | -------- | -------------- |
| TU1  | The literal | Parentheses and commas: `(w, m, v, k)`. The comma is unused today (a lex error), and `;` is taken (statements and guards). A comma is legal only directly inside parentheses or a pattern; `(x)` stays grouping; there is no one-element tuple; the empty tuple is not needed (`@` is unit). Each component is a whole expression, read right to left on its own. |
| TU2  | The type | `(Float, Float, Float, Int)`, written and printed as the literal is; components are the ordinary rank-erased types (B14), so `(Float, Int)` is a pair of a Float array of any rank and an Int array of any rank. Polymorphic components unify as usual: `(a, b) -> (b, a)`. |
| TU3  | Taking apart | Patterns: `(w, m, v, k) := s` at the top level and inside lambdas, and in parameters, `{ (w, m) -> ... }` (monadic) or `{ (w, m) x -> ... }` (dyadic, the left argument a pair). Patterns nest: `((a, b), c)`. Projection by a literal position (this note's first sketch) is the open part: recommended out of v1, so that patterns are the one way and named access is the records' job (Saga 29); a computed position cannot be typed either way. |
| TU4  | Ignoring a part | A wildcard in a pattern, if it lexes unambiguously: `(w, _, _, k) := s`. To check against `_r` (the lambda argument) and the underline decoration; if `_` cannot be a token on its own, an ordinary unused name. |
| TU5  | Mismatch | A pattern of the wrong size, or a tuple where an array is expected, is a type error with spans on both sides ("a pair where a 4-tuple is expected"), never a run-time one. |
| TU6  | Printing | Short components on one line, `(1 2 3, 4.5, 7)`; a component that prints on several lines (a matrix) puts every component on its own block, each introduced by its position. `--box` and DISPLAY draw a tuple as a box with a distinct corner. The exact layout is pinned by goldens. |
| TU7  | Equality | `m_atch` compares tuples part by part. Whether `=` does too (this note's first sketch: when every part has equality) or stays elementwise on arrays only is to decide; recommended: `m_atch` only in v1, since `=` on arrays answers an array, not one truth. Arithmetic does not reach into tuples in v1: `(1, 2) + 1` is a type error; pervasion through tuples can be added later without breaking anything. |
| TU8  | Tuples in arrays | This note's first sketch lets `(Int, Float)` be an array's element type, stored as a tuple of arrays as Futhark does. Recommended in two stages: in v1 a tuple can be boxed (`e_nclose (1, 2.5)` is a `Box (Int, Float)`), so `m_ap` of a function returning a tuple works (it boxes each result) and a vector of boxed tuples is the array of tuples; unboxed arrays of tuples, with the primitives (reshape, take, each, indexing) defined on them, come in a later step once v1 is in use. |
| TU9  | Higher-order built-ins | Unchanged: their types are polymorphic, so `p_ower`, `c_ompose`, `s_wap` and the like accept tuples as soon as the checker has product types. `e_ach` needs a scalar from each call and stays so; `m_ap` is the way to collect tuples. |
| TU10 | Core | A tuple is a Core node of its own, and a pattern desugars to bindings of projections, so the evaluator never sees surface patterns (rule 5). Each surface form has a normalization test: `{ (a, b) -> a }` and the same lambda written with a binding of its argument give identical Core (rule 8). |
| TU11 | Macros | Hygiene (MC30) treats every name a pattern binds as a binder. The formatter round-trips every form (rule 10). |
| TU12 | Records, later | Leave room for named fields in the same brackets; a field syntax is Saga 29's to choose, but nothing in TU1 to TU11 may claim it (for instance, `(w: ..., m: ...)` or `(w = ..., m = ...)` must stay free). |

## Steps

| Step | Slug | Content |
| ---- | ---- | ------- |
| 1 | tuples-decisions | With the user: TU1 to TU12 decided and recorded in lang-choices (a section of its own) and the register; every rule's spec case written, pending. |
| 2 | tuples-values | The comma token, the parenthesized tuple, the Core node, evaluation, printing (TU6) and the formatter; rejections (`1, 2` outside parentheses, a trailing comma); `m_atch` on tuples. |
| 3 | tuples-types | Product types in the checker (TU2, TU5, TU7); `xetal type` prints them; `p_ower` with a tuple state checks and runs (the M13 repro). |
| 4 | tuples-destructure | Patterns in bindings and lambda parameters, nested, with the wildcard if TU4 allows it; size mismatches as type errors; hygiene (TU11); normalization tests (TU10). |
| 5 | tuples-in-arrays | Boxed tuples (TU8): `e_nclose`, `d_isclose`, `m_ap` returning tuples, DISPLAY of boxed tuples. |
| 6 | tuples-tools | The renderers (decorated, canonical, expanded), the live demo's output and Boxed view, the Emacs mode, the syntax poster, the language reference, `xetal doc`. |
| 7 | tuples-retrofit | Programs that pack state rewritten: the Rosetta stone's Comparison state first (its Int counts become Ints), then any demo or library here that packs; goldens rebased on purpose; the siblings told how (asks M13 and D7). |
| 8 | tuples-release | The README tour, a literate document (the train-live step, before and after), the register, CHANGES, pages; Saga 29 replanned on these types (records as named tuples); the lane archived. |

## How other languages do it

| Language | Tuples | Where they fall short |
| -------- | ------ | --------------------- |
| SML, OCaml, F# | Fully built in: `(1, "a") : int * string`, any arity, pattern matching, structural equality | No programming generic over arity (one function for every tuple size) |
| Haskell | `(Int, String)`, pattern matching | `fst` and `snd` only for pairs; library instances (Show, Eq, ...) stop at a fixed arity (about 15) |
| Rust | `(i32, &str)`, `.0` projection, destructuring | Traits implemented only up to 12 elements |
| Swift | Labeled tuples `(x: 1, y: 2)` | Tuples cannot conform to protocols |
| Scala 2, Scala 3 | Scala 2: `Tuple1` to `Tuple22`. Scala 3: generic tuples of any arity | Scala 3 comes close to complete |
| TypeScript | `[number, string]`, labeled and variadic tuple types | Types erased at run time; arrays underneath |
| Julia | `Tuple{Int,String}`, NamedTuple, any arity, dispatch on them | Close to complete |
| C++ | `std::tuple` (a library), `get<N>`, structured bindings | Not built into the language; clumsy |
| Python | Immutable heterogeneous sequences | Dynamic; type hints only advisory |
| Java, Go | Java: none (records instead). Go: multiple return values only | Not values you can pass around |
| APL, J, K, BQN | No tuples: nested arrays may mix types freely | Dynamically typed, so no tuple types |
| Futhark (a typed array language) | Tuples and records; an array of tuples is stored as a tuple of arrays | The closest relative of what X_eTaL wants |
| Idris, Agda | Tuples are nested dependent pairs | Fully general, at the cost of dependent types |

### Does any language support tuples fully?

"Fully" would mean literals, any arity, typed heterogeneous parts,
pattern matching, equality and printing for every arity, and code
generic over arity. Dependently typed languages (Idris, Agda) get there
through general type-level programming. Among mainstream languages,
Scala 3 and Julia come closest. The ML family has everything except
programming generic over arity. Haskell, Rust and Swift stop at
practical limits (instances or traits up to a fixed arity, protocols).

For X_eTaL the natural target is the ML family's level (every arity
built in, nothing generic over arity) plus Futhark's arrays of tuples.

## Alternatives considered

- **Boxes only** (no new type): works today for parts of one element
  type; fails for mixed element types and keeps positional access.
  Recommended to the sibling repositories as the stopgap until this
  saga lands.
- **Records first**: named fields read better, but they need a
  declaration form, field access and update syntax, and nominal types;
  tuples are the smaller step that removes every workaround above, and
  records can be built as named tuples on top of them.
- **A packed vector with named offsets** (a library of `t_ake` and
  `d_rop` helpers): hides the arithmetic but keeps one element type and
  no checking.

## See also

- `plan.md`, Saga 39 (tuples, lanes/tuples) and Saga 29 (algebraic
  data: records, enums, matching, on Saga 39's types)
- `lang-choices.md`, T7 (rank-erased array types), B14 (`Box a`), QD6
  (the first built-in enums)
- `wish-list.md`, named records
