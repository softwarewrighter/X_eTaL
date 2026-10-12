//! Closing a binding's type (T5): a lambda is generalized, a value
//! stays monomorphic and, at the top level, has its numbers defaulted;
//! a top-level condition stays a condition, bare or inside a tuple or a
//! box, so it is an Int in arithmetic and a Bool in a guard, as written
//! inline (T1).

use std::collections::HashSet;
use xetal_base::Diagnostic;
use xetal_core::{Expr, Kind};

use xetal_ty::{Scheme, Type, TypeVar, mono};

use crate::infer::{Global, Infer};

impl Infer {
    /// A lambda value is generalized; anything else stays monomorphic,
    /// and at the top level its numbers default now (Int, Bool; T5).
    pub(crate) fn close(
        &mut self,
        t: &Type,
        value: &Expr,
        top: bool,
    ) -> Result<Scheme, Diagnostic> {
        if matches!(value.kind, Kind::Lam { .. }) {
            let scheme = self.u.generalize(t, &self.fixed());
            self.rec.generalized(value.id, &scheme);
            return Ok(scheme);
        }
        if top {
            self.u
                .default_since(self.mark, value.span, &self.rec.quantified)?;
            let defaulted = self.u.defaulted(t);
            self.u.unify(&defaulted, t, value.span)?;
        }
        Ok(mono(self.u.resolve(t)))
    }

    /// A top-level value whose type holds conditions (`Truthy`, not yet
    /// a Bool or an Int), bare or inside a tuple or a box: its other
    /// numbers default now (T5) and the conditions are generalized, so
    /// each use makes them a Bool or an Int as it needs (T1); its value
    /// is the same at run time.
    pub(crate) fn condition(
        &mut self,
        t: &Type,
        value: &Expr,
    ) -> Result<Option<Scheme>, Diagnostic> {
        let conditions: HashSet<TypeVar> = self.u.conditions(t).into_iter().collect();
        if conditions.is_empty() || matches!(value.kind, Kind::Lam { .. }) {
            return Ok(None);
        }
        let skip: HashSet<TypeVar> = self.rec.quantified.union(&conditions).copied().collect();
        self.u.default_since(self.mark, value.span, &skip)?;
        let scheme = self.u.generalize(t, &self.fixed());
        let only = scheme.vars.iter().all(|v| conditions.contains(v));
        Ok(only.then_some(scheme))
    }

    /// The types the environment fixes (not generalized over).
    fn fixed(&self) -> Vec<Type> {
        let mut fixed: Vec<Type> = self.env.iter().map(|(_, s, _)| s.ty.clone()).collect();
        fixed.extend(self.globals.values().filter_map(|g| match g {
            Global::Pending(t, _) | Global::Open { ty: t, .. } => Some(t.clone()),
            Global::Defined(_) => None,
        }));
        fixed
    }
}
