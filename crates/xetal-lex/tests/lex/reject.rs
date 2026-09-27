//! Rejection tests: every accepted token form has a malformed neighbour
//! that must fail with a specific code and span.

use crate::common::reject;
use xetal_base::{Diagnostic, Span};

fn assert_reject(src: &str, code: &str, start: usize, end: usize) {
    assert_eq!(
        reject(src),
        (code.to_string(), Span::new(start, end)),
        "source {src:?}"
    );
}

#[test]
fn bad_underline_decorations() {
    assert_reject("r__", "bad-decoration", 2, 3);
    assert_reject("r_2_", "bad-decoration", 3, 4);
    assert_reject("r_0", "bad-axis", 2, 3);
    assert_reject("t_11", "bad-axis", 3, 4);
    assert_reject("t_2x", "bad-decoration", 3, 4);
    assert_reject("now_@x", "bad-decoration", 5, 6);
    assert_reject("now_@2", "bad-decoration", 5, 6);
}

#[test]
fn stems_cannot_contain_underscore() {
    assert_reject("a_b", "bad-decoration", 2, 3);
    assert_reject("my_name", "bad-decoration", 3, 4);
}

#[test]
fn bad_superscripts() {
    assert_reject("r^", "bad-decoration", 1, 2);
    assert_reject("r^ x", "bad-decoration", 1, 2);
    assert_reject("+^2", "bad-decoration", 1, 2);
    assert_reject("+^r^s", "bad-decoration", 3, 4);
    assert_reject("r_2^r", "non-canonical-order", 3, 4);
    assert_reject("r_^r", "non-canonical-order", 2, 3);
}

#[test]
fn redundant_bare_underline() {
    assert_reject("+_", "redundant-underline", 1, 2);
    assert_reject("+_ 3", "redundant-underline", 1, 2);
    assert_reject("f^r_", "redundant-underline", 3, 4);
    assert_reject("+^r_", "redundant-underline", 3, 4);
}

#[test]
fn bad_lambda_arguments() {
    assert_reject("_x", "bad-lambda-arg", 0, 2);
    assert_reject("_", "bad-lambda-arg", 0, 1);
    assert_reject("_ r", "bad-lambda-arg", 0, 1);
    assert_reject("_lx", "bad-lambda-arg", 0, 3);
    assert_reject("_r2", "bad-lambda-arg", 0, 3);
    assert_reject("_r_", "bad-lambda-arg", 2, 3);
    assert_reject("_l^r", "bad-lambda-arg", 2, 3);
    assert_reject("_@", "bad-lambda-arg", 0, 2);
}

#[test]
fn bad_namespaces() {
    assert_reject("m.x", "bad-namespace", 0, 3);
    assert_reject("m.+", "bad-namespace", 1, 2);
    assert_reject("a.b.f_", "bad-namespace", 3, 4);
    assert_reject("m.", "bad-namespace", 1, 2);
    assert_reject("m._r", "bad-namespace", 1, 2);
    assert_reject("f_.g", "bad-decoration", 2, 3);
}

#[test]
fn ambiguous_minus_needs_spaces() {
    assert_reject("3-1", "ambiguous-minus", 1, 2);
    assert_reject("a-1", "ambiguous-minus", 1, 2);
    assert_reject("(x)-1", "ambiguous-minus", 3, 4);
    assert_reject("2 +-1", "ambiguous-minus", 3, 4);
    assert_reject("--1", "ambiguous-minus", 1, 2);
    assert_reject("f_-1", "ambiguous-minus", 2, 3);
}

#[test]
fn bad_numbers() {
    assert_reject("2x", "bad-number", 1, 2);
    assert_reject("1.", "bad-number", 0, 2);
    assert_reject("1.2.3", "bad-number", 3, 4);
    assert_reject(".5", "unexpected-char", 0, 1);
    assert_reject("2_", "bad-number", 1, 2);
    assert_reject("2^r", "bad-number", 1, 2);
    assert_reject("99999999999999999999", "number-out-of-range", 0, 20);
    assert_reject("-99999999999999999999", "number-out-of-range", 0, 21);
}

#[test]
fn unexpected_and_non_ascii_characters() {
    assert_reject("1 # 2", "unexpected-char", 2, 3);
    assert_reject("a , b", "unexpected-char", 2, 3);
    assert_reject("x := 3", "unexpected-char", 2, 3);
    assert_reject("'a'", "unexpected-char", 0, 1);
    assert_reject("\u{3c1} 3", "non-ascii", 0, 2);
    assert_reject("x \u{2190} 3", "non-ascii", 2, 5);
    assert_reject("1\u{0}", "unexpected-char", 1, 2);
}

#[test]
fn errors_convert_to_diagnostics_with_spans() {
    let err = xetal_lex::lex("3-1").unwrap_err();
    let diag: Diagnostic = err.into();
    assert_eq!(diag.code, "ambiguous-minus");
    assert_eq!(diag.span, Some(Span::new(1, 2)));
    assert!(diag.message.contains("3 - 1"), "{}", diag.message);
}
