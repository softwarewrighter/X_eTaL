//! Linting whole programs: findings placed, directives scoped, levels applied.

use xetal_lint::Level;
use xetal_linter::{Request, command, command_line, lint};

fn found(text: &str) -> Vec<(String, usize, Level)> {
    let rules = command_line([&[], &[], &[]]).unwrap();
    let findings = lint("t.xtl", text, &rules).expect("loads");
    findings
        .into_iter()
        .map(|f| (f.lint.name.to_string(), f.line, f.level))
        .collect()
}

#[test]
fn findings_are_placed_by_line_in_order() {
    let got = found("x := 1\nCount! := 0\nu:N_ext := { _r }");
    let want = [
        ("mutable-constant", 2, Level::Warn),
        ("function-name-case", 3, Level::Warn),
    ];
    assert_eq!(got, want.map(|(c, l, v)| (c.to_string(), l, v)));
}

#[test]
fn an_own_line_directive_covers_the_whole_statement_below() {
    let text =
        "# lint: allow shadows-builtin\nu:a_pply := { r_ev ->\n  r_ev 1 2 3 }\n{ r_ev -> r_ev } 4";
    assert_eq!(
        found(text),
        [("shadows-builtin".to_string(), 4, Level::Warn)]
    );
}

#[test]
fn comments_and_blank_lines_between_a_directive_and_its_statement_are_skipped() {
    let text = "# lint: allow style\n\n# the counter\nCount! := 0\nTotal! := 0";
    assert_eq!(
        found(text),
        [("mutable-constant".to_string(), 5, Level::Warn)]
    );
}

#[test]
fn a_trailing_directive_covers_its_line() {
    assert!(found("Count! := 0 # lint: allow mutable-constant").is_empty());
}

#[test]
fn a_file_directive_covers_everything_and_can_deny() {
    let got = found("# lint-file: deny style\nCount! := 0\nu:N_ext := { _r }");
    assert!(
        got.iter().all(|(_, _, level)| *level == Level::Deny),
        "{got:?}"
    );
    assert_eq!(got.len(), 2);
}

#[test]
fn a_misspelled_directive_warns_unknown_lint() {
    let got = found("# lint: allow shadow-builtin\nx := 1\n# lint: alow style\ny := 2");
    let want = [
        ("unknown-lint", 1, Level::Warn),
        ("unknown-lint", 3, Level::Warn),
    ];
    assert_eq!(got, want.map(|(c, l, v)| (c.to_string(), l, v)));
}

#[test]
fn denied_findings_fail_the_command_and_are_written_with_a_note() {
    let req = Request {
        input: Some(("t.xtl".into(), "Count! := 0".into())),
        deny: vec!["style".into()],
        ..Request::default()
    };
    let mut err = Vec::new();
    let result = command(&req, &mut err);
    assert_eq!(result.unwrap_err().code, "lint-denied");
    let text = String::from_utf8(err).unwrap();
    assert!(text.starts_with("error[mutable-constant]: "), "{text}");
    assert!(text.contains("at t.xtl:1:"), "{text}");
    assert!(
        text.contains("xetal lint --explain mutable-constant"),
        "{text}"
    );
}

#[test]
fn a_clean_program_writes_nothing() {
    let req = Request {
        input: Some(("t.xtl".into(), "x := 1\nx + 1".into())),
        ..Request::default()
    };
    let mut err = Vec::new();
    assert_eq!(command(&req, &mut err).unwrap(), "");
    assert!(err.is_empty());
}

#[test]
fn list_and_explain_need_no_program() {
    let list = Request {
        list: true,
        ..Request::default()
    };
    assert!(
        command(&list, &mut Vec::new())
            .unwrap()
            .contains("function-name-case")
    );
    let explain = Request {
        explain: Some("mutable-constant".into()),
        ..Request::default()
    };
    assert!(
        command(&explain, &mut Vec::new())
            .unwrap()
            .starts_with("mutable-constant (style, warn by default)")
    );
    let unknown = Request {
        explain: Some("nope".into()),
        ..Request::default()
    };
    assert_eq!(
        command(&unknown, &mut Vec::new()).unwrap_err().code,
        "unknown-lint"
    );
}
