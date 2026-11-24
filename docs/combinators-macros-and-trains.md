# Combinators three ways: library, trains, and macros

A design note. X_eTaL can spell a combinator (a function that only
rearranges or combines other functions and their arguments) in three
ways today or soon: as an ordinary function in a library, as a train
in square brackets, and, once macro libraries exist (Saga 19, decision
MC10), as a macro that rewrites source before it runs. This note says
what each is good for, what each cannot do, and why.

## The five-year-old version

- A **library** combinator (`c:B_`, `c:Y_`) is a **tool in a
  toolbox**: you pick it up while the program runs and use it on
  whatever is in your hands at that moment.
- A **train** (`[f_ g_ h_]`) is a **shorthand the language already
  knows**: the parser turns it into an ordinary lambda before anything
  runs, always the same way.
- A **macro** combinator (`c:B_<`, `c:Y_<`, planned) is a **stencil**
  the compiler lays over the words you wrote, once, before the program
  runs, leaving plain code behind.

In one line: macros see only your text, libraries see only your
values, and trains are the few stencils built into the language.

## Library combinators (`Combinators.xtl`)

Today: `"c:" u_se< "Combinators"`, then `'f_ 'g_ c:B_ x` and the rest
of Smullyan's typed birds (docs/literate/birds.org).

Pros:

- A combinator is a value: it can be quoted and passed (`'c:B_`),
  stored, given to `e_ach`, chosen while the program runs.
- Its arguments can be anything computed at run time.
- One definition, type-checked once as a function; its type documents
  what it does (`c:B_ : (b -> c) -> (a -> b) -> a -> c`).

Cons:

- Every use costs calls at run time: `'f_ 'g_ c:B_ x` is three curried
  applications of a closure before the real work starts.
- `c:Y_` (`Y f = f (Y f)`) only terminates because the function's self
  parameter is lazy (`~s_elf`), so every recursive call forces a thunk.

## Trains (built into the language)

`[f_ g_]` (atop), `[f_ g_ h_]` (fork), and the same between two values,
are desugared into lambdas before type checking (D45): a train costs
nothing extra at run time, and the type checker sees ordinary code.

Pros: no library, no import, no run-time cost; terse; the shapes cover
the commonest combinators (see the table).

Cons: a fixed set of shapes, chosen by the language, not by the user; a
train is always monadic when quoted or named (TR4); some combinators
(Y, Cardinal on its own, Thrush) have no train.

## Macro combinators (`Combinators.xtlm`, planned: Saga 19 step 051)

A macro library beside `Combinators.xtl`, loaded with it under the same
alias (MC11): `c:Y_` stays the function, `c:Y_<` is the macro. A macro
is an ordinary X_eTaL function from source text to source text (MC10),
run before the program is checked or run.

Pros:

- Runs once, at compile time: `"f_" c:B_< "g_"` can expand to
  `{ x -> f_ g_ x }`, so the combinator itself costs nothing when the
  program runs, as a train does, but for any combinator, chosen by a
  library rather than by the language.
- `c:Y_<` can tie the recursive knot in the source: it renames the
  function's self parameter to the name being defined, giving plain
  named recursion - no lazy parameter, no thunk per call, faster, and
  a concrete way to see what Y means.
- `xetal expand` shows exactly what runs, and the expansion is still
  type-checked: a macro cannot sneak an ill-typed program past the
  checker.

Cons:

- It sees only the text written at that spot: the function it rewrites
  must be spelled out in the call, not arrive as an argument.
- It is not a value: `'c:Y_<` cannot be passed to `e_ach` or chosen
  while running.
