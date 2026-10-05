# X_eTaL: a chronology of discovery

> **Scope.** This is a chronological history of the `softwarewrighter/X_eTaL*` ecosystem through 2026-10-05. It interleaves the core language, demos, games, libraries, native extensions, and ML repository. It is intentionally not a commit log: related commits, merge bookkeeping, `CHANGES.md` entries, saga records, and downstream “asks” are combined into the design event they represent.
>
> The unit of this history is **problem → decision → implementation → dogfooding consequence**.

## Recurring design principles

Several preferences recur throughout the history:

- **Readable input, rich presentation.** Source should be typeable on a normal keyboard; rendering may turn it into denser mathematical/array notation.
- **Array-language concision without simply cloning APL.** APL, J and BQN are sources of ideas, not compatibility targets.
- **Static typing as a design tool.** Haskell-style inference and Rust-like compile-time feedback are preferred to pushing every ambiguity to runtime.
- **Dogfood before standardizing.** A workaround may first appear in a demo or library. If it proves general, move it into the core and remove the workaround.
- **Several kinds of extensibility.** Ordinary X_eTaL libraries, compile-time macro libraries, and native Rust extensions solve different problems.
- **Tests are part of the specification.** Spec cases, goldens, property tests, browser tests, documentation examples and performance baselines constrain the implementation.
- **A demo should show the real program.** Rust/Yew may provide a shell, but the interesting computation should remain X_eTaL.
- **Prefer a small useful mechanism to speculative generality.** Several plausible ideas were deliberately deferred or abandoned after experimentation.
- **A tool of thought, tested by use.** A feature should not merely make X_eTaL different. Dogfooding has to show that the language encourages a useful way of representing problems.

## The question asked early: what does it offer as a tool of thought?

Early on, someone looking at X_eTaL asked what it offered as a *tool of thought*, rather than which implementation language or feature checklist distinguished it. At the time that was a hard question to answer: there was a notation, a type checker and one program, and no body of work to point to.

It became a design test instead of an embarrassment. A new feature was not justified by making X_eTaL different from APL, J or BQN. It was justified when writing real programs showed that the language led to a useful way of representing a problem: a rule as one whole-array expression, a game as a few array idioms, a model as shapes and products, a syntax form as a macro.

The rest of this chronology is the answer, in the order it was found:

**Life → axes → classics → higher-order arrays → visual microscopes → games → libraries → ML → macros → Rosetta.**

Each stage was a set of programs that had to be written, and each showed something about how the language makes one think. The answer did not have to exist first. Enough of X_eTaL was built to discover it.

---

# 2026-09-27 — The experiment becomes a language

## 12:25–15:08 — Start with Conway's Life

The first `X_eTaL` commit landed September 27. Planning documents, a Cargo workspace, specification-case harness, `reg-rs` golden tests and a gate followed almost immediately. The project was expected to change quickly, so executable evidence came before a large implementation.

The first forcing function was **Conway's Game of Life**. The desired one-line rule was pinned against the earlier `sw-apl` work and corrected before the rest of the language was built around it.

This established an important method: a compact, recognizable array program would be an acceptance test for the language design. Life forced questions about arrays, axes, rotation, reduction, equality and presentation much earlier than a scalar hello-world would have.

## 15:29–21:07 — Separate what is typed from what is seen

The lexer acquired the initial token set, spans and a rejection corpus. The renderer then established a central X_eTaL idea: **input form and display form are different representations of the same program**. ASCII/US-keyboard-friendly source could be rendered as decorated Unicode or LaTeX, losslessly.

Before proceeding, the project recorded a large set of language choices. Consequential decisions included:

