# The Rosetta stone: idioms compared, on an impossible 3D object

A live demo that compares how languages write the same idiom, as a
rotating split stone: the whole stone rolls up and down through the
idioms, its top half turns through the languages, its bottom half
turns, independently, through the languages it is compared with. The
same program also runs from the command line and writes the picture
as SVG. Decided with the user from the conversation recorded in
`docs/research5.txt` (archival, not normative); this document is the
design. Saga 34 in `docs/plan.md` builds the first milestone; Sagas
35 to 37 are its follow-ups.

## What the user sees

```
                        Reverse            <- roll up: the previous idiom
                   +-----------------+
    turn left <--  |  X_eTaL         |  --> turn right: the top language
                   |  r_ev "ABCDE"   |
                   |  -> "EDCBA"     |
                   +-----------------+
    turn left <--  |  K              |  --> turn right: the bottom language
                   |  |"ABCDE"       |
                   |  -> "EDCBA"     |
                   +-----------------+
                        Rotate             <- roll down: the next idiom
```

- The object has three degrees of freedom and each one means
  something: rolling the whole stone up or down changes the idiom;
  turning the top half changes the language shown on top; turning the
  bottom half changes the language it is compared with. No rotation
  is decorative.
- Unattended, the stone is in attract mode: it dwells on a comparison
  long enough to read (about three seconds), turns the bottom half to
  the next language, dwells again, and so on; when the bottom half has
  come round, the top half turns once; when the top half has come
  round, the whole stone rolls to the next idiom. The three motions
  overlap a little, so the stone tumbles rather than ticks. The
  traversal is the nested loop `for idiom, for top, for bottom`.
- A click on a half pauses or resumes it; dragging a half sideways
  turns it; dragging anywhere up or down rolls the stone; on release
  the nearest face snaps square to the viewer. Choosing from the
  controls turns the stone the short way to the choice (half a second)
  and leaves that half paused: turning is exploring, choosing is
  navigating.
- The controls below the stone: an idiom list; a two-level language
  list for each half ("X_eTaL vs." opens the other languages, the same
  one disabled); and, from the second milestone, check boxes that say
  which languages and idioms are in the rotation (membership) beside
  the radio buttons that say which is facing you (selection).
- Each face shows the language's name, the source (X_eTaL decorated
  and colored by X_eTaL's own view model, the other languages by
  classified spans), and the result, so that the same input giving
  the same output on both halves is what the eye compares.
- A missing implementation is a face, not a hole: the face says "not
  directly expressible" or "no concise built-in idiom", and the stone
  keeps its shape.
- The URL carries the state (`?idiom=rotate&top=xetal&bottom=k`), so
  a document can link to one comparison. The angles are not in the
  URL; indexes are the truth, angles are presentation.
- With `prefers-reduced-motion`, the stone does not move on its own;
  the controls and the keyboard still navigate it.

## Architecture

The demo is an X_eTaL program; Rust is its shell. The rule that keeps
it so, and the acceptance test of the whole design:

> Given a state value and a time, the X_eTaL program produces the
> scene the browser shows, with no browser: `xetal run
> demos/rosetta/rosetta.xtl` with a scripted queue of events writes the
> same frames as SVG files.

```
  browser (Rust, Yew, WASM)                   command line (xetal run)
  pointer, clicks, controls, the clock         a scripted event queue
  URL state, resize                            --draw DIR for the frames
          |                                            |
          v                                            v
     events, one []E_VENT at a time  <--- the same program reads them
          |
          v
  demos/rosetta/rosetta.xtl        the loop: read an event, update, draw
     comparison.xtl                the state machine: indexes, sets,
                                   attract traversal, snapping
     stone.xtl                     geometry: vertices, rotation,
                                   projection, depth order
     lib/Svg.xtl                   the scene: shapes, text, groups,
                                   transforms, clips, to SVG text
     lib/Geometry3D.xtl            matrices: rotate, project
          |
          v
     []S_HOW each frame            the browser shows it; the CLI writes it
          ^
          |
     data.toml  ->  Rust loader  ->  aligned arrays  ->  []T_ABLE
```

Three things stay in Rust because only the host has them: events
(pointer, clock, the controls), the picture sink (a pane, or files),
and the TOML reader. Everything else, including the animation, the
attract-mode sequencing, the projection and the scene, is X_eTaL. If
`rosetta.rs` in the web component starts to hold geometry, traversal
or language knowledge, that is the smell to fix.

### The host boundary: two built-ins, both general

