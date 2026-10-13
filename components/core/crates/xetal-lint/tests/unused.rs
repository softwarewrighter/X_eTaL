//! `unused-binding`: a lambda parameter or local `:=` never used.

fn unused(src: &str) -> Vec<String> {
    let program = xetal_core::lower(src).expect("lowers");
    xetal_lint::lints(&program)
        .into_iter()
        .filter(|d| d.code == "unused-binding")
        .map(|d| d.message)
        .collect()
}

#[test]
fn an_unused_parameter_warns() {
    let found = unused("u:f_irst := { x y -> x }");
    assert_eq!(found.len(), 1);
    assert!(
        found[0].starts_with("parameter `y` is never used"),
        "{}",
        found[0]
    );
}

#[test]
fn an_unused_local_warns() {
    let found = unused("u:f_ := { x -> n := x + 1\nx }");
    assert_eq!(found.len(), 1);
    assert!(
        found[0].starts_with("`n` is bound but never used"),
        "{}",
        found[0]
    );
}

#[test]
fn used_names_pass_even_deep_inside() {
    assert!(unused("u:f_ := { x -> n := x + 1\n{ y -> y + n } 2 }").is_empty());
    assert!(unused("u:f_ := { x -> n! := 0\nn! := n! + x\nn! }").is_empty());
}

#[test]
fn a_use_of_a_shadowing_name_is_not_a_use() {
    let found = unused("u:f_ := { x -> { x -> x } 2 }");
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(found[0].starts_with("parameter `x`"));
}

#[test]
fn discard_implicit_and_hidden_names_are_exempt() {
    assert!(unused("u:f_ := { x -> _ := x + 1\n3 }").is_empty());
    assert!(unused("u:f_ := { _r + 1 }\n{ @ -> 3 } @").is_empty());
    assert!(unused("u:f_ := { t -> (a, b) := t\na + b }").is_empty());
}

#[test]
fn top_level_bindings_are_not_checked() {
    // A script's variables may be meant for the reader or an importer.
    assert!(unused("x := 3\nu:f_ := { _r }").is_empty());
}

/// Each finding's span text, and its fix's span text and replacement.
fn placed(src: &str) -> Vec<(String, Option<(String, String)>)> {
    let program = xetal_core::lower(src).expect("lowers");
    let text = |s: xetal_base::Span| src[s.start..s.end].to_string();
    xetal_lint::lints(&program)
        .into_iter()
        .filter(|d| d.code == "unused-binding")
        .map(|d| {
            let fix = d.fix.first().map(|f| (text(f.span), f.replacement.clone()));
            (text(d.span.expect("placed")), fix)
        })
        .collect()
}

#[test]
fn a_finding_is_at_the_name_and_a_local_is_fixed_to_a_discard() {
    let src = "u:f_ := { x ~y -> n := p_rint! x\n  x }";
    let want = [
        ("y".to_string(), None),
        ("n".to_string(), Some(("n".to_string(), "_".to_string()))),
    ];
    assert_eq!(placed(src), want);
}
