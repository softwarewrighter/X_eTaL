//! `r_equireShape` as the lint reads it: a check it can prove fails is
//! `shape-mismatch`, one it can prove passes is `shape-check-proven`,
//! and after it the value has the checked shape, names and all.

fn found(src: &str, code: &str) -> Vec<String> {
    let program = xetal_core::lower(src).expect("lowers");
    xetal_shape::shapes(&program)
        .into_iter()
        .filter(|d| d.code == code)
        .map(|d| d.message)
        .collect()
}

const DATA: &str = "V := n_umbers []N_GET \"f\"\nN := t_ally V\n";

#[test]
fn a_check_that_fails_is_found() {
    let got = found("2 2 r_equireShape 1 2 3", "shape-mismatch");
    assert_eq!(got.len(), 1, "{got:?}");
    assert!(got[0].contains("3") && got[0].contains("2 2"), "{}", got[0]);
}

#[test]
fn a_checked_value_has_the_checked_shape() {
    let src = format!("{DATA}X := (N c_at 3) r_equireShape V\nX + 4 3 r_eshape 0");
    assert!(found(&src, "shape-mismatch").is_empty());
    let src = format!("{DATA}X := (N c_at 3) r_equireShape V\nX + 4 2 r_eshape 0");
    assert_eq!(found(&src, "shape-mismatch").len(), 1);
    let src = format!("{DATA}X := (2 c_at 3) r_equireShape V\nX + 3 2 r_eshape 0");
    assert_eq!(found(&src, "shape-mismatch").len(), 1);
}

#[test]
fn extents_named_alike_are_equal() {
    let src = format!("{DATA}X := (N c_at 3) r_equireShape V\n(N c_at 3) r_equireShape X");
    assert_eq!(found(&src, "shape-check-proven").len(), 1);
    let src = format!("{DATA}X := (N c_at 3) r_equireShape V\n(N c_at 4) r_equireShape X");
    assert_eq!(found(&src, "shape-mismatch").len(), 1);
}

#[test]
fn a_check_on_a_known_shape_is_proven() {
    let got = found("X := 2 3 r_eshape 0\n2 3 r_equireShape X", "shape-check-proven");
    assert_eq!(got.len(), 1);
    assert!(
        found(
            "V := n_umbers []N_GET \"f\"\n2 3 r_equireShape V",
            "shape-check-proven"
        )
        .is_empty()
    );
}

#[test]
fn extents_the_lint_cannot_see_say_nothing() {
    let src = format!("{DATA}V r_equireShape 1 2 3");
    assert!(found(&src, "shape-mismatch").is_empty());
    assert!(found("u:f_ := { n -> n r_equireShape 1 2 3 }", "shape-mismatch").is_empty());
}
