//! A macro's contract checked around each call (MC31): the shape it
//! takes, before it runs; whether a copying macro used every name and
//! number of its arguments, after.

use xetal_base::Diagnostic;
use xetal_expand::{Contract, MacroCall, Macros, Uses, expand_with};

/// `x:t_wice<` takes `@ text` and copies; `x:f_irst<` writes only the
/// first word of its left (copying, so refused); `x:w_ord<` does the
/// same but says it parses; `x:a_ny<` declares nothing and writes both.
struct Lib;

impl Macros for Lib {
    fn run(&self, call: &MacroCall) -> Result<String, Diagnostic> {
        let (l, r) = (call.left.unwrap_or(""), call.right.unwrap_or(""));
        Ok(match call.name {
            "t_wice<" => format!("{{ @ -> t := 2; t * ({r}) }} @"),
            "f_irst<" | "w_ord<" => format!("{} + {r}", l.split(' ').next().unwrap_or("")),
            _ => format!("{l} {r}").trim().to_string(),
        })
    }

    fn contract(&self, call: &MacroCall) -> Contract {
        match call.name {
            "t_wice<" => Contract {
                takes: Some((false, true)),
                ..Contract::default()
            },
            "w_ord<" => Contract {
                uses: Uses::Parsed,
                ..Contract::default()
            },
            _ => Contract::default(),
        }
    }
}

fn text(src: &str) -> String {
    expand_with(src, &Lib).unwrap().text().to_string()
}

fn error(src: &str) -> Diagnostic {
    expand_with(src, &Lib).unwrap_err()
}

#[test]
fn a_call_of_the_declared_shape_runs() {
    assert_eq!(
        text("@ x:t_wice< \"n + 1\""),
        "{ @ -> t := 2; t * (n + 1) } @"
    );
}

#[test]
fn a_call_of_another_shape_is_refused_before_the_macro_runs() {
    let d = error("\"1\" x:t_wice< \"n + 1\"");
    assert_eq!(d.code, "bad-macro-call");
    assert_eq!(
        d.message,
        "x:t_wice< takes @ on its left and text on its right"
    );
    // Placed at the left argument, the side that differs.
    assert_eq!(d.span.map(|s| (s.start, s.end)), Some((0, 3)));
}

#[test]
fn a_macro_declaring_no_shape_takes_any() {
    assert_eq!(text("\"a\" x:a_ny< \"n\""), "a n");
    assert_eq!(text("@ x:a_ny< \"n\""), "n");
}

#[test]
fn a_copying_macro_that_drops_a_name_is_refused_at_the_argument() {
    let d = error("\"a b\" x:f_irst< \"1\"");
    assert_eq!(d.code, "unused-argument");
    assert!(
        d.message
            .starts_with("x:f_irst< did not use b of the text on its left")
    );
    assert_eq!(d.span.map(|s| (s.start, s.end)), Some((0, 5)));
}

#[test]
fn a_parsing_macro_answers_for_its_text() {
    assert_eq!(text("\"a b\" x:w_ord< \"1\""), "a + 1");
}

#[test]
fn a_copying_macro_that_writes_everything_passes() {
    assert_eq!(text("\"a\" x:f_irst< \"1\""), "a + 1");
}