- command `xetal`, with `x_etal` as an alias;
- word-oriented built-in names, using marks where they carry recognizable array-language meaning;
- **index origin 1**, with explicit zero-origin offsets;
- strings as character vectors;
- explicit rules for quoted operands and chained higher-order operations;
- trains in brackets, including forks and atop;
- dfn-style guards;
- strict evaluation by default with an explicit lazy-parameter mechanism;
- axis subscripts applicable to arbitrary functions;
- APL2/BQN-style nested arrays planned after the Life milestone;
- true division and explicit divide-by-zero errors;
- exact equality plus a separate tolerant comparison;
- explicit application/evaluation order;
- a macro phase planned as a language facility rather than ad-hoc textual substitution.

Already this was not simply an APL clone: array-language ideas were being combined with preferences from statically typed functional languages and Rust.

## 21:27–22:11 — Build a compiler pipeline

The implementation followed a conventional, inspectable pipeline:

**lexer → renderer → parser → canonical formatter → Core IR → desugaring → strict scalar evaluator**

Commands such as `xetal lex`, `parse`, `fmt`, `core`, `eval` and `run` exposed the stages. That inspectability later became valuable for teaching and tooling.

## 22:17–22:40 — Static typing becomes foundational

X_eTaL added a type representation, unification with occurs check, schemes, numeric constraints and **Algorithm W over Core**. `xetal type` and type-checked execution followed, with `--untyped` retained as an escape hatch.

The preference was clear: retain array-language concision while gaining Haskell-like inference and early feedback.

## 22:51–23:45 — Arrays arrive

The code was split into component workspaces; numeric dictionary passing closed a polymorphic-literal gap; and the first substantial array model arrived:

- dense arrays, strands and strings;
- scalar extension;
- identity/left/right tacks;
- structural operations such as shape, reshape, take, drop, selection and catenation;
- character comparison and string structure;
- a persistent typed REPL.

The built-in catalog was generated rather than allowed to become hand-maintained dispatch boilerplate.

---

# 2026-09-28 — Higher-order arrays, axes, and the Life milestone

## 07:38–12:17 — Higher-order programming

The language gained reduce, scan, `e_ach`, `t_able`, `i_nner`, composition and swap, plus index-of, membership, unique, sort, grade and where. Random `r_oll!` gained deterministic testing support. Property tests checked the higher-order and search operations.

## 12:27–12:46 — Axes unlock the original target

Rotate and reverse arrived, then axis subscripts on arbitrary functions and multi-axis rotate/reduce.

At **12:46**, the Life one-liner ran through the general language rules and was committed as a demo and golden test. Nothing special was added just for Life.

This is the first natural “we have a language” milestone: the program that constrained the design now worked because the general semantics were sufficient.

## 16:19–19:24 — Make experimentation convenient

A `justfile` became the project front door. Axis/rotate properties were tested and an animated two-dimensional rotation demo made transformations visible.

## 21:42–23:56 — The language acquires an interface

A front-end-independent view model preceded the TUI. `xetal edit` gained live rendering, type/output panes, scrolling and Emacs-style keys. Lambda arguments evolved from literal subscript `r`/`l` toward rendered alpha/omega. Comments, quoted operands, macros and separators received deliberate visual treatment.

A commented language tour appeared. `xetal run --echo` turned a source file into a notebook-like transcript. Array values were displayed as grids and the REPL rendered decorated input as it was typed.

The architectural choice was significant: **rendered X_eTaL was a view over ordinary source, not a second syntax users had to edit.**

---

# 2026-09-29 — Libraries, literate programming, combinators and serious dogfooding

## 00:08–09:35 — Multi-file programs and libraries

A multi-file source map led to import resolution, cycle detection, namespaces, exports and private names. `Stats` became the first standard library.

The REPL, notebooks, editor and Org sessions were made library-aware. **Emacs `xetal-mode` and `ob-xetal`** made Org Babel a natural literate-programming environment.

## 11:00–15:24 — Functional-programming influence becomes explicit

The “aviary” explored typed combinators. Function power `f_^n`/`p_ower` followed. The `Combinators` library grew to 38 typed birds plus Y, with demos; a Church-encoded `Maybe` library and monad demo tested how functional abstractions coexist with array programming.

