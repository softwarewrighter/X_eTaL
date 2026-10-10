//! Programs and statement sequences: top-level items, local bindings,
//! guards (G1, G2), rebinding and mutation (M1, M2), binding rules (N6,
//! R3).

use xetal_base::{Diagnostic, Span};
use xetal_syntax::{Expr as Surface, ExprKind, FunKind, Stmt, Target};

use crate::lower::{Lower, err};
use xetal_ir::{Expr, Item, Kind, Program};

/// A lowered statement, before the sequence is folded into Core.
pub(crate) enum Piece {
    Let {
        name: String,
        rec: bool,
        set: bool,
        value: Expr,
        span: Span,
    },
    Guard {
        cond: Expr,
        then: Expr,
        span: Span,
    },
    Expr(Expr),
}

impl Lower {
    pub(crate) fn program(
        &mut self,
        program: &xetal_syntax::Program,
    ) -> Result<Program, Diagnostic> {
        let mut items = Vec::new();
        for piece in self.pieces(&program.stmts)? {
            items.push(match piece {
                Piece::Let { name, value, .. } if name.contains(':') => Item::Def { name, value },
                Piece::Let {
                    name,
                    set: true,
                    value,
                    ..
                } => Item::Set { name, value },
                Piece::Let {
                    name, rec, value, ..
                } => Item::Let { name, rec, value },
                Piece::Expr(e) => Item::Eval(e),
                Piece::Guard { span, .. } => {
                    return Err(err(
                        "guard-outside-lambda",
                        span,
                        "a guard belongs inside a lambda",
                    ));
                }
            });
        }
        Ok(Program {
            items,
            notes: std::mem::take(&mut self.notes),
        })
    }

    /// A lambda body: statements folded into one Core expression.
    pub(crate) fn body(&mut self, stmts: &[Stmt], span: Span) -> Result<Expr, Diagnostic> {
        let pieces = self.pieces(stmts)?;
        let mut rest: Option<Expr> = None;
        for piece in pieces.into_iter().rev() {
            rest = Some(self.fold(piece, rest));
        }
        rest.ok_or_else(|| err("bad-lambda", span, "a lambda body needs a statement"))
    }

    /// The pieces of statements: one each, several for a pattern.
    fn pieces(&mut self, stmts: &[Stmt]) -> Result<Vec<Piece>, Diagnostic> {
        let mut pieces = Vec::new();
        for stmt in stmts {
            match stmt {
                Stmt::Bind {
                    target: target @ Target::Tuple(_),
                    value,
                    span,
                } => {
                    let value = self.expr(value)?;
                    pieces.extend(self.pattern(target, value, *span)?);
                }
                stmt => pieces.push(self.statement(stmt)?),
            }
        }
        Ok(pieces)
    }

    pub(crate) fn fold(&mut self, piece: Piece, rest: Option<Expr>) -> Expr {
        match piece {
            Piece::Expr(e) => match rest {
                None => e,
                Some(rest) => {
                    let span = e.span.join(rest.span);
                    let kind = Kind::Let {
                        name: "_".into(),
                        rec: false,
                        value: Box::new(e),
                        body: Box::new(rest),
                    };
                    self.node(span, kind)
                }
            },
            Piece::Guard { cond, then, span } => {
                let other = rest.unwrap_or_else(|| self.node(span, Kind::NoMatch));
                let kind = Kind::If {
                    cond: Box::new(cond),
                    then: Box::new(then),
                    other: Box::new(other),
                };
                self.node(span, kind)
            }
            Piece::Let {
                name,
                rec,
                set,
                value,
                span,
            } => {
                let body = rest.unwrap_or_else(|| self.node(span, Kind::Var(name.clone())));
                let (value, body) = (Box::new(value), Box::new(body));
                let kind = if set {
                    Kind::Set { name, value, body }
                } else {
                    Kind::Let {
                        name,
                        rec,
                        value,
                        body,
                    }
                };
                self.node(span, kind)
            }
        }
    }

    fn statement(&mut self, stmt: &Stmt) -> Result<Piece, Diagnostic> {
        match stmt {
            Stmt::Expr(e) => match self.erase(e)? {
                Some(piece) => Ok(piece),
                None => Ok(Piece::Expr(self.expr(e)?)),
            },
            Stmt::Guard { cond, result, span } => {
                let cond = self.expr(cond)?;
                let then = self.expr(result)?;
                Ok(Piece::Guard {
                    cond,
                    then,
                    span: *span,
                })
            }
            Stmt::Bind {
                target,
                value,
                span,
            } => {
                if matches!(target, Target::Wild) {
                    return self.discard(value, *span);
                }
                let name = self.binding_name(target, *span)?;
                self.once(target, &name, *span)?;
                let rec = is_function_literal(value);
                let set = name.ends_with('!') && self.is_bound(&name);
                if rec && !name.contains(':') {
                    self.bind(&name);
                }
                let value = self.expr(value)?;
                if !name.contains(':') {
                    self.bind(&name);
                }
                Ok(Piece::Let {
                    name,
                    rec,
                    set,
                    value,
                    span: *span,
                })
            }
        }
    }
}

impl Lower {
    /// `_ := expr` discards: it binds the name a body's expression
    /// statements already bind, never read, so it prints nothing (M3).
    fn discard(&mut self, value: &Surface, span: Span) -> Result<Piece, Diagnostic> {
        let value = self.expr(value)?;
        let (name, rec, set) = ("_".to_string(), false, false);
        Ok(Piece::Let {
            name,
            rec,
            set,
            value,
            span,
        })
    }
}

/// A binding whose value is a lambda or train is recursive.
fn is_function_literal(e: &Surface) -> bool {
    match &e.kind {
        ExprKind::Fn(f) | ExprKind::Quote(f) => {
            matches!(f.kind, FunKind::Lambda(_) | FunKind::Train(_))
        }
        _ => false,
    }
}
