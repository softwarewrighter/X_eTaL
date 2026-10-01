# classics lane: handoff for the local agent

Read this after merging `feat/classics` into main. It lists what this
lane could not run or check in its cloud sandbox, what is only partly
done, and what to do locally. Keep it current: each step of the lane
adds to it, and the release step folds what is left into docs/plan.md.

The lane's saga is in `lanes/classics/.agentrail` (run agentrail with
`--saga lanes/classics`); see CLAUDE.md, "Parallel lanes".

## This pull request (pr/classics-1)

Steps 1 to 6 of the lane's 23, rebased onto main at 15121ec with no
conflicts. The lane's saga stays live in `lanes/classics/.agentrail`
(step 7, classics-draw-web, is next; nothing is archived), so after
the merge the lane continues on a fresh branch off main. On the
rebased branch the sandbox ran: lock consistency, the debug and release
builds, the spec corpus and every golden (all pass except
cli-too-deep, below). The full gate (fmt, clippy, every component's
tests, smoke, Emacs ERT, literate, reference, sw-checklist, markdown)
passed on the branch just before the final rebase onto main's newest
commits; run `scripts/gate.sh` in full after merging.

To do locally, in order:

1. `scripts/gate.sh` in full, including the wasm32 check (the png
   crate is new in xetal-draw; xetal-system, which the web engine
   uses, now depends on it).
2. `just serve`: open classics/life-drawn.xtl, turtle.xtl and
   mandelbrot.xtl from Open; they type-check and run, but `[]S_HOW`
   reports error[io] in the browser until step 7 adds the Draw pane.
   Either list them only after step 7 or accept that until then.
3. `just draw demos/classics/life-drawn.xtl` (and turtle.xtl,
   mandelbrot.xtl, pascal.xtl) on the Mac and on Arch: the pictures
   open, and the animations run (glider loops, arrowhead draws
   itself, Mandelbrot zooms).
4. `just pages` and `just literate-html` to rebuild the published
   pages with the new demos.
5. Confirm with the user: RASTER_CELLS (4096) and points as 2 rows
   for `[]P_ATH` (both below).

## Done locally after the merge (main's agent, on the Mac)

1. `scripts/gate.sh` in full on the branch: all passed, including the
   wasm32 check (the png crate builds for the browser), every golden
   (cli-too-deep passes here) and KaTeX on every line (1446).
