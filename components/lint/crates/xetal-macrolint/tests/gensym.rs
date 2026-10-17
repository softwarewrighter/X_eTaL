//! `gensym-unused` and `gensym-not-name`: a fresh name asked for and
//! never used, and one asked for under a name that is not plain.

fn found(text: &str, code: &str) -> Vec<String> {
    let loaded = xetal_program::load_library("M.xtlm", text).expect("loads");
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
fn a_fresh_name_spliced_into_the_text_passes() {
    let text = "m:t_wice< := { @ e ->\n  t := []G_ENSYM \"t\"\n  t c_at \" := 2; \" c_at t c_at \" * \" c_at e }";
    assert!(found(text, "gensym-unused").is_empty());
}

#[test]
fn a_fresh_name_bound_and_never_used_warns_once() {
    let text = "m:t_wice< := { @ e ->\n  t := []G_ENSYM \"t\"\n  \"2 * \" c_at e }";
    let got = found(text, "gensym-unused");
    assert_eq!(got.len(), 1, "{got:?}");
    assert!(
        got[0].starts_with("the fresh name `t` is never used"),
        "{got:?}"
    );
    assert!(got[0].ends_with("@ t"), "{got:?}");
    // The more specific finding replaces unused-binding's at the same place.
    assert!(found(text, "unused-binding").is_empty());
}

#[test]
fn a_fresh_name_asked_for_and_thrown_away_warns() {
    let text = "m:t_wice< := { @ e ->\n  []G_ENSYM \"t\"\n  \"2 * \" c_at e }";
    let got = found(text, "gensym-unused");
    assert_eq!(got.len(), 1, "{got:?}");
    assert!(
        got[0].starts_with("a fresh name asked for and thrown away"),
        "{got:?}"
    );
}

#[test]
fn a_literal_that_is_not_a_plain_name_warns() {
    for bad in ["t u", "1t", "u:t", "a_b_c", ""] {
        let text = format!("m:t_wice< := {{ @ e ->\n  t := []G_ENSYM \"{bad}\"\n  t c_at e }}");
        let got = found(&text, "gensym-not-name");
        assert_eq!(got.len(), 1, "{bad:?}: {got:?}");
        assert!(got[0].contains("not a plain name"), "{got:?}");
    }
}

#[test]
fn plain_names_and_computed_ones_pass() {
    for good in ["t", " t ", "s_um", "x2"] {
        let text = format!("m:t_wice< := {{ @ e ->\n  t := []G_ENSYM \"{good}\"\n  t c_at e }}");
        assert!(found(&text, "gensym-not-name").is_empty(), "{good:?}");
    }
    let computed = "m:t_wice< := { @ e ->\n  t := []G_ENSYM e\n  t c_at e }";
    assert!(found(computed, "gensym-not-name").is_empty());
}
