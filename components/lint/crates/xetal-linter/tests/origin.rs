//! Lints that know where Core came from (docs/lint.md section 8): text
//! copied from a macro call's arguments is the caller's code; text the
//! macro wrote is not reported at the call. `macro-capture` finds a
//! caller's name bound by a name the macro wrote without
//! `[]G_ENSYM` or `## binds:`; `unused-anaphor` a declared anaphor the
//! call's text never reads.

use std::fs;
use std::path::PathBuf;

use xetal_linter::{command_line, lint};

/// A macro library: a capture, a declared anaphor, a gensym, a macro
/// that binds a local of its own it never reads, the shorthand in a
/// lambda the macro opens, and two that declare `## uses: parsed`.
const LIB: &str = r#"## Binds x around the text, undeclared: a capture.
m:c_ap< := { @ e -> "{ @ -> x := 3; " c_at e c_at " } @" }

## binds: it
m:w_ith< := { e body -> "{ @ -> it := (" c_at e c_at "); " c_at body c_at " } @" }

m:f_resh< := { @ e ->
  t := []G_ENSYM "x"
  "{ @ -> " c_at t c_at " := 3; " c_at t c_at " + (" c_at e c_at ") } @"
}

m:w_aste< := { @ e -> "{ @ -> tmp := 1; " c_at e c_at " } @" }

m:d_ef< := { name body -> "u:" c_at name c_at " := { " c_at body c_at " }" }

## The same as c_ap< and w_aste<, declared as reading a notation.
## uses: parsed
m:p_cap< := { @ e -> "{ @ -> x := 3; " c_at e c_at " } @" }

## uses: parsed
m:p_ass< := { @ e -> "(" c_at e c_at ")" }
"#;

/// Lint `body` (after an import of the library as `x:`) in a directory
/// of this test's own, with `warn` turned on: lint, line, column.
fn found(test: &str, body: &str, warn: &[&str]) -> Vec<(String, usize, usize, String)> {
    let dir = std::env::temp_dir().join(format!("xetal-origin-{}-{test}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("M.xtlm"), LIB).unwrap();
    let file: PathBuf = dir.join("t.xtl");
    let text = format!("\"x:\" u_se< \"M\"\n{body}");
    let warn: Vec<String> = warn.iter().map(|s| s.to_string()).collect();
    let rules = command_line([&[], &warn, &[]]).unwrap();
    let got = lint(file.to_str().unwrap(), &text, &rules).expect("loads");
    fs::remove_dir_all(&dir).unwrap();
    got.into_iter()
        .map(|f| (f.lint.name.to_string(), f.line, f.col, f.message))
        .collect()
}

#[test]
fn a_callers_name_bound_by_the_macro_is_a_capture() {
    let got = found("capture", "x := 10\np_rint! @ x:c_ap< \"x + 1\"", &[]);
    assert_eq!(got.len(), 1, "{got:?}");
    assert_eq!(
        (got[0].0.as_str(), got[0].1, got[0].2),
        ("macro-capture", 3, 20)
    );
    assert!(got[0].3.contains("x:c_ap<"), "{}", got[0].3);
    assert!(got[0].3.contains("[]G_ENSYM"), "{}", got[0].3);
}

#[test]
fn every_captured_read_is_reported() {
    let got = found("reads", "p_rint! @ x:c_ap< \"x * x\"", &[]);
    let cols: Vec<usize> = got.iter().map(|f| f.2).collect();
    assert_eq!(cols, [20, 24], "{got:?}");
}

#[test]
fn a_declared_anaphor_or_a_gensym_is_no_capture() {
    let body = "p_rint! \"3 + 4\" x:w_ith< \"it * it\"\nx := 1\np_rint! @ x:f_resh< \"x + 1\"";
    assert_eq!(found("declared", body, &[]), []);
}

#[test]
fn a_name_the_callers_text_binds_itself_is_no_capture() {
    let got = found("own", "p_rint! @ x:c_ap< \"{ x -> x + 1 } 5\"", &[]);
    assert_eq!(got, []);
}

#[test]
fn the_shorthands_names_in_a_lambda_the_macro_wrote_are_no_capture() {
    let got = found(
        "shorthand",
        "\"s_quare\" x:d_ef< \"_r * _r\"\nu:s_quare 7",
        &[],
    );
    assert_eq!(got, []);
}

#[test]
fn an_unread_anaphor_is_not_reported_at_the_call_by_default() {
    assert_eq!(
        found("anaphor", "p_rint! \"3 + 4\" x:w_ith< \"5\"", &[]),
        []
    );
}

#[test]
fn an_unread_anaphor_is_unused_anaphor_when_asked_for() {
    let got = found(
        "anaphor-on",
        "p_rint! \"3 + 4\" x:w_ith< \"5\"",
        &["pedantic"],
    );
    assert_eq!(got.len(), 1, "{got:?}");
    assert_eq!(
        (got[0].0.as_str(), got[0].1, got[0].2),
        ("unused-anaphor", 2, 9)
    );
    assert!(got[0].3.contains("x:w_ith< binds `it`"), "{}", got[0].3);
    assert!(!got[0].3.contains("_ :="), "{}", got[0].3);
}

#[test]
fn a_finding_in_code_the_macro_wrote_is_not_reported_at_the_call() {
    assert_eq!(found("internal", "p_rint! @ x:w_aste< \"1\"", &[]), []);
    let got = found(
        "internal-on",
        "p_rint! @ x:w_aste< \"1\"",
        &["macro-internals"],
    );
    assert_eq!(got.len(), 1, "{got:?}");
    assert_eq!(got[0].0, "macro-internals");
    assert!(got[0].3.contains("x:w_aste< wrote"), "{}", got[0].3);
    assert!(got[0].3.contains("unused-binding"), "{}", got[0].3);
}

#[test]
fn a_finding_in_the_callers_own_text_is_still_reported_there() {
    let got = found("text", "p_rint! @ x:w_aste< \"{ y -> 1 } 2\"", &[]);
    assert_eq!(got.len(), 1, "{got:?}");
    assert_eq!(
        (got[0].0.as_str(), got[0].1, got[0].2),
        ("unused-binding", 2, 24)
    );
}

#[test]
fn a_parsing_macro_binding_a_name_of_its_notation_is_no_capture() {
    let got = found(
        "parsed-capture",
        "x := 10\np_rint! @ x:p_cap< \"x + 1\"",
        &[],
    );
    assert_eq!(got, []);
}

#[test]
fn a_finding_in_a_parsing_macros_expansion_is_not_the_callers() {
    let text = "p_rint! @ x:p_ass< \"{ y -> 1 } 2\"";
    assert_eq!(found("parsed", text, &[]), []);
    let got = found("parsed-on", text, &["macro-internals"]);
    assert_eq!(got.len(), 1, "{got:?}");
    assert_eq!(
        (got[0].0.as_str(), got[0].1, got[0].2),
        ("macro-internals", 2, 23)
    );
    assert!(got[0].3.contains("x:p_ass< wrote"), "{}", got[0].3);
}
