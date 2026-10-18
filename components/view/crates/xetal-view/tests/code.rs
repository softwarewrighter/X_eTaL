//! Code strings in the view model (ST5; docs/code-strings.md step 111
//! item 5), which the terminal editor and the live demo both draw: the
//! inside is segmented and classed as code, placed on the bytes it was
//! written on, and an open code string is decorated while it is typed.

use xetal_view::{Class, view};

fn classes(src: &str) -> Vec<(String, Class)> {
    view(src)
        .into_iter()
        .filter(|s| s.class != Class::Space)
        .map(|s| (s.text, s.class))
        .collect()
}

#[test]
fn the_inside_is_classed_as_code() {
    let got = classes(r#""n = 0" i_f< c"p_rint! u:s_um 3""#);
    let want = [
        ("\"n = 0\"", Class::String),
        ("i\u{332}f<", Class::Macro),
        ("c\"", Class::String),
        ("p\u{332}rint!", Class::Builtin),
        ("\u{1d58}s\u{332}um", Class::UserFunc),
        ("3", Class::Number),
        ("\"", Class::String),
    ];
    let want: Vec<(String, Class)> = want.iter().map(|(t, c)| (t.to_string(), *c)).collect();
    assert_eq!(got, want);
}

#[test]
fn a_plain_string_stays_one_string() {
    let got = classes(r#""n = 0" i_f< "p_rint! u:s_um 3""#);
    assert_eq!(
        got.last().unwrap(),
        &("\"p_rint! u:s_um 3\"".to_string(), Class::String)
    );
}

#[test]
fn each_piece_sits_on_the_bytes_it_was_written_on() {
    let src = r#"x := c"p_rint! \"a_b\" c_at u:x_""#;
    for s in view(src) {
        let written = &src[s.raw.start..s.raw.end];
        if s.class == Class::UserFunc {
            assert_eq!(written, "u:x_");
        }
        if s.class == Class::String && written.contains("a_b") {
            assert_eq!(written, r#"\"a_b\""#);
            assert_eq!(s.text, r#"\"a_b\""#);
        }
    }
}

#[test]
fn the_view_draws_what_decorate_draws() {
    for src in [
        r#""n = 0" i_f< c"p_rint! u:s_um 3; 4""#,
        r###"c#"p_rint! "a"; u:s_um 2"#"###,
        r#"c"p_rint! \"a_b\" c_at u:x_""#,
        "c\"it nn:d_ense W1\n  u:r_elu it\"",
    ] {
        let got: String = view(src).into_iter().map(|s| s.text).collect();
        assert_eq!(got, xetal_render::decorate(src).unwrap(), "{src:?}");
    }
}

#[test]
fn an_open_code_string_is_decorated_while_it_is_typed() {
    let got = classes(r#"x := c"p_rint! u:s_um"#);
    assert!(got.iter().all(|(_, c)| *c != Class::Error), "{got:?}");
    assert!(
        got.contains(&("\u{1d58}s\u{332}um".to_string(), Class::UserFunc)),
        "{got:?}"
    );
    // The open string's text is shown as typed so far, nothing added.
    let shown: String = view(r#"x := c"p_rint!"#)
        .into_iter()
        .map(|s| s.text)
        .collect();
    assert_eq!(shown, "x \u{2190} c\"p\u{332}rint!");
}

#[test]
fn an_open_plain_string_is_still_an_error() {
    assert!(
        classes("x := \"abc")
            .iter()
            .any(|(_, c)| *c == Class::Error)
    );
}

#[test]
fn a_pipeline_join_is_punctuation() {
    let got = classes("c\"x |> nn:r_elu\"");
    assert!(
        got.contains(&("\u{25b7}".to_string(), Class::Punct)),
        "{got:?}"
    );
}
