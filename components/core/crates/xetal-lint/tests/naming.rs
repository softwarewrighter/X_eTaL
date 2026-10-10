//! The style-guide naming lints (lang-choices section 16) over Core.

fn codes(src: &str) -> Vec<String> {
    let program = xetal_core::lower(src).expect("lowers");
    xetal_lint::lints(&program)
        .into_iter()
        .map(|d| d.code)
        .collect()
}

#[test]
fn upper_camel_function_is_function_name_case() {
    assert_eq!(codes("u:N_extGen := { _r + 1 }"), ["function-name-case"]);
}

#[test]
fn a_function_parameter_is_checked_too() {
    assert_eq!(
        codes("u:a_pply := { F_oo x -> F_oo x }"),
        ["function-name-case"]
    );
}

#[test]
fn combinators_and_lower_camel_functions_pass() {
    assert!(
        codes("u:K_ := { x y -> x }\nu:n_extGen := { _r + 1 }\nu:e_mpty? := { _r = 0 }").is_empty()
    );
}

#[test]
fn upper_camel_mutable_is_mutable_constant() {
    assert_eq!(codes("Count! := 0"), ["mutable-constant"]);
    assert!(codes("count! := 0\nMaxSize := 9").is_empty());
}

#[test]
fn all_capitals_is_upper_case_name_but_one_capital_is_not() {
    assert_eq!(codes("MAXSIZE := 9"), ["upper-case-name"]);
    assert!(codes("N := 9\nM2 := 3").is_empty());
}

#[test]
fn local_bindings_are_checked() {
    assert_eq!(codes("{ x -> AB := x + 1\nAB } 3"), ["upper-case-name"]);
}

#[test]
fn the_message_suggests_the_lower_camel_name() {
    let program = xetal_core::lower("u:N_extGen := { _r + 1 }").expect("lowers");
    let d = &xetal_lint::lints(&program)[0];
    assert!(d.message.contains("write `n_extGen`"), "{}", d.message);
}
