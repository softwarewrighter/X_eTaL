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

use crate::machine::Machine;
use xetal_frame::err;
use xetal_frame::{Control, Kont, Wants};

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
        if let Some(wants) = wanted(p.name)
            && let Some(control) = self.typed_line(p, &args, span, wants)
        {
            return control;
        }
        if let Some(control) = self.trapping(p.name, &args, span) {
            return control;
        }
        let mut now = xetal_prim::Now {
            out: &mut *self.out,
            rng: &mut self.rng,
            span,
        };
        match xetal_hof::call(p.name, &args, span, &mut now) {
            Some(kernel) => self.drive(kernel?, None, span),
            None => {
                xetal_prim::call(p.name, &args, span, self.out, &mut self.rng).map(Control::Return)
            }
        }
    }

    /// `[]R_EAD`, `[]K_EY` or `[]E_VENT` with an input queue: the next
    /// line as what it wants, or, when none is, the call left pending
    /// (its argument handed back to it) and the run waiting. `None`
    /// without a queue (standard input, the terminal).
    fn typed_line(
        &mut self,
        p: &Rc<Prim<'a>>,
        args: &[Value<'a>],
        span: Span,
        wants: Wants,
    ) -> Option<Result<Control<'a>, Diagnostic>> {
        match self.inbox.next(wants)? {
            Some(value) => Some(value.map(Control::Return).map_err(|d| d.with_span(span))),
            None => {
                self.stack.push(Kont::PrimArg { p: p.clone(), span });
                args.last().cloned().map(Control::Return).map(Ok)
            }
        }
    }

    /// Read typed lines from a queue fed with [`Machine::feed`]: a run
    /// that needs a line before one is fed stops as Waiting.
    pub fn waiting_for_input(mut self) -> Self {
        self.inbox.open();
        self
    }

    /// A line typed (without its newline); a waiting run can go on.
    pub fn feed(&mut self, line: String) {
        self.inbox.feed(line);
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

/// What a built-in reading from the inbox wants, if it is one.
fn wanted(name: &str) -> Option<Wants> {
    match name {
        "[]R_EAD" => Some(Wants::Line),
        "[]K_EY" => Some(Wants::Key),
        "[]E_VENT" => Some(Wants::Event),
        _ => None,
    }
}
