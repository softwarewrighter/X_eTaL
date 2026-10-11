//! Walking Core: an expression's direct subexpressions.

use crate::ir::{Expr, Kind};

impl Expr {
    /// The direct subexpressions, the function before its arguments.
    pub fn children(&self) -> Vec<&Expr> {
        match &self.kind {
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
}
