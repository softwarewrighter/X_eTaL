//! What the editors offer at the cursor.

use xetal_complete::{Completion, Context, complete};

fn ctx<'a>(files: &'a [String]) -> Context<'a> {
    Context {
        files,
        library: &|name| xetal_libs::standard(name).map(String::from),
    }
}

/// The completion at the end of `text`.
fn at_end(text: &str) -> Option<Completion> {
    complete(text, text.len(), &ctx(&[]))
}

fn labels(c: &Option<Completion>) -> Vec<String> {
    c.as_ref()
        .map(|c| c.candidates.iter().map(|k| k.label.clone()).collect())
        .unwrap_or_default()
}

#[test]
fn a_built_in_is_offered_by_its_typed_prefix_or_its_letters() {
    let c = at_end("1 + r_");
    assert!(labels(&c).contains(&"r_ange".to_string()), "{c:?}");
    let c = c.unwrap();
    assert_eq!((c.start, c.end), (4, 6));
    let range = c.candidates.iter().find(|k| k.label == "r_ange").unwrap();
    assert_eq!(
        (range.insert.as_str(), range.detail.as_str()),
        ("r_ange", "Int -> Int")
    );
    assert!(labels(&at_end("ran")).contains(&"r_ange".to_string()));
    assert!(labels(&at_end("[]S")).iter().any(|l| l == "[]S_HOW"));
    assert!(
        labels(&at_end("x := r_ange")).is_empty(),
        "the whole name typed"
    );
    assert_eq!(at_end("1 + "), None, "nothing being typed");
}

#[test]
fn the_candidates_are_sorted_so_tab_takes_the_first() {
    let found = labels(&at_end("r_a"));
    let mut sorted = found.clone();
    sorted.sort();
    assert_eq!(found, sorted);
    assert_eq!(
        found.first().map(String::as_str),
        Some("r_ange"),
        "{found:?}"
    );
}

#[test]
fn the_texts_own_definitions_are_offered_with_their_line() {
    let text = "u:k_eep := { a _u -> a }\nn := 3\nu:k";
    let c = at_end(text).unwrap();
    let keep = c.candidates.iter().find(|k| k.label == "u:k_eep").unwrap();
    assert_eq!(keep.detail, "defined on line 1");
    let whole = labels(&at_end("u:k_eep := { a _u -> a }\nn := 3\nn + 1 * n"));
    assert!(
        !whole.contains(&"n".to_string()),
        "a name typed whole is not offered: {whole:?}"
    );
}

#[test]
fn a_library_name_is_offered_inside_the_string_after_use() {
    let text = "\"x:\" u_se< \"S";
    let c = at_end(text).unwrap();
    assert_eq!((c.start, c.end), (12, 13));
    assert!(
        labels(&Some(c.clone())).contains(&"Stats".to_string()),
        "{c:?}"
    );
    let files = ["Sq.xtl".to_string(), "Mine.xtlm".to_string()];
    let c = complete(text, text.len(), &ctx(&files)).unwrap();
    let sq = c.candidates.iter().find(|k| k.label == "Sq").unwrap();
    assert_eq!(sq.detail, "your file");
    assert!(
        labels(&at_end("\"x:\" u_se< \"Stats\" ")).is_empty(),
        "the string is closed"
    );
    assert!(labels(&at_end("p_rint! \"S")).is_empty(), "not after u_se<");
}

#[test]
fn an_imported_librarys_exports_follow_its_alias() {
    let text = "\"s:\" u_se< \"Stats\"\ns:m";
    let c = at_end(text).unwrap();
    let mean = c.candidates.iter().find(|k| k.label == "s:m_ean").unwrap();
    assert_eq!(mean.insert, "s:m_ean");
    assert!(!mean.detail.is_empty());
    assert!(
        labels(&at_end("\"s:\" u_se< \"Stats\"\nq:m")).is_empty(),
        "no such alias"
    );
}

#[test]
fn the_lambda_arguments_follow_an_underscore_in_a_block() {
    let c = at_end("u:f_ := { 2 * _");
    assert_eq!(labels(&c), ["_l", "_r", "_u"]);
    assert_eq!(at_end("x := _"), None, "outside a block");
}

#[test]
fn a_lint_directive_offers_the_level_then_the_lints_and_groups() {
    assert_eq!(labels(&at_end("# lint: ")), ["allow", "deny", "warn"]);
    assert_eq!(labels(&at_end("# lint: a")), ["allow"]);
    let after = labels(&at_end("# lint: allow un"));
    assert!(after.contains(&"unused-binding".to_string()), "{after:?}");
    let groups = labels(&at_end("# lint-file: deny s"));
    assert!(groups.contains(&"style".to_string()) && groups.contains(&"suspicious".to_string()));
    let c = at_end("# lint: allow un").unwrap();
    assert_eq!((c.start, c.end), (14, 16));
}
