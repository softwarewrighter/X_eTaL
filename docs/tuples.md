# Tuples: what X_eTaL lacks, what it would take, and how others do it

A design note (asked for by the user, 2026-10-03), the starting point
for Saga 29 (algebraic data) in `plan.md`.

## Where X_eTaL is

X_eTaL has no tuples, records or sum types. Every array, nested ones
included, has one element type (T7), and a boxed item unifies only
with a box of the same type (`Box a`, B14), so a pair of an Int and a
text, `(e_nclose 3) c_at e_nclose "abc"`, is a type error. A state of
mixed types (an Int count with a Float grid; X_eTaL-demos' ask D7)
must be packed by hand into one type. Choices are not typed either: the
first built-in enumerated types (`Color`, `Key`, decision QD6) arrive
with the terminal, made general so Saga 29 can absorb them.

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

Saga 29 steps 1 and 2 (the decisions, then tuples): roughly a saga of
medium size. The hard parts are decisions, not code: projection,
arrays of tuples, and how tuples relate to `Box` and to curried
functions (functions stay curried; tuples are data).

## How other languages do it

| Language | Tuples | Where they fall short |
| -------- | ------ | --------------------- |
| SML, OCaml, F# | Fully built in: `(1, "a") : int * string`, any arity, pattern matching, structural equality | No programming generic over arity (one function for every tuple size) |
| Haskell | `(Int, String)`, pattern matching | `fst` and `snd` only for pairs; library instances (Show, Eq, ...) stop at a fixed arity (about 15) |
| Rust | `(i32, &str)`, `.0` projection, destructuring | Traits implemented only up to 12 elements |
| Swift | Labelled tuples `(x: 1, y: 2)` | Tuples cannot conform to protocols |
| Scala 2, Scala 3 | Scala 2: `Tuple1` to `Tuple22`. Scala 3: generic tuples of any arity | Scala 3 comes close to complete |
| TypeScript | `[number, string]`, labelled and variadic tuple types | Types erased at run time; arrays underneath |
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

## See also

- `plan.md`, Saga 29 (algebraic data: tuples, records, enums, matching)
- `lang-choices.md`, T7 (rank-erased array types), B14 (`Box a`), QD6
  (the first built-in enums)
- `wish-list.md`, named records
