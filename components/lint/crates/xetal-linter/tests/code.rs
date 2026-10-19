//! Code strings and the linter (ST5, step 112): `string-kind` where a
//! string's kind is not the kind its macro takes, with the fix; the
//! inside of a code string on a code side checked on its own before
//! expansion, as a lambda over the names it uses (`code-check`); and
//! the ordinary lints run on that inside, placed there.

use std::fs;
use std::path::PathBuf;

use xetal_linter::{Finding, command_line, lint};

/// `x:s_tage<` copies code into a lambda over `it`; `x:q_uiet<` takes
/// code but answers for it itself (parsed); `x:n_ote<` takes text.
const LIB: &str = r#"## takes: @ code
## binds: it
m:s_tage< := { @ c -> "{ it -> " c_at c c_at " } 1" }

## takes: @ code
## uses: parsed
m:q_uiet< := { @ c -> "0" }

## takes: @ text
## uses: parsed
m:n_ote< := { @ t -> "0" }
"#;

/// Lint `body` (after an import of the library as `x:`, line 1) in a
/// directory of this test's own.
fn found(test: &str, body: &str) -> Vec<Finding> {
    let dir = std::env::temp_dir().join(format!("xetal-code-{}-{test}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("M.xtlm"), LIB).unwrap();
    let file: PathBuf = dir.join("t.xtl");
    let text = format!("\"x:\" u_se< \"M\"\n{body}");
    let rules = command_line([&[], &[], &[]]).unwrap();
    let got = lint(file.to_str().unwrap(), &text, &rules).expect("loads");
    fs::remove_dir_all(&dir).unwrap();
    got
}

/// The findings of one lint: line, column, message.
fn of(got: &[Finding], name: &str) -> Vec<(usize, usize, String)> {
    got.iter()
        .filter(|f| f.lint.name == name)
        .map(|f| (f.line, f.col, f.message.clone()))
        .collect()
}

/// `text` with the fix of the finding of lint `name` made.
fn fixed(got: &[Finding], name: &str, body: &str) -> String {
    let text = format!("\"x:\" u_se< \"M\"\n{body}");
    let f = got.iter().find(|f| f.lint.name == name).expect("found");
    assert!(!f.fix.is_empty(), "no fix: {f:?}");
    xetal_linter::apply(&text, std::slice::from_ref(&f.fix)).0
}

#[test]
fn a_plain_string_where_code_is_taken_is_string_kind_with_its_fix() {
    let body = "p_rint! @ x:s_tage< \"it + 1\"";
    let got = found("plain", body);
    let kind = of(&got, "string-kind");
    assert_eq!(kind.len(), 1, "{got:?}");
    assert_eq!((kind[0].0, kind[0].1), (2, 21));
    assert!(
        kind[0].2.contains("x:s_tage< takes code on its right"),
        "{}",
        kind[0].2
    );
    assert!(fixed(&got, "string-kind", body).ends_with("x:s_tage< c\"it + 1\""));
    // A raw string becomes the verbatim form, the same value.
    let body = "p_rint! @ x:s_tage< r\"it + 1\"";
    let got = found("raw", body);
    assert!(fixed(&got, "string-kind", body).ends_with("x:s_tage< c#\"it + 1\"#"));
}

#[test]
fn a_code_string_where_text_is_taken_is_string_kind_with_its_fix() {
    let body = "p_rint! @ x:n_ote< c\"the size check\"";
    let got = found("text", body);
    let kind = of(&got, "string-kind");
    assert_eq!(kind.len(), 1, "{got:?}");
    assert!(
        kind[0].2.contains("x:n_ote< takes text on its right"),
        "{}",
        kind[0].2
    );
    assert!(fixed(&got, "string-kind", body).ends_with("x:n_ote< \"the size check\""));
    // The kind each side takes, written: nothing to say.
    assert!(
        of(
            &found("right", "p_rint! @ x:s_tage< c\"it + 1\""),
            "string-kind"
        )
        .is_empty()
    );
    assert!(
        of(
            &found("right2", "p_rint! @ x:n_ote< \"words\""),
            "string-kind"
        )
        .is_empty()
    );
}

#[test]
fn code_that_cannot_check_on_its_own_is_found_inside_the_string() {
    // The macro answers for its text (parsed), so its expansion says
    // nothing; the inside, checked as a lambda, does not know n_egg.
    let got = found("unknown", "p_rint! @ x:q_uiet< c\"n_egg 1\"");
    let check = of(&got, "code-check");
    assert_eq!(check.len(), 1, "{got:?}");
    assert_eq!((check[0].0, check[0].1), (2, 23));
    assert!(check[0].2.contains("n_egg"), "{}", check[0].2);
    // A type error, placed through the escapes as written.
    let got = found("type", "p_rint! @ x:q_uiet< c\"1 + \\\"a\\\"\"");
    let check = of(&got, "code-check");
    assert_eq!(check.len(), 1, "{got:?}");
    assert!(
        check[0].2.contains("expected a number, found Char"),
        "{}",
        check[0].2
    );
    // Free names are the lambda's parameters: `y` is not unknown.
    assert!(
        of(
            &found("free", "p_rint! @ x:q_uiet< c\"y + 1\""),
            "code-check"
        )
        .is_empty()
    );
}

#[test]
fn the_ordinary_lints_run_on_the_inside() {
    let got = found("lints", "p_rint! @ x:q_uiet< c\"y := 1; 2\"");
    let unused = of(&got, "unused-binding");
    assert_eq!(unused.len(), 1, "{got:?}");
    assert_eq!((unused[0].0, unused[0].1), (2, 23));
}
