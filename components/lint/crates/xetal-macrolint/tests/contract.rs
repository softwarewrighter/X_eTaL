//! `takes-mismatch`, `ignored-argument` and `binds-unwritten`: a
//! macro's contract (`## takes:`, `## uses:`, `## binds:`, MC31) that
//! its body contradicts.

fn found(text: &str, code: &str) -> Vec<String> {
    let text = format!("## M, a test library.\n\n{text}");
    let loaded = xetal_program::load_library("M.xtlm", &text).expect("loads");
    let combined = loaded.sources.combined();
    let all = xetal_lint::lints(&loaded.program);
    xetal_macrolint::merged(all, &loaded.program, &loaded.sources)
        .into_iter()
        .filter(|d| d.code == code)
        .map(|d| {
            let span = d.span.expect("placed");
            format!("{} @ {}", d.message, &combined[span.start..span.end])
        })
        .collect()
}

#[test]
fn a_body_matching_its_takes_line_passes() {
    for text in [
        "## takes: text text\nm:p_air< := { a b -> a c_at b }",
        "## takes: @ text\nm:o_ne< := { @ e -> e }",
        "## takes: @ @\nm:z_ero< := { @ @ -> \"0\" }",
        "m:p_air< := { a b -> a c_at b }",
    ] {
        assert!(found(text, "takes-mismatch").is_empty(), "{text}");
    }
}

#[test]
fn a_side_taking_text_that_the_body_writes_as_at_is_reported() {
    let got = found(
        "## takes: text text\nm:o_ne< := { @ e -> e }",
        "takes-mismatch",
    );
    assert_eq!(got.len(), 1, "{got:?}");
    assert!(
        got[0].starts_with(
            "## takes: says o_ne< takes text on its left, but its parameter there is @"
        ),
        "{got:?}"
    );
}

#[test]
fn a_side_taking_at_that_the_body_names_is_reported() {
    let got = found(
        "## takes: text @\nm:p_air< := { a b -> a c_at b }",
        "takes-mismatch",
    );
    assert_eq!(got.len(), 1, "{got:?}");
    assert!(
        got[0].contains("takes @ on its right, but its parameter there is named `b`"),
        "{got:?}"
    );
    assert!(got[0].ends_with("@ b"), "{got:?}");
}

#[test]
fn a_copying_macro_that_never_reads_a_side_is_reported_once() {
    let text = "## takes: text text\nm:l_eft< := { a b -> a }";
    let got = found(text, "ignored-argument");
    assert_eq!(got.len(), 1, "{got:?}");
    assert!(
        got[0].starts_with("l_eft< copies its text (## uses: copied) but never reads `b`"),
        "{got:?}"
    );
    assert!(got[0].ends_with("@ b"), "{got:?}");
    assert!(found(text, "unused-binding").is_empty());
}

#[test]
fn a_parsing_macro_may_ignore_a_side() {
    let text = "## takes: text text\n## uses: parsed\nm:l_eft< := { a b -> a }";
    assert!(found(text, "ignored-argument").is_empty());
    // The general lint still says the parameter is unused.
    assert_eq!(found(text, "unused-binding").len(), 1);
}

#[test]
fn an_anaphor_the_text_never_writes_is_reported() {
    let text = "## binds: it\n## takes: text text\nm:w_ith< := { e body -> \"{ @ -> x := (\" c_at e c_at \"); \" c_at body c_at \" } @\" }";
    let got = found(text, "binds-unwritten");
    assert_eq!(got.len(), 1, "{got:?}");
    assert!(
        got[0].starts_with("w_ith< declares ## binds: it, but no text it writes has `it`"),
        "{got:?}"
    );
    let written = text.replace("x := (", "it := (");
    assert!(found(&written, "binds-unwritten").is_empty());
}

#[test]
fn an_anaphor_inside_a_longer_name_is_not_written() {
    let text = "## binds: it\n## takes: text text\nm:w_ith< := { e body -> \"{ @ -> item := (\" c_at e c_at \"); \" c_at body c_at \" } @\" }";
    assert_eq!(found(text, "binds-unwritten").len(), 1);
}
