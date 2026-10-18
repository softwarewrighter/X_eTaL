//! `needless-raw-hashes`: a raw string with more `#` than it needs.

use crate::common::{codes, preserved};

const CODE: &str = "needless-raw-hashes";

#[test]
fn two_hashes_where_none_would_do() {
    // Plain would do too (the pedantic `needless-raw-string`).
    assert_eq!(
        codes(r###"s := r##"plain"##"###),
        [CODE, "needless-raw-string"]
    );
    assert_eq!(
        preserved(r###"s := r##"plain"##"###, CODE),
        r#"s := r"plain""#
    );
}

#[test]
fn a_quote_inside_needs_one_hash() {
    let src = r###"s := r##"a "quoted" word"##"###;
    assert_eq!(codes(src), [CODE]);
    assert_eq!(preserved(src, CODE), r##"s := r#"a "quoted" word"#"##);
}

#[test]
fn a_quote_and_hash_inside_needs_two() {
    let src = r####"s := r###"say "# here"###"####;
    assert_eq!(preserved(src, CODE), r###"s := r##"say "# here"##"###);
}

#[test]
fn the_fewest_hashes_already_is_quiet() {
    for src in [
        r#"s := r"C:\dir""#,
        r##"s := r#"a "quoted" word"#"##,
        r###"s := r##"say "# here"##"###,
        r#"s := "plain""#,
    ] {
        assert!(!codes(src).contains(&CODE.to_string()), "{src}");
    }
}

#[test]
fn a_multi_line_body_is_kept_as_written() {
    let src = "s := r##\"one\n\"two\\\"##\n";
    assert_eq!(preserved(src, CODE), "s := r#\"one\n\"two\\\"#\n");
}

#[test]
fn the_fix_rewrites_only_the_delimiters() {
    let found = xetal_strings::strings(r###"s := r##"x"##"###);
    let spans: Vec<_> = found[0]
        .fix()
        .iter()
        .map(|f| (f.span.start, f.span.end))
        .collect();
    assert_eq!(spans, [(5, 9), (10, 13)]);
}
