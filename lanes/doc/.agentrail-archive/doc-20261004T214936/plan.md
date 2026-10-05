# doc lane: Saga 32, xetal doc (a cross-reference, before the launch)

A lane (branch per PR, from the latest origin/main) whose saga lives in
lanes/doc/.agentrail; run agentrail with `--saga lanes/doc`. Given to
the remote agent by the user (2026-10-04) to develop in parallel; the
local agent skips main's steps 065-069 as these PRs merge.

The user's idea: a generator like rustdoc or JavaDoc that takes a
program, expands every macro, follows its `.xtl` and `.xtlm` imports
into the libraries and System.xtlm, and builds an indexed, searchable
site, so a reader can follow a program down into all the X_eTaL
beneath it in small, commented pieces. It lives in this repository, as
rustdoc ships with rustc (it needs the expansion map, library lookup,
inferred types, spans and the decorated renderer).

Decided with the user (2026-10-04), to record in lang-choices:
- doc comments are `##` blocks, as System.xtlm uses them: a `##` block
  directly above a definition documents it; a leading `##` block is the
  file's own doc; `### Heading` starts a section; `## >> code` is an
  example whose output follows on the next `##` lines; plain `#` stays
  an ordinary comment;
- the examples are run, as rustdoc runs doc tests: `xetal doc --test`,
  in the gate.

Steps
1. doc-model: the decision recorded; `xetal doc FILE --json`: every item
   of a program, its `.xtl` and `.xtlm` imports and System.xtlm (name,
   kind, inferred type, doc comment, sections, examples, file and span,
   source, uses), test-first with goldens.
2. doc-test: `xetal doc --test FILE...` runs every `## >>` example and
   compares its output; in the gate over lib/.
3. doc-site: `xetal doc FILE --out DIR`, a static site in rustdoc's
   style (Rust only, no Node): an index, a page per library and macro
   library, items with types and doc comments, source drawn decorated,
   every name linked to its definition and uses; goldens on lib/.
4. doc-macros: each macro call expands in place, linking into its
   definition (`.xtlm`, System.xtlm), nested to their depth; u_se<
   documented by its comment model (MC21).
5. doc-search: by name and by type (Hoogle-like), over the program,
   its libraries and the built-ins, in the browser without Node.
6. doc-release: `just doc` builds pages/doc for lib/ and two showcase
   programs; README, reference and Help say what it is and how to use
   it; the lane archived.

Rules: TDD; the gate before every commit; one PR per step
(pr/doc-<slug>) from the latest origin/main; README ASCII-only.
