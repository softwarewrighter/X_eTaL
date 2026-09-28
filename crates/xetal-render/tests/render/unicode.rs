//! Raw ASCII -> decorated Unicode (interim rules; step 2 adds namespace
//! superscripts and ligatures).

use crate::UL;
use xetal_render::decorate;

fn dec(src: &str) -> String {
    decorate(src).unwrap_or_else(|e| panic!("{src:?}: {e:?}"))
}

#[test]
fn variables_symbols_and_punctuation_are_unchanged() {
    for src in [
        "x", "board2", "count!", "m:pi", "_l _r", "@", "{ }", "( )", "[ ]", "-1 2.5", "+ != <=",
        ":= -> ?", "\"a_b\"",
    ] {
        assert_eq!(dec(src), src);
    }
}

#[test]
fn the_underlined_letter_is_underlined() {
    assert_eq!(dec("r_ev"), format!("r{UL}ev"));
    assert_eq!(dec("self_"), format!("self{UL}"));
    assert_eq!(dec("u:s_quare 7"), format!("u:s{UL}quare 7"));
    assert_eq!(dec("r_/ o_-"), format!("r{UL}/ o{UL}-"));
}

#[test]
fn axis_subscripts_become_subscript_digits() {
    assert_eq!(dec("o_-_12"), format!("o{UL}-\u{2081}\u{2082}"));
    assert_eq!(dec("r__2"), format!("r{UL}\u{2082}"));
}

#[test]
fn literal_exponents_become_superscripts() {
    assert_eq!(dec("x^2"), "x\u{b2}");
    assert_eq!(dec("x^-1"), "x\u{207b}\u{b9}");
    assert_eq!(dec("2^10"), "2\u{b9}\u{2070}");
    assert_eq!(dec("x^0.5"), "x^0.5"); // no superscript decimal point: shown raw
}

#[test]
fn whitespace_and_comments_are_preserved() {
    assert_eq!(
        dec("a  \t b # r_ev\n c_"),
        format!("a  \t b # r_ev\n c{UL}")
    );
}

#[test]
fn life_line() {
    let src = "u:l_ife := { ('+ r_/_12 -1 0 1 o_-_12 _r) { (_l = 3) + _r * _l = 4 } _r }";
    let want = format!(
        "u:l{UL}ife := {{ ('+ r{UL}/\u{2081}\u{2082} -1 0 1 o{UL}-\u{2081}\u{2082} _r) {{ (_l = 3) + _r * _l = 4 }} _r }}"
    );
    assert_eq!(dec(src), want);
}

#[test]
fn lex_errors_are_reported() {
    assert_eq!(decorate("3-1").unwrap_err().code, "ambiguous-minus");
}
