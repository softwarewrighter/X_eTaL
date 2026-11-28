//! The transcript: plain output as printed, placed text as the grid
//! shows it, errors after, a line per picture.

use xetal_play::Run;
use xetal_runner::Output;
use xetal_terminal::{screen_text, transcript};

fn shown(out: &str, err: &str, pictures: usize) -> Output {
    let run = Run {
        out: out.into(),
        err: err.into(),
        pictures: vec!["<svg/>".into(); pictures],
        ..Run::default()
    };
    Output {
        run: Some(run),
        ..Output::default()
    }
}

#[test]
fn nothing_before_a_run() {
    assert_eq!(transcript(&Output::default()), "");
}

#[test]
fn plain_output_then_errors_then_pictures() {
    let t = transcript(&shown("1 2 3\nname? Ada\n", "error[x]: no\n", 1));
    assert_eq!(t, "1 2 3\nname? Ada\nerror[x]: no\n[picture]\n");
}

#[test]
fn placed_text_as_the_grid_shows_it() {
    // Clear, then "b" at row 2 column 3, then "a" home: the grid's rows.
    let out = "\x1b[2J\x1b[H\x1b[2;3Hb\x1b[1;1Ha";
    assert_eq!(screen_text(out), "a\n  b\n");
}

#[test]
fn colors_leave_only_the_text() {
    assert_eq!(screen_text("\x1b[31mred\x1b[0m"), "red\n");
}
