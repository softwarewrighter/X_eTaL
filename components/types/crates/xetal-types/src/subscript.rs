//! Forms whose type is built from their parts' types. A function under
//! an axis subscript (A6) has the function's type, recorded so
//! elaboration can tell the evaluator how many arguments the function
//! takes. An array's items share one type in the `Arr` class; a
//! tuple's parts keep their own types, and a
//! pattern takes one part of a tuple of a known size.

use xetal_base::Diagnostic;
use xetal_core::Expr;
use xetal_ty::Type;

use crate::infer::Infer;

impl Infer {
    pub(crate) fn subscripted(&mut self, e: &Expr, f: &Expr) -> Result<Type, Diagnostic> {
        let t = self.expr(f)?;
        self.rec.axes.push((e.id, t.clone()));
        Ok(t)
    }

    pub(crate) fn array(&mut self, items: &[Expr]) -> Result<Type, Diagnostic> {
        let elem = self
            .u
            .fresh_in(xetal_ty::Classes::named("Arr").unwrap_or_default());
        for item in items {
            let t = self.expr(item)?;
            self.u.unify(&elem, &t, item.span)?;
        }
        Ok(elem)
    }

    pub(crate) fn tuple(&mut self, parts: &[Expr]) -> Result<Type, Diagnostic> {
        let types = parts.iter().map(|p| self.expr(p));
        Ok(Type::Tuple(types.collect::<Result<_, _>>()?))
    }

    /// Part `index` of a tuple that must have `size` parts (TU3, TU5).
    pub(crate) fn part(
        &mut self,
        index: usize,
        size: usize,
        tuple: &Expr,
    ) -> Result<Type, Diagnostic> {
        let t = self.expr(tuple)?;
        let parts: Vec<Type> = (0..size).map(|_| self.u.fresh()).collect();
        self.u.unify(&Type::Tuple(parts.clone()), &t, tuple.span)?;
        Ok(parts[index].clone())
    }
}