JuliaMono became the recommended display font because reliable mathematical/array glyph rendering now mattered.

## 16:03 onward — TTTML becomes a substantial dogfood application

**TTTML**, a tic-tac-toe machine learner ported from `sw-apl`, showed that the language could do more than compact puzzles. It became a library and literate document; models could be saved and loaded; and a user could play against it.

System names for I/O plus text/number support appeared as the application demanded them. This pattern—application pressure revealing missing general mechanisms—would soon dominate development.

---

# 2026-09-30 — Teaching, comparison and the browser become part of the language

The Life expression gained a generated, checked annotated diagram. Combinators received diagrams aimed at APL, J and BQN readers.

An idioms document put X_eTaL beside Java, JavaScript, Python, C++, Rust, APL2, Dyalog, J and BQN, with executable examples.

The project developed a typeset logo, key/input-form demo, generated built-in reference, refined numeric display, screenshots and video.

## Browser/WASM

The engine was made usable under `wasm32`, with host-provided files and libraries. A Yew editor exposed ASCII source, rendered code, types and output. The live demo was published on GitHub Pages.

Browser Open/Save/Save As, user libraries and `[]R_EAD` followed. Org documents became web pages and their code blocks were shown in decorated form.

Documentation, playground, notebook and implementation were deliberately made to agree on the same programs and rendering rules.

The project also addressed “why another language?” via a Programming Language Checklist and APL-skeptic discussion. A “Hello, library of your own” example made extensibility introductory rather than advanced. Unicode was permitted in strings and comments while source syntax retained the typeable-input philosophy.

---

# 2026-09-30 to 2026-10-01 — Classic programs become a language laboratory

A classics effort began with Pascal's triangle and expanded quickly.

Graphics arrived as `xetal-draw`: arrays could become SVG grids and animated frames. `[]G_RID`, `[]S_HOW`, `[]P_ATH`, a Turtle library, Koch and Sierpinski demonstrations, and Mandelbrot images followed. User libraries gained an explicit `userlibs/` home.

The classics added sieve/primes, GCD, Fibonacci, factorial, Collatz, Tower of Hanoi, quicksort, graph algorithms, Horner evaluation, differences, moving averages, cellular automata, histogram, sorting, run-length encoding, magic squares, Mastermind, truth tables, base conversion, Roman numerals, word frequency, N-Queens, ragged Pascal rows and eventually a tiny APL interpreter.

Hanoi was intentionally shown three ways—recursion, combinators, and an array formulation computing the moves together—because the goal was to discover what *idiomatic X_eTaL* looked like.

## Dogfooding creates core features

The classics exposed missing general operations. Rather than leave workarounds scattered through demos, the core gained:

- replicate/compress;
- encode/decode;
- catenate along an axis;
- nested `Box a` arrays;
- partition and map;
- explicit ASCII boxed display.

A LeetCode string problem moved from approximation to an exact formulation after nested arrays/partition landed.

This became a durable rule: **a downstream workaround is evidence; a successful general abstraction migrates downward into the core and the workaround is removed.**

The browser also moved execution into a worker, streamed output, gained Stop/spinner behavior, became a PWA, and added notebook/step execution.

---

# 2026-10-01 — `X_eTaL-demos` begins

At 18:12 UTC, `softwarewrighter/X_eTaL-demos` was scaffolded. This was the first major ecosystem split. The core already had examples, but a dedicated repository could optimize for visual, topical and explanatory demonstrations.

It established its own gate, pinned/vendored X_eTaL build, demo template, regression runner and GitHub Pages catalog.

## Visual demos, in order

1. **Life Microscope** — the first live demo, making the original acceptance program inspectable.
2. **Mandelbrot** — the set appears step by step, with orbit and zoom views.
3. **Reaction-Diffusion** — Gray-Scott patterns growing live.
4. **Julia** — related fractal sets from one function.
5. **Wave Tank** — waves through slits, a lens and ripples.
6. **CA Lab** — cellular-automaton rules as lookup tables; it also exposed an X_eTaL slowdown.
7. **Langton's Ant** — the ant represented as a one-hot mask.

