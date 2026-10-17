//! `shape-mismatch` on item-by-item functions and the inner product:
//! shapes the lint can prove differ, from literals, `r_eshape` and the
//! names bound to them; silence where data decides.

fn found(src: &str) -> Vec<String> {
    let program = xetal_core::lower(src).expect("lowers");
    xetal_shape::shapes(&program)
        .into_iter()
        .filter(|d| d.code == "shape-mismatch")
        .map(|d| d.message)
        .collect()
}

#[test]
fn shapes_that_differ_are_found() {
    let got = found("(2 3 r_eshape 0) + 3 2 r_eshape 1");
    assert_eq!(got.len(), 1, "{got:?}");
    assert!(
        got[0].contains("2 3") && got[0].contains("3 2"),
        "{}",
        got[0]
    );
}

#[test]
fn names_carry_their_shapes() {
    assert_eq!(found("X := 2 3 r_eshape 0\nX * 3 2 r_eshape 1").len(), 1);
    assert_eq!(found("v := 1 2 3\nv - 1 2").len(), 1);
}

#[test]
fn ranks_that_differ_are_found() {
    assert_eq!(found("1 2 + 2 2 r_eshape 0").len(), 1);
    assert_eq!(found("(1 r_eshape 5) + 1 2 3").len(), 1);
}

#[test]
fn a_scalar_extends_and_equal_shapes_pass() {
    assert!(found("5 + 2 3 r_eshape 0").is_empty());
    assert!(found("(2 3 r_eshape 0) + 2 3 r_eshape 1").is_empty());
    assert!(found("n_eg 1 2 3").is_empty());
}

#[test]
fn data_the_lint_cannot_see_is_not_reported() {
    assert!(found("u:f_ := { a -> a + 1 2 3 }").is_empty());
    assert!(found("X := n_umbers []N_GET \"f\"\nX + 1 2").is_empty());
}

#[test]
fn a_parameter_or_a_mutation_forgets_the_shape() {
    assert!(found("X := 1 2\n{ X -> X + 1 2 3 } 1 2 3").is_empty());
    assert!(found("X! := 1 2\nX! := 1 2 3\nX! + 1 2 3").is_empty());
}

#[test]
fn the_inner_product_pairs_last_and_first() {
    assert_eq!(
        found("(2 3 r_eshape 1) '+ '* i_nner 2 4 r_eshape 1").len(),
        1
    );
    assert!(found("(2 3 r_eshape 1) '+ '* i_nner 3 4 r_eshape 1").is_empty());
    assert!(found("5 '+ '* i_nner 1 2 3").is_empty());
    let after = "P := (2 3 r_eshape 1) '+ '* i_nner 3 4 r_eshape 1\nP + 2 2 r_eshape 0";
    assert_eq!(found(after).len(), 1);
}

#[test]
fn a_finding_is_placed_at_the_application() {
    let program = xetal_core::lower("v := 1 2 3\nv - 1 2").expect("lowers");
    let span = xetal_shape::shapes(&program)[0].span.expect("placed");
    assert_eq!((span.start, span.end), (11, 18));
}

#[test]
fn an_array_on_one_side_gives_the_shape_when_it_runs() {
    let src = "V := n_umbers []N_GET \"f\"\nX := V + 1 2 3\nX + 1 2";
    assert_eq!(found(src).len(), 1);
    assert!(found("V := n_umbers []N_GET \"f\"\nX := V + 5\nX + 1 2").is_empty());
}

#[test]
fn a_reduce_drops_the_leading_axis() {
    assert_eq!(found("('+ r_/ 2 3 r_eshape 0) + 1 2").len(), 1);
    assert!(found("('+ r_/ 2 3 r_eshape 0) + 1 2 3").is_empty());
    assert!(found("('+ r_/_2 2 3 r_eshape 0) + 1 2").is_empty());
}
