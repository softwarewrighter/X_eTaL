//! Machine-applicable suggestions: a finding's edit to the file as
//! written, `--fix` applying them, and `--format json` for editors.

use std::path::PathBuf;

use xetal_linter::{Edit, Request, command, fixed, lint};

const REDUNDANT: &str = "u:f_ := { v -> '{ x -> n_eg x } e_ach v }";

#[test]
fn a_redundant_lambda_carries_its_edit() {
    let found = lint("t.xtl", REDUNDANT, &[]).expect("loads");
    let edit = found[0].fix.as_ref().expect("an edit");
    assert_eq!(&REDUNDANT[edit.start..edit.end], "{ x -> n_eg x }");
    assert_eq!(edit.replacement, "n_eg");
}

#[test]
fn findings_without_a_sure_edit_carry_none() {
    let found = lint("t.xtl", "Count! := 0", &[]).expect("loads");
    assert_eq!(found[0].fix, None);
}

#[test]
fn edits_apply_from_the_end_and_are_counted() {
    let text = "v := 1 2 3\n'{ x -> x } e_ach v\n'{ a -> n_eg a } e_ach v";
    let found = lint("t.xtl", text, &[]).expect("loads");
    let (out, n) = fixed(text, &found);
    assert_eq!(out, "v := 1 2 3\n'i_d e_ach v\n'n_eg e_ach v");
    assert_eq!(n, 2);
}

#[test]
fn overlapping_edits_apply_once() {
    let text = "u:f_ := { v -> 1 }";
    let edits = [Edit::new(0, 4, "a"), Edit::new(2, 6, "b")];
    assert_eq!(
        xetal_linter::apply(text, &edits),
        ("a := { v -> 1 }".into(), 1)
    );
}

/// A file of this test's own, in the system's temporary directory.
fn scratch(name: &str, text: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!("xetal-fix-{}-{name}.xtl", std::process::id()));
    std::fs::write(&path, text).unwrap();
    path
}

#[test]
fn fix_rewrites_the_file_and_reports_what_is_left() {
    let path = scratch("rewrites", &format!("{REDUNDANT}\nCount! := 0\n"));
    let name = path.to_string_lossy().to_string();
    let text = std::fs::read_to_string(&path).unwrap();
    let req = Request {
        input: Some((name, text)),
        fix: true,
        ..Request::default()
    };
    let mut err = Vec::new();
    command(&req, &mut Vec::new(), &mut err).unwrap();
    let after = std::fs::read_to_string(&path).unwrap();
    std::fs::remove_file(&path).unwrap();
    assert_eq!(after, "u:f_ := { v -> 'n_eg e_ach v }\nCount! := 0\n");
    let err = String::from_utf8(err).unwrap();
    assert!(err.contains("fixed 1 finding(s) in "), "{err}");
    assert!(err.contains("warning[mutable-constant]"), "{err}");
    assert!(!err.contains("redundant-lambda"), "{err}");
}

#[test]
fn fix_needs_a_file() {
    let req = Request {
        input: Some(("-e".into(), REDUNDANT.into())),
        fix: true,
        ..Request::default()
    };
    let got = command(&req, &mut Vec::new(), &mut Vec::new());
    assert_eq!(got.unwrap_err().code, "fix-needs-file");
}

#[test]
fn json_is_one_object_a_line_on_standard_output() {
    let req = Request {
        input: Some(("t.xtl".into(), format!("{REDUNDANT}\nCount! := 0"))),
        json: true,
        ..Request::default()
    };
    let (mut out, mut err) = (Vec::new(), Vec::new());
    command(&req, &mut out, &mut err).unwrap();
    assert!(err.is_empty());
    let out = String::from_utf8(out).unwrap();
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines.len(), 2, "{out}");
    assert_eq!(
        lines[0],
        concat!(
            r#"{"lint":"redundant-lambda","group":"complexity","level":"warn","#,
            r#""file":"t.xtl","line":1,"col":17,"#,
            r#""message":"this lambda only passes its arguments to `n_eg`: write `'n_eg`","#,
            r#""fix":{"start":16,"end":31,"replacement":"n_eg"}}"#
        )
    );
    assert!(
        lines[1].contains(r#""lint":"mutable-constant""#),
        "{}",
        lines[1]
    );
    assert!(lines[1].ends_with(r#""fix":null}"#), "{}", lines[1]);
}

#[test]
fn json_escapes_what_json_must() {
    assert_eq!(
        xetal_linter::json_string("a\"b\\c\nd\u{1}"),
        r#""a\"b\\c\nd\u0001""#
    );
}
