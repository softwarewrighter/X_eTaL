//! The labels for one operand of a failed application.

use xetal_base::Span;
use xetal_core::{Expr, Kind};
use xetal_lex::Number;
use xetal_ty::Type;

use crate::gives::{article, gives};

/// How a variable is bound: by a binding (the value's span, and what
/// gives its type when a built-in does), or as a lambda's parameter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Bound {
    Value(Span, Option<String>),
    Param,
}

/// What to say of operand `e`, whose type resolved to `ty`, where it is
/// written (and, for a variable, where it was bound); `bound` says how a
/// name is bound, `None` for one that is not local. Nothing is said of
/// a type still open (a variable), which explains nothing.
pub fn said(e: &Expr, ty: &Type, bound: &dyn Fn(&str) -> Option<Bound>) -> Vec<(Span, String)> {
    let mut vars = Vec::new();
    ty.vars(&mut vars);
    if !vars.is_empty() {
        return Vec::new();
    }
    let what = article(&ty.to_string());
    let one = |text: String| vec![(e.span, text)];
    match &e.kind {
        Kind::Lit(Number::Float(_)) => one("a Float literal".into()),
        Kind::Lit(Number::Int(_)) => one(format!("a whole-number literal, here {what}")),
        Kind::Str(_) => one("a string: a vector of Char".into()),
        Kind::Var(name) => variable(e.span, name, &what, bound(name)),
        Kind::Global(name) => one(format!("`{name}` is {what}")),
        _ => one(gives(e).unwrap_or_else(|| format!("this is {what}"))),
    }
}

/// A variable's labels: here, and at its binding.
fn variable(at: Span, name: &str, what: &str, bound: Option<Bound>) -> Vec<(Span, String)> {
    match bound {
        Some(Bound::Value(value, gives)) => {
            let why = gives.map_or(String::new(), |g| format!(": {g}"));
            vec![
                (at, format!("`{name}` is {what}")),
                (value, format!("`{name}` gets its type here{why}")),
            ]
        }
        Some(Bound::Param) => vec![(
            at,
            format!("`{name}`, the lambda's parameter, is {what} as it is used"),
        )],
        None => vec![(at, format!("`{name}` is {what}"))],
    }
}
