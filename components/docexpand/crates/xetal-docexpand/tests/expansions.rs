//! Each macro call of a file with the text its macro gave, and the
//! calls in that text expanded in turn, to their depth.

use std::path::PathBuf;

use xetal_docexpand::expansions;

fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../doc/fixtures")
}

#[test]
fn a_system_macro_call_and_its_expansion() {
    let src = "x := 3\ny := \"x > 0\" i_f< \"1; -1\"\n";
    let found = expansions("-e", src).expect("expanded");
    assert_eq!(found.len(), 1);
    let e = &found[0];
    assert_eq!((e.name.as_str(), e.line), ("i_f<", 2));
    assert_eq!(e.call, "\"x > 0\" i_f< \"1; -1\"");
    assert_eq!(e.text, "({ @ -> (x > 0) ? 1; -1 } @)");
    assert!(e.nested.is_empty());
}

#[test]
fn a_statement_is_not_parenthesized_and_a_library_macro_is_found_by_its_alias() {
    let path = fixtures().join("app.xtl");
    let text = std::fs::read_to_string(&path).expect("fixture");
    let found = expansions(&path.to_string_lossy(), &text).expect("expanded");
    let e = found.iter().find(|e| e.name == "g:w_hen<").expect("w_hen");
    assert!(e.text.starts_with("{ @ -> (n_ot 1 = 1) ? @;"), "{}", e.text);
    assert_eq!(e.line, 13);
}

#[test]
fn calls_in_an_expansion_are_expanded_in_turn() {
    let src = "\"1 > 0\" u_nless< \"p_rint! @ f_ormat< \\\"{1 + 1}\\\"\"\n";
    let found = expansions("-e", src).expect("expanded");
    let outer = &found[0];
    assert_eq!(outer.name, "u_nless<");
    assert_eq!(outer.nested.len(), 1, "{outer:?}");
    assert_eq!(outer.nested[0].name, "f_ormat<");
    assert_eq!(outer.nested[0].line, 1);
}

#[test]
fn an_expansion_as_json() {
    let found = expansions("-e", "y := \"1 > 0\" i_f< \"1; 2\"\n").expect("expanded");
    assert_eq!(
        found[0].to_json(),
        "{\"line\": 1, \"macro\": \"i_f<\", \"call\": \"\\\"1 > 0\\\" i_f< \\\"1; 2\\\"\", \"text\": \"({ @ -> (1 > 0) ? 1; 2 } @)\", \"nested\": []}"
    );
}

#[test]
fn a_file_without_calls_has_no_expansions() {
    assert!(expansions("-e", "x := 1\n").expect("expanded").is_empty());
}
