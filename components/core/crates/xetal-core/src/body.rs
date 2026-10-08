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

    /// A lambda's body, with its pattern parameters (TU3) taken apart
    /// first: each pattern binds the parts of its parameter's fresh name.
    pub(crate) fn patterned(
        &mut self,
        l: &xetal_syntax::Lambda,
        span: Span,
    ) -> Result<Expr, Diagnostic> {
        let mut pieces = Vec::new();
        if let xetal_syntax::Params::Named(ps) = &l.params {
            for p in ps
                .iter()
                .filter(|p| matches!(p.name, Some(Target::Tuple(_))))
            {
                let value = self.node(p.span, Kind::Var(format!("%p{}", p.span.start)));
                pieces.extend(self.pattern(p.name.as_ref().expect("filtered"), value, p.span)?);
            }
        }
        let mut body = self.body(&l.body, span)?;
        for piece in pieces.into_iter().rev() {
            body = self.fold(piece, Some(body));
        }
        Ok(body)
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

    /// A tuple pattern bound to `value` (TU3, TU10): the value under a
    /// fresh name, then each named part a binding of its projection,
    /// nested patterns in turn; `_` binds nothing (TU4).
    pub(crate) fn pattern(
        &mut self,
        target: &Target,
        value: Expr,
        span: Span,
    ) -> Result<Vec<Piece>, Diagnostic> {
        let Target::Tuple(parts) = target else {
            return Ok(Vec::new());
        };
        let tmp = self.fresh_name();
        self.bind(&tmp);
        let mut pieces = vec![Piece::Let {
            name: tmp.clone(),
            rec: false,
            set: false,
            value,
            span,
        }];
        for (index, part) in parts.iter().enumerate() {
            let tuple = Box::new(self.node(span, Kind::Var(tmp.clone())));
            let proj = self.node(
                span,
                Kind::Proj {
                    index,
                    size: parts.len(),
                    tuple,
                },
            );
            match part {
                Target::Wild => {}
                Target::Tuple(_) => pieces.extend(self.pattern(part, proj, span)?),
                name => {
                    let name = self.binding_name(name, span)?;
                    let set = name.ends_with('!') && self.is_bound(&name);
                    if !name.contains(':') {
                        self.bind(&name);
                    }
                    pieces.push(Piece::Let {
                        name,
                        rec: false,
                        set,
                        value: proj,
                        span,
                    });
                }
            }
        }
        Ok(pieces)
    }

    fn fold(&mut self, piece: Piece, rest: Option<Expr>) -> Expr {
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
            Stmt::Expr(e) => Ok(Piece::Expr(self.expr(e)?)),
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
                let name = self.binding_name(target, *span)?;
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

    /// Which names may be bound where: `u:` functions at the top level,
    /// plain variables anywhere, unqualified functions only in lambdas.
    fn binding_name(&self, target: &Target, span: Span) -> Result<String, Diagnostic> {
        let top = self.lambdas == 0;
        match target {
            Target::Var(v)
                if v.ns.is_none() || (!crate::train::fresh(&v.ns).is_empty() && !top) =>
            {
                Ok(format!(
                    "{}{}{}",
                    crate::train::fresh(&v.ns),
                    v.name,
                    if v.mutable { "!" } else { "" }
                ))
            }
            Target::Var(v) if (hidden(&v.ns) || v.ns.as_deref() == Some("h")) && top => {
                Ok(format!("{}:{}", v.ns.as_deref().unwrap_or(""), v.name))
            }
            Target::Var(_) => Err(err(
                "bad-binding",
                span,
                "a plain variable takes no namespace prefix; libraries export with l:",
            )),
            Target::Func(f) if hidden(&f.ns) && top => {
                Ok(format!("{}:{}", f.ns.as_deref().unwrap_or(""), f.spelled()))
            }
            // A program's own functions (u:) and its helpers (h:, PN1).
            Target::Func(f) if matches!(f.ns.as_deref(), Some("u" | "h")) && top => Ok(format!(
                "{}:{}",
                f.ns.as_deref().unwrap_or("u"),
                f.spelled()
            )),
            Target::Func(f)
                if (f.ns.is_none() || !crate::train::fresh(&f.ns).is_empty()) && !top =>
            {
                Ok(format!("{}{}", crate::train::fresh(&f.ns), f.spelled()))
            }
            Target::Func(f) if f.ns.is_none() => Err(err(
                "bad-binding",
                span,
                format!(
                    "a top-level function is the program's or a helper: write u:{0} (the program's) or h:{0} (a helper)",
                    f.spelled()
                ),
            )),
            Target::Func(_) | Target::Tuple(_) | Target::Wild => Err(err(
                "bad-binding",
                span,
                "only u: and h: functions can be defined here (other namespaces are imported and read-only)",
            )),
        }
    }
}

/// A namespace the macro phase gives a library (uppercase: `LA`, `PA`);
/// aliases are lowercase, so no program can write one.
fn hidden(ns: &Option<String>) -> bool {
    ns.as_deref()
        .is_some_and(|n| n.starts_with(|c: char| c.is_ascii_uppercase()))
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
