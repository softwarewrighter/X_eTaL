//! A diagnostic located in its file: FILE:LINE:COLUMN and the line with
//! the spot underlined (Saga 27).

use xetal_base::{Diagnostic, Place, Span};

#[test]
fn a_place_has_its_line_column_and_text() {
    let text = "n := 3\nm := n + 2.5\n";
    let p = Place::of("p.xtl", text, Span::new(12, 19));
    assert_eq!((p.line, p.col, p.width), (2, 6, 7));
    assert_eq!(p.text, "m := n + 2.5");
    assert_eq!(p.to_string(), "p.xtl:2:6");
}

#[test]
fn a_place_past_the_end_or_inside_a_character_is_safe() {
    let p = Place::of("-e", "x ← 1", Span::new(3, 99));
    assert_eq!((p.line, p.col, p.width), (1, 3, 3));
    let end = Place::of("-e", "ab", Span::new(2, 2));
    assert_eq!((end.col, end.width), (3, 1));
}

#[test]
fn a_located_error_shows_the_line_and_a_mark() {
    let d = Diagnostic::new("type-mismatch", "expected Int, found Float")
        .with_span(Span::new(5, 8))
        .with_help("convert with f_loat")
        .placed_in("-e", "1 + f 2.5");
    assert_eq!(
        d.to_string(),
        "error[type-mismatch]: expected Int, found Float\n \
         --> -e:1:6\n  |\n1 | 1 + f 2.5\n  |      ^^^\n  = help: convert with f_loat"
    );
}

#[test]
fn a_label_shows_its_own_line_with_its_words() {
    let text = "n := 2.5\n\n\n\n\n\n\n\n\n1 + n";
    let d = Diagnostic::new("type-mismatch", "expected Int, found Float")
        .with_span(Span::new(17, 22))
        .with_label(Span::new(5, 8), "n is a Float here")
        .with_note("a note")
        .placed_in("p.xtl", text);
    assert_eq!(
        d.to_string(),
        "error[type-mismatch]: expected Int, found Float\n  \
         --> p.xtl:10:1\n   |\n10 | 1 + n\n   | ^^^^^\n   |\n 1 | n := 2.5\n   \
         |      --- n is a Float here\n   = note: a note"
    );
}

#[test]
fn an_unlocated_error_keeps_its_short_form() {
    let d = Diagnostic::new("io", "cannot read x").with_help("check the path");
    assert_eq!(
        d.to_string(),
        "error[io]: cannot read x\n  help: check the path"
    );
    let spanned = Diagnostic::new("e", "m").with_span(Span::new(1, 2));
    assert_eq!(spanned.to_string(), "error[e]: m at 1..2");
}
