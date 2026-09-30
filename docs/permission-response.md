# XeTaL's answers to ngn's permission request

ngn's [Array Language Implementation Permission
Request](https://ngn.codeberg.page/funny/reg.html) is partly satire, but
its serious questions make a compact language-design checklist. Here it
is filled in for XeTaL, checked against the repository
(`docs/lang-choices.md` and the spec).

| Question | XeTaL answer | Status |
| -------- | ------------ | ------ |
| What kind of implementation? | Statically typed | Implemented |
| Lazy? | Mostly no: strict by default, with explicit `~` lazy parameters using call-by-need | Specified and implemented |
| Implementation of? | A new APL-family language, drawing particularly from APL2, Dyalog, J and BQN | Intentional design |
| Implementation language? | Rust | Implemented |
| Purpose? | Expressivity and readability, static safety, experimentation | Core goal |
| Character set? | ASCII-only source; Unicode and LaTeX presentation | Implemented and enforced |
| Target platform? | Portable native, and the web through WebAssembly | Implemented (the live demo) |
| Index origin? | 1 | Decided, implemented, not configurable |
| <code>&#8835;</code> and <code>&#8593;</code> behavior? | Closer to APL2 and BQN; XeTaL uses named operations, not those glyphs | Nested-array model decided; nesting still phased |
| Comparison tolerance? | None globally: `=` is exact; `e_q~` is explicitly tolerant (about 1e-14, relative) | Decided and implemented |
| CAPTCHA: character scalars? | XeTaL has rank-0 Char values; strings are Char vectors | Yes |

A few of these deserve more precise answers.

## 1. "I would like to make a ..."

Statically typed. XeTaL really is statically typed, rather than "typed"
being aspirational: programs are checked before they run, and there is
`--untyped` for experiments. The design uses Haskell-like polymorphic
typing with rank-erased array types: `7` and `1 2 3` can both have
element type Int; rank and shape are runtime properties.

"Lazy" gets a qualified answer. XeTaL is strict by default, but `~`
parameters are call-by-need. So on ngn's form:

```
[X] statically typed
[ ] lazy (except explicitly lazy parameters)
```

That is a fairly distinctive combination.

## 2. "... implementation of APL/J/K ..."

None of the boxes is exactly right. XeTaL is an APL-family language:
APL2 and Dyalog semantics with J and BQN influences, rather than an
implementation of any one of them. The repository compares XeTaL with
APL2, Dyalog APL, J and BQN directly (`docs/idioms.md`). For example, it
chooses 1-origin indexing like APL2 and Dyalog while taking
leading-axis defaults from J and BQN. So XeTaL is not "APL implemented
in Rust with funny syntax": it makes explicit semantic choices among
the descendants.

## 3. Purpose

Of ngn's intentionally silly choices, Expressivity is closest, but
XeTaL has a more concrete research purpose: to explore whether
APL-style whole-array programming can combine static typing,
functional composition, readable ASCII source, mathematical
presentation, visualization and modern tooling without losing
concision. That is the README's own statement of its design space.

## 4. Character set

An unusually unambiguous answer: ASCII only, for the canonical program
representation. XeTaL separates the source from its presentation:

```
ASCII source --> lexer, parser, type checker --> program

ASCII source --render--> Unicode
             --render--> LaTeX
```

Unicode is not accepted as source: a conformance test
(`spec/lex/reject-non-ascii.case`) feeds <code>x &#8592; 3</code> and requires a
non-ASCII error. So the complete answer is: ASCII required for
source; Unicode and LaTeX encouraged for presentation. Typography is a
lossless view of the program, not its storage representation.

## 5. Target platform

The form's choices (Solaris, MacOS 9, Palm, Blackberry, System/Z,
Plan9, Android, Symbian) are deliberately awful. XeTaL's real answer:
portable Rust, native and in the browser through WebAssembly, since
the language's semantics do not depend on a particular host.

## 6. Index origin

Definitive: 1. And 1-origin is part of the language, not a mutable
setting like APL's index origin: `r_ange 5` is `1 2 3 4 5`, and index 1
selects the first item. Where zero-based displacement is natural,
`o_ffsets n` gives `0` to `n - 1`. Indices are 1-origin; offsets can
naturally be 0-origin; those are different concepts.

## 7. "<code>&#8835;</code> and <code>&#8593;</code> should ..."

The literal choices are Dyalog, GNU APL, the migration level, random,
transpose, or a rickroll. XeTaL checks none of them, since it does not
use those glyphs. Its design (A7): nested arrays follow APL2 and BQN,
any element may itself be an array, with enclose and disclose
built-ins and no explicit box type. Eventually `"ab" "cde"` will be a
two-element vector whose items are Char vectors.

Nested arrays are planned, not built: today's arrays are flat. So the
answer is APL2- and BQN-like: specified, not yet implemented. The
Dyalog question <code>1&#8801;&#8834;1</code> becomes a conformance test when
enclose and disclose land (today, for a simple scalar, it is
`1 m_atch 1`).

## 8. Comparison tolerance

Effectively the form's "no tolerance", with a qualification: XeTaL has
no global comparison tolerance. `=` is exact equality and `e_q~` is
tolerant equality, with a fixed relative tolerance of about 1e-14:
`(0.1 + 0.2) = 0.3` is 0 and `(0.1 + 0.2) e_q~ 0.3` is 1. No invisible
state changes what `=` means, and the code documents itself: `x = y`
means equal, `x e_q~ y` means numerically close enough.

## 9. CAPTCHA: "Select the squares with character scalars"

XeTaL has a real Char element type, and strings are Char vectors. The
type system is rank-erased, so rank is not part of the static type: a
character scalar is a rank-0 Char, and `"abc"` is a rank-1 Char array
of shape 3. Every value is an array; a scalar is one of rank 0.

## The completed form

```
ARRAY LANGUAGE IMPLEMENTATION PERMISSION REQUEST

From: Software Wrighter

I would like to make a
    [X] statically typed
    [~] selectively lazy
implementation of
    [X] none of the above: a new APL-family language
        drawing on APL2, Dyalog, J and BQN
in
    [X] Rust
Purpose:
    [X] Expressivity
    [X] static safety
    [X] readable array programming
    [X] make ML and math algorithms small enough to see
Character set:
    [X] ASCII only -- SOURCE
    [X] Unicode and LaTeX -- PRESENTATION
Target platform:
    [X] native
    [X] WebAssembly, in the browser
    [ ] Blackberry
Index origin:
    [ ] 0   [X] 1   [ ] 0.5   [ ] quantum superposition
    Not configurable; o_ffsets exists when 0-origin offsets are wanted.
Disclose and take should:
    [X] not exist under those glyphs
    [X] use APL2/BQN-like nested-array semantics, via enclose and disclose
Comparison tolerance:
    [X] none: = exact; e_q~ tolerant (about 1e-14, relative)
CAPTCHA:
    Character scalar = rank-0 Char.  String = rank-1 Char array.
```

## What to test next

XeTaL now has concrete answers to essentially every non-joke question
on the form. The area to turn into spec tests next is the APL scalar
and nesting edge cases: enclose of a scalar, disclose, nested match,
empty nested arrays, and first and pick on nested arrays. Those decide
whether "APL2/BQN-like" is precise enough.

See also [XeTaL fills in the Programming Language
Checklist](why-another-language.md) and [XeTaL and the APL skeptic's
bingo card](xetal-apl-skeptics-response.md).
