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
fn an_inside_that_does_not_lex_is_drawn_as_typed() {
    // `a_b_c` is no name (a function name has one underline): the
    // lexer reports it when the inside is checked (step 112); the
    // drawing shows it as typed.
    assert_eq!(dec(r#"c"x := a_b_c""#), r#"c"x := a_b_c""#);
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
