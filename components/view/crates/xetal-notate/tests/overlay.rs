//! A notation call drawn or marked on a view (MC32).

use xetal_base::Span;
use xetal_notate::{Notation, drawn, marked};
use xetal_view::{Class, html, view};

const SRC: &str = "y := \"L\" e:e_q< \"a + b\"";

fn notation() -> Notation {
    Notation {
        left: Some(Span::new(5, 8)),
        right: Some(Span::new(16, 23)),
        latex: "L = \\sum a + b".into(),
    }
}

#[test]
fn drawn_puts_one_math_box_after_the_macro_name() {
    let out = drawn(view(SRC), &[notation()]);
    let boxes: Vec<&_> = out.iter().filter(|s| s.class == Class::Notation).collect();
    assert_eq!(boxes.len(), 2);
    assert_eq!(boxes[0].text, "");
    assert_eq!(boxes[1].text, "L = \\sum a + b");
    // The macro name and the binding stay as code, in order.
    let texts: String = out.iter().map(|s| s.text.as_str()).collect();
    assert_eq!(
        texts,
        "y \u{2190}  e:e_q< L = \\sum a + b".replace(
            "e:e_q<",
            &out.iter().find(|s| s.class == Class::Macro).unwrap().text
        )
    );
    // Spans are kept, so the map from raw to drawn still holds.
    assert!(out.iter().any(|s| s.raw == Span::new(5, 8)));
    assert!(out.iter().any(|s| s.raw == Span::new(16, 23)));
}

#[test]
fn marked_keeps_the_typed_text() {
    let out = marked(view(SRC), &[notation()]);
    let marked: Vec<&_> = out.iter().filter(|s| s.class == Class::Notation).collect();
    assert_eq!(marked.len(), 2);
    assert_eq!(marked[0].text, "\"L\"");
    assert_eq!(marked[1].text, "\"a + b\"");
}

#[test]
fn a_notation_is_a_span_of_its_own_class_in_html() {
    let out = html(&drawn(view(SRC), &[notation()]));
    assert!(
        out.contains("<span class=\"c-notation\">L = \\sum a + b</span>"),
        "{out}"
    );
}

#[test]
fn a_one_sided_notation_is_drawn_on_that_side() {
    let src = "@ x:s_how< \"a\"";
    let n = Notation {
        left: None,
        right: Some(Span::new(11, 14)),
        latex: "a".into(),
    };
    let out = drawn(view(src), &[n]);
    assert_eq!(out.iter().filter(|s| s.class == Class::Notation).count(), 1);
}
