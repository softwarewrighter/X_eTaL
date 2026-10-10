//! Parsing `# lint:` and `# lint-file:` comments.

use xetal_linter::directives;

#[test]
fn own_line_and_trailing_directives() {
    let ds = directives("# lint: allow shadows-builtin\nx := 1  # lint: deny style, all\n");
    assert_eq!(ds.len(), 2);
    assert!(ds[0].own_line && !ds[0].file);
    assert_eq!(
        (ds[0].line, ds[0].col, ds[0].level.as_str()),
        (1, 1, "allow")
    );
    assert_eq!(ds[0].names, ["shadows-builtin"]);
    assert!(!ds[1].own_line);
    assert_eq!((ds[1].line, ds[1].col), (2, 9));
    assert_eq!(ds[1].names, ["style", "all"]);
}

#[test]
fn file_directive() {
    let ds = directives("#!/usr/bin/env xetal\n# lint-file: warn pedantic\n");
    assert_eq!(ds.len(), 1);
    assert!(ds[0].file);
    assert_eq!(ds[0].level(), Some(xetal_lint::Level::Warn));
}

#[test]
fn a_hash_in_a_string_is_not_a_comment() {
    assert!(directives("s := \"# lint: allow all\"").is_empty());
    let ds = directives("s := \"a\\\"#\" # lint: allow all");
    assert_eq!(ds.len(), 1);
    assert_eq!(ds[0].col, 13);
}

#[test]
fn doc_comments_headings_and_prose_are_not_directives() {
    let text = "## lint: allow all\n### lint: allow all\n# a lint: allow all\n# linting is fun";
    assert!(directives(text).is_empty());
}

#[test]
fn a_bad_level_is_kept_for_reporting() {
    let ds = directives("# lint: alow style");
    assert_eq!(ds[0].level(), None);
    assert_eq!(ds[0].level, "alow");
}
