//! `string-holds-directive`: a line of a string that reads as a
//! `# lint:` comment, which does nothing there.

use crate::common::codes;

const CODE: &str = "string-holds-directive";

fn spans(text: &str) -> Vec<(usize, usize)> {
    xetal_strings::strings(text)
        .into_iter()
        .filter(|d| d.code == CODE)
        .filter_map(|d| d.span.map(|s| (s.start, s.end)))
        .collect()
}

#[test]
fn a_line_inside_a_raw_string_is_reported_at_the_directive() {
    // Probe r3 of the lint plan.
    let src = "s := r\"one\n  # lint-file: allow all\ntwo\"\nCount! := 0";
    assert_eq!(codes(src), [CODE]);
    let at = src.find("# lint-file").unwrap();
    assert_eq!(spans(src), [(at, at + "# lint-file: allow all".len())]);
}

#[test]
fn a_plain_string_that_is_a_directive_is_reported() {
    let src = r##"s := "# lint: allow mutable-constant""##;
    assert_eq!(codes(src), [CODE]);
}

#[test]
fn each_such_line_is_reported_and_has_no_fix() {
    let src = "s := r#\"# lint: allow x\nok\n\t# lint: warn y\"#";
    let found: Vec<_> = xetal_strings::strings(src)
        .into_iter()
        .filter(|d| d.code == CODE)
        .collect();
    assert_eq!(found.len(), 2);
    assert!(found.iter().all(|d| d.fix().is_empty()));
}

#[test]
fn a_comment_after_the_string_is_not_reported() {
    // Probe r2 and r4: the directive is a real comment.
    for src in [
        r#"Count! := r"C:\" # lint: allow mutable-constant"#,
        r##"Count! := r#"a "quoted" # word"# # lint: allow mutable-constant"##,
    ] {
        assert!(codes(src).is_empty(), "{src}: {:?}", codes(src));
    }
}

#[test]
fn a_line_merely_mentioning_lint_is_quiet() {
    for src in [
        r#"s := "see # lint: later""#,
        r##"s := "# linter""##,
        r##"s := "#lint: allow x""##,
    ] {
        assert!(codes(src).is_empty(), "{src}");
    }
}
