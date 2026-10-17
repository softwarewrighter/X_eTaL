//! The string lints (`xetal-strings`) through the driver: placed in the
//! file as written, leveled by groups and `# lint:` comments, and fixed
//! without changing any string's value.

use xetal_lint::Level;
use xetal_linter::{Finding, command_line, fixed, lint};

fn findings(text: &str, warn: &[&str]) -> Vec<Finding> {
    let warn: Vec<String> = warn.iter().map(|s| s.to_string()).collect();
    let rules = command_line([&[], &warn, &[]]).unwrap();
    lint("t.xtl", text, &rules).expect("loads")
}

fn names(found: &[Finding]) -> Vec<(&str, usize, usize)> {
    found.iter().map(|f| (f.lint.name, f.line, f.col)).collect()
}

fn values(text: &str) -> Vec<String> {
    let lits = xetal_strings::literals(text);
    lits.into_iter().map(|l| l.value).collect()
}

const TEXT: &str = "x := 1\ns := r##\"a \"b\" c\"##\np := \"C:\\\\dir\\\\f\"\nq := r\"plain\"\nd := \"# lint: allow all\"\n";

#[test]
fn the_default_levels_report_hashes_and_directives_in_place() {
    let found = findings(TEXT, &[]);
    let want = [
        ("needless-raw-hashes", 2, 6),
        ("string-holds-directive", 5, 7),
    ];
    assert_eq!(names(&found), want);
    assert_eq!((found[0].end_line, found[0].end_col), (2, 20));
    assert_eq!(found[1].level, Level::Warn);
}

#[test]
fn pedantic_turns_on_the_raw_and_escape_lints() {
    let found = findings(TEXT, &["pedantic"]);
    let want = [
        ("needless-raw-hashes", 2, 6),
        ("escapes-could-be-raw", 3, 6),
        ("needless-raw-string", 4, 6),
        ("string-holds-directive", 5, 7),
    ];
    assert_eq!(names(&found), want);
}

#[test]
fn every_fix_keeps_every_value() {
    let found = findings(TEXT, &["pedantic"]);
    let (after, n) = fixed(TEXT, &found);
    assert_eq!(n, 3);
    assert_eq!(values(&after), values(TEXT));
    let want = "x := 1\ns := r#\"a \"b\" c\"#\np := r\"C:\\dir\\f\"\nq := \"plain\"\n";
    assert!(after.starts_with(want), "{after}");
    assert!(names(&findings(&after, &["pedantic"])).len() == 1);
}

#[test]
fn a_comment_allows_a_string_lint() {
    // On its own line: a trailing comment after `"##` is not yet found
    // (the directive scanner reads `r##"a"##` as `"a"` then a comment,
    // lint plan section 3).
    let text = "# lint: allow needless-raw-hashes\ns := r##\"a\"##\nt := r##\"b\"##\n";
    assert_eq!(names(&findings(text, &[])), [("needless-raw-hashes", 3, 6)]);
}

#[test]
fn a_directive_written_in_a_string_sets_no_level() {
    let text = "d := \"# lint: allow all\"\nCount! := 0\n";
    let found = findings(text, &[]);
    assert_eq!(
        names(&found),
        [("string-holds-directive", 1, 7), ("mutable-constant", 2, 1)]
    );
}
