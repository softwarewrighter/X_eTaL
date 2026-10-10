//! The lowering state: fresh node ids and names, and local scopes.

use xetal_ir::{Expr, Kind, Program};

use std::collections::HashSet;

use xetal_base::{Diagnostic, NodeId, Span};
use xetal_syntax::Target;

/// Parse and lower a whole program to Core.
pub fn lower(src: &str) -> Result<Program, Diagnostic> {
    let program = xetal_syntax::parse(src)?;
    let mut lower = Lower {
        next_id: 0,
        fresh: 0,
        scopes: vec![HashSet::new()],
        lambdas: 0,
        src: src.to_string(),
        notes: Vec::new(),
        interactive: crate::bind::is_interactive(),
        erased: HashSet::new(),
    };
    lower.program(&program)
}

pub(crate) fn err(code: &str, span: Span, message: impl Into<String>) -> Diagnostic {
    Diagnostic::new(code, message).with_span(span)
}

pub(crate) struct Lower {
    pub(crate) next_id: u32,
    pub(crate) fresh: u32,
    /// Names bound locally (parameters, local bindings), innermost last.
    pub(crate) scopes: Vec<HashSet<String>>,
    /// How many lambdas enclose the current point.
    pub(crate) lambdas: usize,
    /// The source, for spelling train elements in notes.
    pub(crate) src: String,
    /// Notes for errors at particular spans (D47).
    pub(crate) notes: Vec<xetal_ir::SpanNote>,
    /// A session (REPL, notebook) is lowering: `[]E_X` is allowed (M4).
    pub(crate) interactive: bool,
    /// Names `[]E_X` unbound and not yet bound again.
    pub(crate) erased: HashSet<String>,
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

    /// A variable without `!` is bound once in its scope (M1): a file's
    /// top level, or a lambda body with its parameters. A namespaced
    /// variable (`h:limit`) is remembered by a mark no name can spell.
    pub(crate) fn once(
        &mut self,
        target: &Target,
        name: &str,
        span: Span,
    ) -> Result<(), Diagnostic> {
        if !matches!(target, Target::Var(_)) || name.ends_with('!') {
            return Ok(());
        }
        let key = if name.contains(':') {
            format!("={name}")
        } else {
            name.to_string()
        };
        let Some(scope) = self.scopes.last_mut() else {
            return Ok(());
        };
        let fresh = scope.insert(key);
        self.erased.remove(name);
        if !fresh {
            let message = format!(
                "{name} is already bound in this scope: a name is bound once; write {name}! for a variable that changes"
            );
            return Err(err("rebind", span, &message));
        }
        Ok(())
    }

    pub(crate) fn bind(&mut self, name: &str) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string());
        }
    }
}
