//! The typed lints, run after a successful type check: `int-condition`,
//! a guard whose condition is an Int (a count), not a truth.

use xetal_linter::{command_line, lint};

fn found(text: &str, warn: &[&str]) -> Vec<(String, usize, String)> {
    let warn: Vec<String> = warn.iter().map(|s| s.to_string()).collect();
    let rules = command_line([&[], &warn, &[]]).unwrap();
    let findings = lint("t.xtl", text, &rules).expect("loads");
    findings
        .into_iter()
        .map(|f| (f.lint.name.to_string(), f.line, f.message))
        .collect()
}

const COUNT: &str = "u:f_ := { x -> x > 2 ? 1\n0 }\nu:g_ := { x -> t_ally x ? 1\n0 }";

#[test]
fn an_int_condition_is_found_when_asked_for() {
    let got = found(COUNT, &["int-condition"]);
    assert_eq!(got.len(), 1, "{got:?}");
    assert_eq!((got[0].0.as_str(), got[0].1), ("int-condition", 3));
    assert!(got[0].2.contains("unless it is 1 or 0"), "{}", got[0].2);
}

#[test]
fn it_is_pedantic_and_off_by_default() {
    assert!(found(COUNT, &[]).is_empty());
    assert_eq!(found(COUNT, &["pedantic"]).len(), 1);
}

#[test]
fn a_program_that_does_not_type_check_gets_no_typed_findings() {
    let text = "u:g_ := { x -> t_ally x ? 1\n0 }\n1 + \"a\"";
    assert!(found(text, &["int-condition"]).is_empty());
}
