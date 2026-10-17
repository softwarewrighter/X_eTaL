//! The pipeline examples' hand-written equivalents
//! (docs/spatial-poc/pipes-oracle.xtl) with the checks a pipeline's
//! shape annotations lower to: the lint proves every one, and finds a
//! weight of the wrong shape before the program runs.

const ORACLE: &str = include_str!("../../../../../docs/spatial-poc/pipes-oracle.xtl");

/// The annotations ` : P [2 2]`, ` [2 1]`, ` : c [4]` as checks.
const CHECKS: &str = "2 2 s_hapeIs P\n\
2 1 s_hapeIs (0 m_ax X '+ '* i_nner W1) '+ '* i_nner W2\n\
4 s_hapeIs c\n\
3 s_hapeIs e\n";

fn codes(src: &str) -> Vec<String> {
    let program = xetal_core::lower(src).expect("lowers");
    xetal_shape::shapes(&program)
        .into_iter()
        .map(|d| d.code)
        .collect()
}

#[test]
fn the_oracle_has_no_mismatch() {
    assert!(codes(ORACLE).is_empty(), "{:?}", codes(ORACLE));
}

#[test]
fn every_annotation_is_proven() {
    let got = codes(&format!("{ORACLE}{CHECKS}"));
    assert_eq!(got, ["shape-check-proven"; 4]);
}

#[test]
fn a_weight_of_the_wrong_shape_is_found() {
    let broken = ORACLE.replace("W := 3 2 r_eshape", "W := 2 2 r_eshape");
    assert!(codes(&broken).contains(&"shape-mismatch".to_string()));
}
