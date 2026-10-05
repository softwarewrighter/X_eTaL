//! The calls in a text, as other tools see them (`xetal doc` draws
//! each call's expansion): where each stands, its sides, whether it is
//! a statement of its own; `u_se<` and a macro being defined are not
//! calls.

use xetal_expand::calls;
use xetal_lex::lex;

#[test]
fn calls_are_found_with_their_sides_and_place() {
    let src =
        "\"a:\" u_se< \"A\"\n\"c\" i_f< \"1; 2\"\nx := 1 + @ l_ine< @\nm:w_< := { a b -> a }\n";
    let found = calls(&lex(src).expect("lexes")).expect("calls");
    let shown: Vec<(&str, (bool, bool), bool)> = found
        .iter()
        .map(|c| (c.name.as_str(), c.texts, c.statement))
        .collect();
    assert_eq!(
        shown,
        [
            ("i_f<", (true, true), true),
            ("l_ine<", (false, false), false)
        ]
    );
    let first = &found[0];
    assert_eq!(
        &src[first.span.start..first.span.end],
        "\"c\" i_f< \"1; 2\""
    );
}

#[test]
fn a_call_with_a_missing_side_is_an_error() {
    let err = calls(&lex("i_f< \"x\"\n").expect("lexes")).unwrap_err();
    assert_eq!(err.code, "bad-macro-call");
}
