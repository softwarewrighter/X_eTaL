//! Type foundations: unification, the Num constraint, schemes, display.

use xetal_base::Span;
use xetal_types::{Type, Unifier};

fn span() -> Span {
    Span::new(0, 1)
}

fn fun(a: Type, b: Type) -> Type {
    Type::Fn(Box::new(a), Box::new(b))
}

#[test]
fn equal_types_unify_and_different_ones_do_not() {
    let mut u = Unifier::default();
    assert!(u.unify(&Type::Int, &Type::Int, span()).is_ok());
    let e = u.unify(&Type::Int, &Type::Bool, span()).unwrap_err();
    assert_eq!(e.code, "type-mismatch");
    assert_eq!(e.message, "expected Int, found Bool");
    assert_eq!(e.span, Some(span()));
}

#[test]
fn variables_are_solved() {
    let mut u = Unifier::default();
    let a = u.fresh();
    let b = u.fresh();
    u.unify(
        &fun(a.clone(), b.clone()),
        &fun(Type::Int, Type::Float),
        span(),
    )
    .unwrap();
    assert_eq!(u.resolve(&a), Type::Int);
    assert_eq!(u.resolve(&fun(a, b)).to_string(), "Int -> Float");
}

#[test]
fn occurs_check() {
    let mut u = Unifier::default();
    let a = u.fresh();
    let e = u.unify(&a, &fun(a.clone(), Type::Int), span()).unwrap_err();
    assert_eq!(e.code, "infinite-type");
}

#[test]
fn num_constraint() {
    let mut u = Unifier::default();
    let n = u.fresh_num();
    assert!(u.unify(&n, &Type::Float, span()).is_ok());
    let m = u.fresh_num();
    let e = u.unify(&m, &Type::Unit, span()).unwrap_err();
    assert_eq!(e.message, "expected a number, found Unit");
    // the constraint moves to a variable it is unified with
    let p = u.fresh_num();
    let q = u.fresh();
    u.unify(&p, &q, span()).unwrap();
    assert!(u.unify(&q, &Type::Unit, span()).is_err());
}

#[test]
fn generalize_and_instantiate() {
    let mut u = Unifier::default();
    let a = u.fresh();
    let identity = fun(a.clone(), a);
    let scheme = u.generalize(&identity, &[]);
    assert_eq!(scheme.to_string(), "a -> a");
    let one = u.instantiate(&scheme);
    let two = u.instantiate(&scheme);
    u.unify(&one, &fun(Type::Int, Type::Int), span()).unwrap();
    u.unify(&two, &fun(Type::Bool, Type::Bool), span()).unwrap();
}

#[test]
fn free_variables_in_the_environment_stay_monomorphic() {
    let mut u = Unifier::default();
    let a = u.fresh();
    let scheme = u.generalize(&fun(a.clone(), a.clone()), std::slice::from_ref(&a));
    let inst = u.instantiate(&scheme);
    u.unify(&inst, &fun(Type::Int, Type::Int), span()).unwrap();
    assert_eq!(u.resolve(&a), Type::Int);
}

#[test]
fn display() {
    let mut u = Unifier::default();
    let (a, b) = (u.fresh(), u.fresh());
    let k = fun(a.clone(), fun(b, a));
    assert_eq!(u.generalize(&k, &[]).to_string(), "a -> b -> a");
    let (f, x) = (u.fresh(), u.fresh());
    let apply = fun(fun(x.clone(), f.clone()), fun(x, f));
    assert_eq!(u.generalize(&apply, &[]).to_string(), "(a -> b) -> a -> b");
    let n = u.fresh_num();
    let add = fun(n.clone(), fun(n.clone(), n));
    assert_eq!(u.generalize(&add, &[]).to_string(), "Num a => a -> a -> a");
    assert_eq!(Type::Array(Box::new(Type::Int)).to_string(), "Array Int");
    assert_eq!(fun(Type::Unit, Type::Int).to_string(), "Unit -> Int");
}

mod props {
    use proptest::prelude::*;
    use xetal_base::Span;
    use xetal_types::{Type, TypeVar, Unifier};

    fn arb_type() -> impl Strategy<Value = Type> {
        let leaf = prop_oneof![
            Just(Type::Unit),
            Just(Type::Bool),
            Just(Type::Int),
            Just(Type::Float),
            (1u32..5).prop_map(|v| Type::Var(TypeVar(v))),
        ];
        leaf.prop_recursive(4, 16, 2, |inner| {
            prop_oneof![
                (inner.clone(), inner.clone())
                    .prop_map(|(a, b)| Type::Fn(Box::new(a), Box::new(b))),
                inner.prop_map(|t| Type::Array(Box::new(t))),
            ]
        })
    }

    proptest! {
        #[test]
        fn a_type_unifies_with_itself(t in arb_type()) {
            let mut u = Unifier::default();
            prop_assert!(u.unify(&t, &t, Span::default()).is_ok());
        }

        #[test]
        fn unification_never_panics_and_resolve_is_idempotent(a in arb_type(), b in arb_type()) {
            let mut u = Unifier::default();
            let _ = u.unify(&a, &b, Span::default());
            let once = u.resolve(&a);
            prop_assert_eq!(u.resolve(&once), once);
        }
    }
}
