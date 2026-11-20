//! Defaulting (T5): constrained variables left open at the top level
//! become Int (a number) or Bool (a condition).

use std::collections::HashSet;

use xetal_base::{Diagnostic, Span};

use crate::ty::{Type, TypeVar};
use crate::unify::Unifier;

impl Unifier {
    /// The last variable made so far; pass it to [`Unifier::default_since`].
    pub fn mark(&self) -> u32 {
        self.next
    }

    /// Default every constrained variable created after `mark` (the
    /// variables of one top-level item, including those it shares with
    /// a monomorphic definition): the item is evaluated now (T5).
    /// Variables quantified by a scheme (`skip`) stay open.
    pub fn default_since(
        &mut self,
        mark: u32,
        span: Span,
        skip: &HashSet<TypeVar>,
    ) -> Result<(), Diagnostic> {
        let mut open = Vec::new();
        for v in mark + 1..=self.next {
            self.resolve(&Type::Var(TypeVar(v))).vars(&mut open);
        }
        open.retain(|v| !skip.contains(v));
        for v in open {
            let t = Type::Var(v);
            let defaulted = self.defaulted(&t);
            self.unify(&defaulted, &t, span)?;
        }
        Ok(())
    }

    /// Settle a function's result (T10): a variable that appears only in
    /// the final result of `ty`, not in an argument, and is both a
    /// number and a condition (a comparison used as a number) is Int,
    /// as Haskell defaults a numeric type.
    pub fn settle_result(&mut self, ty: &Type, span: Span) -> Result<(), Diagnostic> {
        let (mut args, mut result) = (Vec::new(), Vec::new());
        let mut t = self.resolve(ty);
        while let Type::Fn(a, b) = t {
            a.vars(&mut args);
            t = *b;
        }
        t.vars(&mut result);
        for v in result.into_iter().filter(|v| !args.contains(v)) {
            let c = self.classes.get(&v).copied().unwrap_or_default();
            if c.names().eq(["Num", "Truthy"]) || c.names().eq(["Truthy", "Num"]) {
                self.unify(&Type::Int, &Type::Var(v), span)?;
            }
        }
        Ok(())
    }

    /// The variables of `ty` that are conditions only (`Truthy`, not
    /// `Num`): a top-level value keeps them open (T1).
    pub fn conditions(&self, ty: &Type) -> Vec<TypeVar> {
        let mut vars = Vec::new();
        self.resolve(ty).vars(&mut vars);
        vars.retain(|v| {
            let c = self.classes.get(v).copied().unwrap_or_default();
            c.names().eq(["Truthy"])
        });
        vars
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
                let names: Vec<&str> = c.names().collect();
                let default = match (names.contains(&"Num"), names.contains(&"Truthy")) {
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
