//! Algorithm W over Core programs: top-level items, late-bound module
//! definitions, top-level defaulting (T5).

use std::collections::HashMap;

use xetal_base::{Diagnostic, Span};
use xetal_core::{Expr, Item, Kind, Program};

use crate::ty::{Scheme, Type};
use crate::unify::Unifier;

pub(crate) enum Global {
    Defined(Scheme),
    /// Used before its definition: one monomorphic type until then.
    Pending(Type, Span),
}

pub(crate) struct Infer {
    pub u: Unifier,
    pub globals: HashMap<String, Global>,
    /// Lexical bindings, innermost last.
    pub env: Vec<(String, Scheme)>,
}

/// Infer a whole program; one line per item: `name : type` for
/// bindings and definitions, the type for expressions.
pub fn infer_program(program: &Program) -> Result<Vec<String>, Diagnostic> {
    let mut inf = Infer {
        u: Unifier::default(),
        globals: HashMap::new(),
        env: Vec::new(),
    };
    let mut lines = Vec::new();
    for item in &program.items {
        lines.push(inf.item(item)?);
    }
    for global in inf.globals.values() {
        if let Global::Pending(_, span) = global {
            return Err(
                Diagnostic::new("undefined-name", "this name is never defined").with_span(*span),
            );
        }
    }
    Ok(lines)
}

impl Infer {
    fn item(&mut self, item: &Item) -> Result<String, Diagnostic> {
        match item {
            Item::Def { name, value } => {
                let v = match self.globals.get(name) {
                    Some(Global::Pending(t, _)) => t.clone(),
                    Some(Global::Defined(_)) => {
                        return Err(Diagnostic::new(
                            "duplicate-definition",
                            format!("{name} is already defined in this file"),
                        )
                        .with_span(value.span));
                    }
                    None => self.u.fresh(),
                };
                self.globals
                    .insert(name.clone(), Global::Pending(v.clone(), value.span));
                let t = self.expr(value)?;
                self.u.unify(&v, &t, value.span)?;
                self.globals.remove(name);
                let scheme = self.close(&t, value, true)?;
                let line = format!("{name} : {scheme}");
                self.globals.insert(name.clone(), Global::Defined(scheme));
                Ok(line)
            }
            Item::Let { name, rec, value } => {
                let t = self.binding(name, *rec, value)?;
                let scheme = self.close(&t, value, true)?;
                let line = format!("{name} : {scheme}");
                self.env.push((name.clone(), scheme));
                Ok(line)
            }
            Item::Set { name, value } => {
                let t = self.set(name, value)?;
                Ok(format!("{name} : {}", self.u.defaulted(&t)))
            }
            Item::Eval(e) => {
                let t = self.expr(e)?;
                Ok(self.close(&t, e, true)?.to_string())
            }
        }
    }

    /// The type of a binding's value; a recursive binding sees itself.
    pub(crate) fn binding(
        &mut self,
        name: &str,
        rec: bool,
        value: &Expr,
    ) -> Result<Type, Diagnostic> {
        if !rec {
            return self.expr(value);
        }
        let v = self.u.fresh();
        self.env.push((name.to_string(), mono(v.clone())));
        let t = self.expr(value);
        self.env.pop();
        let t = t?;
        self.u.unify(&v, &t, value.span)?;
        Ok(t)
    }

    pub(crate) fn set(&mut self, name: &str, value: &Expr) -> Result<Type, Diagnostic> {
        let Some((_, scheme)) = self.env.iter().rev().find(|(n, _)| n == name) else {
            return Err(
                Diagnostic::new("undefined-name", format!("{name} is not defined"))
                    .with_span(value.span),
            );
        };
        let current = scheme.ty.clone();
        let t = self.expr(value)?;
        self.u.unify(&current, &t, value.span)?;
        Ok(t)
    }

    /// A lambda value is generalized; anything else stays monomorphic,
    /// and at the top level its numbers default now (Int, Bool; T5).
    pub(crate) fn close(
        &mut self,
        t: &Type,
        value: &Expr,
        top: bool,
    ) -> Result<Scheme, Diagnostic> {
        if matches!(value.kind, Kind::Lam { .. }) {
            let mut fixed: Vec<Type> = self.env.iter().map(|(_, s)| s.ty.clone()).collect();
            fixed.extend(self.globals.values().filter_map(|g| match g {
                Global::Pending(t, _) => Some(t.clone()),
                Global::Defined(_) => None,
            }));
            return Ok(self.u.generalize(t, &fixed));
        }
        if top {
            let defaulted = self.u.defaulted(t);
            self.u.unify(&defaulted, t, value.span)?;
        }
        Ok(mono(self.u.resolve(t)))
    }
}

pub(crate) fn mono(ty: Type) -> Scheme {
    Scheme {
        vars: Vec::new(),
        num: Vec::new(),
        truthy: Vec::new(),
        ty,
    }
}
