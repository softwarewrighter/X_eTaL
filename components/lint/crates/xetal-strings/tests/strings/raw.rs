//! `needless-raw-string`: a raw string a plain one spells the same.

use crate::common::{codes, preserved};

const CODE: &str = "needless-raw-string";

#[test]
fn a_raw_string_with_nothing_to_escape_could_be_plain() {
    assert_eq!(codes(r#"s := r"plain text""#), [CODE]);
    assert_eq!(
        preserved(r#"s := r"plain text""#, CODE),
        r#"s := "plain text""#
    );
}

#[test]
fn hashes_go_too() {
    let src = r##"s := r#"plain"#"##;
    assert!(codes(src).contains(&CODE.to_string()));
    assert_eq!(preserved(src, CODE), r#"s := "plain""#);
}

#[test]
fn an_empty_raw_string_is_the_empty_string() {
    assert_eq!(preserved(r#"s := r"""#, CODE), r#"s := """#);
}

#[test]
fn unicode_and_hashes_inside_are_kept() {
    let src = "s := r\"caf\u{e9} # not a comment\"";
    assert_eq!(preserved(src, CODE), "s := \"caf\u{e9} # not a comment\"");
}

#[test]
fn a_backslash_quote_or_newline_needs_the_raw_form() {
    for src in [
        r#"s := r"C:\dir""#,
        r##"s := r#"a "word""#"##,
        "s := r\"one\ntwo\"",
    ] {
        assert!(codes(src).is_empty(), "{src}: {:?}", codes(src));
    }
}

#[test]
fn plain_strings_are_not_reported() {
    assert!(codes(r#"s := "plain text""#).is_empty());
}
