//! Tuple types (TU2, TU7): printed as written, unified part by part,
//! in the Match class (what m_atch compares) and not in Eq.

use xetal_base::Span;
use xetal_ty::{Classes, Type, Unifier};

fn span() -> Span {
    Span::new(0, 1)
}

#[test]
fn a_tuple_type_prints_as_written() {
    let t = Type::Tuple(vec![Type::Float, Type::Int]);
    assert_eq!(t.to_string(), "(Float, Int)");
    let boxed = Type::Box(Box::new(t));
    assert_eq!(boxed.to_string(), "Box (Float, Int)");
}

#[test]
fn tuples_unify_part_by_part_and_only_at_one_size() {
    let mut u = Unifier::default();
    let a = u.fresh();
    let pair = Type::Tuple(vec![a.clone(), Type::Int]);
    u.unify(&pair, &Type::Tuple(vec![Type::Char, Type::Int]), span())
        .unwrap();
    assert_eq!(u.resolve(&a), Type::Char);
    let triple = Type::Tuple(vec![Type::Int, Type::Int, Type::Int]);
    let e = u.unify(&pair, &triple, span()).unwrap_err();
    assert_eq!(e.message, "expected a tuple of 2, found a tuple of 3");
}

#[test]
fn match_admits_tuples_and_eq_does_not() {
    let mut u = Unifier::default();
    let pair = Type::Tuple(vec![Type::Int, Type::Float]);
    let m = u.fresh_in(Classes::named("Match").unwrap());
    assert!(u.unify(&m, &pair, span()).is_ok());
    let eq = u.fresh_in(Classes::named("Eq").unwrap());
    let e = u.unify(&eq, &pair, span()).unwrap_err();
    assert_eq!(e.message, "expected an array, found a tuple");
    // A tuple of functions is not something m_atch can compare.
    let f = Type::Fn(Box::new(Type::Int), Box::new(Type::Int));
    let m = u.fresh_in(Classes::named("Match").unwrap());
    assert!(
        u.unify(&m, &Type::Tuple(vec![f, Type::Int]), span())
            .is_err()
    );
}

#[test]
fn an_array_variable_rejects_a_tuple_and_prints_bare() {
    let arr = Classes::named("Arr").unwrap();
    let mut u = Unifier::default();
    let a = u.fresh_in(arr);
    let e = u
        .unify(&a, &Type::Tuple(vec![Type::Int, Type::Int]), span())
        .unwrap_err();
    assert_eq!(e.message, "expected an array, found a tuple");
    // Arr is never named; no class at all is `Any` (TU9).
    assert_eq!(arr.names().collect::<Vec<_>>(), Vec::<&str>::new());
    assert_eq!(Classes::default().names().collect::<Vec<_>>(), ["Any"]);
    assert_eq!(Classes::named("Any"), Some(Classes::default()));
}

#[test]
fn a_box_may_hold_a_tuple_where_its_variable_is_an_array() {
    let mut u = Unifier::default();
    let a = u.fresh_in(Classes::named("Arr").unwrap());
    let boxed = Type::Box(Box::new(Type::Tuple(vec![Type::Int, Type::Float])));
    assert!(u.unify(&a, &boxed, span()).is_ok());
}

#[test]
fn the_array_class_never_hides_eq() {
    let both = Classes::named("Eq")
        .unwrap()
        .union(Classes::named("Arr").unwrap());
    assert_eq!(both.names().collect::<Vec<_>>(), ["Eq"]);
}
