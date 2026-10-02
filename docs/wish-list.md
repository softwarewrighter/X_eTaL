# Wish list: ideas, not commitments

These are things XeTaL does not have and that **no saga plans yet**.
They are ideas to discuss, not promises: nothing here is scheduled,
some may never be done, and any that is wanted becomes a saga (or
steps) in [`plan.md`](plan.md) first, with its language decisions
made with the user and recorded in [`lang-choices.md`](lang-choices.md).

Each idea has a rough **priority** (how much it would help the
language's purpose: readable, typed, whole-array programming) and a
rough **size**:

| Size | Roughly |
| ---- | ------- |
| S | one saga step |
| M | two to four steps |
| L | a saga of its own |
| XL | several sagas |

Already planned, so not listed here: transpose, trains, nested arrays,
the trace and stepping debugger, the APL workspace ports (LEARN,
COURSE, PLOT and the rest), the quad names, file and system I/O and
the FFI-like `[]S_VO`, raw strings, complex numbers, checked `::`
signatures, axes above 9, the `_` wildcard parameter, `d_isplay`,
evaluator speed (vector kernels), fuzzing, the live demo's line-by-line
view and phone layout, and transducers (being researched).

## Priority 1: the gaps users meet first

| Idea | Why | Size |
| ---- | --- | ---- |
| **Error handling** | There is no way to catch an error and carry on: a failing `[]N_GET` or a domain error ends the program. A typed result (a library like `Maybe`, or built-in `t_ry` taking a handler) would let programs recover. The design is an open question already raised. | M |
| **Formatting numbers** | `f_ormat` has no width or precision: tables and reports cannot line up columns or round to 2 places. APL's dyadic format (`8 2 f_ormat x`) is the model. | S |
| **Big and exact numbers** | Integers overflow at 64 bits (`2 ^ 70` is an error); there are no exact rationals. Arbitrary-precision integers (and maybe rationals) as a numeric type would suit teaching and number theory demos. | M |
| **Amend (functional update)** | No way to say "this array with these items replaced" except by building it again. An amend function, `values a_t indices array` (APL's `@`, BQN's under-select), keeps values immutable and fills a common need. | M |
| **Rank** | Applying a function to each row, or to each cell of a given rank, needs `e_ach` over a reshape today. APL's rank operator (or a subscript spelling for it) is one of the most useful array ideas still missing. | M |
| **Modified assignment** | Updating a variable names it twice: `count! := count! + 1`. APL says `COUNT+<-1`; XeTaL could say `count! +:= 1` (for `!` variables only, typed like the function). A combinator cannot do it, since a binding is not a value. A language decision. | S |
| **Key / group** | Grouping items by a key (counts per category, sums per group) is everyday data work: APL's key operator. | M |
| **Shareable live-demo links** | A program in the live demo cannot be shared: putting it in the URL (compressed in the fragment) would let anyone send a working example. | S |
| **A comment-keeping formatter** | `xetal fmt` drops comments, so it cannot be used on real files. A formatter that keeps them (and their alignment) would make `fmt` a daily tool. | M |

## Priority 2: tooling and everyday comfort

| Idea | Why | Size |
| ---- | --- | ---- |
| **Language server (LSP)** | Types on hover, errors as you type, the drawn form beside the text, go to definition, in any editor (VS Code, Neovim, Emacs eglot). | L |
| **Library documentation** | Doc comments on `l:` names, and `xetal doc` producing a reference page for a library, as `docs/reference.md` is for the built-ins. | S |
| **Tests in XeTaL** | An `a_ssert` (or `e_xpect`) built-in and `xetal test FILE`, so libraries can carry their own tests, not only goldens. | S |
| **REPL comforts** | History across sessions, tab completion of names, and commands to list what is defined or imported. | M |
| **Named records** | No way to name the parts of a value (a board, a point, a model); Church encoding types a maybe only by use (CB3). Records or algebraic data types would close that gap. A large language decision. | L |
| **Pattern matching on arguments** | Taking a vector apart in the parameter list (`{ (x y) -> ... }`) instead of `f_irst` and `d_rop`. | M |
| **Under** | Apply a function "under" another and undo it (BQN's under): rotate, change, rotate back, in one step. | M |
| **CSV and JSON** | Reading and writing tables and simple structured data, for data-work demos. | M |
| **Dates and times** | Today's date, durations, formatting: needed by any program that keeps records. | S |
| **SVG charts** | Line, bar and scatter charts as pictures (`[]S_HOW`), beyond the planned character plots. A library on `[]P_ATH` and `[]G_RID`. | M |

## Priority 3: bigger bets

| Idea | Why | Size |
| ---- | --- | ---- |
| **A compiler** | Compile XeTaL to WebAssembly or native code (Cranelift), for speed beyond what the interpreter's vector kernels give. | XL |
| **Parallel and GPU kernels** | Element-wise and reduce primitives on many cores or a GPU, as array languages are suited to. | XL |
| **A notebook format** | A saved notebook (source, results, pictures) that the live demo and the CLI both open, or a Jupyter kernel. | M |
| **Packages** | Sharing libraries beyond `userlibs/`: versions, a registry, `xetal add NAME`. | L |
| **Execute** | Running text as code (APL's execute). Hard in a typed language: the result's type is not known ahead; it may be limited to `--untyped` or to a declared type. | M |
| **Concurrency** | Running independent computations at once (a parallel `e_ach`), then message passing. | L |
| **Accessibility** | The drawn form read aloud sensibly (an underlined r as "function rev"), and the live demo usable with a screen reader. | M |
| **Regular expressions** | Text matching for data cleaning; deliberately not core so far, but a library could add it. | M |

## Suggested order

If any of these are taken up, the first four of priority 1 (error
handling, number formatting, amend, rank) would most widen what
programs can say, and shareable live-demo links are the cheapest win
for showing the language to others.
