//! Accepted token forms (docs/design.md section 2).

use crate::common::{dump, kinds};

#[test]
fn plain_and_decorated_names() {
    assert_eq!(kinds("r"), ["Noun(r)"]);
    assert_eq!(kinds("square2"), ["Noun(square2)"]);
    assert_eq!(kinds("r_"), ["Func(r)"]);
    assert_eq!(kinds("t_2"), ["Func(t, axes=[2])"]);
    assert_eq!(kinds("t_12"), ["Func(t, axes=[1,2])"]);
    assert_eq!(kinds("now_@"), ["Func(now, niladic)"]);
    assert_eq!(kinds("m.f_"), ["Func(f, ns=m)"]);
    assert_eq!(kinds("m.t_12"), ["Func(t, ns=m, axes=[1,2])"]);
}

#[test]
fn superscript_derivations() {
    assert_eq!(kinds("+^r"), ["Func(+, deriv=r)"]);
    assert_eq!(kinds("+^r_2"), ["Func(+, deriv=r, axes=[2])"]);
    assert_eq!(kinds("+^reduce"), ["Func(+, deriv=reduce)"]);
    assert_eq!(kinds("f^s"), ["Func(f, deriv=s)"]);
    assert_eq!(kinds("m.f^e_@"), ["Func(f, ns=m, deriv=e, niladic)"]);
}

#[test]
fn symbol_functions() {
    assert_eq!(
        kinds("+ - * / = < > |"),
        [
            "Func(+)", "Func(-)", "Func(*)", "Func(/)", "Func(=)", "Func(<)", "Func(>)", "Func(|)"
        ]
    );
    assert_eq!(kinds("+_1"), ["Func(+, axes=[1])"]);
    assert_eq!(kinds("-_@"), ["Func(-, niladic)"]);
}

#[test]
fn lambda_args_unit_and_punctuation() {
    assert_eq!(
        kinds("{ _l ; _r } ( @ ) [ ]"),
        [
            "LBrace",
            "LamArg(l)",
            "Semi",
            "LamArg(r)",
            "RBrace",
            "LParen",
            "Unit",
            "RParen",
            "LBracket",
            "RBracket"
        ]
    );
    assert_eq!(kinds("now_ @"), ["Func(now)", "Unit"]);
}

#[test]
fn numbers_and_the_negative_literal_rule() {
    assert_eq!(kinds("42 2.5 0"), ["Num(42)", "Num(2.5)", "Num(0)"]);
    assert_eq!(kinds("-1 0 1"), ["Num(-1)", "Num(0)", "Num(1)"]);
    assert_eq!(kinds("3 - 1"), ["Num(3)", "Func(-)", "Num(1)"]);
    assert_eq!(kinds("3 -1"), ["Num(3)", "Num(-1)"]);
    assert_eq!(kinds("(-1)"), ["LParen", "Num(-1)", "RParen"]);
    assert_eq!(kinds("{-1}"), ["LBrace", "Num(-1)", "RBrace"]);
    assert_eq!(kinds("[-1]"), ["LBracket", "Num(-1)", "RBracket"]);
    assert_eq!(kinds("x;-2.5"), ["Noun(x)", "Semi", "Num(-2.5)"]);
    assert_eq!(kinds("- x"), ["Func(-)", "Noun(x)"]);
    assert_eq!(kinds("-x"), ["Func(-)", "Noun(x)"]);
    assert_eq!(kinds("-9223372036854775808"), ["Num(-9223372036854775808)"]);
}

#[test]
fn symbols_and_brackets_abut_names() {
    assert_eq!(kinds("1+2"), ["Num(1)", "Func(+)", "Num(2)"]);
    assert_eq!(kinds("f_(x)"), ["Func(f)", "LParen", "Noun(x)", "RParen"]);
    assert_eq!(kinds("x=y"), ["Noun(x)", "Func(=)", "Noun(y)"]);
}

#[test]
fn newlines_are_tokens_other_whitespace_is_not() {
    assert_eq!(
        kinds("a\n b\r\n\tc"),
        ["Noun(a)", "Newline", "Noun(b)", "Newline", "Noun(c)"]
    );
    assert_eq!(kinds("x\n-1"), ["Noun(x)", "Newline", "Num(-1)"]);
    assert!(kinds("  \t ").is_empty());
}

#[test]
fn dump_shows_byte_spans() {
    assert_eq!(dump("square_ 7"), ["0..7 Func(square)", "8..9 Num(7)"]);
}

#[test]
fn life_line_lexes() {
    let src = "life = { (+^r -1 0 1 t_12 _r) { (_l = 3) + _r * _l = 4 } _r }";
    let got = kinds(src);
    assert_eq!(got.len(), 26);
    assert_eq!(got[4], "Func(+, deriv=r)");
    assert_eq!(got[8], "Func(t, axes=[1,2])");
}
