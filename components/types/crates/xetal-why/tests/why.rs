//! What a type mismatch says of each side (D163).

use xetal_base::Span;
use xetal_core::{Item, lower};
use xetal_ty::Type;
use xetal_why::{Bound, gives, said, signature};

/// The value of the last item of `src`, lowered.
fn last(src: &str) -> xetal_core::Expr {
    match lower(src).expect("lowers").items.pop() {
        Some(Item::Eval(e) | Item::Let { value: e, .. }) => e,
        other => panic!("not an expression: {other:?}"),
    }
}

#[test]
fn a_built_in_whose_result_is_one_type_says_so() {
    assert_eq!(
        gives(&last("7 / 2")).as_deref(),
        Some("`/` always gives a Float")
    );
    assert_eq!(gives(&last("1 + 2")), None, "+ gives what it is given");
    assert_eq!(signature("+"), Some("Num a => a -> a -> a"));
    assert_eq!(signature("n_o_such"), None);
}

#[test]
fn a_literal_says_what_it_is() {
    let none = |_: &str| None;
    assert_eq!(
        said(&last("2.5"), &Type::Float, &none)[0].1,
        "a Float literal"
    );
    assert_eq!(
        said(&last("3"), &Type::Int, &none)[0].1,
        "a whole-number literal, here an Int"
    );
}

#[test]
fn a_variable_says_where_it_was_bound() {
    let bound = |_: &str| Some(Bound::Value(Span::new(5, 10), Some("why".into())));
    let labels = said(&last("x"), &Type::Int, &bound);
    assert_eq!(labels[0].1, "`x` is an Int");
    assert_eq!(
        labels[1],
        (Span::new(5, 10), "`x` gets its type here: why".into())
    );
    let param = |_: &str| Some(Bound::Param);
    assert_eq!(
        said(&last("x"), &Type::Float, &param)[0].1,
        "`x`, the lambda's parameter, is a Float as it is used"
    );
}

#[test]
fn an_open_type_says_nothing() {
    let mut u = xetal_ty::Unifier::default();
    assert!(said(&last("x"), &u.fresh(), &|_: &str| None).is_empty());
}

#[test]
fn a_common_mismatch_gets_a_hint_and_others_none() {
    use xetal_base::Diagnostic;
    let help = |m: &str| {
        let d = xetal_why::hinted(Diagnostic::new("type-mismatch", m));
        d.notes.first().cloned().unwrap_or_default()
    };
    assert!(help("expected Int, found Float").contains("f_loat"));
    assert!(help("expected a number, found Box Int").contains("d_isclose"));
    assert!(help("expected Box Int, found Int").contains("e_nclose"));
    assert!(help("expected a number, found a -> a").contains("quote"));
    assert!(help("expected a -> b, found Float").contains("too many"));
    assert_eq!(help("expected Char, found Bool"), "");
    let other = xetal_why::hinted(Diagnostic::new("rebind", "expected Int, found Float"));
    assert!(other.notes.is_empty(), "only a type mismatch is hinted");
}
