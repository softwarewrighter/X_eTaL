//! Class constraints on type variables, and generalization,
//! instantiation and defaulting of types.

use std::collections::HashMap;

use crate::ty::{Scheme, Type};

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

    /// A fresh copy of a scheme's type, and the fresh types standing for
    /// its `Num` variables, in the order of `scheme.num` (the number-type
    /// arguments a use of the scheme passes, T6).
    pub fn instantiate(&mut self, scheme: &Scheme) -> (Type, Vec<Type>) {
        let mut map = HashMap::new();
        for v in &scheme.vars {
            let classes = Classes {
                num: scheme.num.contains(v),
                truthy: scheme.truthy.contains(v),
            };
            map.insert(*v, self.fresh_in(classes));
        }
        let nums = scheme.num.iter().map(|v| map[v].clone()).collect();
        (scheme.ty.rename(&map), nums)
    }
}

/// A scheme with no quantified variables.
pub fn mono(ty: Type) -> Scheme {
    Scheme {
        vars: Vec::new(),
        num: Vec::new(),
        truthy: Vec::new(),
        ty,
    }
}
