# XeTaL and the APL Skeptic's Bingo Card

> **XeTaL is not an attempt to "fix APL," replace APL/J/K/BQN, or invent
> a general-purpose language from a bag of favorite features.**
>
> It is an experiment: can APL-style whole-array programming retain its
> expressive density while gaining a functional core, static type
> inference, ASCII canonical source, and lossless mathematical
> presentation?

The APL Wiki's [*Humor*](https://aplwiki.com/wiki/Humour) page includes a complaint-bingo card for
conversations with people unreceptive to APL. XeTaL inherits many of
those objections simply by being an array language, and creates a few
new ones of its own.

This is XeTaL's answer.

------------------------------------------------------------------------

## The complaint bingo card

### "I can't imagine how you have any readability concerns."

**Fair criticism:** dense array notation can become write-only code.

**XeTaL:** readability is a primary design constraint. Canonical source
is ASCII; a lossless presentation layer renders Unicode/LaTeX. Names
such as `r_eshape`, `t_ally`, `f_irst`, and `e_ach` favor recognition,
while common mathematical operations remain concise. The target is
**semantic density, not minimum character count**.

### "write only language"

Classic APL's reputation did not come from nowhere. XeTaL explicitly
starts from "code is read more often than written." Definitions may be
verbose when that makes their uses clearer; ambiguity is rejected rather
than guessed. Types, pretty-printing, tracing, and visualization are
intended to make dense expressions inspectable.

### "how come I've never heard of it"

Correct. XeTaL is an experimental research/teaching language, not an
established production ecosystem. Its case has to be made by useful
experiments, not popularity.

### "how will I know what it does if it isn't a word?"

A word is not automatically clearer than notation. `multiply` is not
necessarily clearer than `*`, and prose describing an equation can
obscure its structure. XeTaL deliberately mixes conventional symbols
with readable names:

``` text
+ - * / = < >
r_eshape r_ange t_ally f_irst
e_ach t_able
r_/ s_\ o_-
```

The question is which notation gives the best **semantic compression
ratio**.

### "how many alt codes do you use to type this all out"

**Zero.** XeTaL source is ASCII. Unicode is presentation, not required
input.

``` text
ASCII source -> parser/typechecker/evaluator
      |
      +-----> decorated Unicode
      +-----> LaTeX
```

### "that looks applicable only for a small variety of problems"

**Partly true.** XeTaL does not need to be the best language for
everything. Its natural domain includes arrays/tensors, numerical work,
transformations, simulations, linear algebra, grids/images, data work,
and ML. TTTML is useful precisely because a small temporal-difference
learner can be expressed while leaving the model and update rule
visible.

### "how come Dyalog is closed source, it must be bad"

It doesn't. Dyalog is a mature commercial APL. XeTaL is an open
experimental language in Rust. XeTaL can learn from Dyalog, APL2, J, and
BQN without any of them needing to be "bad."

### "RTL evaluation is purposefully obscure"

XeTaL deliberately retains right-to-left evaluation with no precedence
on its array-language surface. The cost is unfamiliarity; the benefit is
one uniform composition rule instead of a precedence table. The parser,
formatter, and trace/explainer must make that rule inspectable.

### "looks like BF"

Only visually, to someone unfamiliar with both. Brainfuck is a tiny
low-level instruction language. XeTaL has arrays, higher-order
functions, type inference, lexical closures, scalar extension, axes,
trains, structural operations, search/order operations, and selective
laziness.

### "I don't think anyone understands APL"

People demonstrably do, but expert idiom can become opaque to newcomers.
XeTaL's words, rendered view, types, examples, and execution
visualization are intended to make the learning curve observable rather
than mystical.

### "it's dead"

APL is old; array programming is not. Tensor libraries, dataframe
systems, SIMD/GPU programming, numerical computing, and ML repeatedly
rediscover the value of collection-level operations. XeTaL asks which
array-language ideas remain useful when surrounded by modern language
design.

### "how do you possibly remember all the symbols"

You largely don't have to. Source is ASCII and many primitives are
named. Decorated notation is generated from source rather than typed
from a special keyboard.

