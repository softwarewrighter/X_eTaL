//! Applied groups (D-11): `(expr)_ x` is drawn with both parentheses
//! underlined and no `_`; the drawing reads back as typed.

use crate::UL;
use xetal_render::{code_pieces, decorate, latex, undecorate};

fn drawn(src: &str) -> String {
    decorate(src).unwrap_or_else(|e| panic!("{src:?}: {e:?}"))
}

#[test]
fn both_parentheses_underlined_and_no_apply_mark() {
    assert_eq!(
        drawn("(u:p_ow 3)_ 2"),
        format!("({UL}\u{1d58}p{UL}ow 3){UL} 2")
    );
}

#[test]
fn nested_groups_each_underlined() {
    assert_eq!(
        drawn("((a f_ b)_ c)_ d"),
        format!("({UL}({UL}a f{UL} b){UL} c){UL} d")
    );
}

#[test]
fn plain_parentheses_and_tuples_are_not_underlined() {
    assert_eq!(drawn("(1 + 2) * 3"), "(1 + 2) \u{d7} 3");
    assert_eq!(drawn("u:g_ (1, 2)"), format!("\u{1d58}g{UL} (1, 2)"));
}

#[test]
fn a_power_on_a_plain_group_beside_one_keeps_its_meaning() {
    assert_eq!(drawn("(f_)_ (x)^2"), format!("({UL}f{UL}){UL} (x)\u{b2}"));
}

#[test]
fn the_drawing_reads_back_as_typed() {
    for src in [
        "(u:p_ow 3)_ 2",
        "((a f_ b)_ c)_ d",
        "(f_)_ (x)^2",
        "x := (1 + 2) * 3",
        ")_ x",
        "[(a)_ b]",
    ] {
        assert_eq!(undecorate(&drawn(src)).unwrap(), src, "{src:?}");
    }
}

#[test]
fn inside_a_code_string() {
    let text: String = code_pieces("c\"(a f_ b)_ c\"")
        .unwrap()
        .into_iter()
        .map(|p| p.text)
        .collect();
    assert_eq!(text, format!("c\"({UL}a f{UL} b){UL} c\""));
    assert_eq!(undecorate(&text).unwrap(), "c\"(a f_ b)_ c\"");
}

#[test]
fn latex_underlines_both_parentheses() {
    assert_eq!(
        latex("(a)_ b").unwrap(),
        r"{\underline{(}}{\mathrm{a}}{\underline{)}}\ {\mathrm{b}}"
    );
}
