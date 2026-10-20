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

/// An unused parameter's fix writes `_u` in place of its name; the `~`
/// of a lazy one stays, so it becomes `~_u`.
#[test]
fn an_unused_parameter_offers_the_fix() {
    let fix = |src: &str| {
        let program = xetal_core::lower(src).expect("lowers");
        let found = xetal_lint::lints(&program);
        let d = found
            .iter()
            .find(|d| d.code == "unused-binding")
            .expect("found");
        let fix = &d.fix()[0];
        let mut out = src.to_string();
        out.replace_range(fix.span.start..fix.span.end, &fix.replacement);
        out
    };
    assert_eq!(fix("u:f_irst := { x y -> x }"), "u:f_irst := { x _u -> x }");
    assert_eq!(fix("u:k_eep := { a ~b -> a }"), "u:k_eep := { a ~_u -> a }");
}

/// `_u` says the parameter is unused (L8): nothing to report, and no
/// `# lint: allow` needed; lazy, repeated and alone alike.
#[test]
fn the_unused_parameter_is_exempt() {
    assert!(unused("u:f_irst := { x _u -> x }").is_empty());
    assert!(unused("u:k_eep := { a ~_u -> a }").is_empty());
    assert!(unused("u:z_ero := { _u _u -> 0 }").is_empty());
    assert!(unused("u:a_nswer := { _u -> 42 }").is_empty());
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
            let fix = d
                .fix()
                .first()
                .map(|f| (text(f.span), f.replacement.clone()));
            (text(d.span.expect("placed")), fix)
        })
        .collect()
}

#[test]
fn a_finding_is_at_the_name_and_a_local_is_fixed_to_a_discard() {
    let src = "u:f_ := { x ~y -> n := p_rint! x\n  x }";
    let want = [
        ("y".to_string(), Some(("y".to_string(), "_u".to_string()))),
        ("n".to_string(), Some(("n".to_string(), "_".to_string()))),
    ];
    assert_eq!(placed(src), want);
}
