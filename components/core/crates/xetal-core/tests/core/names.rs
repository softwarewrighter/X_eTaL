//! Lowering records where each binding's name is written, for lints
//! that rename (a fix needs the name's own span, not the binding's).

use xetal_core::lower;
use xetal_ir::{Expr, Item, Kind, Program};

/// The text of the name recorded for `node`'s binding, if any.
fn named(src: &str, program: &Program, node: &Expr) -> Option<String> {
    let span = program.name_span(node.id)?;
    Some(src[span.start..span.end].to_string())
}

/// The text of the parameter recorded for the lambda `node`, if any.
fn param(src: &str, program: &Program, node: &Expr) -> Option<String> {
    let span = program.param_span(node.id)?;
    Some(src[span.start..span.end].to_string())
}

fn value(item: &Item) -> &Expr {
    match item {
        Item::Def { value, .. } | Item::Let { value, .. } | Item::Set { value, .. } => value,
        Item::Eval(e) => e,
    }
}

#[test]
fn a_top_level_binding_names_its_value() {
    let src = "count! := 0\ncount! := count! + 1\nu:s_ay := { v -> v }";
    let p = lower(src).expect("lowers");
    let names: Vec<_> = p.items.iter().map(|i| named(src, &p, value(i))).collect();
    let want = ["count!", "count!", "u:s_ay"].map(|s| Some(s.to_string()));
    assert_eq!(names, want);
    let second = p.name_span(value(&p.items[1]).id).unwrap();
    assert_eq!(second.start, 12, "the second binding's own name");
}

#[test]
fn a_parameter_names_its_lambda() {
    let src = "u:f_ := { a b -> a + b }";
    let p = lower(src).expect("lowers");
    let outer = value(&p.items[0]);
    assert_eq!(param(src, &p, outer).as_deref(), Some("a"));
    assert_eq!(named(src, &p, outer).as_deref(), Some("u:f_"));
    let Kind::Lam { body, .. } = &outer.kind else {
        panic!("a lambda: {outer:?}")
    };
    assert_eq!(param(src, &p, body).as_deref(), Some("b"));
}

#[test]
fn a_local_binding_names_its_value() {
    let src = "u:f_ := { v -> w := v + 1\n  w }";
    let p = lower(src).expect("lowers");
    let Kind::Lam { body, .. } = &value(&p.items[0]).kind else {
        panic!("a lambda")
    };
    let Kind::Let { value, .. } = &body.kind else {
        panic!("a let: {body:?}")
    };
    assert_eq!(named(src, &p, value).as_deref(), Some("w"));
}

#[test]
fn the_shorthand_and_a_discard_have_no_name() {
    let src = "u:f_ := { _r + 1 }\n_ := 2";
    let p = lower(src).expect("lowers");
    assert_eq!(param(src, &p, value(&p.items[0])), None);
    assert_eq!(named(src, &p, value(&p.items[1])), None);
}

#[test]
fn a_lazy_parameter_is_its_name_without_the_mark() {
    let src = "u:f_ := { c ~t -> c ? t\n  0 }";
    let p = lower(src).expect("lowers");
    let Kind::Lam { body, .. } = &value(&p.items[0]).kind else {
        panic!("a lambda")
    };
    let span = p.param_span(body.id).expect("recorded");
    assert_eq!((&src[span.start..span.end], span.start), ("t", 13));
}

/// The text `program.renamed(old, new)` gives `src`, edits applied.
fn renamed(src: &str, old: &str, new: &str) -> Option<String> {
    let p = lower(src).expect("lowers");
    let mut edits = p.renamed(old, new)?;
    edits.sort_by_key(|(span, _)| std::cmp::Reverse(span.start));
    let mut out = src.to_string();
    for (span, text) in edits {
        out.replace_range(span.start..span.end, &text);
    }
    Some(out)
}

#[test]
fn a_rename_reaches_every_binding_and_use() {
    let src = "Count! := 0\nCount! := Count! + 1\nu:f_ := { v -> v + Count! }\nu:f_ Count!";
    let want = "count! := 0\ncount! := count! + 1\nu:f_ := { v -> v + count! }\nu:f_ count!";
    assert_eq!(renamed(src, "Count!", "count!").as_deref(), Some(want));
}

#[test]
fn a_function_rename_reaches_its_calls_and_quotes() {
    let src = "u:s_ay := { v -> p_rint! v }\nu:s_ay 1\n'u:s_ay e_ach 2 3";
    let want = "u:s_ay! := { v -> p_rint! v }\nu:s_ay! 1\n'u:s_ay! e_ach 2 3";
    assert_eq!(renamed(src, "u:s_ay", "u:s_ay!").as_deref(), Some(want));
}

#[test]
fn no_rename_onto_a_name_in_use() {
    assert_eq!(
        renamed(
            "Count! := 0\ncount! := 1\nCount! + count!",
            "Count!",
            "count!"
        ),
        None
    );
}

#[test]
fn a_parameter_and_a_local_are_renamed_too() {
    let src = "u:f_ := { Acc! -> Acc! := Acc! + 1\n  Acc! }";
    let want = "u:f_ := { acc! -> acc! := acc! + 1\n  acc! }";
    assert_eq!(renamed(src, "Acc!", "acc!").as_deref(), Some(want));
}
