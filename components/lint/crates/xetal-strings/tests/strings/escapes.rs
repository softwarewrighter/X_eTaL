//! `escapes-could-be-raw`: a plain string with several `\"` or `\\`.

use crate::common::{codes, preserved};

const CODE: &str = "escapes-could-be-raw";

#[test]
fn two_backslashes_become_a_raw_string_without_hashes() {
    let src = r#"p := "C:\\dir\\file""#;
    assert_eq!(codes(src), [CODE]);
    assert_eq!(preserved(src, CODE), r#"p := r"C:\dir\file""#);
}

#[test]
fn quotes_inside_need_one_hash() {
    let src = r#"c := "x := \"3 + 4\"""#;
    assert_eq!(codes(src), [CODE]);
    assert_eq!(preserved(src, CODE), r##"c := r#"x := "3 + 4""#"##);
}

#[test]
fn a_quote_then_hash_needs_two_hashes() {
    let src = r##"c := "say \"# here\" \\ there""##;
    assert_eq!(
        preserved(src, CODE),
        r###"c := r##"say "# here" \ there"##"###
    );
}

#[test]
fn a_macro_argument_keeps_its_value() {
    let src = r#""\"a\" \\ \"b\"" x:w_ith< "5""#;
    assert_eq!(codes(src), [CODE]);
    assert_eq!(preserved(src, CODE), r##"r#""a" \ "b""# x:w_ith< "5""##);
}

#[test]
fn one_escape_is_not_enough() {
    assert!(codes(r#"s := "say \"hi""#).is_empty());
    assert!(codes(r#"s := "C:\\""#).is_empty());
}

#[test]
fn a_newline_or_tab_escape_keeps_it_plain() {
    assert!(codes(r#"s := "\"a\"\n""#).is_empty());
    assert!(codes(r#"s := "\\a\\\tb""#).is_empty());
}

#[test]
fn raw_strings_are_not_reported() {
    assert!(codes(r#"s := r"C:\\dir\\""#).is_empty());
}
