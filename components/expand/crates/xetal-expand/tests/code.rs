//! Code strings checked with the file (ST5, step 112): every code
//! string's inside is lexed, and one passed where its macro takes code
//! (`## takes: @ code`) is parsed, before the macro runs; an error is
//! placed inside the string, where it was written.

use xetal_expand::{Contract, MacroCall, Macros, expand_with};

/// `x:s_tage<` takes code on its right; `x:n_ote<` takes any text.
struct Staged;

impl Macros for Staged {
    fn run(&self, call: &MacroCall) -> Result<String, xetal_base::Diagnostic> {
        Ok(call.right.unwrap_or("0").to_string())
    }

    fn contract(&self, call: &MacroCall) -> Contract {
        match call.name {
            "s_tage<" => Contract {
                takes: Some((false, true)),
                code: (false, true),
                ..Contract::default()
            },
            _ => Contract::default(),
        }
    }
}

/// The error for `src`: its code and where it starts.
fn error(src: &str) -> (String, usize) {
    let d = expand_with(src, &Staged).unwrap_err();
    (d.code, d.span.map(|s| s.start).unwrap_or(usize::MAX))
}

#[test]
fn a_code_string_that_does_not_lex_is_an_error_inside_it() {
    // Anywhere, even with no macro: the inside is X_eTaL by declaration.
    let src = "s := c\"x ` y\"";
    assert_eq!(
        error(src),
        ("unexpected-char".into(), src.find('`').unwrap())
    );
    // The escaped form: the place counts the escapes as written.
    let src = "s := c\"p_rint! \\\"a\\\" ` 1\"";
    assert_eq!(
        error(src),
        ("unexpected-char".into(), src.find('`').unwrap())
    );
    let src = "s := c#\"a \"b\"#";
    assert_eq!(error(src).0, "bad-string");
    assert_eq!(error(src).1, src.find("\"b").unwrap());
    // A plain or raw string is text: nothing is lexed.
    assert!(expand_with("s := \"x ` y\"", &Staged).is_ok());
    assert!(expand_with("s := r\"x ` y\"", &Staged).is_ok());
}

#[test]
fn code_on_a_code_side_that_does_not_parse_is_an_error_inside_it() {
    let src = "@ x:s_tage< c\"n_eg (x\"";
    assert_eq!(error(src), ("unclosed".into(), src.find('(').unwrap()));
    let src = "@ x:s_tage< c\"x |> y\"";
    assert_eq!(
        error(src),
        ("unexpected-token".into(), src.find("|>").unwrap())
    );
    assert!(expand_with("@ x:s_tage< c\"3 + 4; n_eg 1\"", &Staged).is_ok());
}

#[test]
fn only_a_code_side_parses_its_code_string() {
    // A macro that takes any text (a pipeline's notation) is not parsed.
    assert!(expand_with("@ x:n_ote< c\"x |> y\"", &Staged).is_ok());
    // A plain string on a code side is the linter's (string-kind).
    assert!(expand_with("@ x:s_tage< \"n_eg 1\"", &Staged).is_ok());
}

#[test]
fn code_on_a_code_side_parses_as_a_lambdas_body() {
    assert!(expand_with("@ x:s_tage< c\"n < 3 ? 1; _r * 2\"", &Staged).is_ok());
}
