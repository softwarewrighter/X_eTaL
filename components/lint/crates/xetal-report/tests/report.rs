//! The report an editor shows says what `xetal lint` prints, line for
//! line, and carries the fixed text.

use xetal_linter::Request;
use xetal_report::{Kind, report};

const UNUSED: &str = "u:k_ := { x y -> x }\nu:k_ 1 2";

/// What `xetal lint` writes on standard error for `text`.
fn printed(text: &str) -> String {
    let req = Request {
        input: Some(("t.xtl".into(), text.into())),
        ..Request::default()
    };
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let _ = xetal_linter::command(&req, &mut out, &mut err);
    String::from_utf8(err).unwrap()
}

fn texts(text: &str) -> (Vec<String>, Vec<Kind>) {
    let r = report("t.xtl", text);
    let lines = r.lines.iter().map(|l| l.text.clone()).collect();
    (lines, r.lines.iter().map(|l| l.kind).collect())
}

#[test]
fn a_finding_is_reported_as_the_command_prints_it_and_its_fix_is_made() {
    let r = report("t.xtl", UNUSED);
    let (lines, kinds) = texts(UNUSED);
    assert_eq!(lines.join("\n") + "\n", printed(UNUSED));
    assert!(
        lines[0].starts_with("warning[unused-binding]: parameter `y` is never used"),
        "{lines:?}"
    );
    assert!(lines[0].ends_with(" at t.xtl:1:13"), "{lines:?}");
    assert_eq!(lines[2], "1 warning(s), 0 error(s)");
    assert_eq!(kinds, [Kind::Warning, Kind::Note, Kind::Note]);
    assert_eq!(r.fixable, 1);
    assert_eq!(r.fixed.as_deref(), Some("u:k_ := { x _u -> x }\nu:k_ 1 2"));
}

#[test]
fn a_clean_text_has_the_count_only_and_nothing_to_fix() {
    let r = report("t.xtl", "u:k_ := { x _u -> x }\nu:k_ 1 2");
    let lines: Vec<&str> = r.lines.iter().map(|l| l.text.as_str()).collect();
    assert_eq!(lines, ["0 warning(s), 0 error(s)"]);
    assert_eq!(r.lines[0].kind, Kind::Note);
    assert_eq!((r.fixed, r.fixable), (None, 0));
}

#[test]
fn a_denied_finding_is_an_error_line() {
    let (lines, kinds) = texts("# lint-file: deny style\nCount! := 0");
    assert!(lines[0].starts_with("error[mutable-constant]"), "{lines:?}");
    assert_eq!(lines[2], "0 warning(s), 1 error(s)");
    assert_eq!(kinds, [Kind::Error, Kind::Note, Kind::Note]);
}

#[test]
fn a_text_that_does_not_load_is_its_error() {
    let r = report("t.xtl", "1 + 'x");
    assert_eq!(r.lines.len(), 1, "{:?}", r.lines);
    assert_eq!(r.lines[0].kind, Kind::Error);
    assert!(
        r.lines[0].text.starts_with("error[bad-quote]"),
        "{:?}",
        r.lines
    );
    assert_eq!(r.fixed, None);
}

#[test]
fn a_library_is_linted_as_one() {
    let (lines, _) = texts("l:f_ := { x -> 1 }");
    assert!(
        lines[0].starts_with("warning[unused-binding]: parameter `x`"),
        "{lines:?}"
    );
    assert_eq!(
        report("t.xtl", "l:f_ := { x -> 1 }").fixed.as_deref(),
        Some("l:f_ := { _u -> 1 }")
    );
}
