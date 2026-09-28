# arrays

Saga 4 of X_eTaL (see docs/plan.md). Milestone M3 from docs/PRD.md:
dense arrays with 1-origin indexing, structural built-ins, scalar
extension, strings as Char vectors, printed arrays, and a REPL. Two
enabling steps come first: the components layout (expand up and out
when sw-checklist limits are reached, never merge) and the fix for
the polymorphic-literal gap found in Saga 3.

Specification: docs/lang-choices.md (governs). Relevant decisions:
A1-A7, B4, B5, T2, T3, D-10, section 10a (printed results), section
13 (strings), T5/T6 (types). Ask the user before resolving anything
lang-choices leaves open.

Rules for every step: strict TDD (failing test first, rejection tests
for every accepted rule), tests are the spec (spec/**/*.case, crate
tests, proptest, reg-rs goldens), no panics on any input, design new
code to the stricter gates (25 LOC/fn, 5 fns/module, 5 modules/crate,
5 crates/component; lib.rs a facade) and when a unit is full add a
sibling module, crate or component -- never merge. Run the gate
before committing, commit (including .agentrail/ and Cargo.lock
files) before agentrail complete, push to origin/main.

## Steps

1. components-layout -- restructure into components/<name>/ multi-crate
   workspaces as in ../../sw-ml-study/sw-mlpl: one shared target dir
   (.cargo/config.toml), path dependencies across components,
   scripts/build-all.sh builds every component, scripts/gate.sh and
   scripts/check-locks.sh cover every workspace; split xetal-types
   (at the 7-module limit) into crates; no behavior change (spec and
   goldens pass unchanged); CLAUDE.md, architecture.md, README
   development section.
2. num-dictionaries -- close the polymorphic-literal gap: Num-quantified
   functions take a hidden number-type argument (dictionary passing,
   elaborated into Core by the checker), so a literal inside a
   polymorphic function used at Float is a Float (`u:k_ := { @ -> 1 }`
   used as Float prints 1.0, including through p_rint!); design.md.
3. array-core -- xetal-array: dense row-major arrays, rank-0 scalars,
   strands, empty arrays, scalar extension by one lifting rule, shape
   errors, T2/T3/D-10 over arrays, printed arrays (10a); Array types.
4. structural-builtins -- s_hape r_eshape r_ange o_ffsets t_ally
   f_irst t_ake d_rop s_elect r_avel c_at (B4, B5), 1-origin (A5),
   typed; property tests (shape of reshape, take/drop).
5. strings -- strings as Char vectors (section 13), printing,
   equality and structural built-ins on strings.
6. repl -- xetal repl with persistent definitions and types.
7. m3-docs-release -- README M3 tour (every command a golden), docs
   sync, retrospective.
