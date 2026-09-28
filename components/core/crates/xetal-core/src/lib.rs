//! Core IR and desugaring of the surface AST (docs/design.md section 4).
//!
//! Every surface form lowers to a small calculus: literals, arrays,
//! variables, globals, built-ins, lambdas, application, let, guards.
//! Sugar forms lower to identical Core (normalization-equivalence tests).

mod body;
mod expr;
mod ir;
mod show;
mod train;

pub use ir::{Expr, Item, Kind, Param, Program};

use std::collections::HashSet;

use xetal_base::{Diagnostic, NodeId, Span};

/// Pipeline stage name used in diagnostics and by the CLI.
pub const STAGE: &str = "core";

/// Parse and lower a whole program to Core.
pub fn lower(src: &str) -> Result<Program, Diagnostic> {
    let program = xetal_syntax::parse(src)?;
    let mut lower = Lower {
        next_id: 0,
        fresh: 0,
        scopes: vec![HashSet::new()],
        lambdas: 0,
    };
    lower.program(&program)
}

pub(crate) fn err(code: &str, span: Span, message: impl Into<String>) -> Diagnostic {
    Diagnostic::new(code, message).with_span(span)
}

pub(crate) struct Lower {
    next_id: u32,
    fresh: u32,
    /// Names bound locally (parameters, local bindings), innermost last.
    scopes: Vec<HashSet<String>>,
    /// How many lambdas enclose the current point.
    lambdas: usize,
}

impl Lower {
    pub(crate) fn node(&mut self, span: Span, kind: Kind) -> Expr {
        self.next_id += 1;
        Expr {
            id: NodeId(self.next_id),
            span,
            kind,
        }
    }

    /// A fresh name no source program can spell (`%1`, `%2`, ...).
    pub(crate) fn fresh_name(&mut self) -> String {
        self.fresh += 1;
        format!("%{}", self.fresh)
    }

    pub(crate) fn is_bound(&self, name: &str) -> bool {
        self.scopes.iter().any(|s| s.contains(name))
    }

    pub(crate) fn bind(&mut self, name: &str) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string());
        }
    }
}
