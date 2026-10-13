//! Walking Core: every binding site (a lambda's parameter or a local
//! `:=`) with the node that binds it (`Expr::children` walks the rest).

use xetal_base::Span;
use xetal_ir::{Expr, Kind, Param, Program};

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
    for child in e.children() {
        binders(child, f);
    }
}

/// Where the name bound by node `e` (from [`binders`]) is written, or
/// the node's own span when that is not recorded.
pub(crate) fn name_at(program: &Program, e: &Expr) -> Span {
    let at = match &e.kind {
        Kind::Lam { .. } => program.param_span(e.id),
        Kind::Let { value, .. } => program.name_span(value.id),
        _ => None,
    };
    at.unwrap_or(e.span)
}
