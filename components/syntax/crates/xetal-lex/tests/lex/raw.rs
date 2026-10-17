//! Raw strings (ST4): `r"..."` and `r#"..."#`, Rust's rules: verbatim,
//! no escapes, may span lines; the same `Str` token as `"..."`.

use crate::common::{dump, kinds, reject};
use xetal_base::Span;

#[test]
fn a_raw_string_is_verbatim() {
    assert_eq!(kinds(r#"r"a\b\n""#), [r#"Str("a\\b\\n")"#]);
    assert_eq!(kinds(r#"r"""#), [r#"Str("")"#]);
    assert_eq!(kinds("r\"\u{e9}\u{2192}\""), ["Str(\"\u{e9}\u{2192}\")"]);
}

#[test]
fn a_raw_string_spans_lines() {
    assert_eq!(
        dump("r\"a\n  b\"\n1"),
        ["0..8 Str(\"a\\n  b\")", "8..9 Newline", "9..10 Num(1)"]
    );
}

#[test]
fn hashes_let_it_hold_quotes() {
    assert_eq!(kinds(r###"r#"say "hi""#"###), [r#"Str("say \"hi\"")"#]);
    assert_eq!(kinds(r###"r##"a"#b"##"###), [r##"Str("a\"#b")"##]);
}

#[test]
fn it_stands_where_a_string_may() {
    assert_eq!(kinds(r#"x := r"a""#), ["Var(x)", "Assign", r#"Str("a")"#]);
    assert_eq!(
        kinds(r#"@ e:p_ipe< r"X |> it""#),
        ["Unit", "Func(e:p_ipe<)", r#"Str("X |> it")"#]
    );
    assert_eq!(dump(r#"r"ab" 1"#), [r#"0..5 Str("ab")"#, "6..7 Num(1)"]);
}

#[test]
fn r_alone_is_still_a_name() {
    assert_eq!(kinds("r"), ["Var(r)"]);
    assert_eq!(kinds("r # comment"), ["Var(r)"]);
    assert_eq!(kinds("r#x"), ["Var(r)"]);
    assert_eq!(kinds("r_ev 1"), ["Func(r_ev)", "Num(1)"]);
}

#[test]
fn unterminated_raw_strings_are_rejected() {
    assert_eq!(reject("r\"ab"), ("bad-string".into(), Span::new(0, 4)));
    assert_eq!(reject("r#\"ab\""), ("bad-string".into(), Span::new(0, 6)));
    assert_eq!(reject("r##\"a\"#"), ("bad-string".into(), Span::new(0, 7)));
}

#[test]
fn another_name_touching_a_quote_is_still_reserved() {
    assert_eq!(reject("ar\"x\""), ("bad-string".into(), Span::new(2, 3)));
    assert_eq!(reject("b\"x\""), ("bad-string".into(), Span::new(1, 2)));
}
