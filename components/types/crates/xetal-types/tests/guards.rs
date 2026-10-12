//! `guard_types`: the type each guard's condition is inferred to have,
//! for the typed lints (Saga 40 step 6).

use xetal_types::{Type, guard_types};

fn guards(src: &str) -> Vec<(String, Type)> {
    let program = xetal_core::lower(src).expect("lowers");
    let found = guard_types(&program).expect("type-checks");
    found
        .into_iter()
        .map(|(span, ty)| (src[span.start..span.end].to_string(), ty))
        .collect()
}

#[test]
fn a_comparison_is_a_truth_and_a_count_an_int() {
    let src = "u:f_ := { x -> x > 2 ? 1\n0 }\nu:g_ := { x -> t_ally x ? 1\n0 }";
    let got = guards(src);
    assert_eq!(got.len(), 2, "{got:?}");
    // A comparison is Truthy: a variable that defaults to Bool (T5).
    assert_eq!(got[0].0, "x > 2");
    assert!(matches!(got[0].1, Type::Var(_)), "{got:?}");
    assert_eq!(got[1], ("t_ally x".to_string(), Type::Int));
}

#[test]
fn a_condition_left_polymorphic_stays_a_variable() {
    let got = guards("u:f_ := { x -> x ? 1\n0 }");
    assert!(matches!(got[0].1, Type::Var(_)), "{got:?}");
}

#[test]
fn a_program_that_does_not_type_check_is_an_error() {
    let program = xetal_core::lower("1 + \"a\"").expect("lowers");
    assert!(guard_types(&program).is_err());
}
