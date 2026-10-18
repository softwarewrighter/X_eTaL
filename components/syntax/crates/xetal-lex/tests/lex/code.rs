//! Code strings (ST5, docs/code-strings.md): `c"..."` with the usual
//! escapes and `c#"..."#` verbatim, both may span lines; the token is
//! `Code`, whose value is the same text a plain string of it would give.

use crate::common::{dump, kinds, reject};

#[test]
fn a_code_string_is_its_own_token() {
    assert_eq!(
        kinds(r#"c"it nn:d_ense W1""#),
        [r#"Code("it nn:d_ense W1")"#]
    );
    assert_eq!(kinds(r#"c"""#), [r#"Code("")"#]);
}

#[test]
fn its_escapes_are_the_usual_ones() {
    assert_eq!(
        kinds(r#"c"it c_at \"label: \" c_at x""#),
        [r#"Code("it c_at \"label: \" c_at x")"#]
    );
    assert_eq!(kinds(r#"c"a\\b\tc\n""#), [r#"Code("a\\b\tc\n")"#]);
}

#[test]
fn the_hash_form_is_verbatim() {
    assert_eq!(
        kinds(r###"c#"it c_at "label: ""#"###),
        [r#"Code("it c_at \"label: \"")"#]
    );
    assert_eq!(kinds(r###"c#"a\n"#"###), [r#"Code("a\\n")"#]);
    assert_eq!(kinds(r####"c##"a"#b"##"####), [r##"Code("a\"#b")"##]);
}

#[test]
fn either_form_spans_lines() {
    assert_eq!(
        dump("c\"it nn:d_ense W1\n  |> nn:r_elu\"\n1"),
        [
            "0..32 Code(\"it nn:d_ense W1\\n  |> nn:r_elu\")",
            "32..33 Newline",
            "33..34 Num(1)"
        ]
    );
    assert_eq!(kinds("c#\"a\nb\"#"), ["Code(\"a\\nb\")"]);
}

#[test]
fn it_stands_where_a_string_may() {
    assert_eq!(
        kinds(r#""it" nn:p_ipe< c"it nn:d_ense W1""#),
        [
            r#"Str("it")"#,
            "Func(nn:p_ipe<)",
            r#"Code("it nn:d_ense W1")"#
        ]
    );
    assert_eq!(
        kinds(r#"x := c"1 + 2""#),
        ["Var(x)", "Assign", r#"Code("1 + 2")"#]
    );
}

#[test]
fn c_alone_and_names_starting_with_c_are_still_names() {
    assert_eq!(kinds("c"), ["Var(c)"]);
    assert_eq!(kinds("c_at x"), ["Func(c_at)", "Var(x)"]);
    assert_eq!(kinds("count 1"), ["Var(count)", "Num(1)"]);
}

#[test]
fn rejections() {
    // Unterminated, either form.
    assert_eq!(reject(r#"c"abc"#).0, "bad-string");
    assert_eq!(reject("c\"abc\n").0, "bad-string");
    assert_eq!(reject(r##"c#"abc""##).0, "bad-string");
    // An escape a plain string refuses is refused here too.
    assert_eq!(reject(r#"c"a\qb""#).0, "bad-string");
    // Only `r` and `c` open a string: a longer name touching `"` is an
    // error, including the dropped raw-code form `cr"..."`.
    assert_eq!(reject(r#"xc"a""#).0, "bad-string");
    assert_eq!(reject(r#"cr"a""#).0, "bad-string");
}
