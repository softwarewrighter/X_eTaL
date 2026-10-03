//! Calling built-ins: a first-order one at once (`xetal-prim`), a
//! higher-order one as a kernel (`components/hof`, B6) whose every call
//! of its operand is a step of the machine (D50). And functions under
//! an axis subscript (A6), which wait for their arguments as a built-in
//! value.

use std::rc::Rc;

use xetal_array::Array;
use xetal_base::{Diagnostic, Span};
use xetal_core::Kind;
use xetal_kernel::{Kernel, Next};
use xetal_value::{Prim, Slot, Value};

use crate::kont::{Control, Kont};
use crate::machine::{Machine, err};

impl<'a> Machine<'a, '_> {
    /// A higher-order built-in's kernel goes on with the value of the
    /// call it asked for (`None` to start): it asks for another call,
    /// which runs above it as ordinary work, or gives its value.
    pub(crate) fn drive(
        &mut self,
        mut kernel: Kernel<'a, Value<'a>>,
        last: Option<Value<'a>>,
        span: Span,
    ) -> Result<Control<'a>, Diagnostic> {
        let next = kernel.resume(last).map_err(|d| match d.span {
            Some(_) => d,
            None => d.with_span(span),
        })?;
        match next {
            Next::Call(f, x) => {
                self.stack.push(Kont::Kernel { kernel, span });
                self.apply(f, Slot::Value(x), span)
            }
            Next::Done(v) => Ok(Control::Return(v)),
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
        match xetal_hof::call(p.name, &args, span) {
            Some(kernel) => self.drive(kernel?, None, span),
            None => {
                xetal_prim::call(p.name, &args, span, self.out, &mut self.rng).map(Control::Return)
            }
        }
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
