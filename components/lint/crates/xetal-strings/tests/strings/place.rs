//! Where findings are: byte offsets in the text, located by line and
//! column (characters, from 1) as `xetal lint` reports them; a text
//! that does not lex has none.

use crate::common::{codes, fixed, values};

#[test]
fn line_and_column_count_characters_from_one() {
    let text = "x := 1\ncaf\u{e9} := r##\"y\"##";
    assert_eq!(xetal_strings::line_col(text, 0), (1, 1));
    let at = text.find("r##").unwrap();
    assert_eq!(xetal_strings::line_col(text, at), (2, 9));
    assert_eq!(xetal_strings::line_col(text, text.len()), (2, 17));
}

#[test]
fn an_offset_past_the_text_or_inside_a_character_does_not_panic() {
    let text = "\u{e9}";
    assert_eq!(xetal_strings::line_col(text, 1), (1, 1));
    assert_eq!(xetal_strings::line_col(text, 99), (1, 2));
}

#[test]
fn a_text_that_does_not_lex_has_no_findings() {
    assert!(codes("s := r##\"never closed\"#").is_empty());
    assert!(codes("s := \"bad \\q escape\"").is_empty());
}

#[test]
fn findings_come_in_text_order() {
    let text = "a := \"\\\\x\\\\\"\nb := r##\"a \"y\"##\nc := r\"z\"";
    assert_eq!(
        codes(text),
        [
            "escapes-could-be-raw",
            "needless-raw-hashes",
            "needless-raw-string"
        ]
    );
    assert_eq!(values(&fixed(text, "needless-raw-hashes")), values(text));
}
