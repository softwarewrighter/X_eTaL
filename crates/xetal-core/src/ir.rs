//! The Core IR: the small calculus every surface form desugars into.
//! Every node carries a NodeId and the span of the surface construct it
//! came from (for traces and the explainer).

use std::collections::HashSet;

use xetal_base::{NodeId, Span};
use xetal_lex::Number;

/// A lowered program: top-level items in order.
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub items: Vec<Item>,
}

impl Program {
    /// Turn the integer literals with the given ids into Float literals
    /// (numeric elaboration after type inference, T5).
    pub fn float_literals(&mut self, ids: &HashSet<NodeId>) {
        for item in &mut self.items {
            match item {
                Item::Def { value, .. } | Item::Let { value, .. } | Item::Set { value, .. } => {
                    float_literals(value, ids);
                }
                Item::Eval(e) => float_literals(e, ids),
            }
        }
    }
}

fn float_literals(e: &mut Expr, ids: &HashSet<NodeId>) {
    let children: Vec<&mut Expr> = match &mut e.kind {
        Kind::Lit(Number::Int(n)) if ids.contains(&e.id) => {
            e.kind = Kind::Lit(Number::Float(*n as f64));
            Vec::new()
        }
        Kind::Array(items) => items.iter_mut().collect(),
        Kind::Axes { f: x, .. } | Kind::Lam { body: x, .. } => vec![x],
        Kind::App(f, x) => vec![f, x],
        Kind::App2 { f, left, right } => vec![f, left, right],
        Kind::Let { value, body, .. } | Kind::Set { value, body, .. } => vec![value, body],
        Kind::If { cond, then, other } => vec![cond, then, other],
        _ => Vec::new(),
    };
    for child in children {
        float_literals(child, ids);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    /// A module-level definition (`u:f_ := ...`): defined once per file,
    /// late-bound so definitions may refer to each other.
    Def { name: String, value: Expr },
    /// A top-level variable binding; later items see it (lexical).
    Let {
        name: String,
        rec: bool,
        value: Expr,
    },
    /// Reassigning a mutable `!` variable in place (M2).
    Set { name: String, value: Expr },
    /// An expression whose value is printed.
    Eval(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub id: NodeId,
    pub span: Span,
    pub kind: Kind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    Lit(Number),
    Str(String),
    Unit,
    /// A strand: an array of the element values.
    Array(Vec<Expr>),
    /// A local or top-level variable (lexical).
    Var(String),
    /// A namespace-qualified name (`u:s_quare`, `m:pi`).
    Global(String),
    /// A built-in function by its spelling (`+`, `r_/`, `o_-`).
    Prim(String),
    /// A function specialized to axes (A6): `f_12`.
    Axes {
        axes: Vec<u8>,
        f: Box<Expr>,
    },
    Lam {
        param: Param,
        lazy: bool,
        body: Box<Expr>,
    },
    /// Monadic application `f x`.
    App(Box<Expr>, Box<Expr>),
    /// Dyadic application `x f y`: means `App(App(f, x), y)`, evaluated
    /// function first, then the right argument, then the left (E4).
    App2 {
        f: Box<Expr>,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Let {
        name: String,
        rec: bool,
        value: Box<Expr>,
        body: Box<Expr>,
    },
    Set {
        name: String,
        value: Box<Expr>,
        body: Box<Expr>,
    },
    /// A guard: `cond ? then`, otherwise `other`.
    If {
        cond: Box<Expr>,
        then: Box<Expr>,
        other: Box<Expr>,
    },
    /// No guard matched and no statement followed (G2): a runtime error.
    NoMatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Param {
    Name(String),
    /// `{ @ -> ... }`: the argument must be Unit.
    Unit,
}
