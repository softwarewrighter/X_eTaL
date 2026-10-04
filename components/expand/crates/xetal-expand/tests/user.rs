//! Macros of macro libraries (MC10, MC12), run here by a stand-in for
//! the evaluator: the text a macro gives replaces its call and is
//! expanded again, to the depth limit.

use xetal_base::Diagnostic;
use xetal_expand::{Macros, expand, expand_with};

/// `x:u_nless<` as X1 writes it, `x:l_oop<` calling itself forever,
/// `x:o_ops<` failing.
struct Control;

impl Macros for Control {
    fn run(&self, ns: &str, name: &str, left: &str, right: &str) -> Result<String, Diagnostic> {
        match (ns, name) {
            ("x", "u_nless<") => Ok(format!("{{ @ -> ({left}) ? @; {right}; @ }} @")),
            ("x", "l_oop<") => Ok(format!("\"{left}\" x:l_oop< \"{right}\"")),
            ("x", "t_wice<") => Ok(format!("\"{left}\" i_f< \"{right}; {right}\"")),
            _ => Err(Diagnostic::new("unknown-macro", "no such macro")),
        }
    }
}

#[test]
fn a_statement_call_is_replaced_by_the_text_it_gives() {
    let out = expand_with("\"n = 0\" x:u_nless< \"p_rint! n\"\n", &Control).unwrap();
    assert_eq!(out.text(), "{ @ -> (n = 0) ? @; p_rint! n; @ } @\n");
}

#[test]
fn inside_an_expression_the_text_is_parenthesized() {
    let out = expand_with("1 + \"c\" x:t_wice< \"2\"", &Control).unwrap();
    assert_eq!(out.text(), "1 + (({ @ -> (c) ? 2; 2 } @))");
}

#[test]
fn a_definition_is_not_a_call() {
    let src = "m:u_nless< := { c b -> c }\n";
    assert_eq!(expand_with(src, &Control).unwrap().text(), src);
}

#[test]
fn a_macro_calling_itself_stops_at_the_depth_limit() {
    let d = expand_with("\"a\" x:l_oop< \"b\"", &Control).unwrap_err();
    assert_eq!(d.code, "macro-depth");
    assert_eq!(d.span.map(|s| s.start), Some(0));
}

#[test]
fn a_failing_macro_is_reported_at_its_call() {
    let d = expand_with("y := 1\n\"a\" x:o_ops< \"b\"", &Control).unwrap_err();
    assert_eq!(d.code, "unknown-macro");
    let span = d.span.unwrap();
    assert_eq!(
        &"y := 1\n\"a\" x:o_ops< \"b\""[span.start..span.end],
        "x:o_ops<"
    );
}

#[test]
fn without_macro_libraries_a_namespaced_macro_is_unknown() {
    let d = expand("\"a\" x:u_nless< \"b\"").unwrap_err();
    assert_eq!(d.code, "unknown-macro");
}
