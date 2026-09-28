//! Raw ASCII -> LaTeX math (one way, for post-processing).

use xetal_render::latex;

fn tex(src: &str) -> String {
    latex(src).unwrap_or_else(|e| panic!("{src:?}: {e:?}"))
}

#[test]
fn names() {
    assert_eq!(tex("x"), r"{\mathrm{x}}");
    assert_eq!(tex("count!"), r"{\mathrm{count}!}");
    assert_eq!(tex("r_ev"), r"{\mathrm{\underline{r}ev}}");
    assert_eq!(
        tex("u:s_quare"),
        r"{\mathrm{u}{:}\mathrm{\underline{s}quare}}"
    );
    assert_eq!(tex("o_-_12"), r"{\mathrm{\underline{o}}-_{12}}");
    assert_eq!(tex("r_/"), r"{\mathrm{\underline{r}}/}");
}

#[test]
fn exponents_and_symbols() {
    assert_eq!(tex("x^0.5"), r"{\mathrm{x}}{^{0.5}}");
    assert_eq!(
        tex("* | & != <= >="),
        r"{\ast}\ {\mid}\ {\&}\ {\neq}\ {\leq}\ {\geq}"
    );
    assert_eq!(tex("3 -1"), r"{3}\ {-1}");
}

#[test]
fn punctuation_and_whitespace() {
    assert_eq!(
        tex("{ _l ; _r_ }"),
        r"{\{}\ {\_\mathrm{l}}\ {;}\ {\_\mathrm{r}\_}\ {\}}"
    );
    assert_eq!(tex("x := y"), r"{\mathrm{x}}\ {\mathrel{:=}}\ {\mathrm{y}}");
    assert_eq!(tex("a\nb"), "{\\mathrm{a}}\\\\\n{\\mathrm{b}}");
}
