//! An expansion laid out as a program (D159): long blocks one statement
//! per line, long expressions broken inside their parentheses, the
//! spelling, order and map of every token kept.

use xetal_base::Span;
use xetal_mapped::Mapped;
use xetal_reflow::{WIDTH, laid_out};

fn lay(src: &str) -> String {
    laid_out(&Mapped::new(src), 0..4).text().to_string()
}

/// The same program: equal canonical forms (spans differ, as they must).
fn same(a: &str, b: &str) {
    assert_eq!(
        xetal_fmt::canonical(a).unwrap(),
        xetal_fmt::canonical(b).unwrap()
    );
}

/// A block whose one line is longer than WIDTH.
fn long_block() -> String {
    let fill = (0..9)
        .map(|i| format!("v{i} := {i} + {i}"))
        .collect::<Vec<_>>()
        .join("; ");
    format!("{{ @ -> {fill}; v0 + v8 }} @")
}

#[test]
fn text_that_fits_is_left_alone() {
    let src = "{ @ -> a := 1; b := 2; a + b } @";
    assert_eq!(lay(src), src);
    assert!(src.len() <= WIDTH);
}

#[test]
fn a_long_block_goes_one_statement_per_line() {
    let src = long_block();
    assert!(src.len() > WIDTH);
    let out = lay(&src);
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines[0], "{ @ ->");
    assert_eq!(lines[1], "  v0 := 0 + 0");
    assert_eq!(lines[10], "  v0 + v8");
    assert_eq!(lines[11], "} @");
    same(&out, &src);
}

#[test]
fn nested_blocks_indent_further() {
    let inner = "{ x -> y := x * 2; z := y + 1; z }";
    let src = format!("{{ @ -> f_ := {inner}; {} f_ 3 }} @", "w := 1; ".repeat(8));
    let out = lay(&src);
    assert!(out.contains(&format!("\n  f_ := {inner}\n")), "{out}");
    same(&out, &src);
    let both_long = format!(
        "{{ @ -> f_ := {{ x -> {}; x }}; f_ 3 }} @",
        "y := x * 2; ".repeat(8)
    );
    let out = lay(&both_long);
    assert!(out.contains("\n  f_ := { x ->\n    y := x * 2\n"), "{out}");
    assert!(out.contains("\n    x\n  }\n"), "{out}");
    same(&out, &both_long);
}

#[test]
fn a_long_expression_breaks_at_a_space_inside_its_parentheses() {
    let terms = (0..12)
        .map(|i| format!("(a{i} * b{i})"))
        .collect::<Vec<_>>()
        .join(" + ");
    let src = format!("x := ({terms})");
    let out = lay(&src);
    assert!(out.lines().count() > 1, "{out}");
    assert!(
        out.lines().all(|l| l.chars().count() <= WIDTH + 12),
        "{out}"
    );
    same(&out, &src);
}

#[test]
fn nothing_is_broken_where_tokens_touch_or_in_a_string_or_brackets() {
    let pad = "p := 1; ".repeat(10);
    let src = format!("{{ @ -> {pad}q := 3 -1; r := \"a; b c\"; s := [f_ g_ h_] 2^3 }} @");
    let out = lay(&src);
    assert!(out.contains("q := 3 -1"), "{out}");
    assert!(out.contains("\"a; b c\""), "{out}");
    assert!(out.contains("[f_ g_ h_] 2^3"), "{out}");
    same(&out, &src);
}

#[test]
fn a_comment_in_a_gap_keeps_its_line() {
    let pad = "p := 1; ".repeat(10);
    let src = format!("{{ @ -> {pad}# the end\n  q := 2; q }} @");
    let out = lay(&src);
    assert!(out.contains("# the end\n"), "{out}");
    same(&out, &src);
}

#[test]
fn tokens_keep_their_map_and_the_breaks_map_to_the_call() {
    let src = long_block();
    let out = laid_out(&Mapped::new(&src), 40..44);
    let v8 = out.text().find("v0 + v8").unwrap() + 5;
    let back = out.span(Span::new(v8, v8 + 2));
    assert_eq!(&src[back.start..back.end], "v8");
    let nl = out.text().find('\n').unwrap();
    assert_eq!(out.span(Span::new(nl, nl + 1)), Span::new(40, 44));
}
