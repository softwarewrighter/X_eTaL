//! `$` labels a pipeline stage's value in a macro's text (S14): one
//! token touching the name or the tuple after it.

use crate::common::{dump, kinds, reject};

#[test]
fn dollar_labels_a_name_or_a_tuple() {
    assert_eq!(kinds("it $h"), ["Var(it)", "Label", "Var(h)"]);
    assert_eq!(
        dump("x $logits"),
        ["0..1 Var(x)", "2..3 Label", "3..9 Var(logits)"]
    );
    assert_eq!(kinds("$(Q, K)")[..2], ["Label", "LParen"]);
}

#[test]
fn a_dollar_must_touch_what_it_labels() {
    assert_eq!(reject("x $ h").0, "bad-label");
    assert_eq!(reject("x $").0, "bad-label");
    assert_eq!(reject("x $1").0, "bad-label");
}

#[test]
fn a_labeled_pipeline_in_a_code_string_lexes() {
    assert_eq!(
        kinds("it nn:d_ense W1 $h [4 3] |> h")[3..5],
        ["Label", "Var(h)"]
    );
}
