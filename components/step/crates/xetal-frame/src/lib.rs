//! The steppable evaluator's state as data (D50): what to do next
//! ([`Control`]) and the stack of pending work, each frame waiting for
//! the value of what runs above it ([`Kont`]). A crate of its own so
//! `xetal-step` keeps to its modules; it holds no logic beyond the
//! inbox of fed lines ([`Inbox`]).

mod inbox;

pub use inbox::{Inbox, Wants, value_of};

use std::cell::RefCell;
use std::rc::Rc;

use xetal_base::{Diagnostic, Span};
use xetal_core::Expr;
use xetal_value::{Env, Outcome, Prim, Slot, Value};

/// Evaluate an expression, or give a value to the frame on top.
pub enum Control<'a> {
    Eval(&'a Expr, Env<'a>),
    Return(Value<'a>),
}

/// A pending piece of work, waiting for the value of what runs above it.
pub enum Kont<'a> {
    /// The program's items from `next` on.
    Items { next: usize },
    /// A definition's value, to store under `name`.
    Def { name: &'a str },
    /// A top-level expression's value, to print (or keep).
    Show { span: Span },
    /// A binding's value; then `body`, or at the top level the next item.
    Bind {
        name: &'a str,
        env: Env<'a>,
        inner: Option<Env<'a>>,
        body: Option<&'a Expr>,
    },
    /// A `!` variable's new value; then `body`, or the next item.
    Assign {
        slot: Rc<RefCell<Slot<'a>>>,
        env: Env<'a>,
        body: Option<&'a Expr>,
    },
    /// A guard's condition; then one branch.
    Choose {
        then: &'a Expr,
        other: &'a Expr,
        env: Env<'a>,
        span: Span,
    },
    /// An array's items (or a tuple's parts, when `tuple`), right to
    /// left: `left` still to evaluate.
    Array {
        items: &'a [Expr],
        tuple: bool,
        left: usize,
        done: Vec<Value<'a>>,
        env: Env<'a>,
    },
    /// A tuple's value, for its part `index` (a pattern, TU10).
    Part { index: usize, span: Span },
    /// The function under an axis subscript.
    Axes {
        axes: &'a [u8],
        arity: Option<usize>,
        span: Span,
    },
    /// `f x`: the function's value, then its argument.
    Arg {
        x: &'a Expr,
        env: Env<'a>,
        span: Span,
    },
    /// `f x`: the argument's value, for `f`.
    Call { f: Value<'a>, span: Span },
    /// `x f y`: the function's value, then y, then x.
    Pair {
        l: &'a Expr,
        r: &'a Expr,
        env: Env<'a>,
        span: Span,
    },
    /// `x f y`: y's value; x next.
    PairRight {
        f: Value<'a>,
        l: &'a Expr,
        env: Env<'a>,
        span: Span,
    },
    /// `x f y`: x's value; then f x, then that applied to y.
    PairLeft {
        f: Value<'a>,
        right: Slot<'a>,
        span: Span,
    },
    /// A function's value, to apply to `arg`.
    ApplyTo { arg: Slot<'a>, span: Span },
    /// A lazy argument's value, stored in its slot (E1).
    Force {
        slot: Rc<RefCell<Slot<'a>>>,
        expr: &'a Expr,
        env: Env<'a>,
    },
    /// A built-in's last argument, forced.
    PrimArg { p: Rc<Prim<'a>>, span: Span },
    /// A niladic function's argument, which must be `@`.
    UnitBody {
        body: &'a Expr,
        env: Env<'a>,
        span: Span,
    },
    /// A function call in progress (the depth a runaway recursion grows).
    Called,
    /// A higher-order built-in waiting for the call it asked for.
    Kernel {
        kernel: xetal_kernel::Kernel<'a, Value<'a>>,
        span: Span,
    },
    /// `'body []T_RAP 'handler` (ER2): the body runs above; an error
    /// raised there unwinds to this frame, which calls the handler.
    Trap {
        body: Value<'a>,
        handler: Value<'a>,
        /// How many times the body has run.
        runs: usize,
        span: Span,
    },
    /// The handler of a trap running above; its outcome decides.
    Handling {
        body: Value<'a>,
        handler: Value<'a>,
        runs: usize,
        span: Span,
    },
    /// `'body []E_NSURE 'cleanup`: the body runs above; the cleanup runs
    /// after it, whatever happened.
    Ensure { cleanup: Value<'a>, span: Span },
    /// The cleanup running above; then the body's result goes on. With
    /// an error and an outcome `decided` (a warning's handler chose to
    /// unwind), the unwinding goes on to the trap with that outcome.
    Cleaning {
        result: Result<Value<'a>, Diagnostic>,
        decided: Option<Rc<Outcome<'a>>>,
    },
    /// A warning (`[]W_ARN`, ER3) raised at this point of the stack,
    /// which stays as it was; the handler of the trap at `trap_at`
    /// runs above. Continue gives `default` here; the other outcomes
    /// unwind to the trap.
    Continuing {
        error: Rc<Diagnostic>,
        default: Value<'a>,
        trap_at: usize,
    },
}

pub fn err(code: &str, span: Span, message: impl Into<String>) -> Diagnostic {
    Diagnostic::new(code, message).with_span(span)
}
