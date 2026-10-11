//! `missing-bang` and `needless-bang`: a function's name and the
//! effects it calls directly (lang-choices section 16: `!` marks a
//! function with a side effect).

fn found(src: &str, code: &str) -> Vec<String> {
    let program = xetal_core::lower(src).expect("lowers");
    xetal_lint::lints(&program)
        .into_iter()
        .filter(|d| d.code == code)
        .map(|d| d.message)
        .collect()
}

#[test]
fn a_function_calling_an_effect_needs_a_bang() {
    let got = found("u:s_ay := { v -> p_rint! v }", "missing-bang");
    assert_eq!(got.len(), 1);
    assert!(
        got[0].contains("`p_rint!`") && got[0].contains("write `u:s_ay!`"),
        "{}",
        got[0]
    );
}

#[test]
fn an_effect_inside_an_operand_or_a_later_parameter_counts() {
    assert_eq!(
        found("u:s_ay := { v -> '{ p_rint! _r } e_ach v }", "missing-bang").len(),
        1
    );
    assert_eq!(
        found("u:r_olls := { n k -> r_oll! n + k }", "missing-bang").len(),
        1
    );
}

#[test]
fn calling_a_users_bang_function_counts() {
    let src = "u:n_ow! := { @ -> r_oll! 6 }\nu:l_ater := { @ -> u:n_ow! @ }";
    let got = found(src, "missing-bang");
    assert_eq!(got.len(), 1);
    assert!(got[0].contains("`u:n_ow!`"), "{}", got[0]);
}

#[test]
fn a_bang_function_passed_in_does_not_count() {
    assert!(found("u:t_wice := { f_! -> f_! @\nf_! @ }", "missing-bang").is_empty());
}

#[test]
fn pure_and_marked_functions_pass() {
    assert!(
        found(
            "u:s_ay! := { v -> p_rint! v }\nu:d_ouble := { _r * 2 }",
            "missing-bang"
        )
        .is_empty()
    );
}

#[test]
fn a_bang_function_calling_no_effect_is_needless() {
    let got = found("u:d_ouble! := { _r * 2 }", "needless-bang");
    assert_eq!(got.len(), 1);
    assert!(got[0].contains("write `u:d_ouble`"), "{}", got[0]);
    // A passed-in effect, a mutable variable or a system function may be
    // the effect: not needless.
    let src = "u:t_wice! := { f_! -> f_! @ }\nu:s_how! := { v -> []S_HOW v }";
    assert!(found(src, "needless-bang").is_empty());
}
