//! The system macros i_f<, u_nless< and e_ach<: what each call expands
//! to, by where it stands (MC12, MC14-MC17).

use xetal_expand::expand;

fn text(src: &str) -> String {
    expand(src).unwrap().text().to_string()
}

fn code(src: &str) -> String {
    expand(src).unwrap_err().code
}

#[test]
fn a_program_without_macros_is_itself() {
    let src = "x := 3\n'+ r_/ 1 2 3 # u_nless< in a comment\n\"i_f<\"\n";
    assert_eq!(text(src), src);
}

#[test]
fn u_se_is_left_for_the_imports() {
    let src = "\"c:\" u_se< \"Combinators\"\nc:K_ 1 2\n";
    assert_eq!(text(src), src);
}

#[test]
fn unless_as_a_statement_is_a_guarded_niladic_call() {
    assert_eq!(
        text("\"n = 0\" u_nless< \"p_rint! 100 / n\"\n"),
        "{ @ -> (n = 0) ? @; p_rint! 100 / n; @ } @\n"
    );
}

#[test]
fn unless_inside_an_expression_is_parenthesized() {
    assert_eq!(
        text("x := \"n = 0\" u_nless< \"p_rint! n\"\n"),
        "x := ({ @ -> (n = 0) ? @; p_rint! n; @ } @)\n"
    );
}

#[test]
fn if_chooses_between_two_texts_at_run_time() {
    assert_eq!(
        text("\"x > 0\" i_f< \"1; -1\""),
        "{ @ -> (x > 0) ? 1; -1 } @"
    );
    assert_eq!(
        text("2 * \"x > 0\" i_f< \"1; -1\""),
        "2 * ({ @ -> (x > 0) ? 1; -1 } @)"
    );
}

#[test]
fn a_statement_inside_a_lambda_is_a_statement() {
    assert_eq!(
        text("u:f_ := { x -> \"x = 0\" u_nless< \"p_rint! x\"; x }"),
        "u:f_ := { x -> { @ -> (x = 0) ? @; p_rint! x; @ } @; x }"
    );
}

#[test]
fn each_writes_one_statement_per_word() {
    assert_eq!(
        text("\"a b\" e_ach< \"u:$w := 1\"\n"),
        "u:a := 1\nu:b := 1\n"
    );
    assert_eq!(
        text("\"sum max\" e_ach< \"u:s_$w := 1\""),
        "u:s_sum := 1\nu:s_max := 1"
    );
}

#[test]
fn macros_in_arguments_are_expanded_too() {
    assert_eq!(
        text("\"a b\" e_ach< \"\\\"$w > 0\\\" u_nless< \\\"p_rint! $w\\\"\""),
        "{ @ -> (a > 0) ? @; p_rint! a; @ } @\n{ @ -> (b > 0) ? @; p_rint! b; @ } @"
    );
}

#[test]
fn wrong_shapes_are_errors() {
    assert_eq!(code("x i_f< \"1; 2\""), "bad-macro-call");
    assert_eq!(code("\"x\" i_f< 3"), "bad-macro-call");
    assert_eq!(code("\"a\" \"x\" i_f< \"1; 2\""), "bad-macro-call");
    assert_eq!(code("\"x\" i_f< \"1; 2\" \"y\""), "bad-macro-call");
    assert_eq!(code("\"a\" i_f< \"b\" i_f< \"1; 2\""), "bad-macro-call");
    assert_eq!(code("i_f< \"1; 2\""), "bad-macro-call");
}

#[test]
fn wrong_arguments_are_errors() {
    assert_eq!(code("\"x\" i_f< \"1\""), "bad-macro-argument");
    assert_eq!(code("\"x\" i_f< \"1; 2; 3\""), "bad-macro-argument");
    assert_eq!(code("\"x\" i_f< \"; 2\""), "bad-macro-argument");
    assert_eq!(code("\" \" e_ach< \"u:$w := 1\""), "bad-macro-argument");
    assert_eq!(code("\"a b\" e_ach< \"u:x := 1\""), "bad-macro-argument");
}

#[test]
fn unknown_and_misplaced_macros_are_errors() {
    assert_eq!(code("\"a\" x_yz< \"b\""), "unknown-macro");
    assert_eq!(code("\"a\" c:x_yz< \"b\""), "unknown-macro");
    assert_eq!(code("1 + \"a b\" e_ach< \"$w\""), "misplaced-macro");
}

#[test]
fn nested_calls_expand_from_the_outside_in() {
    let mut src = String::from("p_rint! 1");
    for _ in 0..4 {
        let quoted = src.replace('\\', "\\\\").replace('"', "\\\"");
        src = format!("\"0\" u_nless< \"{quoted}\"");
    }
    let out = text(&src);
    assert_eq!(out.matches("{ @ -> (0) ? @;").count(), 4, "{out}");
    assert!(out.contains("p_rint! 1"), "{out}");
}
