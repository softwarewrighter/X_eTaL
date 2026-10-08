//! Warnings: a parameter or local binding that shadows a built-in (L7),
//! and a library's bare top-level function (PN2: deprecated; `h:`).

use xetal_base::Diagnostic;
use xetal_catalog::find;
use xetal_ir::{Expr, Item, Kind, Param, Program};

/// The warnings for a lowered program.
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
    let shadow = |name: &str, out: &mut Vec<Diagnostic>| {
        if find(name).is_some() {
            let message = format!("`{name}` here shadows the built-in {name}");
            out.push(Diagnostic::warning("shadows-builtin", message).with_span(e.span));
        }
    };
    match &e.kind {
        Kind::Lam { param, body, .. } => {
            if let Param::Name(name) = param {
                shadow(name, out);
            }
            walk(body, out);
        }
        Kind::Let {
            name, value, body, ..
        } => {
            shadow(name, out);
            walk(value, out);
            walk(body, out);
        }
        Kind::Array(items) | Kind::Tuple(items) => items.iter().for_each(|x| walk(x, out)),
        Kind::Axes { f, .. } => walk(f, out),
        Kind::App(f, x) => {
            walk(f, out);
            walk(x, out);
        }
        Kind::App2 { f, left, right } => [f, left, right].iter().for_each(|x| walk(x, out)),
        Kind::Set { value, body, .. } => [value, body].iter().for_each(|x| walk(x, out)),
        Kind::If { cond, then, other } => [cond, then, other].iter().for_each(|x| walk(x, out)),
        _ => {}
    }
}
