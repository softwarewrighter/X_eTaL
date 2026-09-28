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
        "x", "1 + 2", "_l _r @", "x^0.5", "q:x", "\"a_b\"", "x;\n{ }", "x # a; b",
    ] {
        assert_eq!(raw(s), s);
    }
}

#[test]
fn underlines_namespaces_subscripts_and_exponents() {
    assert_eq!(raw(&format!("r{UL}ev")), "r_ev");
    assert_eq!(raw(&format!("\u{1d58}s{UL}quare 7")), "u:s_quare 7");
    assert_eq!(raw("\u{1d50}pi"), "m:pi");
    assert_eq!(raw(&format!("o{UL}-\u{2081}\u{2082}")), "o_-_12");
    assert_eq!(raw(&format!("r{UL}\u{2082}")), "r__2");
    assert_eq!(raw("x\u{207b}\u{b9}"), "x^-1");
}

#[test]
fn single_glyphs_become_their_ascii_tokens() {
    assert_eq!(raw("x \u{2190} 3"), "x := 3");
    assert_eq!(raw("{ x \u{2192} x }"), "{ x -> x }");
    assert_eq!(raw("a\u{22c4} b \u{235d} note"), "a; b # note");
    assert_eq!(
        raw(
            "a \u{2212} b \u{d7} c \u{f7} d \u{2260} e \u{2264} f \u{2265} g \u{2227} h \u{2228} i"
        ),
        "a - b * c / d != e <= f >= g & h | i"
    );
}

#[test]
fn rejects_non_decoration_unicode_with_span() {
    let err = undecorate("x \u{263a} 3").unwrap_err();
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

#[test]
fn a_namespace_superscript_must_precede_a_name() {
    let err = undecorate("\u{1d58} x").unwrap_err();
    assert_eq!(err.code, "bad-namespace");
}
