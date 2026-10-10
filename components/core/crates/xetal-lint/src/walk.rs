//! Walking Core: an expression's children, and every binding site (a
//! lambda's parameter or a local `:=`) with the node that binds it.

use xetal_ir::{Expr, Kind, Param};

/// The direct subexpressions of `e`, in the order the warnings have always been reported.
pub(crate) fn children(e: &Expr) -> Vec<&Expr> {
    match &e.kind {
        Kind::Array(items) | Kind::Tuple(items) => items.iter().collect(),
        Kind::Axes { f, .. } | Kind::Proj { tuple: f, .. } => vec![f],
        Kind::Lam { body, .. } => vec![body],
        Kind::App(f, x) => vec![f, x],
        Kind::App2 { f, left, right } => vec![f, left, right],
        Kind::Let { value, body, .. } | Kind::Set { value, body, .. } => vec![value, body],
        Kind::If { cond, then, other } => vec![cond, then, other],
        _ => Vec::new(),
    }
}

/// Call `f` with each name `e` binds (at any depth) and its binding node.
pub(crate) fn binders(e: &Expr, f: &mut dyn FnMut(&str, &Expr)) {
    match &e.kind {
        Kind::Lam {
            param: Param::Name(name),
            ..
        }
        | Kind::Let { name, .. } => f(name, e),
        _ => {}
    }
    for child in children(e) {
        binders(child, f);
    }
}
