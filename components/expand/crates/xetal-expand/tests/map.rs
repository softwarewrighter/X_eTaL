//! The map from the expanded text back to where it was written: code
//! from a macro's argument maps into the string it was written in, the
//! macro's own text to the whole call.

use xetal_base::Span;
use xetal_expand::expand;

#[test]
fn text_without_macros_maps_to_itself() {
    let e = expand("1 + 2").unwrap();
    assert_eq!(e.span(Span::new(4, 5)), Span::new(4, 5));
}

#[test]
fn argument_code_maps_into_its_string() {
    let src = "x := 1\n\"n = 0\" u_nless< \"p_rint! 100 / n\"";
    let e = expand(src).unwrap();
    let at = e.text().find("100").unwrap();
    let back = e.span(Span::new(at, at + 3));
    assert_eq!(&src[back.start..back.end], "100");
}

#[test]
fn the_macros_own_text_maps_to_the_call() {
    let src = "x := 1\n\"n = 0\" u_nless< \"p_rint! n\"";
    let e = expand(src).unwrap();
    let at = e.text().find("->").unwrap();
    let back = e.span(Span::new(at, at + 2));
    assert_eq!(&src[back.start..back.end], &src[7..]);
}

#[test]
fn text_after_a_call_keeps_its_place() {
    let src = "\"0\" u_nless< \"1\"\ny := 2";
    let e = expand(src).unwrap();
    let at = e.text().find("y").unwrap();
    assert_eq!(e.span(Span::new(at, at + 1)), Span::new(17, 18));
}

#[test]
fn an_error_inside_an_argument_points_into_it() {
    let src = "\"x\" u_nless< \"\\\"a\\\" x_yz< \\\"b\\\"\"";
    let d = expand(src).unwrap_err();
    let span = d.span.unwrap();
    assert_eq!(d.code, "unknown-macro");
    assert!(src[span.start..].starts_with("x_yz<"), "{span:?}");
}

#[test]
fn an_error_in_an_argument_written_without_escapes_points_into_it() {
    let src = "\"x\" u_nless< \"1 + 2 i_f< 3\"";
    let d = expand(src).unwrap_err();
    let span = d.span.unwrap();
    assert_eq!(&src[span.start..span.end], "i_f<");
}
