//! `missing-example`: a macro whose doc block shows no `## >>`
//! example of a call.

fn found(text: &str) -> Vec<String> {
    let loaded = xetal_program::load_library("M.xtlm", text).expect("loads");
    let combined = loaded.sources.combined();
    xetal_macrolint::merged(Vec::new(), &loaded.program, &loaded.sources)
        .into_iter()
        .filter(|d| d.code == "missing-example")
        .map(|d| {
            let span = d.span.expect("placed");
            format!("{} @ {}", d.message, &combined[span.start..span.end])
        })
        .collect()
}

#[test]
fn a_macro_with_an_example_passes() {
    let text = "## M, a test library.\n\n## Its argument.\n## >> \"x:\" u_se< \"M\"\n## >> @ x:i_d< \"3\"\n## 3\nm:i_d< := { @ e -> e }";
    assert!(found(text).is_empty());
}

#[test]
fn a_macro_with_prose_only_or_no_doc_is_reported() {
    for text in [
        "## M, a test library.\n\n## Its argument.\nm:i_d< := { @ e -> e }",
        "m:i_d< := { @ e -> e }",
        "## The header of the file.\n\nm:i_d< := { @ e -> e }",
    ] {
        let got = found(text);
        assert_eq!(got.len(), 1, "{text}: {got:?}");
        assert!(
            got[0].starts_with("the macro i_d< has no `## >>` example"),
            "{got:?}"
        );
        assert!(got[0].ends_with(":i_d<"), "{got:?}");
    }
}

#[test]
fn a_helper_needs_no_example() {
    let text = "## M, a test library.\n\nh:t_rim := { _r }\n## Its argument.\n## >> \"x:\" u_se< \"M\"\n## >> @ x:i_d< \"3\"\n## 3\nm:i_d< := { @ e -> h:t_rim e }";
    assert!(found(text).is_empty());
}
