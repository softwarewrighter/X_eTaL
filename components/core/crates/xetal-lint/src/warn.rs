//! Warnings: a parameter or local binding that shadows a built-in (L7),
//! and a library's bare top-level function (PN2: deprecated; `h:`).

use xetal_base::Diagnostic;
use xetal_catalog::find;
use xetal_ir::{Expr, Item, Kind, Param, Program};

use crate::naming::naming;
use crate::unused::unused;

/// Every lint's findings for a lowered program, at the lint's own code:
/// the [`warnings`] `xetal run` prints, then the style lints that only
/// `xetal lint` reports. Levels are not applied here (the driver does).
pub fn lints(program: &Program) -> Vec<Diagnostic> {
    let mut out = warnings(program);
    out.extend(naming(program));
    out.extend(unused(program));
    out.extend(xetal_bang::bangs(program));
    out
}

/// The warnings `xetal run` prints for a lowered program.
pub fn warnings(program: &Program) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    for item in &program.items {
        if let Item::Def { name, value } = item {
            deprecated(name, value, &mut out);
        }
        match item {
            Item::Def { value, .. }
            | Item::Let { value, .. }
            | Item::Set { value, .. }
            | Item::Eval(value) => {
                walk(value, &mut out);
            }
        }
    }
    out
}

/// A library's bare top-level function: the macro phase hides it in the
/// library's private namespace (`PA:f_`; its `h:` helpers are `HA:`).
fn deprecated(name: &str, value: &Expr, out: &mut Vec<Diagnostic>) {
    let Some((ns, key)) = name.split_once(':') else {
        return;
    };
    // A bare variable stays as it is (PN5); a function's name is underlined.
    let private = key.contains('_')
        && ns.len() > 1
        && ns.starts_with('P')
        && ns[1..].bytes().all(|b| b.is_ascii_uppercase());
    if private {
        let message = format!(
            "write h:{key}; bare top-level functions in a library are deprecated (xetal migrate FILE rewrites them)"
        );
        out.push(Diagnostic::warning("deprecated-private", message).with_span(value.span));
    }
}

fn walk(e: &Expr, out: &mut Vec<Diagnostic>) {
    let bound = match &e.kind {
        Kind::Lam {
            param: Param::Name(name),
            ..
        }
        | Kind::Let { name, .. } => Some(name),
        _ => None,
    };
    if let Some(name) = bound.filter(|name| find(name).is_some()) {
        let message = format!("`{name}` here shadows the built-in {name}");
        out.push(Diagnostic::warning("shadows-builtin", message).with_span(e.span));
    }
    e.children().into_iter().for_each(|child| walk(child, out));
}
