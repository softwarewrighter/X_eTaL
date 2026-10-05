//! Outcome a and Error (ER2): a handler's outcome carries the protected
//! body's type; Error is a nominal built-in type.

use xetal_base::Span;
use xetal_ty::{Classes, Type, Unifier};

fn span() -> Span {
    Span::new(0, 1)
}

fn outcome(t: Type) -> Type {
    Type::Outcome(Box::new(t))
}

#[test]
fn an_outcome_unifies_only_with_an_outcome_of_the_same_type() {
    let mut u = Unifier::default();
    let a = u.fresh();
    u.unify(&outcome(a.clone()), &outcome(Type::Char), span())
        .unwrap();
    assert_eq!(u.resolve(&a), Type::Char);
    let e = u
        .unify(&outcome(Type::Int), &outcome(Type::Char), span())
        .unwrap_err();
    assert_eq!(e.message, "expected Int, found Char");
    let e = u
        .unify(&Type::Int, &outcome(Type::Int), span())
        .unwrap_err();
    assert_eq!(e.message, "expected Int, found Outcome Int");
}

#[test]
fn outcomes_and_errors_print_and_are_in_no_class() {
    assert_eq!(outcome(Type::Char).to_string(), "Outcome Char");
    assert_eq!(
        outcome(Type::Box(Box::new(Type::Int))).to_string(),
        "Outcome (Box Int)"
    );
    assert_eq!(Type::Named("Error").to_string(), "Error");
    for class in ["Num", "Ord", "Truthy", "Eq"] {
        let mut u = Unifier::default();
        let v = u.fresh_in(Classes::named(class).unwrap());
        assert!(u.unify(&v, &outcome(Type::Int), span()).is_err(), "{class}");
        let v = u.fresh_in(Classes::named(class).unwrap());
        let err = u.unify(&v, &Type::Named("Error"), span()).is_err();
        assert_eq!(err, class != "Eq", "{class}");
    }
}
