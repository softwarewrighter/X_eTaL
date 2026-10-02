# Why another language? XeTaL fills in the checklist

The [Programming Language
Checklist](https://www.mcmillen.dev/language_checklist.html), by Colin
McMillen, Jason Reed and Elly Fong-Jones (2011; an expanded 2024
version also circulates), is a form letter for anyone announcing a new
language. It begins "You appear to be advocating a new..." and then
explains, one checkbox at a time, why your language will not work.

XeTaL has concrete answers to a surprising amount of it, so here it is,
filled in. The answers describe the implementation in this repository;
where a checkbox is only partly true, it is marked `[~]`, and where we
honestly do not know, `[?]`.

## "You appear to be advocating a new..."

```
[X] functional          [ ] imperative        [ ] object-oriented
[ ] procedural          [ ] stack-based       [ ] "multi-paradigm"

[~] lazy                [X] eager
[X] statically-typed    [ ] dynamically-typed
[~] pure                [X] impure
[ ] non-hygienic        [~] visual
[ ] beginner-friendly   [ ] non-programmer-friendly
[?] completely incomprehensible
```

The qualifications matter.

**Functional:** definitely. XeTaL is an array language on the surface
with a functional core underneath. Every function takes exactly one
argument, and what looks like a function between two arguments is
currying: `x f_ y` lowers to `App(App(f_, x), y)`, and `xetal core`
shows it. Reduce (`r_/`), each (`e_ach`), outer product (`t_able`),
inner product (`i_nner`) and the rest are ordinary curried functions
that take a quoted function as an argument (`'+ r_/ A`), not a separate
class of APL operators.

**Eager, lazy:** strict by default, but a lambda parameter can be
declared call-by-need with `~`: `{ ~x -> ... }` evaluates its argument
on first use, once, or never. That is selective laziness, not a lazy
language. It is enough to write your own control structures, and to
run the textbook Y combinator.

**Pure, impure:** values are immutable and the core is functional, but
effects are allowed and marked. A variable that may be bound again ends
in `!`:

```
count! := count! + 1
```

and so do the built-ins with effects, such as `p_rint!` and `r_oll!`.
Effects should not be spooky, and the `!` makes them visible where they
happen.

**Visual:** the source is plain ASCII, but it is always shown drawn:
`r_ev` with its r underlined, `u:` as a raised u, `:=` as an arrow.
The drawing is a view of the source, not a second language.

## "You appear to believe that..."

Mostly unchecked.

```
[ ] Syntax is what makes programming difficult
[ ] Garbage collection is free
[ ] Computers have infinite memory
[ ] Nobody really needs:
    [ ] concurrency
    [ ] a REPL
    [ ] debugger support
    [ ] IDE support
    [ ] I/O
    [ ] interaction with foreign code
[ ] The entire world speaks 7-bit ASCII
[ ] Scaling up to large software projects will be easy
[ ] Convincing programmers to adopt a new language will be easy
[ ] Programmers love writing lots of boilerplate
[ ] Undefined behavior is fine
[ ] Spooky action at a distance makes programming more fun
```

Not believing we can do without something is not the same as having
it. XeTaL has a REPL (`xetal repl`), a terminal editor (`xetal edit`),
an Emacs mode with Org Babel support, a live editor in the browser, and
file and keyboard I/O (`[]N_PUT`, `[]N_GET`, `[]R_EAD`). It does not
yet have concurrency, a debugger (a stepping trace is planned), or a
foreign-function interface.

The ASCII item deserves its own answer. XeTaL's source is ASCII, but
not because the world speaks ASCII:

```
[ ] The entire world speaks 7-bit ASCII
[X] Git diffs should
```

It is an architectural decision. The ASCII text is the one source that
every tool works on; the typography is a presentation of it:

```
        ASCII source (canonical)
     |---> compiler, type checker
     |---> git, diff, grep
     |---> terminals, editors
     v
  presentation
     |---> Unicode (the terminal, the browser)
     '---> LaTeX   (papers, KaTeX)
```

## "Unfortunately, your language (has/lacks)..."

This is where the checklist becomes useful.

| Feature | XeTaL |
| ------- | ----- |
| comprehensible syntax | debatable by design; the drawn form helps |
| semicolons | yes, as a statement separator, drawn as a diamond (so is a newline) |
| significant whitespace | a newline separates statements; indentation does not matter |
| macros | yes, but only the system's, named with a trailing `<`: `u_se<` imports a library |
| implicit type conversion | limited: Bool to Int, and numeric literals |
| explicit casting | yes, for example `f_loat` |
| type inference | yes, Hindley-Milner style, with constraints (`Num a =>`) |
| goto | no |
| exceptions | no; errors are diagnostics, and a try/catch design is still open |
| closures | yes |
| tail recursion | recursion yes; tail-call optimization is not part of the semantics |
| coroutines | no |
| reflection | no |
| subtyping | no |
| multiple inheritance | no |
| operator overloading | not in the C++ sense; constrained polymorphism instead |
| algebraic datatypes | not yet |
| recursive types | no; the Y combinator's type is infinite, so it runs with `--untyped` |
| polymorphic types | yes |
| covariant array typing | no |
| monads | not a language mechanism; the `Maybe` library is one |
| dependent types | no |
| infix operators | the surface looks infix, but every function is curried |
| nested comments | no; `#` comments run to the end of the line |
| multiline strings | no; a string stays on one line (use `\n`) |
| regular expressions | no |
| call-by-value | the default |
| call-by-name | no |
| call-by-need | yes, with `~` |
| call-by-reference | no |
| call/cc | no |

XeTaL is not collecting every feature from the research literature. Its
unusual features cluster around one idea: arrays, functional
composition, static inference, and typography.

## Type-system accusations

```
[ ] Your type system is intentionally unsound
[ ] Your language cannot be unambiguously parsed
[X] Shape errors can still occur at run time
```

The last one is worth dwelling on. XeTaL's static types leave out rank
and shape: `7` and `1 2 3` are both `Int`. So `1 2 + 3 4` type-checks
and gives `4 6`, while `1 2 + 3 4 5` also type-checks but fails when it
runs:

```
error[shape-mismatch]: shapes differ: 2 and 3
```

That is a deliberate tradeoff: APL's polymorphism over arrays of any
rank, without turning XeTaL into a language with dependent or shape
types.

As for parsing, one of XeTaL's rules is that ambiguity is an error. The
parser never chooses between two readings, and never looks at types to
decide. Whether a name is a value or a function is visible in the name
itself:

```
square      a value
s_quare     a function (its s underlined)
's_quare    the function passed as an argument
```

That may look odd at first, but it does real grammatical work.

## "The name of your language..."

```
[~] makes it impossible to find on Google
[ ] is impossible to pronounce
[ ] is a curse word in __________
```

Resolved: say Ecks-e-tal, as the file type `.xtl` is said eks-tee-ell
([the name](name.md) has every spelling). The expansion is descriptive:
the eXperimental Extensible Typed Array Language. And the underscore is
not decoration: in XeTaL an underscore after a letter underlines it,
and an underlined letter makes a name a function.

## "Your language relies on a sufficiently smart compiler"

```
[ ] Yes
```

Values are immutable, so an implementation may keep arrays virtual:
`r_ange n` as an arithmetic progression; rotate, reverse, take, drop
and transpose as index transformations over the original; chains of
element-wise operations fused; reductions streamed. Then
`'+ r_/ r_ange 1000000000` need never build a billion-element array.
But these are optimizations that preserve the ordinary finite-array
semantics; no program depends on them to be correct. That is a
healthier position than "it will be fast once the compiler figures
everything out".

## "Your implementation has the following flaws..."

```
[ ] Hardware does not work that way
[ ] Compilers do not work that way
[ ] VMs do not work that way
[ ] Parser conflicts resolved using rand()
[ ] Compiler required at run time
[ ] Run time required at compile time
[ ] Compiler errors completely inscrutable
[ ] Dangerous behavior is only a warning
[?] Compiler crashes if you look at it funny
```

XeTaL does have `r_oll!`, but programs call it, not the parser. The
lexer, parser, type checker and evaluator are required never to panic
on any input: every failure is a diagnostic with a span. Whether that
holds for every funny look is the `[?]`.

## The philosophical objections

```
[ ] Programmers must understand category theory to write Hello World
[ ] Programmers should develop RSI writing Hello World
[X] The most significant program written in the language is not its compiler
[ ] No language specification
[ ] "The implementation is the spec"
[ ] The implementation is closed source
[ ] Fewer than 100 programmers are smart enough to use the language
```

The specification is the test suite (`spec/**/*.case`, the crate tests
and the CLI goldens in `reg/`) together with `docs/lang-choices.md`,
and the implementation follows them, not the other way round. And the
most significant programs are not the compiler: they are the Life
one-liner, the standard libraries, and TTTML, a tic-tac-toe learner
that expresses its temporal-difference learning in a few pages, with
no framework hiding the algorithm.

## "Your complex sample code would be one line in..."

```
[X] APL
```

Fair, and not a defect XeTaL is trying to fix. The claim is not that
XeTaL is more concise than APL. The question is closer to: can we keep
much of APL's density while adding static types, ASCII source, readable
names, ordinary tooling, explicit functional semantics and a rich
mathematical presentation? APL winning at code golf is fine.

## "You have reinvented..."

The original ends by accusing you of reinventing Lisp, JavaScript,
Java, C++, PHP or Brainfuck, but worse. For XeTaL:

```
[ ] reinvented Lisp but worse
[ ] reinvented JavaScript but worse
[ ] reinvented Java but worse
[ ] reinvented C++ but worse
[ ] reinvented PHP but worse
[ ] reinvented Brainfuck, non-ironically
[X] reinvented APL with fewer glyphs
[X] reinvented Haskell's types underneath APL
[X] reinvented LaTeX backwards
```

The last is the best one-line summary of XeTaL:

```
LaTeX:   ASCII-ish description  --->  mathematical typography

XeTaL:   ASCII program  --->  mathematical typography
                        --->  the same executable semantics
```

## The verdict

The checklist ends with a range of verdicts, from "interesting ideas,
won't fly" to less charitable ones. The useful answer is not to tick
one of them. The checklist shows why XeTaL is more defensible than a
language that set out to fix programming: it tests one narrow
hypothesis.

> Can a language in the APL family keep its array expressiveness while
> combining a functional core, static type inference, ASCII canonical
> source, and a lossless mathematical presentation?

And the implementation is already making that hypothesis face the
awkward cases: shape against type, nested arrays, scalar extension,
exact against tolerant equality, effects, evaluation order, axes,
higher-order functions, and now match, enclose and disclose. That is a
stronger answer to "why another language?" than any checklist.

See also [XeTaL's answers to ngn's permission request](permission-response.md)
and [XeTaL and the APL skeptic's bingo card](xetal-apl-skeptics-response.md).