After several pages repeated the same browser machinery, a shared **microscope shell** was extracted. Repetition in applications was again used to discover the abstraction rather than predicting it first.

---

# 2026-10-02 — The ecosystem splits into specialized repositories

## Core: trains become trustworthy terse syntax

Trains received property tests proving equivalence to desugared forms. Errors inside a train were improved first to identify the failing element and then to explain what the train meant at that point.

The tour, reference and syntax poster showed trains beside their desugarings. Existing lambdas that merely spelled out trains were retrofitted. The principle was: **concision must remain explainable and testable.**

## `X_eTaL-games` starts — 13:57 UTC

Games stress interaction, state and terminal behavior differently from scientific demos. The new repo established scripted-input tests and a live catalog.

Games arrived in this order:

1. **Horse Race** — field as one vector.
2. **Guess** — a small COR24-style terminal game.
3. **Robot Chase** — robots move together as an array operation.
4. **Trek Adventure** — a text adventure organized around tables.
5. **Trek** — a larger Star Trek-style game with an array galaxy.
6. **Tic-Tac-Toe** — line evaluation through indexing.
7. **Shut the Box** — candidate tile subsets considered together.
8. **Minesweeper** — neighbor counts by rotation and opening by flood fill.
9. **2048** — rows slid and merged as array transformations.

The games immediately generated upstream requirements for terminal and browser interaction. An initial WASI-oriented direction was later **dropped** in favor of a steppable evaluator and host-managed terminal model inspired by `web-sw-tos`. The reversal is important: a plausible architecture was tried conceptually and rejected when a better fit emerged.

## `X_eTaL-libraries` starts — 16:08 UTC

The library repo established per-library directories, pinned export types, regression tests, pages and tooling.

The initial libraries were:

- **Check** — textual assertions;
- **Strings** — text functions influenced by J, BQN and dfns;
- **Sets** — vectors as sets;
- **Numbers** — number theory;
- **Combinatorics** — counting/enumeration;
- **Lists**;
- **Matrix** — linear algebra by elimination;
- **Random**.

The repository quickly became a requirements generator: if several libraries had to implement the same structural operation, that was evidence the operation might belong in the language.

## `X_eTaL-extensions` starts — 18:15 UTC

The third new sibling explored **native extension**, not source-library, extensibility.

The sequence was intentionally layered:

1. define and test a native extension **ABI V1**;
2. build an SDK and **Hello** extension;
3. implement discovery/loading/validation/calling;
4. add `xetal-x`, a host carrying an `ext:` store;
5. define the bridge protocol from X_eTaL to native code;
6. place an X_eTaL facade over the raw extension;
7. add a **Clock** extension and measure bridge cost.

This gave “Extensible” a concrete second meaning beyond importing X_eTaL source.

## Core responds to sibling pressure

During the same period:

