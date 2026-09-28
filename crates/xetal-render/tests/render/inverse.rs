//! Decorated Unicode -> raw ASCII.

use crate::UL;
use xetal_base::Span;
use xetal_render::undecorate;

fn raw(text: &str) -> String {
    undecorate(text).unwrap_or_else(|e| panic!("{text:?}: {e:?}"))
}

#[test]
fn ascii_passes_through() {
    for s in [
        "x", "1 + 2", "_l _r @", "x^0.5", "u:x", "\"a_b\"", "x;\n{ }",
    ] {
        assert_eq!(raw(s), s);
    }
}

#[test]
fn an_underlined_letter_is_followed_by_underscore() {
    assert_eq!(raw(&format!("r{UL}ev")), "r_ev");
    assert_eq!(raw(&format!("self{UL}")), "self_");
    assert_eq!(raw(&format!("u:s{UL}quare 7")), "u:s_quare 7");
}

#[test]
fn subscripts_and_superscripts() {
    assert_eq!(raw(&format!("o{UL}-\u{2081}\u{2082}")), "o_-_12");
    assert_eq!(raw(&format!("r{UL}\u{2082}")), "r__2");
    assert_eq!(raw("x\u{b2}"), "x^2");
    assert_eq!(raw("x\u{207b}\u{b9}"), "x^-1");
    assert_eq!(raw("2\u{b9}\u{2070}"), "2^10");
}

#[test]
fn rejects_non_decoration_unicode_with_span() {
    let err = undecorate("x \u{2190} 3").unwrap_err();
    assert_eq!(
        (err.code.as_str(), err.span),
        ("not-decorated", Some(Span::new(2, 5)))
    );
}

#[test]
fn rejects_an_underline_not_under_a_letter() {
    let err = undecorate(&format!("{UL}x")).unwrap_err();
    assert_eq!(
        (err.code.as_str(), err.span),
        ("bad-underline", Some(Span::new(0, 2)))
    );
    let err = undecorate(&format!("+{UL}")).unwrap_err();
    assert_eq!(
        (err.code.as_str(), err.span),
        ("bad-underline", Some(Span::new(0, 3)))
    );
}
