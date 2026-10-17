//! What the lint knows of each expression, bottom up, reporting what
//! the rules prove on the way. Every node is visited once, so a
//! finding inside an operand or a lambda is found too.

use xetal_base::{Diagnostic, Span};
use xetal_catalog::find;
use xetal_ir::{Expr, Kind, Param};
use xetal_lex::Number;

use crate::env::Env;
use crate::ext::{Ext, Fact};
use crate::rules::{dyadic, inner, monadic};

pub(crate) fn infer(e: &Expr, env: &mut Env, out: &mut Vec<Diagnostic>) -> Fact {
    match &e.kind {
        Kind::Lit(Number::Int(n)) => Fact {
            shape: Some(Vec::new()),
            value: Some(vec![Ext::Num(*n)]),
        },
        Kind::Lit(_) => Fact::shaped(Some(Vec::new())),
        Kind::Array(items) => strand(items, env, out),
        Kind::Var(name) => env.lookup(name),
        Kind::App(f, x) => {
            let x = infer(x, env, out);
            infer(f, env, out);
            operated(f).map_or_else(Fact::default, |rule| monadic(rule, x))
        }
        Kind::App2 { f, left, right } => {
            let args = [infer(left, env, out), infer(right, env, out)];
            infer(f, env, out);
            applied(f, args, e.span, out)
        }
        Kind::Lam { param, body, .. } => {
            let name = match param {
                Param::Name(name) => Some(name.as_str()),
                Param::Unit => None,
            };
            scoped(name, Fact::default(), body, env, out);
            Fact::default()
        }
        Kind::Let {
            name, value, body, ..
        } => {
            let fact = infer(value, env, out);
            scoped(Some(name), fact, body, env, out)
        }
        _ => {
            e.children().into_iter().for_each(|c| {
                infer(c, env, out);
            });
            Fact::default()
        }
    }
}

/// `body` with `name` bound to `fact`.
fn scoped(
    name: Option<&str>,
    fact: Fact,
    body: &Expr,
    env: &mut Env,
    out: &mut Vec<Diagnostic>,
) -> Fact {
    let mark = env.mark();
    if let Some(name) = name {
        env.bind(name, fact);
    }
    let result = infer(body, env, out);
    env.reset(mark);
    result
}

/// A strand: a vector of its items, whose value is known when every
/// item is a known single number.
fn strand(items: &[Expr], env: &mut Env, out: &mut Vec<Diagnostic>) -> Fact {
    let facts: Vec<Fact> = items.iter().map(|i| infer(i, env, out)).collect();
    let one = |f: &Fact| match (&f.shape, &f.value) {
        (Some(s), Some(v)) if s.is_empty() && v.len() == 1 => Some(v[0].clone()),
        _ => None,
    };
    let value = facts.iter().map(one).collect();
    let shape = Some(vec![Ext::Num(items.len() as i64)]);
    Fact { shape, value }
}

/// The shape rule of built-in `f` when it takes `arity` arguments.
fn prim(f: &Expr, arity: usize) -> Option<&'static str> {
    let Kind::Prim(name) = &f.kind else {
        return None;
    };
    find(name)
        .filter(|b| b.arity == arity && !b.shape.is_empty())
        .map(|b| b.shape)
}

/// The rule of `f` applied to its data: a monadic built-in, or one
/// given its operand first (`'+ r_/ x` is `(r_/ '+) x`).
fn operated(f: &Expr) -> Option<&'static str> {
    match &f.kind {
        Kind::App(g, _) => prim(g, 2).filter(|r| *r == "reduce"),
        _ => prim(f, 1),
    }
}

/// `left f right`: a dyadic built-in, or the inner product with its
/// two operands given (`A '+ '* i_nner B`).
fn applied(f: &Expr, args: [Fact; 2], span: Span, out: &mut Vec<Diagnostic>) -> Fact {
    if let Some(rule) = prim(f, 2) {
        return dyadic(rule, args, span, out);
    }
    match &f.kind {
        Kind::App(g, _) => match &g.kind {
            Kind::App(h, _) if prim(h, 4) == Some("inner") => inner(args, span, out),
            _ => Fact::default(),
        },
        _ => Fact::default(),
    }
}
