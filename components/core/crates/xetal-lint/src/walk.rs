//! Walking Core: every binding site (a lambda's parameter or a local
//! `:=`) with the node that binds it (`Expr::children` walks the rest).

use xetal_ir::{Expr, Kind, Param};

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
