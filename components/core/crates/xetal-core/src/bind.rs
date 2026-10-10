//! What a statement binds: a name, under the binding rules (N6, R3,
//! PN1-PN3: which namespaces a definition may name, at the top level or
//! in a body), or a tuple pattern (TU3-TU5), whose parts bind in turn;
//! and, in a session only, what `[]E_X "a"` unbinds (M4).

use std::cell::Cell;

use xetal_base::{Diagnostic, Span};
use xetal_syntax::{Expr as Surface, ExprKind, FunKind, Target};

use crate::body::Piece;
use crate::lower::{Lower, err};
use xetal_ir::{Expr, Kind};

impl Lower {
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

    /// Which names may be bound where: `u:` functions at the top level,
    /// plain variables anywhere, unqualified functions only in lambdas.
    pub(crate) fn binding_name(&self, target: &Target, span: Span) -> Result<String, Diagnostic> {
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

thread_local! {
    /// Whether this thread is lowering for an interactive session.
    static INTERACTIVE: Cell<bool> = const { Cell::new(false) };
}

/// Mark the lowering on this thread as a session's (the REPL, the
/// browser REPL, notebooks): only there may `[]E_X "a"` unbind a name.
pub fn interactive(on: bool) {
    INTERACTIVE.with(|i| i.set(on));
}

pub(crate) fn is_interactive() -> bool {
    INTERACTIVE.with(Cell::get)
}

impl Lower {
    /// `[]E_X "a b"` standing as a top-level statement of a session:
    /// the names are unbound (M4) and the statement is nothing. Anywhere
    /// else, or with a name that is not a literal, it is an error, so a
    /// program cannot use it to get around binding once (M1).
    pub(crate) fn erase(&mut self, e: &Surface) -> Result<Option<Piece>, Diagnostic> {
        let ExprKind::Monadic { f, arg } = &e.kind else {
            return Ok(None);
        };
        let FunKind::Name(n) = &f.kind else {
            return Ok(None);
        };
        if n.ns.as_deref() != Some(xetal_lex::SYSTEM) || n.spelled() != "E_X" {
            return Ok(None);
        }
        let ExprKind::Str(names) = &arg.kind else {
            return Err(err(
                "erase-not-literal",
                e.span,
                "[]E_X takes the names to unbind as a literal string: []E_X \"a\"",
            ));
        };
        if !self.interactive || self.scopes.len() > 1 {
            let message = "[]E_X unbinds a name in an interactive session only, as a statement of its own; in a program, bind a new name, or write a! for a value that changes";
            return Err(err("erase-outside-session", e.span, message));
        }
        for name in names.split_whitespace() {
            self.scopes[0].remove(name);
            self.scopes[0].remove(&format!("={name}"));
            self.erased.insert(name.to_string());
        }
        let value = self.node(e.span, Kind::Unit);
        let (name, rec, set) = ("_".to_string(), false, false);
        Ok(Some(Piece::Let {
            name,
            rec,
            set,
            value,
            span: e.span,
        }))
    }
}