- transpose was added (`o_\` and `t_ranspose`, later axis swaps);
- long integer-strand type checking was made linear;
- exponent-form float literals were added;
- the games' terminal request was promoted into a core plan;
- sibling-repository asks became explicitly tracked.

The ecosystem was now operating as a feedback system rather than a hub with passive examples.

---

# 2026-10-02 to 2026-10-03 — Applications drive the core

## Demos move from grids toward physics and ML

After the first visual gallery, `X_eTaL-demos` added, in order:

8. **N-body** — every pair represented together as a displacement cube.
9. **Image Pipeline** — windows generated by rotation, with one mechanism serving blur and edge filters.
10. **Ternary Net** — one classifier compared in FP32, FP16, INT8 and ternary forms.
11. **MoE Router** — tokens routed by meaning to the top two of sixteen experts.
12. **MoE Epsilon** — perturb a token and watch expert-selection boundaries move.
13. **CNN Digits** — a tiny MNIST CNN trained offline, with learned weights consumed by X_eTaL.

Every demo gained CLI and browser regression baselines. The browser page was no longer allowed to drift away from command-line behavior.

The demos also discovered a significant `t_able`/`i_nner` performance regression. That became an upstream core issue rather than a demo-specific workaround.

## Games are rewritten to discover idiomatic X_eTaL

After getting games running, the next pass asked whether they were actually *X_eTaL-shaped* programs rather than imperative programs translated mechanically.

Shared `Play`, `Text`, `Board` and `State` libraries were extracted. Horse Race used trains; Guess expressed its answer through a fork; Robot Chase used a sign fork and inner product; Trek Adventure represented refusals as data; Trek used shared state/board/text abstractions; Tic-Tac-Toe used inner-product ratings and negamax; Shut the Box mapped move lists and scored with inner product; Minesweeper shared neighbor logic; and 2048 expressed a move as a train with rotation by function power.

The retrospective summarized each game by **one array idea**. The goal had shifted from “X_eTaL can run games” to “games can teach array programming.”

## Libraries expand, then deliberately freeze

The library repo added, in order:

- **Format**
- **Plot**
- **Dates**
- **Statistics**
- **Graphs**
- **Bits**
- **Polynomials**
- **Grouping**
- **Csv**
- **Search**
- **Geometry**

At that point expansion was deliberately frozen. Breadth was no longer the problem; readiness, consistency and upstream blockers were.

Transpose illustrates the feedback loop. Matrix and Combinatorics carried local transpose workarounds. Once core X_eTaL gained transpose, those copies were deleted.

## Extensions prove the host boundary with real capabilities

The extension repo moved beyond Hello/Clock:

- **SQLite** exposed statements, numbers, text, names and quoting.
- **CSV import** loaded external data into SQLite.
- A **Mauna Loa CO2 notebook** combined SQLite with X_eTaL analysis.
- SQLite-in-browser was investigated, but the toolchain cost was judged inappropriate and the idea was explicitly dropped.
- A UI host and **Canvas** extension opened native windows.
- Life ran in a native window.
- Headless canvas saved frames so graphical programs remained testable without a display.

The decision not to force SQLite into the browser is representative: browser/native parity was useful, but not at any cost.

---

# 2026-10-03 — `X_eTaL-ML` splits out and specialization becomes explicit

At 15:43 UTC, `softwarewrighter/X_eTaL-ML` was scaffolded. Machine-learning content had begun to dominate the general demo repository, so the split was by **purpose**, not merely file count.

The first ML content was moved rather than duplicated:

1. **Ternary Net**
2. **MoE Router / MoE Epsilon**
3. **CNN Digits**

The repo inherited demo/microscope and library tooling, then added an **NN library** and later an **Attention microscope**.

This was an ecosystem-design milestone: X_eTaL was now a set of domains sharing a language and conventions, not one repository with miscellaneous satellites.

---

# 2026-10-03 — Terminal, stepping and performance become core requirements

The games' terminal request led away from the earlier WASI idea and toward a **steppable evaluator**. Higher-order built-ins became steps of the same machine; execution could pause for typed input and resume; and a terminal grid/line editor was layered over it. The browser terminal used the same stepping model in a worker.

This architecture later made structured error handling and interactive hosts easier.

Benchmarks and a baseline/profile also appeared. The demo repository's slowdown report made performance a language-level nonfunctional requirement. A deterministic cost guard for higher-order operations followed.

The core added generated `docs/status.md` and `docs/asks.md`, recording what worked and what sibling repositories still needed. The cross-repository feedback loop was now explicit infrastructure.

---

# 2026-10-03 to 2026-10-04 — Macros become the third extensibility layer

Some abstractions could not be ordinary eager functions. Control forms need selective evaluation; domain notation benefits from compile-time checking; extension bindings should not require repetitive facades.

The first system macros were `i_f<`, `u_nless<` and `e_ach<`; `xetal expand` made their transformations visible. Soon `.xtlm` files allowed **macro libraries of your own**, and long namespace prefixes supported larger ecosystems.

This gave “Extensible” three concrete meanings:

1. reusable runtime X_eTaL source libraries;
2. compile-time X_eTaL macro libraries;
3. native Rust extensions through a host ABI.

## Libraries test whether macros are actually useful

The library repo made an explicit decision: **no gratuitous macros**. Use a macro where compile-time transformation solves a real problem.

Domain macros followed:

- `d:d_ate<` — checked date literals;
- `py:p_oly<` — polynomial notation;
- `g:g_raph<` — graphs written using names;
- `b:f_ields<` — named bit fields;
- `cs:c_olumns<` — named, typed CSV columns;
- `k:c_ases<` — source-named table-driven checks.

The earlier Control library was redesigned and ultimately retired as core/system macros clarified what belonged where.

## Core system macros become self-hosting

Rust-generated system macros were rewritten in **X_eTaL itself** in `lib/System.xtlm`, over small primitive hooks.

Compiler-oriented macros followed: `l_ine<`, `f_ile<`, `i_nclude<`, `c_fg<`, `e_rror<`.

Then Rust-inspired convenience/debugging macros: `d_bg<`, `a_ssert<`, `f_ormat<`, `p_anic<`, and `t_odo<`.

Macro hygiene was added, with explicit declared anaphora for intentional visible bindings. `Combinators.xtlm` explored compile-time Y/Bluebird forms; timing showed that macro form was not automatically faster, preventing “macro” from becoming synonymous with “optimization.”

---

# 2026-10-04 — Extensions become multimedia programs

A retained 3D scene API produced lines, points and a cube. Then:

- **Audio** decoded/played sound while exposing playing data as arrays.
- **Spectrum Visualizer** let X_eTaL perform analysis while Rust handled playback/drawing.
- **Synth** generated every audio sample in X_eTaL.
- **Oscilloscope** drew the synthesizer while it played.
- A real **trigger** made the scope behave more like an instrument.

The extension philosophy became clearer: Rust owns OS/device/native capabilities; X_eTaL owns the array computation where practical.

The extension repo then used `.xtlm` for an **FFI binding macro**: one signature line could generate a facade. Every facade was rewritten through it, so macro and native-extension layers reinforced each other.

---

# 2026-10-04 — Documentation becomes executable infrastructure

`xetal doc --json` produced a cross-reference model with items, inferred types, documentation, examples, source and resolved uses.

`xetal doc --out DIR` generated a static site with file/item indexes, built-ins, imports, sections, inferred types, examples, decorated source and definition/use links.

`xetal doc --test` executed `## >>` examples as sessions.

Macro expansions were then shown in place and linked to their definitions. Search by **name or type** added a Hoogle-like dimension.

Because parsing, typing, rendering and expansion were explicit compiler stages, documentation could reuse compiler knowledge rather than scrape source text.

---

# 2026-10-04 to 2026-10-05 — The specialized galleries mature

## General demos after the ML split

`X_eTaL-demos` refocused on broadly visual array ideas:

14. **Fourier Epicycles** — a curve as circles-on-circles, computed through matrix products.
15. **Abelian Sandpile** — every unstable cell topples together.
16. **Stencil Macros** — kernels written as pictures and turned into code by a macro library.

The repo tightened a truthfulness rule: **the listing shown on a page is exactly the program X_eTaL ran**. Demo titles gained links or story dialogs. Performance baselines became part of the gate.

## ML as its own teaching surface

`X_eTaL-ML` moved demo data into files so pages showed all inputs actually consumed. CNN Digits became a clearer CLI program and live page.

The **Attention microscope** joined the gallery as the fourth major ML demonstration. ML workloads gained per-host benchmark baselines, and the NN library provided reusable operations.

The split paid off: ML could optimize for “why is this computation understandable as arrays?” while general demos stayed broad.

---

# 2026-10-05 — Typed errors and resumable conditions

The libraries had asked for raise/catch support. Rather than add an untyped exception escape hatch, the core designed it around the type system and steppable machine.

## `[]S_IGNAL`

`"code" []S_IGNAL "message"` creates an error of one's own. Its polymorphic result lets it stand where any value was expected because successful evaluation never returns normally.

## `[]T_RAP`, `Outcome a`, and `[]E_NSURE`

A trapped body and typed handler can answer `[]R_ECOVER`, `[]R_ETRY`, or `[]H_ALT`. `[]E_NSURE` guarantees cleanup. `Error` and `Outcome a` are real type-system concepts rather than magic arrays.

## Resumable warnings

`default []W_ARN "code" "message"` raises a resumable condition carrying the value execution should continue with. A handler may answer `[]C_ONTINUE`. Passing the warning outward preserves resumability; trying to continue a non-resumable signal is itself an error.

Again the preference is visible: when adding a dynamic mechanism, use static typing to constrain its shape as far as possible.

---

# 2026-10-05 — Release engineering catches up

## Stop copying the core into every sibling

The siblings initially tracked vendored copies of X_eTaL. Reproducible, but bulky.

They converged on a simpler convention: track a known-good X_eTaL commit, clone it into ignored working storage, check out that revision, build it, and use those artifacts.

## Generated Pages leave `main`

Generated `pages/` content moved to `gh-pages`; some histories were purged of old pages/vendor content. Gates learned to fail on stale generated pages.

## Verify what is deployed

A cross-repository `check-live` checked GitHub Pages links in all six READMEs, while repo-specific browser checks verified functioning pages rather than merely HTTP success.

## Performance gates

The higher-order slowdown seen in real demos was closed with leaner evaluator kernels and follow-up fixes. The recorded `bench/inner.xtl` case moved from roughly **8.3 s to 0.81 s**; the demo repo measured `t_able` and `i_nner` faster than its pre-regression baseline.

Performance baselines became guarded artifacts: a slowdown should require deliberate approval.

## Faster development gate

A full gate had grown to around twenty minutes in some workflows. The default gate became dependency/change-aware; `--full` retained the complete release check. The project was now optimizing **developer feedback latency** as well as program runtime.

---

# 2026-10-05 — Native extensions reach HTTP

`X_eTaL-extensions` added **HTTP serving** where the X_eTaL program is the request loop and Rust supplies the host capability.

This follows the same boundary as audio and graphics: native code provides the difficult external capability; X_eTaL retains the interesting logic.

---

# 2026-10-05 — The Rosetta Stone turns the language on itself

The next major demo was planned as an “impossible” rotating Rosetta stone comparing the same idiom across languages.

The important decision was that it should be **substantially an X_eTaL program**:

- X_eTaL owns the state machine;
- X_eTaL owns the 3D projection;
- X_eTaL libraries generate the SVG scene;
- Rust/Yew/WASM is the shell for browser events, display and file loading.

The object has three conceptual axes: idiom, top language and bottom language. Unattended, it can tumble through that product space.

For the MVP, comparison data stays in **TOML**. The temptation to invent a new format was deliberately resisted. Data is planned as **aligned higher-order arrays rather than records**, making the Rosetta axes explicit.

A later `.xtln` milestone is deliberately separate: a safe, data-only X_eTaL notation with **no code execution**, designed from X_eTaL's value model rather than invented prematurely.

This is an apt endpoint for the first week: Life originally forced the language to exist; now a flagship visualization is intended to be written in X_eTaL itself, using its libraries and exposing any missing features through dogfooding.

---

# Repository birth and role summary

| Started | Repository | Role that emerged |
|---|---|---|
| Sep 27 | `X_eTaL` | Language, compiler/evaluator, renderer, tooling, browser playground and standard mechanisms |
| Oct 1 | `X_eTaL-demos` | Visual/topical demos and microscopes; independent performance/usability pressure |
| Oct 2 | `X_eTaL-games` | Interactive/stateful dogfooding; terminal requirements; idiomatic array examples |
| Oct 2 | `X_eTaL-libraries` | Reusable source libraries; proving ground for library vs macro vs core |
| Oct 2 | `X_eTaL-extensions` | Native ABI/host capabilities: SQLite, graphics, audio, 3D, HTTP |
| Oct 3 | `X_eTaL-ML` | ML-specific demos, microscopes and libraries |

# Named demo/application chronology at a glance

## Core `X_eTaL`

Life; animated rotate; language tour; Combinators/aviary; Maybe/monads; TTTML learner/play; Hello user library; Pascal triangle; Turtle/Koch/Sierpinski; Mandelbrot drawing; sieve/primes/GCD/Fibonacci/factorial/Collatz; Tower of Hanoi; quicksort; graph algorithms; sequences/cellular automata; histogram/sort/RLE; magic squares; Mastermind; truth tables/base conversion/Roman numerals; duck/rotate; LeetCode number-string problems; magmas/Rock-Paper-Scissors variants; word frequency; N-Queens; ragged Pascal; tiny APL; Fibonacci via Y; macro demonstrations; user macro library.

## `X_eTaL-demos`

Life Microscope → Mandelbrot → Reaction-Diffusion → Julia → Wave Tank → CA Lab → Langton's Ant → N-body → Image Pipeline → Ternary Net → MoE Router/Epsilon → CNN Digits → **ML demos moved to `X_eTaL-ML`** → Fourier Epicycles → Abelian Sandpile → Stencil Macros.

## `X_eTaL-games`

Horse Race → Guess → Robot Chase → Trek Adventure → Trek → Tic-Tac-Toe → Shut the Box → Minesweeper → 2048; followed by an idiomatic-X_eTaL rewrite pass over the collection.

## `X_eTaL-libraries`

Check → Strings → Sets → Numbers → Combinatorics → Lists → Matrix → Random → Format → Plot → Dates → Statistics → Graphs → Bits → Polynomials → Grouping → Csv → Search → Geometry; then domain macro libraries/features for dates, polynomials, graphs, bit fields, CSV columns and table-driven checks.

## `X_eTaL-extensions`

Hello → Clock → SQLite → CSV import → Mauna Loa data notebook → Canvas/Life window → retained 3D scene/cube → Audio → Spectrum Visualizer → Synth → Oscilloscope → triggered Oscilloscope → macro-generated FFI facades → HTTP server.

## `X_eTaL-ML`

Ternary Net → MoE Router/Epsilon → CNN Digits → NN library → Attention microscope.

---

# The larger arc

In about a week, the project passed through several qualitatively different stages:

1. **Can a typeable, statically typed array language express the compact programs I want?**
2. **Can the implementation be inspectable, testable and pleasant to use?**
3. **Can real programs expose missing semantics instead of designing features in isolation?**
4. **Which capabilities belong in the core, a source library, a macro, or a native extension?**
5. **Can examples teach the language visually and truthfully?**
6. **Can multiple repositories evolve independently while feeding requirements back upstream?**
7. **Can the ecosystem be fast, reproducible and publishable enough for a wider audience?**

The answer was not produced by a single up-front design. It emerged from a repeated loop:

**choose a concrete program → discover friction → decide whether the friction is local or general → implement the smallest general mechanism → test it at the language level → retrofit the programs → document what was learned.**

That loop is arguably the defining historical feature of X_eTaL.

It is also the reply to the early question about a tool of thought. Nobody could say at the start what X_eTaL offered as a way of thinking. The programs said it, one stage at a time: Life showed a rule as a single expression over a whole grid; axes and the classics showed which shapes of problem fit arrays; higher-order arrays and the microscopes showed computation that can be watched; games and libraries showed the same few idioms recurring; ML showed models as shapes and products; macros showed the notation itself as something a program can extend; and the Rosetta stone turns the question around, setting X_eTaL beside other languages idiom by idiom so a reader can judge the answer directly.
