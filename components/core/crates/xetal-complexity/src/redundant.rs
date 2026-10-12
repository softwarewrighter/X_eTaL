//! `redundant-lambda` over a program. A lambda that is a definition's
//! whole value (`u:n_ot := { x -> n_eg x }`) is the definition, with its
//! parameters named for a reader, so only operands and other lambdas
//! inside expressions are checked.

use xetal_base::Diagnostic;
use xetal_ir::{Expr, Item, Kind, Program};

use crate::chain::{chain, passed_to};

/// The redundant-lambda findings of every lambda in `program`.
pub fn redundant(program: &Program) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    for item in &program.items {
        let (Item::Def { value, .. }
        | Item::Let { value, .. }
        | Item::Set { value, .. }
        | Item::Eval(value)) = item;
        walk(value, !matches!(item, Item::Eval(_)), &mut out);
    }
    out
}

/// Check `e` and what is below it; `defined` when `e` is a binding's value.
fn walk(e: &Expr, defined: bool, out: &mut Vec<Diagnostic>) {
    match &e.kind {
        Kind::Lam { .. } => {
            let (params, body) = chain(e);
            if !defined {
                out.extend(finding(&params, body, e));
            }
            walk(body, false, out);
        }
        Kind::Let { value, body, .. } | Kind::Set { value, body, .. } => {
            walk(value, true, out);
            walk(body, false, out);
        }
        _ => e.children().into_iter().for_each(|c| walk(c, false, out)),
    }
}

/// The finding for lambda `at`, if it only passes its arguments on,
/// with its fix: the name in place of the lambda (its quote stays).
/// Power is never named: `'^` is an exponent (D-6), not a quote.
fn finding(params: &[&str], body: &Expr, at: &Expr) -> Option<Diagnostic> {
    let name = passed_to(params, body).filter(|n| *n != "^")?;
    let message = if name == "i_d" {
        "this lambda returns its argument unchanged: write `'i_d`".to_string()
    } else {
        format!("this lambda only passes its arguments to `{name}`: write `'{name}`")
    };
    let found = Diagnostic::warning("redundant-lambda", message).with_span(at.span);
    Some(found.with_fixes([(at.span, name.to_string())]))
}
