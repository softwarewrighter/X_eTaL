//! The machine as the callback the higher-order built-ins use
//! (`components/hof`, B6): a call runs the machine above a barrier until
//! the function's value comes back to it. And functions under an axis
//! subscript (A6), which wait for their arguments as a built-in value.

use std::rc::Rc;

use xetal_array::Array;
use xetal_base::{Diagnostic, Span};
use xetal_core::Kind;
use xetal_value::{Caller, Prim, Slot, Value};

use crate::kont::{Control, Kont};
use crate::machine::{Machine, err};

impl<'a> Caller<'a> for Machine<'a, '_> {
    fn call(&mut self, f: &Value<'a>, x: Value<'a>, span: Span) -> Result<Value<'a>, Diagnostic> {
        self.stack.push(Kont::Barrier);
        let result = self.nested(f, x, span);
        if result.is_err() {
            self.unwind(|k| matches!(k, Kont::Barrier));
        }
        result
    }
}

impl<'a> Machine<'a, '_> {
    /// Run `f x` to its value, above the barrier just pushed.
    fn nested(&mut self, f: &Value<'a>, x: Value<'a>, span: Span) -> Result<Value<'a>, Diagnostic> {
        let mut control = self.apply(f.clone(), Slot::Value(x), span)?;
        loop {
            if let Control::Return(v) = &control
                && matches!(self.stack.last(), Some(Kont::Barrier))
            {
                self.stack.pop();
                return Ok(v.clone());
            }
            control = self
                .transition(control)?
                .ok_or_else(|| err("internal", span, "the program ended inside a call"))?;
        }
    }

    /// A built-in given one more argument: waiting for the rest, or called.
    pub(crate) fn prim(
        &mut self,
        p: &Rc<Prim<'a>>,
        v: Value<'a>,
        span: Span,
    ) -> Result<Control<'a>, Diagnostic> {
        let mut args = p.args.clone();
        args.push(v);
        if args.len() < p.arity {
            return Ok(Control::Return(Value::Prim(Rc::new(Prim {
                name: p.name,
                arity: p.arity,
                args,
            }))));
        }
        let result = match xetal_hof::call(p.name, &args, span, self) {
            Some(result) => result,
            None => xetal_prim::call(p.name, &args, span, self.out, &mut self.rng),
        };
        result.map(Control::Return)
    }

    /// `f_axes`: a built-in value `#axes` holding the axes and f, which
    /// takes f's arguments (`arity` from its type, else what f shows).
    pub(crate) fn axes(
        &mut self,
        axes: &[u8],
        arity: Option<usize>,
        f: Value<'a>,
        span: Span,
    ) -> Result<Value<'a>, Diagnostic> {
        let n = arity.unwrap_or_else(|| visible_arity(&f));
        if n == 0 {
            return Err(err(
                "not-a-function",
                span,
                format!("{f} is not a function"),
            ));
        }
        let digits = axes.iter().map(|d| Value::Int(i64::from(*d))).collect();
        Ok(Value::Prim(Rc::new(Prim {
            name: "#axes",
            arity: 2 + n,
            args: vec![Value::Array(Rc::new(Array::vector(digits))), f],
        })))
    }
}

/// The arguments a function value visibly takes: what a built-in still
/// lacks, or a lambda's parameters written together.
fn visible_arity(f: &Value<'_>) -> usize {
    match f {
        Value::Prim(p) => p.arity.saturating_sub(p.args.len()),
        Value::Closure(c) => {
            let (mut n, mut body) = (1, c.body);
            while let Kind::Lam { body: inner, .. } = &body.kind {
                (n, body) = (n + 1, inner);
            }
            n
        }
        _ => 0,
    }
}