- It works on strings, so renaming (`s_elf` to the function's name)
  must touch whole names only.
- Errors appear in generated code, so diagnostics must point back at
  the macro call (Saga 27).
- Inlining in many places makes a bigger program.
- Text duplication changes evaluation: a macro for the Warbler that
  writes its argument twice evaluates it twice (and runs its effects
  twice) unless it binds it to a name first; one for the Kestrel that
  drops an argument never evaluates it. A careful macro binds
  arguments once (`{ v -> f_ v v } (x)`).

## Proposed: two more train shapes (step 201)

Brainstormed with the user (2026-11-23) while writing APL's PRIMES,
`(2=+/[1]0=(iota N)jot.|iota N)/iota N`, as one train. Today it needs a
lambda for the comparisons and a fork for the self table (it runs, in
demos/classics/primes.xtl):

```
[[[{ 2 = '+ r_/_2 0 = _r } [i_d 'm_od t_able i_d]] r_eplicate i_d] r_ange]
```

Two gaps show, and step 201 decides both, to be written test-first
once decided (each with its rejection tests, rules 8 and 9):

1. **A constant in a fork's left slot** (Dyalog and J allow it): in
   `[A g h]` with A a value, `[A g h] x` is `A g (h x)`, and dyadically
   `x [A g h] y` is `A g (x h y)`. Today a value in a train is
   `error[bad-train]: a train holds functions only`, so `2 =` has to be
   wrapped as `{ 2 = _r }` or named (`u:t_wo := ('=)_ 2`). Class still
   comes from tokens: a number, a string or a value name is a value,
   so the reading is never ambiguous. A constant in the right or middle
   slot stays an error.
2. **A one-element train is selfie and commute**, as APL's commute
   operator (U+2368) is both: `[F] x` is `x F x` (the Warbler) and
   `x [F] y` is `y F x` (the Cardinal, as `s_wap`). Today `[F]` is
   `error[bad-train]: a train needs at least two functions`, so the
   slot is free. It needs no new name or token, and it takes a derived
   function as it is: `['m_od t_able] r_ange 5` is the remainder
   table. The display may draw it with the commute glyph. `[]` and a
   bracketed value (`[3]`) stay errors.

With both, PRIMES reads close to the APL:

```
[[2 = '+ r_/_2 0 = ['m_od t_able]] r_eplicate i_d] r_ange
```

Considered and set aside:

- A built-in name (`s_elfie`, `r_eflex`, J's word, `w_arbler`): a
  higher-order built-in takes its function first, and a derived
  function such as `'m_od t_able` cannot be passed as one, so
  `'{ _l m_od t_able _r } r_eflex` is no shorter than the fork it
  replaces. `s_elf` is also the conventional lazy self parameter
  (`{ ~s_elf n -> ... }`).
- `:)` (a face, like the APL glyph): lexes without a clash, but an
  unbalanced parenthesis in source breaks every tool that counts
  brackets (editors, the formatter, macro text).
- `:~` (a tilde with two dots, which is what the glyph is): close to
  `~`, the lazy parameter mark.
- `:s_` and a `:x_` family: reads as a namespaced name whose namespace
  was dropped (`x :s_` and `x:s_` would differ only by a space), and
  it would be a third system namespace beside `[]` and the built-ins.
- Atop chains without nesting (BQN's `[F G H K]` with a compose mark):
  mostly unneeded once constants may stand in a fork, since the
  constants caused most of the nesting.

## What cannot be done, and why

| Cannot | Library | Train | Macro | Why |
| ------ | ------- | ----- | ----- | --- |
| Use a value only known while running (a computed count, a function passed in) | can | can | cannot | a macro runs before there are values |
| Be passed, stored or chosen at run time | can | can (quoted, monadic) | cannot | a macro is not a value, it is a rewrite |
| Remove the combinator's own run-time cost | cannot | can | can | only a rewrite before running removes calls |
| Add a new shape of notation | cannot | cannot (fixed set) | can | only a macro library extends what source can say |
| Specialize by type | cannot | cannot | cannot | macros run before type checking; types are inferred after |
| Self-application (Mockingbird `M x = x x`, L, U) | cannot (no finite type) | cannot | cannot as written | the type checker rejects it however it is spelled; Y avoids it by naming (`c:Y_` by recursion, `c:Y_<` by renaming) |

## Which combinator can be expressed how

Built-ins and trains exist today; `c:X_` is the library; `c:X_<` is the
planned macro form. "Inline" means the macro writes the combinator's
lambda in place; any combinator can be inlined when its function
arguments are written at the call.

| Bird | Meaning | Built-in | Train | Library | Macro (planned) |
| ---- | ------- | -------- | ----- | ------- | --------------- |
| I, Idiot | `I x = x` | `i_d` | `[i_d]` slot | `c:I_` | trivial |
| K, Kestrel | `K x y = x` | `l_eft` | `x [l_eft ...] y` slot | `c:K_` | inline (drops y unevaluated) |
| KI, Kite | `KI x y = y` | `r_ight` | `x [... r_ight] y` slot | K applied to I | inline |
| B, Bluebird | `B f g x = f (g x)` | `c_ompose` | atop `[f_ g_] x` | `c:B_` | inline |
| B1, Blackbird | `B1 f g x y = f (g x y)` | | dyadic atop `x [f_ g_] y` | `c:B_1` | inline |
| C, Cardinal | `C f x y = f y x` | `s_wap` | proposed: `x [f_] y` | `c:C_` | inline (swaps the text) |
| W, Warbler | `W f x = f x x` | | `[i_d f_ i_d] x`, proposed: `[f_] x` | `c:W_` | inline, binding x once |
| S, Starling | `S f g x = f x (g x)` | | hook `[i_d f_ g_] x` | `c:S_` | inline, binding x once |
| Phoenix (S') | `Phi f g h x = f (g x) (h x)` | | fork `[g_ f_ h_] x` | `c:P_hi` | inline |
| Psi | `Psi f g x y = f (g x) (g y)` | | | `c:P_si` | inline |
| dyadic fork | `(x f y) g (x h y)` | | `x [f_ g_ h_] y` | | inline |
| T, Thrush | `T x f = f x` | | | `c:T_` | inline |
| V, Vireo | `V x y f = f x y` | | | `c:V_` | inline |
| D, E, F, G, H, J, O, Q, R, starred birds | rearrangements | | | `c:D_` ... `c:W_ss` | inline (binding duplicated arguments) |
| Y, Sage Bird | `Y f = f (Y f)` | | | `c:Y_` (lazy self) | named recursion: the self parameter renamed |
| function power | `f` applied n times | `p_ower`, `f_^n` (literal n) | | | unrolled only for a literal n |
| M, L, U (self-applying) | `M x = x x` | | | none: no finite type | none (still ill-typed) |

## Summary

- Use a **train** for the common shapes: free, terse, built in.
- Use the **library** when a combinator is data: passed, stored,
  chosen while running, or applied to functions computed at run time.
- Use a **macro** (once available) to remove a combinator's overhead or
  to show what a combinator means, as `c:Y_<` turns a fixed point into
  named recursion - and check the result with `xetal expand`.

See also: docs/literate/birds.org (the birds, each run), and
docs/literate/trains.org (each train beside its bird).
