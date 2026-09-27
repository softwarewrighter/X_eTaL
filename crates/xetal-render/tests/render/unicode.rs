//! Raw ASCII -> decorated Unicode.

use crate::ul;
use xetal_render::decorate;

fn dec(src: &str) -> String {
    decorate(src).unwrap_or_else(|e| panic!("{src:?}: {e:?}"))
}

#[test]
fn nouns_and_plain_tokens_are_unchanged() {
    for src in [
        "r", "square2", "_l", "_r", "@", ";", "{ }", "( )", "[ ]", "-1 2.5", "+", "|",
    ] {
        assert_eq!(dec(src), src);
    }
}

#[test]
fn underline_marks_named_functions() {
    assert_eq!(dec("r_"), ul("r"));
    assert_eq!(dec("square_ 7"), format!("{} 7", ul("square")));
    assert_eq!(dec("m.f_"), format!("m.{}", ul("f")));
}

#[test]
fn axis_subscripts_become_subscript_digits() {
    assert_eq!(dec("t_2"), format!("{}\u{2082}", ul("t")));
    assert_eq!(dec("t_12"), format!("{}\u{2081}\u{2082}", ul("t")));
    assert_eq!(dec("+_1"), "+\u{2081}");
}

#[test]
fn derivations_become_superscript_letters() {
    assert_eq!(dec("+^r"), "+\u{2b3}");
    assert_eq!(dec("+^s_2"), "+\u{2e2}\u{2082}");
    assert_eq!(dec("f^e"), "f\u{1d49}");
    assert_eq!(
        dec("+^reduce"),
        "+\u{2b3}\u{1d49}\u{1d48}\u{1d58}\u{1d9c}\u{1d49}"
    );
}

#[test]
fn niladic_is_the_underlined_or_derived_name_touching_at() {
    assert_eq!(dec("now_@"), format!("{}@", ul("now")));
    assert_eq!(dec("now_ @"), format!("{} @", ul("now")));
    assert_eq!(dec("+_@"), format!("{}@", ul("+")));
    assert_eq!(dec("f^e_@"), "f\u{1d49}@");
    assert_eq!(dec("+@"), "+@");
}

#[test]
fn words_without_superscript_glyphs_stay_raw() {
    assert_eq!(dec("+^q"), "+^q");
    assert_eq!(dec("+^rq_2"), "+^rq_2");
    assert_eq!(dec("f^R_@"), "f^R_@");
}

#[test]
fn whitespace_and_newlines_are_preserved() {
    assert_eq!(dec("a  \t b\n c_"), format!("a  \t b\n {}", ul("c")));
}

#[test]
fn life_line() {
    let src = "life = { (+^r -1 0 1 t_12 _r) { (_l = 3) + _r * _l = 4 } _r }";
    let want = format!(
        "life = {{ (+\u{2b3} -1 0 1 {}\u{2081}\u{2082} _r) {{ (_l = 3) + _r * _l = 4 }} _r }}",
        ul("t")
    );
    assert_eq!(dec(src), want);
}

#[test]
fn lex_errors_are_reported() {
    let err = decorate("3-1").unwrap_err();
    assert_eq!(err.code, "ambiguous-minus");
}
