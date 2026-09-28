//! Core nodes the elaborator adds. They reuse the id and span of the
//! node they elaborate.

use xetal_core::{Expr, Kind, Param};
use xetal_lex::Number;
use xetal_ty::{Type, TypeVar};

/// Hidden parameters in scope: a quantified variable and its name.
pub(crate) type Scope = Vec<(TypeVar, String)>;

fn node(like: &Expr, kind: Kind) -> Expr {
    Expr {
        id: like.id,
        span: like.span,
        kind,
    }
}

/// The zero of number type `t`: a literal, or the hidden parameter that
/// carries it. A variable no binding quantifies defaults to Int (T5).
pub(crate) fn zero(like: &Expr, t: &Type, scope: &Scope) -> Expr {
    let kind = match t {
        Type::Float => Kind::Lit(Number::Float(0.0)),
        Type::Var(v) => match scope.iter().rev().find(|(w, _)| w == v) {
            Some((_, name)) => Kind::Var(name.clone()),
            None => Kind::Lit(Number::Int(0)),
        },
        _ => Kind::Lit(Number::Int(0)),
    };
    node(like, kind)
}

/// `{ name -> body }`, a hidden parameter.
pub(crate) fn lam(name: String, body: Expr) -> Expr {
    let kind = Kind::Lam {
        param: Param::Name(name),
        lazy: false,
        body: Box::new(body.clone()),
    };
    node(&body, kind)
}

/// `f x`.
pub(crate) fn app(f: Expr, x: Expr) -> Expr {
    node(&f.clone(), Kind::App(Box::new(f), Box::new(x)))
}

/// The integer literal `n` at number type `t`.
pub(crate) fn literal(like: &Expr, n: i64, t: &Type, scope: &Scope) -> Expr {
    match zero(like, t, scope).kind {
        Kind::Lit(Number::Float(_)) => node(like, Kind::Lit(Number::Float(n as f64))),
        Kind::Var(name) => {
            let kind = Kind::App2 {
                f: Box::new(node(like, Kind::Prim("+".into()))),
                left: Box::new(node(like, Kind::Lit(Number::Int(n)))),
                right: Box::new(node(like, Kind::Var(name))),
            };
            node(like, kind)
        }
        _ => node(like, Kind::Lit(Number::Int(n))),
    }
}
