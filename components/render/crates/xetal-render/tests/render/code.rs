//! Code strings drawn (ST5, docs/code-strings.md step 111 item 2): the
//! inside decorated with the rules of the code around it, the `c` and
//! the quotes kept; a plain or raw string untouched.

use xetal_render::{decorate, undecorate};

fn dec(src: &str) -> String {
    decorate(src).unwrap_or_else(|e| panic!("{src:?}: {e:?}"))
}

#[test]
fn the_inside_is_decorated_as_code() {
    let plain = dec(r#""n = 0" i_f< "p_rint! u:s_um 3; 4""#);
    let code = dec(r#""n = 0" i_f< c"p_rint! u:s_um 3; 4""#);
    // The plain string is drawn as typed; the code string's inside is
    // drawn as the same text outside a string would be.
    assert!(plain.ends_with(r#""p_rint! u:s_um 3; 4""#), "{plain}");
    let inside = dec("p_rint! u:s_um 3; 4");
    assert!(code.ends_with(&format!("c\"{inside}\"")), "{code}");
    assert_ne!(inside, "p_rint! u:s_um 3; 4");
}

#[test]
fn the_verbatim_form_is_decorated_too() {
    let code = dec(r###"c#"p_rint! "a"; u:s_um 2"#"###);
    assert_eq!(code, format!("c#\"{}\"#", dec(r#"p_rint! "a"; u:s_um 2"#)));
}

#[test]
fn escapes_are_resolved_before_and_restored_after() {
    // An inner string escaped in c"..." is decorated as a string (its
    // text untouched) and drawn escaped again, so the drawing is still
    // a c"..." whose text is the same.
    let code = dec(r#"c"p_rint! \"a_b\" c_at u:x_""#);
    let inner = dec(r#"p_rint! "a_b" c_at u:x_"#).replace('"', "\\\"");
    assert_eq!(code, format!("c\"{inner}\""));
}

#[test]
fn a_code_string_spanning_lines_keeps_its_lines() {
    let src = "c\"it nn:d_ense W1\n  |> nn:r_elu\"";
    let code = dec(src);
    assert_eq!(code.lines().count(), 2, "{code}");
    assert!(code.starts_with("c\""));
}

#[test]
fn what_does_not_lex_is_drawn_as_typed() {
    // `a_b_c` is no name (a function name has one underline): the
    // lexer reports it when the inside is checked (step 112); the
    // drawing shows from where it fails as typed (`_c`) and the code
    // before it decorated, and still turns back into the source.
    let drawn = dec(r#"c"x := a_b_c""#);
    assert_eq!(drawn, "c\"x \u{2190} a\u{332}b_c\"");
    assert_eq!(undecorate(&drawn).unwrap(), r#"c"x := a_b_c""#);
}

#[test]
fn a_code_string_inside_a_code_string_is_drawn_as_a_string() {
    let src = r##"c#"x c:f_< c"u:s_um 2""#"##;
    let code = dec(src);
    assert!(code.contains(r#"c"u:s_um 2""#), "{code}");
}

#[test]
fn the_drawing_turns_back_into_the_source() {
    for src in [
        r#""n = 0" i_f< c"p_rint! u:s_um 3; 4""#,
        r###"c#"p_rint! "a"; u:s_um 2"#"###,
        r#"c"p_rint! \"a_b\" c_at u:x_""#,
        "c\"it nn:d_ense W1\n  |> nn:r_elu\"",
        r#"x := "p_rint! u:s_um 3""#,
    ] {
        let back = undecorate(&dec(src)).unwrap_or_else(|e| panic!("{src:?}: {e:?}"));
        assert_eq!(back, src);
    }
}

#[test]
fn a_pipeline_draws_its_joins_as_triangles() {
    // S13: `|>` is drawn `▷`; the stages are code, decorated.
    let src = "Y := @ pipe:p_ipe< c\"\n   X\n|> it nn:d_ense W1\n|> nn:r_elu\"";
    let drawn = dec(src);
    assert!(
        drawn.contains(
            "\n\u{25b7} it \u{207f}\u{207f}d\u{332}ense W1\n\u{25b7} \u{207f}\u{207f}r\u{332}elu\""
        ),
        "{drawn}"
    );
    assert_eq!(undecorate(&drawn).unwrap(), src);
    assert_eq!(dec("x |> y"), "x \u{25b7} y");
    assert_eq!(undecorate("x \u{25b7} y").unwrap(), "x |> y");
}

#[test]
fn what_does_not_lex_is_drawn_as_typed_and_the_rest_as_code() {
    // A pipeline's labels (` : m`) are its own notation, not X_eTaL: the
    // rest of the inside is still decorated around them.
    let src = r#"@ pipe:p_ipe< c"z |> 'm_ax r_/ it : m |> e_xp z - m : e""#;
    let drawn = dec(src);
    assert!(
        drawn.contains("it : m \u{25b7} e\u{332}xp z \u{2212} m : e\""),
        "{drawn}"
    );
    assert_eq!(undecorate(&drawn).unwrap(), src);
    let shaped = dec(r#"c"X |> nn:r_elu it nn:d_ense W1 : h [4 3] |> h""#);
    assert!(
        shaped.contains("\u{207f}\u{207f}d\u{332}ense W1 : h [4 3] \u{25b7} h"),
        "{shaped}"
    );
}

#[test]
fn a_label_is_drawn_as_a_small_triangle() {
    // S14: `$h` is drawn `▹h`, a small sibling of the join's `▷`.
    let src = r#"@ flow:p_ipe< c"z |> 'm_ax r_/ it $m |> e_xp z - m $e |> e / '+ r_/ e""#;
    let drawn = dec(src);
    assert!(
        drawn.contains("it \u{25b9}m \u{25b7} e\u{332}xp z \u{2212} m \u{25b9}e"),
        "{drawn}"
    );
    assert_eq!(undecorate(&drawn).unwrap(), src);
    assert!(dec(r#"c"X |> f_ it $(Q, K, V)""#).contains("\u{25b9}(Q, K, V)"));
}
