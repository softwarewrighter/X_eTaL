//! `|>` joins pipeline stages (S13): one token, never `|` then `>`.

use crate::common::{dump, kinds};

#[test]
fn bar_greater_is_one_token() {
    assert_eq!(kinds("x |> y"), ["Var(x)", "Pipe", "Var(y)"]);
    assert_eq!(dump("x|>y"), ["0..1 Var(x)", "1..3 Pipe", "3..4 Var(y)"]);
}

#[test]
fn spaced_apart_they_are_still_or_and_greater() {
    assert_eq!(kinds("x | > y"), ["Var(x)", "Sym(|)", "Sym(>)", "Var(y)"]);
    assert_eq!(kinds("x | y"), ["Var(x)", "Sym(|)", "Var(y)"]);
    assert_eq!(kinds("x >= y"), ["Var(x)", "Sym(>=)", "Var(y)"]);
}

#[test]
fn a_pipeline_in_a_code_string_lexes_with_its_joins() {
    assert_eq!(kinds("it nn:d_ense W1 |> nn:r_elu")[3], "Pipe");
    assert_eq!(kinds("c\"x |> f_ it\""), ["Code(\"x |> f_ it\")"]);
}