### "why can't I write a for loop" --- FREE SPACE

Because XeTaL wants the first question to be **what operation is being
applied to the array?**, not **how does an index walk through memory?**

Typical answers are `e_ach`, `r_/`, `s_\`, `t_able`, `i_nner`,
`r_eshape`, `s_elect`, `t_ake`, or `d_rop`. These are iteration
expressed at a higher semantic level. If scalar imperative iteration is
fundamentally clearer for a problem, XeTaL may simply not be its best
language.

### "looks like line noise to me"

That reaction is legitimate before the notation is learned. XeTaL's
unusual answer is to keep both an ASCII source view and a
mathematical/decorated view, losslessly related. Source encoding and
human presentation do not have to be the same thing.

### "this is like code golfers made a language"

Code golf minimizes characters. XeTaL permits verbose definitions, uses
words where useful, carries static types, rejects ambiguity, and values
explanatory tooling. A one-line Life is interesting because it exposes
array composition, not because it wins a byte-count contest.

### "this is dumb"

Possibly. That is what experiments are for. XeTaL is deliberately small
enough that claims about notation, array semantics, typing,
presentation, and ML expressiveness can be implemented and tested.

### "where do you get a keyboard to type that"

The ordinary US keyboard already in front of you. This objection is
almost a XeTaL design requirement.

### "looks like regex"

Regex is a useful warning: compact notation becomes readable after its
vocabulary is learned but can exceed working memory. XeTaL's words,
consistent naming, rendering, types, and explainer are attempts to move
that boundary.

### "I rewrote this with words, see, now it can be understood!"

Sometimes that is an improvement. XeTaL does not assume glyphs beat
words. But spelling every mathematical relation as English can also hide
structure. XeTaL deliberately occupies the middle ground:
**word-oriented ASCII source, mathematical presentation, terse
composition**.

### "I tried to learn it for 15 minutes, I think no one can"

Fifteen minutes can establish that array programming is unfamiliar, not
that it is incomprehensible. But XeTaL should provide early payoff:
scalar extension, shape transformations, reduction/scan, table/inner
operations, Life, and an inspectable ML example such as TTTML. If those
merely look clever rather than explanatory, XeTaL has failed an
important goal.

### "character count does not correlate to ergonomic language use"

**Agreed.** Better measures include conceptual operations per
expression, number of moving parts, readability after learning, quality
of diagnostics, static errors caught, inspectability, and correspondence
between notation and the problem.

### "this looks nothing like JavaScript, how can you read it"

It isn't intended to. Familiar syntax has value, but can import a
scalar/statement-oriented model XeTaL is explicitly examining. XeTaL
should borrow familiarity where it does not conflict with its semantics.
**Novelty should pay rent.**

### "I think it's broken, I typed in 1*2+3"

Different evaluation rules deserve good diagnostics. XeTaL's rule is
right-to-left with no conventional precedence. The formatter/canonical
printer and trace/explainer should expose exactly how an expression
parses and evaluates. A surprising rule is acceptable only when it is
uniform.

### "we write in words in English not symbols"

We also write `E=mc^2`, `f(x)`, `x<y`, sums, and integrals because
notation can expose structure better than prose. XeTaL asks which
concepts are clearer as names, which as conventional notation, and
whether the computer can present both without creating two source
languages.

### "there's a reason we aren't using APL right now"

Yes: ecosystem size, familiarity, tooling history, integration,
historical input methods, and the success of C-family syntax and scalar
programming all matter. XeTaL does not require an alternate history in
which APL should have won. It asks which ideas from the array-language
tradition remain valuable today.

------------------------------------------------------------------------

## Other APL-community jokes XeTaL should answer

### `IO delenda est`: index origin

Configurable index origin is itself an APL-community joke because it
invites off-by-one surprises. XeTaL makes a deliberately boring choice:

``` text
index origin = 1
r_ange 5   => 1 2 3 4 5
o_ffsets 5 => 0 1 2 3 4
```

Index origin is not configurable. Indices and offsets are distinct
concepts. One can disagree about choosing 1 rather than 0, but a XeTaL
program cannot silently change the meaning of indexing through global
state.

### "To write three lines of APL, / And make the damn things run"

XeTaL should resist turning this into a boast about one-liners. The
better goal is to make those lines small because the algorithm is
expressed at the right level, then provide enough tooling to understand
why they run.

The one-line Life example therefore matters most when accompanied by its
pretty form, annotations, shapes, and execution explanation.

### "Peak Engineering happened in the 1970s"

The APL keyboard meme is affectionate evidence of the glyph problem
XeTaL is testing:

> **Keep the mathematical expressiveness; retire the keyboard
> requirement.**

XeTaL is not anti-glyph. It is **anti-glyph-as-canonical-file-format**.

### APL vs. J vs. K vs. BQN vs. XeTaL

XeTaL should not enter the traditional dialect rivalry by declaring
itself the winner. Its design deliberately crosses boundaries:

-   APL2/Dyalog influence in nested-array thinking and 1-origin
    indexing;
-   J/BQN influence in leading-axis behavior and array semantics;
-   Haskell-like static inference and constrained polymorphism;
-   Rust-like explicitness and tooling expectations;
-   XeTaL's own ASCII-to-mathematical-presentation experiment.

That makes it a **design experiment in the APL family**, not "APL 2.0."

### Minimal primitives vs. "MAKE MORE PRIMITIVES"

The Wiki's BQN-vs-KamilaLisp meme captures a real design tension: seek a
small orthogonal basis or make every convenience primitive.

XeTaL should deliberately sit between those extremes. A primitive earns
its place when it is semantically fundamental, common enough to improve
readability, important to optimization/diagnostics, or a recognizable
array-programming concept. Otherwise it should preferably be a library
composition.

The target is neither minimum primitive count nor maximum convenience.
It is a **coherent vocabulary**.

### Empty arrays

APL's insistence that empty arrays are real arrays rather than
embarrassing edge cases is worth preserving. XeTaL should be judged by
whether operations have principled identities, fills, types, and shapes
on empties---not by whether the examples make good jokes.

------------------------------------------------------------------------

## What XeTaL is *not* claiming

XeTaL does **not** need these claims to succeed:

-   APL syntax was perfect.
-   APL should have displaced mainstream languages.
-   Every algorithm should be an array expression.
-   Fewer characters always means better code.
-   Symbols are inherently superior to words.
-   Static typing solves shape errors.
-   XeTaL will replace Python, Rust, Julia, APL, J, K, or BQN.
-   A new programming language deserves adoption merely because it is
    new.

The claim is narrower and testable:

> **Can an APL-family language preserve much of the expressive power of
> whole-array programming while combining a functional core, static type
> inference, ASCII canonical source, and lossless mathematical
> presentation - and can modern tooling make those dense programs easier
> to understand?**

Life, TTTML, the ML examples, the type checker, and the execution
visualizer are experiments intended to answer that question.

------------------------------------------------------------------------

## The shortest answer to the skeptic

> **XeTaL isn't "APL with different punctuation." It is an experiment in
> separating how array programs are typed, represented, and displayed:
> ASCII source for machines and tools, mathematical notation for
> readers, static types for mistakes the compiler can catch, and
> APL-style array composition for the algorithm itself.**

Or, more defensively humorous:

> **Yes, it looks like APL. No, you don't need an APL keyboard. Yes,
> arrays can be empty. No, you can't change the index origin. Yes, `for`
> loops still exist conceptually; we just want you to admit what you're
> doing to the array first.**

------------------------------------------------------------------------

## Source note

This response was prompted by the APL Wiki *Humor* page, especially its
sections on dialect rivalry and criticism of APL, the complaint-bingo
image, the poem about making three lines of APL run, the index-origin
jokes, the "Peak Engineering" keyboard meme, the BQN-vs-KamilaLisp
primitive-design meme, and the discussion of empty-array jokes.

The jokes are useful because they encode real recurring design
objections. XeTaL should answer the objections rather than merely
dismiss the jokes.

See also [XeTaL fills in the Programming Language
Checklist](why-another-language.md) and [XeTaL's answers to ngn's
permission request](permission-response.md).
