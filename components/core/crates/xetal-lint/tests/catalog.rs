//! The lint catalog: every lint a pass reports is registered, names are
//! unique, and each has a summary and an explanation.

use std::collections::HashSet;

#[test]
fn names_are_unique_kebab_case() {
    let mut seen = HashSet::new();
    for lint in xetal_lint::all() {
        assert!(seen.insert(lint.name), "duplicate {}", lint.name);
        assert!(
            lint.name
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b == b'-')
        );
    }
}

#[test]
fn every_lint_is_documented() {
    for lint in xetal_lint::all() {
        assert!(
            !lint.summary.is_empty() && lint.explain.contains("Example"),
            "{}",
            lint.name
        );
    }
}

#[test]
fn every_reported_code_is_registered() {
    let src = "u:N_ext := { r_ev -> r_ev }\nCount! := 0\nMAX := 1\n{ x -> o_- := x\no_- } 1";
    let program = xetal_core::lower(src).expect("lowers");
    let found = xetal_lint::lints(&program);
    assert!(found.len() >= 4);
    for d in found {
        assert!(
            xetal_lint::lint(&d.code).is_some(),
            "{} is not registered",
            d.code
        );
    }
}

#[test]
fn group_names_and_levels_parse_back() {
    for group in xetal_lint::Group::ALL {
        assert!(!group.name().is_empty());
    }
    for level in [
        xetal_lint::Level::Allow,
        xetal_lint::Level::Warn,
        xetal_lint::Level::Deny,
    ] {
        assert_eq!(xetal_lint::Level::parse(level.name()), Some(level));
    }
    assert_eq!(xetal_lint::Level::parse("forbid"), None);
}
