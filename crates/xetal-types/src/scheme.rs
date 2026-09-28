//! Class constraints on type variables, and generalization,
//! instantiation and defaulting of types.

use std::collections::HashMap;

use xetal_base::{Diagnostic, Span};

use crate::ty::{Scheme, Type, TypeVar};

/// The classes a type variable must belong to.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct Classes {
    pub num: bool,
    pub truthy: bool,
}

impl Classes {
    pub(crate) fn admits(self, t: &Type) -> bool {
        (!self.num || matches!(t, Type::Int | Type::Float))
            && (!self.truthy || matches!(t, Type::Bool | Type::Int))
    }

    pub(crate) fn describe(self) -> &'static str {
        match (self.num, self.truthy) {
            (true, true) => "Int",
            (true, false) => "a number",
            _ => "Bool or Int",
        }
    }
}
use crate::unify::Unifier;

impl Unifier {
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
        let class = |v: &&crate::TypeVar| self.classes.get(v).copied().unwrap_or_default();
        let num = vars.iter().filter(|v| class(v).num).copied().collect();
        let truthy = vars.iter().filter(|v| class(v).truthy).copied().collect();
        Scheme {
            vars,
            num,
            truthy,
            ty,
        }
    }

    /// A fresh copy of a scheme's type.
    pub fn instantiate(&mut self, scheme: &Scheme) -> Type {
        let mut map = HashMap::new();
        for v in &scheme.vars {
            let classes = Classes {
                num: scheme.num.contains(v),
                truthy: scheme.truthy.contains(v),
            };
            map.insert(*v, self.fresh_in(classes));
        }
        scheme.ty.rename(&map)
    }

    /// Default every constrained variable created after `mark` (the
    /// variables of one top-level item, including those it shares with
    /// a monomorphic definition): the item is evaluated now (T5).
    pub(crate) fn default_since(&mut self, mark: u32, span: Span) -> Result<(), Diagnostic> {
        for v in mark + 1..=self.next {
            let t = Type::Var(TypeVar(v));
            let defaulted = self.defaulted(&t);
            self.unify(&defaulted, &t, span)?;
        }
        Ok(())
    }

    /// Resolve `ty` and default its constrained variables: a number (or
    /// a number used as a condition) is Int, a condition is Bool (T5).
    pub fn defaulted(&self, ty: &Type) -> Type {
        let ty = self.resolve(ty);
        let mut vars = Vec::new();
        ty.vars(&mut vars);
        let map = vars
            .into_iter()
            .filter_map(|v| {
                let c = self.classes.get(&v).copied().unwrap_or_default();
                let default = match (c.num, c.truthy) {
                    (true, _) => Type::Int,
                    (false, true) => Type::Bool,
                    _ => return None,
                };
                Some((v, default))
            })
            .collect();
        ty.rename(&map)
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
