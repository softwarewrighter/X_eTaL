//! `reject-undocumented`, `reject-unplaced` and `reject-bad-place`: the
//! codes a macro rejects its call with, as its doc block and its users
//! see them.

fn found(text: &str, code: &str) -> Vec<String> {
    let loaded = xetal_program::load_library("M.xtlm", text).expect("loads");
    xetal_macrolint::merged(Vec::new(), &loaded.program, &loaded.sources)
        .into_iter()
        .filter(|d| d.code == code)
        .map(|d| d.message)
        .collect()
}

const DOCUMENTED: &str = "## M, a test library.

## Its argument, which may not be empty (bad-macro-argument).
## >> \"x:\" u_se< \"M\"
## >> @ x:i_d< \"3\"
## 3
m:i_d< := { @ e ->
  0 = t_ally e ? \"bad-macro-argument right\" []R_EJECT \"an empty argument\"
  e }";

#[test]
fn a_code_its_doc_names_passes() {
    for lint in ["reject-undocumented", "reject-unplaced", "reject-bad-place"] {
        assert!(found(DOCUMENTED, lint).is_empty(), "{lint}");
    }
}

#[test]
fn a_code_its_doc_does_not_name_is_reported() {
    let text = DOCUMENTED.replace("(bad-macro-argument)", "");
    let got = found(&text, "reject-undocumented");
    assert_eq!(got.len(), 1, "{got:?}");
    assert!(
        got[0].starts_with("i_d< rejects its call with `bad-macro-argument`"),
        "{got:?}"
    );
}

#[test]
fn a_code_shown_in_an_example_counts_as_documented() {
    let text = "## M, a test library.

## Its argument, never empty.
## >> \"x:\" u_se< \"M\"
## >> @ x:i_d< \"\"
## error[empty-argument]: an empty argument
m:i_d< := { @ e ->
  0 = t_ally e ? \"empty-argument right\" []R_EJECT \"an empty argument\"
  e }";
    assert!(found(text, "reject-undocumented").is_empty());
}

#[test]
fn a_word_containing_the_code_is_not_a_mention() {
    let text = DOCUMENTED.replace("(bad-macro-argument)", "(bad-macro-arguments)");
    assert_eq!(found(&text, "reject-undocumented").len(), 1);
}

#[test]
fn a_code_with_no_place_is_unplaced() {
    let text = DOCUMENTED.replace("bad-macro-argument right", "bad-macro-argument");
    let got = found(&text, "reject-unplaced");
    assert_eq!(got.len(), 1, "{got:?}");
    assert!(got[0].contains("lands on the macro's name"), "{got:?}");
    let call = DOCUMENTED.replace("bad-macro-argument right", "bad-macro-argument call");
    assert!(found(&call, "reject-unplaced").is_empty());
}

#[test]
fn a_misspelled_place_is_reported() {
    let text = DOCUMENTED.replace("bad-macro-argument right", "bad-macro-argument rigth");
    let got = found(&text, "reject-bad-place");
    assert_eq!(got.len(), 1, "{got:?}");
    assert!(got[0].contains("`rigth`"), "{got:?}");
    assert!(found(&text, "reject-unplaced").is_empty());
}

#[test]
fn a_computed_code_is_not_checked() {
    let text = "m:e_rr< := { code message -> code []R_EJECT message }";
    for lint in ["reject-undocumented", "reject-unplaced", "reject-bad-place"] {
        assert!(found(text, lint).is_empty(), "{lint}");
    }
}
