//! Doc comments and section headings in the view (I4): their markers
//! drawn as `xetal render` draws them, and a style of their own, apart
//! from plain remarks, in every front end.

use xetal_view::{Class, ansi, html, view};

fn classes(src: &str) -> Vec<(String, Class)> {
    view(src)
        .into_iter()
        .filter(|s| s.class != Class::Space)
        .map(|s| (s.text, s.class))
        .collect()
}

fn pairs(want: &[(&str, Class)]) -> Vec<(String, Class)> {
    want.iter().map(|(t, c)| (t.to_string(), *c)).collect()
}

#[test]
fn a_doc_line_has_its_marker_and_the_doc_class() {
    let want = [
        ("\u{235d}\u{2502}", Class::Doc),
        (" Twice.", Class::Doc),
        ("x", Class::Variable),
    ];
    assert_eq!(classes("## Twice.\nx"), pairs(&want));
}

#[test]
fn a_heading_has_its_marker_and_the_heading_class() {
    let want = [
        ("\u{235d}\u{2501}\u{2501}", Class::Heading),
        (" Sizes", Class::Heading),
    ];
    assert_eq!(classes("### Sizes"), pairs(&want));
}

#[test]
fn a_remark_stays_a_comment() {
    let want = [
        ("x", Class::Variable),
        ("\u{235d}", Class::Comment),
        ("# after code", Class::Comment),
    ];
    assert_eq!(classes("x ## after code"), pairs(&want));
}

#[test]
fn backquoted_code_in_a_doc_line_is_decorated() {
    let got = classes("  ## see `r_ev`.");
    assert_eq!(got[0], ("\u{235d}\u{2502}".into(), Class::Doc));
    assert!(got.contains(&("r\u{332}ev".into(), Class::Builtin)));
    assert_eq!(got.last(), Some(&(".".into(), Class::Doc)));
}

#[test]
fn doc_and_heading_have_their_own_html_and_ansi_styles() {
    let out = html(&view("### Sizes\n## Twice.\n# plain"));
    assert!(out.contains("<span class=\"c-heading\">"), "{out}");
    assert!(out.contains("<span class=\"c-doc\">"), "{out}");
    assert!(out.contains("<span class=\"c-comment\">"), "{out}");
    let (doc, remark) = (ansi(&view("## a")), ansi(&view("# a")));
    assert_ne!(doc, remark.replacen('\u{235d}', "\u{235d}\u{2502}", 1));
}