2. The live demo: the four drawing demos (pascal, life-drawn, turtle,
   mandelbrot) are hidden from Open until the Draw pane exists (the
   user's decision; a test keeps any listed demo from drawing). Step 7
   (classics-draw-web) brings them back.
3. Pictures: all four demos run; their 12 pictures were checked in
   headless Chrome at two moments: they draw correctly and the
   animations play (the glider moves, the arrowhead draws itself).
4. `just pages` rebuilt pages/ (the literate export and the LaTeX
   gallery included).
5. The user confirmed: RASTER_CELLS stays 4096; `[]P_ATH` keeps points
   as 2 rows; transpose (`o_\`) is planned (docs/plan.md,
   cross-cutting), after which `[]P_ATH` may also take n by 2.

Also: scripts/check-modes.sh (main's gate) now checks demos/**/*.xtl,
so a demo in a subfolder must be executable and start with #!.

## The sandbox's restrictions (and what each one left unverified)

| Restriction | Effect | Do locally |
| ----------- | ------ | ---------- |
| static.rust-lang.org blocked, so no `wasm32-unknown-unknown` target | the gate's "live demo's engine builds for the browser (wasm32)" check never ran on this branch | `scripts/gate.sh` in full; in particular `(cd components/web && cargo check --target wasm32-unknown-unknown)` after the store and system changes (Store::show, xetal-draw via xetal-system) |
| No trunk, no browser | the live demo was never built or opened; `just serve`, `just pages` and `scripts/live-screenshot.sh` never ran | `just serve`, open life-drawn.xtl and pascal.xtl from Open; once step 7 lands, check the Draw pane; rebuild pages/ with `just pages` |
| Per-argument size limit (about 128 KB) | golden `cli-too-deep` fails here (its 200 KB argument); it fails the same way on main in this sandbox, so not a regression | `scripts/reg.sh run` locally: it should pass |
| No GUI viewer | `just draw FILE` (scripts/open-picture.sh) never opened a window | `just draw demos/classics/life-drawn.xtl` on the Mac (open) and an Arch box (xdg-open) |
| SVG animation not viewable | frames were checked as still PNGs (cairosvg), not as running SMIL animation | open `work/draw/life-drawn-1.svg` in Safari, Chrome and Firefox; the glider should loop with no jump |

Emacs: the sandbox installed emacs-nox 29.3 from Ubuntu, so the ERT
tests and `just check-literate` did run here. `scripts/literate-html.sh`
(the HTML export into pages/literate/) was not run, to leave pages/ to
the web-playground lane: run `just literate-html` after merging.

## Fixed along the way (worth knowing)

- `xetal run FILE | head` panicked (broken pipe); a closed stdout now
  ends the run with exit 141 (step 2, golden cli-pipe-closed).
- `scripts/literate.sh --check` used BSD `mktemp -t literate`, which GNU
  mktemp rejects: the check failed on Linux (the Arch servers). It now
  uses `mktemp "${TMPDIR:-/tmp}/literate.XXXXXX"`, which both accept.

## Merging

- Rebase onto main once, at merge time (the branch was kept
  fast-forward, never rebased while pushed).
- Expected conflicts: `components/web/crates/xetal-web/src/demos.rs`
  (both lanes add demos: keep both lists), `README.md` quick start and
  documentation list, `docs/plan.md` (the lane's section sits before
  Saga 11), and Cargo.lock files (regenerate with
  `scripts/check-locks.sh --fix`).
- Goldens rebased on this branch on purpose: run-classics-pascal,
  cli-no-subcommand (usage shows [OPTIONS] for the global --draw),
  just-list (draw recipe). If main added recipes too, rebase just-list
  again after the merge.

## Step by step: what is done, partial, or untested

| Step | State | Notes for the local agent |
| ---- | ----- | ------------------------- |
| 1 classics-index | done | |
| 2 classics-broken-pipe | done | |
| 3 classics-trig | done | |
| 4 classics-draw-grid | done | wasm32 check and browser viewing untested (above); the web host has no Draw pane yet, so in the live demo `[]S_HOW` fails with error[io] "no place to show pictures" until step 7 |
| 5 classics-draw-path | done | `[]P_ATH` takes points as 2 rows (x over y), not the n by 2 matrix first proposed to the user: there is no transpose (`o_\` is only reserved, A2), and turtle scans give rows. If transpose is added later, consider accepting both shapes (a decision for the user). docs/literate/libraries.org does not yet list Turtle (step 8 adds it). Animated path pictures were inspected as single frames only |
| 6 classics-draw-raster | done | Threshold for drawing a grid as an image chosen here, not by the user: more than 4096 cells a frame (64 by 64); confirm or change `RASTER_CELLS` in xetal-draw/src/raster.rs. Mandelbrot moved here from classics-puzzles (that step's prompt still lists it: skip it there). The demo is sized for speed (60 by 90 frames, about 8 s release): the evaluator does about 650k point-steps a second, so the zoom is coarse; the plan.md cross-cutting item on evaluator speed (primitive operands as vector kernels) would allow bigger views. Raster pictures were checked as stills; watch the zoom animate in a browser. New dependency: the png crate (pure Rust) in xetal-draw, so check the wasm32 build |
| 7 classics-literate | done | `ob-xetal` takes `:results file :file PATH` (Org links a file only when the block asks for file results; the default stays `output`). `:pictures all` from the plan was not built: a block saves its last picture. `scripts/literate-html.sh` was not run here (to leave pages/ to the web lane): run `just literate-html` and look at classics.html, where the glider, Koch and arrowhead pictures should show and animate. `scripts/frames-to-webp.py` needs Python's cairosvg and Pillow (`pip install cairosvg pillow`, and the cairo library: `brew install cairo` / `pacman -S cairo`). Found and fixed here: a REPL session replayed `[]S_HOW` on every later line (xetal-store replay counts) |

## Language gaps found by the programs

Each is a candidate for a decision with the user, not something this
lane resolved.

- Transpose (`o_\`, reserved in A2): paths had to use rows; any
  program wanting columns of points needs it.
- Complex numbers (deferred in lang-choices section 15): Mandelbrot
  carries z as two Float arrays.
