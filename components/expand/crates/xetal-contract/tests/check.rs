//! The checks a contract gives (MC31): the call's shape before the
//! macro runs, and after it, that a copying macro used every name and
//! number of its arguments.

use xetal_contract::{Contract, Uses, shape, unused};

#[test]
fn a_call_of_the_declared_shape_passes() {
    let c = Contract {
        takes: Some((false, true)),
        ..Contract::default()
    };
    assert!(shape(&c, "x:t_wice<", (false, true)).is_none());
    let any = Contract::default();
    assert!(shape(&any, "x:t_wice<", (true, true)).is_none());
}

#[test]
fn a_call_of_another_shape_is_refused_at_the_side_that_differs() {
    let c = Contract {
        takes: Some((false, true)),
        ..Contract::default()
    };
    let d = shape(&c, "x:t_wice<", (true, true)).unwrap();
    assert_eq!(d.code, "bad-macro-call");
    assert_eq!(
        d.message,
        "x:t_wice< takes @ on its left and text on its right"
    );
    assert!(d.notes.iter().any(|n| n == "macro-place: left"));
    let d = shape(&c, "x:t_wice<", (false, false)).unwrap();
    assert!(d.notes.iter().any(|n| n == "macro-place: right"));
    let c = Contract {
        takes: Some((true, true)),
        ..Contract::default()
    };
    let d = shape(&c, "w_ith<", (false, false)).unwrap();
    assert_eq!(
        d.message,
        "w_ith< takes text on its left and text on its right"
    );
}

fn copied() -> Contract {
    Contract::default()
}

#[test]
fn a_copying_macro_that_writes_every_name_and_number_passes() {
    let args = [None, Some("x + 2.5 * f_ y")];
    let text = "{ @ -> t := 2; t * (x + 2.5 * f_ y) } @";
    assert!(unused(&copied(), "x:t_wice<", &args, text).is_none());
    // Punctuation and separators are free; names may gain a namespace.
    let args = [Some("s_quare"), Some("_r * _r; 3")];
    assert!(unused(&copied(), "x:d_ef<", &args, "u:s_quare := { _r * _r }\n3").is_none());
    // A number written as an exponent counts as that number.
    let args = [None, Some("y^2")];
    assert!(unused(&copied(), "x:s_q<", &args, "y * 2").is_none());
}

#[test]
fn a_dropped_name_or_number_is_refused_at_the_argument() {
    let args = [None, Some("(y - p)^2")];
    let d = unused(&copied(), "e:s_um<", &args, "'+ r_/ y - p").unwrap();
    assert_eq!(d.code, "unused-argument");
    assert_eq!(
        d.message,
        "e:s_um< did not use 2 of the text on its right; a macro that copies its text writes every name and number of it (## uses: copied)"
    );
    assert!(d.notes.iter().any(|n| n == "macro-place: right"));
    let args = [Some("a b"), Some("1")];
    let d = unused(&copied(), "x:f_irst<", &args, "a 1").unwrap();
    assert!(
        d.message
            .starts_with("x:f_irst< did not use b of the text on its left")
    );
    assert!(d.notes.iter().any(|n| n == "macro-place: left"));
}

#[test]
fn a_parsing_macro_is_not_checked() {
    let c = Contract {
        uses: Uses::Parsed,
        ..Contract::default()
    };
    let args = [None, Some("(y - p)^2")];
    assert!(unused(&c, "e:s_um<", &args, "'+ r_/ y - p").is_none());
}

#[test]
fn text_that_does_not_lex_is_left_to_the_parser() {
    let args = [None, Some("two and two: {2 + 2} {{four}}")];
    assert!(
        unused(
            &copied(),
            "f_ormat<",
            &args,
            "\"two and two: \" c_at f_ormat 2 + 2"
        )
        .is_none()
    );
    let args = [None, Some("x + 1")];
    assert!(unused(&copied(), "x:b_ad<", &args, "x + \"unterminated").is_none());
}

#[test]
fn text_handed_on_inside_a_string_counts() {
    // A macro delegating to another macro, or quoting its text into a message.
    let args = [Some("c"), Some("2")];
    assert!(unused(&copied(), "x:t_wice<", &args, "\"c\" u_nless< \"2; 2\"").is_none());
    let args = [Some("x > 10"), Some("x is big")];
    let text = "{ @ -> (x > 10) ? @; []E_RR \"assertion failed: x > 10 (x is big)\"; @ } @";
    assert!(unused(&copied(), "a_ssert<", &args, text).is_none());
}

#[test]
fn an_expansion_that_refuses_the_call_is_not_checked() {
    let args = [Some("a"), Some("b")];
    let text = "\"too-big\" e_rror< \"keep it small\"";
    assert!(unused(&copied(), "y:s_mall<", &args, text).is_none());
}

#[test]
fn a_code_side_is_named_code_in_the_refusal() {
    let c = Contract {
        takes: Some((false, true)),
        code: (false, true),
        ..Contract::default()
    };
    assert!(shape(&c, "x:s_tage<", (false, true)).is_none());
    let d = shape(&c, "x:s_tage<", (false, false)).unwrap();
    assert_eq!(
        d.message,
        "x:s_tage< takes @ on its left and code on its right"
    );
}
