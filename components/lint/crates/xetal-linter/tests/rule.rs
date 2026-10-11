//! Resolving levels: narrowest scope, then most specific target, then last.

use xetal_lint::{Level, lint};
use xetal_linter::{Rule, Scope, command_line, resolve};

fn rule(
    scope: Scope,
    target: &str,
    level: Level,
    lines: std::ops::RangeInclusive<usize>,
    order: usize,
) -> Rule {
    let target = target.to_string();
    Rule {
        scope,
        target,
        level,
        lines,
        order,
    }
}

#[test]
fn defaults_apply_without_rules() {
    assert_eq!(
        resolve(lint("mutable-constant").unwrap(), 1, &[]),
        Level::Warn
    );
    assert_eq!(
        resolve(lint("function-name-case").unwrap(), 1, &[]),
        Level::Allow
    );
    assert_eq!(
        resolve(lint("upper-case-name").unwrap(), 1, &[]),
        Level::Allow
    );
}

#[test]
fn a_lint_beats_its_group_whatever_the_order() {
    let s = |v: &[&str]| v.iter().map(|x| x.to_string()).collect::<Vec<_>>();
    let rules = command_line([&s(&["function-name-case"]), &[], &s(&["pedantic"])]).unwrap();
    assert_eq!(
        resolve(lint("function-name-case").unwrap(), 1, &rules),
        Level::Allow
    );
    assert_eq!(
        resolve(lint("upper-case-name").unwrap(), 1, &rules),
        Level::Deny
    );
}

#[test]
fn a_group_beats_all_and_deny_beats_allow_on_the_command_line() {
    let s = |v: &[&str]| v.iter().map(|x| x.to_string()).collect::<Vec<_>>();
    let rules = command_line([&s(&["all"]), &s(&["pedantic"]), &[]]).unwrap();
    assert_eq!(
        resolve(lint("upper-case-name").unwrap(), 1, &rules),
        Level::Warn
    );
    assert_eq!(
        resolve(lint("shadows-builtin").unwrap(), 1, &rules),
        Level::Allow
    );
    let rules = command_line([&s(&["style"]), &[], &s(&["style"])]).unwrap();
    assert_eq!(
        resolve(lint("mutable-constant").unwrap(), 1, &rules),
        Level::Deny
    );
}

#[test]
fn a_narrower_scope_wins_only_on_its_lines() {
    let rules = [
        rule(Scope::CommandLine, "style", Level::Deny, 1..=usize::MAX, 2),
        rule(Scope::Statement, "all", Level::Allow, 3..=4, 0),
    ];
    let l = lint("mutable-constant").unwrap();
    assert_eq!(resolve(l, 2, &rules), Level::Deny);
    assert_eq!(resolve(l, 4, &rules), Level::Allow);
    assert_eq!(resolve(l, 5, &rules), Level::Deny);
}

#[test]
fn an_unknown_command_line_name_is_an_error() {
    let err = command_line([&["styel".to_string()], &[], &[]]).unwrap_err();
    assert_eq!(err.code, "unknown-lint");
}
