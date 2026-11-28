# Interactive programs: the terminal

A program that reads what its user types runs the same at the command
line and in the browser. At the command line it reads the keyboard;
in the live demo its output pane is a terminal, and the program waits
there for the line, the key or the event it reads. In both, it can
draw in color and place text on the screen.

## Reading

| Call | Reads |
| ---- | ----- |
| `[]R_EAD @` | a line, without its newline (Enter ends it) |
| `[]K_EY @` | one key, without Enter, as a Key: `[]K_CHAR` gives a printing key's character, and the Terminal library names the others (`"t:" u_se< "Terminal"`, then `t:UP`, `t:ENTER`, ...) |
| `[]E_VENT @` | an event: a key, a click or a clock tick (`[]E_KIND`, `[]E_AT`, `[]E_KEY`) |

Piped input is read the same way, which is how the goldens play the
interactive demos: `printf '1122\n3344\n' | xetal run --seed 3
demos/classics/mastermind-play.xtl`.

## Drawing

| Call | Gives |
| ---- | ----- |
| `[]T_E @` | the terminal's facts: rows, columns, 1 when output is a terminal, 1 when it does screen control |
| `([]C_OLOR n) []F_G text`, `[]B_G`, `[]B_OLD` | text in a color (1 to 8: black, red, green, yellow, blue, magenta, cyan, white), on a background, in bold |
| `row col []A_T text` | text placed at a row and a column (1-origin) |
| `[]C_LS @` | the text that clears the screen and moves the cursor home |
| `[]E_RR text` | a line on standard error (red in the live demo) |

The drawing calls give text to print: the codes a terminal reads,
which the live demo's pane draws too. A program that should also read
well in a file asks `[]T_E` first, as Mastermind does with its pegs:

```
u:p_aint := { c t -> 1 = 4 s_elect []T_E @ ? ([]C_OLOR c) []F_G t; t }
```

The interactive demos: `demos/classics/mastermind-play.xtl` (pegs and
verdict in color) and `demos/tttml-play.xtl` (play the trained
machine; run `demos/tttml-train.xtl` first).

## In a page of your own: xetal-play

A web page built with Rust and Yew runs a program with the crate
`xetal-play` (in `components/web/crates/xetal-play`; docs/vendoring.md
says how a repository names it). `Interactive` is a run the page
steps: it runs a slice at a time, stops where the program reads, and
goes on from that very call when the page feeds it what was read. It
never blocks the page and never runs the program again from the start.

```rust
use std::sync::Arc;
use xetal_play::{Interactive, Memory, Step, add_library, install};

install(Arc::new(Memory::default()));        // the files a run reads and writes
add_library("Rules.xtl", RULES)?;            // imported as "r:" u_se< "Rules"
let mut run = match Interactive::start(PLAY, seed, Box::new(out)) {
    Ok(run) => run,
    Err(done) => return show(done),          // did not load or check: done.err says why
};
loop {
    match run.step(1_000_000) {              // at most this many transitions
        Step::Running => continue,           // or yield to the page and come back
        Step::Waiting => break,              // wants a line: feed it on Enter
        Step::WaitingKey => break,           // wants a key, by name: "a", "Up", "Enter"
        Step::WaitingEvent => break,         // wants an event line: "click 120 80", "tick 0.05"
        Step::Done(done) => { show(done); break } // done.err, done.pictures
    }
}
// later, when the user types a line:
run.feed(line);                              // then step again
```

What the program prints goes to `out` (any `Write`) as it runs;
pictures it shows with `[]S_HOW` come back in `Done`'s `pictures`, or
from `xetal_store::take_shown()` while it runs. To draw colors and
placed text the way the live demo does, install a terminal that says
it does screen control before starting, and draw the output on a cell
grid (the live demo's is `xetal-screen`):

```rust
xetal_tty::install(Arc::new(xetal_tty::Forward { error: on_error, facts: [24, 80, 1, 1] }));
```

`Interactive::notebook` runs the same program a statement at a time,
giving the page each statement's source before it runs.

## The terminal pane: xetal-terminal

A page that wants the live demo's terminal as it is takes the crate
`xetal-terminal` (in `components/web/crates/xetal-terminal`), the very
pane the live demo's output uses. Its `Terminal` component runs a
program in a worker as soon as it is shown:

```rust
use xetal_terminal::Terminal;

html! {
    <Terminal src={PLAY} files={vec![("Rules.xtl".into(), RULES.into())]}
        seed={7} restart={*restarts} />
}
```

- `src` is the program; `files` its libraries and data files, by name,
  as `xetal-store` holds them; `seed` the seed for `r_oll!`; changing
  `restart` (or any of the others) runs it again from the start.
- Output is drawn on a cell grid of 24 rows by 80 columns once the
  program places or colors text, so `[]A_T`, `[]C_LS` and `[]F_G` work.
- While the program waits, the pane takes the keyboard: a typed line
  for `[]R_EAD` (Enter gives it, Ctrl-C stops the run), one key for
  `[]K_EY`, and for `[]E_VENT` keys as `key NAME`, clock ticks, and a
  click on a picture as `click X Y` in the picture's own pixels.
- `[]S_HOW` pictures appear in place, under the output.
- `xetal_terminal::transcript(&output)` gives what the pane shows as
  text (the grid's rows when text was placed), for tests.

Its stylesheet is `terminal.css` beside the crate, scoped to
`.xetal-terminal`; link it from the page's `index.html` with
`<link data-trunk rel="css" href=".../xetal-terminal/terminal.css" />`.
The page also builds the worker that runs programs, as the live
demo's `index.html` does (`xetal-runner`, `data-type="worker"`).
