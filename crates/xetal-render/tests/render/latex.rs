//! Raw ASCII -> LaTeX math (one way, for post-processing).

use xetal_render::latex;

fn tex(src: &str) -> String {
    latex(src).unwrap_or_else(|e| panic!("{src:?}: {e:?}"))
}

#[test]
fn names() {
    assert_eq!(tex("square"), r"{\mathrm{square}}");
    assert_eq!(tex("t_12"), r"{\underline{\mathrm{t}}_{12}}");
    assert_eq!(tex("now_@"), r"{\underline{\mathrm{now}}_{@}}");
    assert_eq!(tex("m.f_"), r"{\mathrm{m}.\underline{\mathrm{f}}}");
    assert_eq!(tex("+^r_2"), r"{+^{\mathrm{r}}_{2}}");
    assert_eq!(tex("+^q"), r"{+^{\mathrm{q}}}");
    assert_eq!(tex("+_@"), r"{\underline{+}_{@}}");
}

#[test]
fn symbols_numbers_and_punctuation() {
    assert_eq!(tex("* |"), r"{\ast}\ {\mid}");
    assert_eq!(tex("3 -1"), r"{3}\ {-1}");
    assert_eq!(tex("3 - 1"), r"{3}\ {-}\ {1}");
    assert_eq!(
        tex("{ _l ; _r }"),
        r"{\{}\ {\_\mathrm{l}}\ {;}\ {\_\mathrm{r}}\ {\}}"
    );
    assert_eq!(tex("(@)[]"), r"{(}{@}{)}{[}{]}");
}

#[test]
fn whitespace_is_explicit() {
    assert_eq!(tex("a  b"), r"{\mathrm{a}}\ \ {\mathrm{b}}");
    assert_eq!(tex("a\nb"), "{\\mathrm{a}}\\\\\n{\\mathrm{b}}");
}