The program needs two things from the host that no built-in gives
today. Both are designed as general facilities, not Rosetta's.

1. **Events.** `[]K_EY` already reads a key (a `Key` value, typed by
   the terminal's queue, and the program waits for one). Rosetta needs
   the same for pointer and clock events: a built-in that gives the
   next event, with readers for its kind, position and elapsed time,
   so the program's loop is `e := []E_VENT @; s := u:u_pdate s e;
   []S_HOW u:s_cene s`. Decided (RS1): `[]E_VENT @ : Unit -> Event`, a
   nominal type as `Key` is, read by `[]E_KIND e` (`tick`, `down`,
   `move`, `up`, `click`, `key`), `[]E_AT e` (`x y`, or the seconds
   since the last tick) and `[]E_KEY e`; the events arrive through the
   line queue `[]K_EY` uses, one line each (`down 120 80`), the browser
   posting them and `xetal run --events FILE` reading them from a file,
   so a scripted queue is a text file and an empty queue ends the
   program.
2. **Tables.** The data is read by two built-ins, strings only, data
   only (RS2): `"file" []L_IST "idioms"` gives a top-level list of
   strings (`Box Char`), and `("file" "source") []T_ABLE ("idioms"
   "languages")` gives the table of tables `source` as a matrix whose
   rows and columns follow the two named lists, a missing cell the
   empty string; `available` is derived in X_eTaL as `"" /= cell`. A
   value that is not a string, a missing list, or a file that does not
   parse is an error naming the file and the key; nothing is ever
   evaluated. The axes are named in the call, so the file stays plain
   TOML that any tool reads. Both built-ins are useful to any program
   with tabular data, and they are the seam where the `.xtln` reader of
   Saga 37 slots in without the program noticing.

### The data model: aligned arrays, not records

The comparison database is a matrix, not a list of records: the
idioms are one axis, the languages another, and the stone is a
navigator over their coordinates. The internal representation keeps
that shape, as the research recommended (think xarray, not structs):

```
idioms       : I         names, one per row          (Box Char)
languages    : L         names, one per column       (Box Char)
source       : I x L     the code                    (Box Char)
output       : I x L     what it gives               (Box Char)
notes        : I x L     "not directly expressible"  (Box Char)
available    : I x L     "" /= source                (Bool)
```

The display of the stone is then two cells of the same row, `source[i;
t]` and `source[i; b]`: the top and bottom halves are two cursors into
one language axis, not two data dimensions. Attract mode walks
coordinates. The state is the triple of indexes `(i, t, b)`, the
membership masks of the three axes, and the mode; the animation state
is the three angles and velocities, and nothing is ever inferred from
an angle.

The X_eTaL type system keeps the planes separate (a `Bool` plane and
`Char` planes cannot share an axis), which is why there are several
aligned arrays and not one rank-3 array. That is not parallel arrays
pretending to be records: they are variables over shared, named
dimensions, and whether X_eTaL wants to name those dimensions is the
question Saga 36 asks.

### The authoring format: TOML now, `.xtln` later

`demos/rosetta/data.toml` is the one database. It is TOML because it
is content that humans edit, that Rust already reads, that other tools
and the sibling repositories can consume, and because a code snippet
in it is a string that is never evaluated. The schema is the data
model, not a list of records, so the loader is a lookup, not a
normalization:

```toml
idioms = ["reverse", "rotate", "shape", "sum"]
languages = ["apl2", "dyalog", "j", "bqn", "k", "uiua", "xetal"]

[names]
reverse = "Reverse"
rotate = "Rotate left by one"
xetal = "X_eTaL"
k = "K"

[input]
reverse = '"ABCDE"'

[source.reverse]
xetal = 'r_ev "ABCDE"'
k = "|v"
j = "|. v"

[output.reverse]
xetal = '"EDCBA"'

[notes.compose]
apl2 = "not directly expressible"
```

(`demos/rosetta/data.toml` begins with this schema, RS3.)

The same file generates the array table of `docs/idioms.md` (one
database, not two): `scripts/idioms.py` (`just idioms`) copies the
prose of `docs/idioms.template.md` through and expands its one marker
into the table, every non-ASCII glyph an HTML entity, a cell's note in
parentheses after its source (or standing alone where the language has
no source); the gate fails when the document is not current
(`scripts/idioms.py --check`). The mainstream table stays in the
template until Saga 35 puts those languages in the data. The X_eTaL
column keeps being run by `spec/integration/idioms.case`, so every
X_eTaL cell on the stone is an example that executes.

`.xtln`, X_eTaL notation, is deliberately not designed from this use
case (Saga 37 says why and what it is for). The program reads its data
through `[]T_ABLE` so that the day the file is `data.xtln` instead,
nothing above the loader changes.

### Why SVG, generated by X_eTaL

The stone in sw-ml-study's m-poc is CSS 3D: faces positioned by the
browser, dragged by pointer handlers in JavaScript. That is the right
tool when JavaScript owns the object. Here X_eTaL owns it, and an
array language computing `projected := vertices +.* rotation` for a
4 x 3 matrix of corners, sorting faces by depth and emitting a scene is
the demo's second point: the stone is also a showcase of array
programming. SVG is the output a program can produce as a value;
`[]P_ATH` and `[]G_RID` already produce it and `[]S_HOW` already
displays it in the browser pane and writes it from the CLI.

The language does not get a 3D primitive. `lib/Geometry3D.xtl` is
matrices (rotations about three axes, a perspective projection, depth
order), `lib/Svg.xtl` is a small declarative scene library (polygon,
path, text, group, clip, transform, gradient), and the SVG text is
produced by the library, not by concatenation scattered through the
demo. The first steps of the saga build these two libraries on a
rotating wireframe cube before any Rosetta semantics exist, because
they decide whether the language can own the visualization at all; if
the cube is not elegant, the plan changes before the stone is built.

The impossible geometry is three interpenetrating carousels sharing a
viewport: a vertical one of idioms, and two horizontal ones of
languages, clipped to the top and bottom halves of the current idiom
face. The viewer perceives one stone; the scene is three transformed
groups with clips. No physics is simulated, and nothing is lost by
that: the object's rotations correspond to the data's axes, not to a
rigid body.

### Syntax coloring

X_eTaL source on a face is colored by `xetal-view` (segments with
semantic classes), the same model the editor, the poster and the HTML
export use, so the demo cannot drift from the language. The other
languages are colored from classified spans in the data (`plain`,
`keyword`, `function`, `operator`, `number`, `string`, `comment`,
`punctuation`), given per cell; no parsers are written for eight
languages in the first milestone. The fonts: a monospace with the APL
glyphs for the array languages, and the view model's for X_eTaL.

## The page, as built (Saga 34 step 12)

`pages/rosetta/` is a page of its own beside the live editor, built by
`just pages` from the `rosetta` component (one Yew crate). It starts
the live demo's worker with the program, its two libraries and the
data file, with frames replacing one another; shows the latest frame
as inline SVG; and sends events: the pointer in the picture's own
pixels, keys by their names, a tick each time the program waits, and
`choose AXIS ITEM` from the three lists under the stone. The program
prints `at IDIOM TOP BOTTOM` whenever it moves, and the page reads
that line for its lists and the address bar, so the page never infers
the stone's position from anything but the program's word. With
`prefers-reduced-motion` the page sends the three pauses as it opens.
The page holds no geometry, traversal or language.

## The -ilities

- Availability: GitHub Pages, static, no server; the same build
  pipeline as the other live demos (`just pages`).
- Reliability: the state machine is tested as transitions (given top
  X_eTaL and bottom K, choosing another idiom leaves both; a click on
  the top pauses the top only; a drag of the top does not move the
  bottom), from the CLI with scripted events, not through CSS or the
  browser; the geometry is tested on known rotations (a quarter turn
  of a unit square); visual regression on three screenshots (front,
  angled, mid-tumble) as the demos repositories do.
- Usability: faces hold square to the viewer when at rest; selection
  turns the short way; the keyboard reaches everything (arrows for the
  three axes, space to pause, tab through the controls);
  `prefers-reduced-motion` honored; the URL is a bookmark.
- Maintainability: one database; data and program separated;
  languages and idioms added by editing TOML only (the architectural
  acceptance test of the first milestone: adding Rust touches no
  program file); the scene library and the geometry library reusable
  by other demos.
- Discoverability: linked from the X_eTaL catalog and the demos'
  index; each idiom's X_eTaL cell links to the reference entry of its
  built-in; the microscope view (a later step) shows the matrix with
  the two cursors, so the array under the object is visible.
- Serviceability: `xetal run demos/rosetta/rosetta.xtl --events
  FILE --draw DIR` reproduces any frame from a bug report's URL and
  event log; the data file is checked by `just rosetta-check` (every
  idiom has a name, every language a name, every X_eTaL cell runs).
- Traceability: the design here; the decisions of the lane in
  `docs/lang-choices.md` (the event and table built-ins) and
  `docs/design.md` (the register); the data file's cells cite their
  source where one exists (a manual, a reference card).
- Upgradability: the host boundary is two built-ins and a picture sink,
  so a desktop host (an X_eTaL extension with a window and a GPU, the
  follow-on in `X_eTaL-extensions`) runs the same program; the TOML
  reader is replaced by the `.xtln` reader behind `[]T_ABLE`.
- Performance: a frame is `update` then `scene` then SVG text. The
  budget is 30 frames a second for a scene of about 60 shapes in the
  worker. Measured at the profiling step; if the interpreter is short,
  the interpreter is what gets faster (this is the benchmark Saga 30
  wanted), and only a measured shortfall moves work into Rust.
- Understandability: a literate document (`docs/literate/rosetta.org`)
  walks the geometry, the three carousels and the attract traversal
  with pictures drawn by the program itself.

## Milestones

### M1, Saga 34: the stone, TOML, the array languages

The APL family on both halves: APL2, Dyalog, J, BQN, K, Uiua and
X_eTaL (APL\360 when the classics lane's eras work gives it idioms),
with the idioms of `docs/idioms.md`'s array table and the classics the
lane has in common. Three axes, attract mode, the controls, the URL,
the keyboard, reduced motion, `idioms.md` generated from the data,
screenshots, the literate document. The steps are in `docs/plan.md`.

Not in M1: diagonal drags (both axes at once), the microscope view,
language sets beyond the one family, mainstream languages, a desktop
host.

### M2, Saga 35: more languages, sets, the microscope

- The mainstream languages of `docs/idioms.md` (Java, JavaScript,
  Python, C++, Rust) and then C++, FORTH, Go, Haskell, Kotlin and
  Scala: data only, grouped into families (array, functional, systems,
  mainstream, stack). The acceptance test: no program file changes.
- Language sets per half (a preset or custom membership for the top
  and for the bottom, so the top can hold X_eTaL alone while the
  bottom cycles every other language) and idiom sets (array,
  functional, I/O, all); attract mode walks the enabled ones. The two
  halves may have different numbers of faces.
- Diagonal drags: a drag of a half maps dx to that half's language and
  dy to the idiom.
- The microscope: a small matrix beside the stone with the two cursors,
  moving as the stone is turned.
- A desktop host, in `X_eTaL-extensions`, running the same program with
  a native window: planned there, when the extensions' scene work is
  ready.

### M3, Saga 36: labeled axes, an experiment

Rosetta is the first program whose data is several aligned arrays over
named dimensions (`source[idiom; language]`), and the ML goals of the
language will bring more (`weights[layer; expert; in; out]`). This
saga uses the Rosetta program to ask whether X_eTaL wants a way to
associate coordinates with dimensions (selection by label, alignment
by name, an axis's names printed with its values), and what that would
look like with the decoration system; it decides with the user and
commits to nothing from this use case alone. Its result is a decision
in `lang-choices.md`, possibly "not yet".

### M4, Saga 37: `.xtln`, X_eTaL notation

A safe, data-only serialization of X_eTaL values: type, rank, shape,
nesting, strings, empty arrays with their kind (T9), and later the
dimension labels of Saga 36 if they exist. Values only, by
construction: the reader has no node for a call, a definition, a
macro, an import, a quad or a binding, so a `.xtln` file is no more
executable than JSON (the pickle problem, avoided at the grammar).
Round trip tests (`read (write x)` is `x` for every value kind), fuzz
tests on malformed and hostile input, `[]T_ABLE` and a general
`[]R_EAD_XTLN` (name to be decided) over it, `xetal` writing it from
the REPL, and Rosetta's `data.toml` converted as the first user. The
syntax is designed then, from the value model, not from the shape of
TOML or JSON; `docs/research5.txt` records the reasons.

## Risks and the steps that retire them

| Risk | Retired by |
| ---- | ---------- |
| X_eTaL cannot express the projection and the scene cleanly | the cube steps (34.2 to 34.5) come first and are reviewed before the stone |
| A frame is too slow in the worker | the profiling step; the interpreter is what gets faster |
| The nested-data ergonomics are poor for the database | the data step records the frictions in `docs/dogfooding.md`; Saga 36 follows from them |
| Features leak into the Rust shell | the CLI acceptance test (frames from a scripted queue) runs in the gate |
| Two comparison databases drift | `docs/idioms.md` generated from `data.toml`, checked by the gate |
| The host boundary is Rosetta-shaped | `[]E_VENT` and `[]T_ABLE` are general and documented in the reference like any built-in |
