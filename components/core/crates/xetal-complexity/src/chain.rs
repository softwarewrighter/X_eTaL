//! A lambda literal as written: `{ a b -> body }` is two nested lambdas
//! in Core, read back here as one with two parameters.

use xetal_ir::{Expr, Kind, Param};

/// The named parameters of the lambdas nested directly in `e`, and the
/// body below them. A lazy or `@` lambda ends the chain: it is a
/// different kind of lambda (the first one's body is returned, with no
/// parameters, so its walk goes on).
pub(crate) fn chain(e: &Expr) -> (Vec<&str>, &Expr) {
    let mut params = Vec::new();
    let mut e = e;
    while let Kind::Lam { param, lazy, body } = &e.kind {
        match param {
            Param::Name(name) if !*lazy => params.push(name.as_str()),
            _ if params.is_empty() => return (params, body),
            _ => break,
        }
        e = body;
    }
    (params, e)
}

/// The function a lambda with these parameters and body stands for, by
/// name: `i_d` for `{ x -> x }`, `f` for `{ x -> f x }` and for
/// `{ a b -> a f b }`, where f is a built-in or a named function.
pub(crate) fn passed_to<'a>(params: &[&str], body: &'a Expr) -> Option<&'a str> {
    match (params, &body.kind) {
        ([x], Kind::Var(v)) if v == x => Some("i_d"),
        ([x], Kind::App(f, arg)) if is_var(arg, x) => named(f),
        ([a, b], Kind::App2 { f, left, right })
            if a != b && is_var(left, a) && is_var(right, b) =>
        {
            named(f)
        }
        _ => None,
    }
}

fn is_var(e: &Expr, name: &str) -> bool {
    matches!(&e.kind, Kind::Var(v) if v == name)
}

/// A built-in or a named function (not a local: quoting one is an error).
fn named(f: &Expr) -> Option<&str> {
    match &f.kind {
        Kind::Prim(name) | Kind::Global(name) => Some(name),
        _ => None,
    }
}
