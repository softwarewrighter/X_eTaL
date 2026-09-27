//! Decorated Unicode -> raw ASCII.

use crate::{UL, ul};
use xetal_base::Span;
use xetal_render::undecorate;

fn raw(text: &str) -> String {
    undecorate(text).unwrap_or_else(|e| panic!("{text:?}: {e:?}"))
}

#[test]
fn ascii_passes_through() {
    for s in ["r", "1 + 2", "_l _r @", "+^q_2", "f^R_@", "x;\n{ }"] {
        assert_eq!(raw(s), s);
    }
}

#[test]
fn underline_runs_become_trailing_underscores() {
    assert_eq!(raw(&ul("r")), "r_");
    assert_eq!(raw(&format!("{} 7", ul("square"))), "square_ 7");
    assert_eq!(raw(&format!("m.{}", ul("f"))), "m.f_");
    assert_eq!(raw(&format!("{}@", ul("now"))), "now_@");
    assert_eq!(raw(&format!("{} @", ul("now"))), "now_ @");
    assert_eq!(raw(&format!("{}@", ul("+"))), "+_@");
}

#[test]
fn subscripts_and_superscripts() {
    assert_eq!(raw(&format!("{}\u{2081}\u{2082}", ul("t"))), "t_12");
    assert_eq!(raw("+\u{2081}"), "+_1");
    assert_eq!(raw("+\u{2b3}"), "+^r");
    assert_eq!(raw("+\u{2e2}\u{2082}"), "+^s_2");
    assert_eq!(raw("f\u{1d49}@"), "f^e_@");
    assert_eq!(
        raw("+\u{2b3}\u{1d49}\u{1d48}\u{1d58}\u{1d9c}\u{1d49}"),
        "+^reduce"
    );
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
fn rejects_an_underline_on_nothing() {
    let err = undecorate(&format!("{UL}x")).unwrap_err();
    assert_eq!(
        (err.code.as_str(), err.span),
        ("bad-underline", Some(Span::new(0, 2)))
    );
    let err = undecorate(&format!("a {UL}")).unwrap_err();
    assert_eq!(
        (err.code.as_str(), err.span),
        ("bad-underline", Some(Span::new(1, 4)))
    );
}

#[test]
fn touching_underline_runs_split_at_a_stem_class_change() {
    assert_eq!(raw(&format!("{}{}@", ul("r"), ul("+"))), "r_+_@");
    assert_eq!(raw(&format!("{}{}@", ul("+"), ul("-"))), "+_-_@");
}
