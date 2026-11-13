//! Workspace commands (APL's )FNS and )VARS, and a definition shown): a
//! line starting with `)` lists or shows what the session defined; it
//! is not code and is not kept.

use xetal_repl::{Reply, Session};

fn say(s: &mut Session, line: &str) -> (String, String) {
    match s.feed(line) {
        Reply::Done { out, err } => (out, err),
        Reply::More => panic!("{line:?} should complete"),
    }
}

fn workspace() -> Session {
    let mut s = Session::new("-e", 1);
    for line in [
        "x := 3",
        "count! := 0",
        "u:f_ := { a b -> a + b }",
        "## u:d_ouble n: twice n.",
        "u:d_ouble := { 2 * _r }",
        "3 u:f_ 4",
        "u:p_air := { a ->",
        "  (a, a) }",
    ] {
        if let Reply::Done { err, .. } = s.feed(line) {
            assert_eq!(err, "", "{line:?}");
        }
    }
    s
}

#[test]
fn fns_lists_the_functions_with_their_types() {
    let mut s = workspace();
    assert_eq!(
        say(&mut s, ")fns"),
        (
            "u:f_ : Num a => a -> a -> a\nu:d_ouble : Num a => a -> a\nu:p_air : Any a => a -> (a, a)\n"
                .into(),
            String::new()
        )
    );
}

#[test]
fn vars_lists_the_variables_with_their_types() {
    let mut s = workspace();
    assert_eq!(say(&mut s, ")vars").0, "x : Int\ncount! : Int\n");
}

#[test]
fn def_shows_the_definition_drawn_with_its_doc_and_type() {
    let mut s = workspace();
    let (out, err) = say(&mut s, ")def u:d_ouble");
    assert_eq!(err, "");
    let drawn =
        xetal_render::decorate("## u:d_ouble n: twice n.\nu:d_ouble := { 2 * _r }").unwrap();
    assert_eq!(out, format!("{drawn}\nu:d_ouble : Num a => a -> a\n"));
    let (out, _) = say(&mut s, ")def u:p_air");
    assert!(out.starts_with(&xetal_render::decorate("u:p_air := { a ->\n  (a, a) }").unwrap()));
}

#[test]
fn commands_are_not_code_and_are_not_kept() {
    let mut s = workspace();
    say(&mut s, ")fns");
    assert_eq!(say(&mut s, "u:d_ouble x").0, "6\n");
    let mut empty = Session::new("-e", 1);
    assert_eq!(say(&mut empty, ")fns"), (String::new(), String::new()));
    assert_eq!(say(&mut empty, "  )vars"), (String::new(), String::new()));
}

#[test]
fn unknown_commands_and_names_are_errors() {
    let mut s = workspace();
    let (out, err) = say(&mut s, ")clear");
    assert_eq!(out, "");
    assert!(err.contains("error[unknown-command]"), "{err}");
    assert!(err.contains(")fns, )vars, )def NAME"), "{err}");
    let (_, err) = say(&mut s, ")def u:n_one");
    assert!(err.contains("error[unknown-name]"), "{err}");
    let (_, err) = say(&mut s, ")def");
    assert!(err.contains("error[unknown-command]"), "{err}");
}
