//! `redundant-lambda`: a lambda that only passes its arguments on to a
//! named function (`{ x -> n_eg x }` is `'n_eg`), or returns its
//! argument (`{ x -> x }` is `'i_d`).

fn found(src: &str) -> Vec<String> {
    let program = xetal_core::lower(src).expect("lowers");
    xetal_lint::lints(&program)
        .into_iter()
        .filter(|d| d.code == "redundant-lambda")
        .map(|d| d.message)
        .collect()
}

#[test]
fn a_lambda_calling_a_built_in_is_its_name() {
    let got = found("u:f_ := { v -> '{ x -> n_eg x } e_ach v }");
    assert_eq!(got.len(), 1);
    assert!(got[0].contains("write `'n_eg`"), "{}", got[0]);
}

#[test]
fn a_two_parameter_lambda_calling_a_dyad_is_its_name() {
    let got = found("u:f_ := { v -> '{ a b -> a + b } r_/ v }");
    assert_eq!(got.len(), 1);
    assert!(got[0].contains("write `'+`"), "{}", got[0]);
}

#[test]
fn a_lambda_calling_a_user_function_is_its_name() {
    let src = "u:s_q := { _r * _r }\nu:f_ := { v -> '{ x -> u:s_q x } e_ach v }";
    let got = found(src);
    assert_eq!(got.len(), 1);
    assert!(got[0].contains("write `'u:s_q`"), "{}", got[0]);
}

#[test]
fn a_lambda_returning_its_argument_is_the_identity() {
    let got = found("u:f_ := { v -> '{ x -> x } e_ach v }");
    assert_eq!(got.len(), 1);
    assert!(got[0].contains("write `'i_d`"), "{}", got[0]);
    assert_eq!(found("u:f_ := { v -> '{ _r } e_ach v }").len(), 1);
}

#[test]
fn lambdas_doing_more_than_passing_on_pass() {
    for src in [
        "u:f_ := { v -> '{ x -> v + x } e_ach v }",
        "u:f_ := { v -> '{ a b -> b + a } r_/ v }",
        "u:f_ := { v -> '{ x -> n_eg n_eg x } e_ach v }",
        "u:f_ := { v -> '{ a b -> n_eg b } r_/ v }",
        "u:f_ := { g_ v -> '{ x -> g_ x } e_ach v }",
    ] {
        assert!(found(src).is_empty(), "{src}: {:?}", found(src));
    }
}

#[test]
fn a_definition_by_a_lambda_passes() {
    assert!(found("u:n_ot := { x -> n_eg x }\nu:I_ := { x -> x }").is_empty());
    assert!(found("u:f_ := { v -> g_ := { x -> n_eg x }\ng_ v }").is_empty());
}
