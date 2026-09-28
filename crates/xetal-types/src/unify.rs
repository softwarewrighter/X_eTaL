//! Unification over a substitution, with an occurs check and the `Num`
//! constraint on type variables.

use std::collections::{HashMap, HashSet};

use xetal_base::{Diagnostic, Span};

use crate::ty::{Scheme, Type, TypeVar};

#[derive(Debug, Default)]
pub struct Unifier {
    subst: HashMap<TypeVar, Type>,
    num: HashSet<TypeVar>,
    next: u32,
}

impl Unifier {
    pub fn fresh(&mut self) -> Type {
        self.next += 1;
        Type::Var(TypeVar(self.next))
    }

    /// A fresh variable that must be a number.
    pub fn fresh_num(&mut self) -> Type {
        let t = self.fresh();
        if let Type::Var(v) = t {
            self.num.insert(v);
        }
        t
    }

    /// Apply the substitution everywhere in `ty`.
    pub fn resolve(&self, ty: &Type) -> Type {
        match ty {
            Type::Var(v) => match self.subst.get(v) {
                Some(t) => self.resolve(t),
                None => ty.clone(),
            },
            Type::Fn(a, b) => Type::Fn(Box::new(self.resolve(a)), Box::new(self.resolve(b))),
            Type::Array(t) => Type::Array(Box::new(self.resolve(t))),
            other => other.clone(),
        }
    }

    pub fn unify(&mut self, expected: &Type, found: &Type, span: Span) -> Result<(), Diagnostic> {
        let (a, b) = (self.resolve(expected), self.resolve(found));
        match (&a, &b) {
            _ if a == b => Ok(()),
            (Type::Var(v), t) | (t, Type::Var(v)) => self.bind(*v, t, span),
            (Type::Fn(a1, a2), Type::Fn(b1, b2)) => {
                self.unify(a1, b1, span)?;
                self.unify(a2, b2, span)
            }
            (Type::Array(x), Type::Array(y)) => self.unify(x, y, span),
            _ => Err(
                Diagnostic::new("type-mismatch", format!("expected {a}, found {b}"))
                    .with_span(span),
            ),
        }
    }

    fn bind(&mut self, v: TypeVar, t: &Type, span: Span) -> Result<(), Diagnostic> {
        let mut vars = Vec::new();
        t.vars(&mut vars);
        if vars.contains(&v) {
            return Err(Diagnostic::new(
                "infinite-type",
                format!("a type would contain itself: {t}"),
            )
            .with_span(span));
        }
        if self.num.contains(&v) {
            match t {
                Type::Var(w) => {
                    self.num.insert(*w);
                }
                Type::Int | Type::Float => {}
                other => {
                    return Err(Diagnostic::new(
                        "type-mismatch",
                        format!("expected a number, found {other}"),
                    )
                    .with_span(span));
                }
            }
        }
        self.subst.insert(v, t.clone());
        Ok(())
    }

    /// Quantify the variables of `ty` not free in `env` (the types of
    /// the enclosing bindings).
    pub fn generalize(&self, ty: &Type, env: &[Type]) -> Scheme {
        let ty = self.resolve(ty);
        let mut fixed = Vec::new();
        for t in env {
            self.resolve(t).vars(&mut fixed);
        }
        let mut vars = Vec::new();
        ty.vars(&mut vars);
        vars.retain(|v| !fixed.contains(v));
        let num = vars
            .iter()
            .copied()
            .filter(|v| self.num.contains(v))
            .collect();
        Scheme { vars, num, ty }
    }

    /// A fresh copy of a scheme's type.
    pub fn instantiate(&mut self, scheme: &Scheme) -> Type {
        let mut map = HashMap::new();
        for v in &scheme.vars {
            let fresh = if scheme.num.contains(v) {
                self.fresh_num()
            } else {
                self.fresh()
            };
            map.insert(*v, fresh);
        }
        scheme.ty.rename(&map)
    }
}
