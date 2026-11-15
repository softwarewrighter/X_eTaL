//! A machine's state as one value (D50), so a debugger can copy it at a
//! checkpoint and put it back to travel in time (Saga 43). Values are
//! immutable and shared, so a copy is a copy of pointers; what is not
//! (a `!` variable's slot, written in place) is journaled: with the
//! journal on, every slot write first records the slot's old value.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use xetal_arith::Rng;
use xetal_value::{Env, Slot, Value};

use crate::{Control, Inbox, Kont};

/// Slot writes, each with the value the slot had before it.
pub type Journal<'a> = Vec<(Rc<RefCell<Slot<'a>>>, Slot<'a>)>;

/// What a run is between two transitions.
pub struct State<'a> {
    pub globals: HashMap<String, Value<'a>>,
    /// The top level's bindings so far.
    pub env: Env<'a>,
    pub stack: Vec<Kont<'a>>,
    pub control: Option<Control<'a>>,
    pub rng: Rng,
    /// Lines fed for `[]R_EAD`, `[]K_EY` and `[]E_VENT`, and what the
    /// run waits for; without a queue those read standard input.
    pub inbox: Inbox,
    /// When on, every slot write's old value, in order.
    pub journal: Option<Journal<'a>>,
}

impl<'a> State<'a> {
    /// A copy to come back to; `None` while a higher-order built-in is
    /// part way through (its progress is a closure, which cannot be
    /// copied). The copy keeps no journal.
    pub fn snapshot(&self) -> Option<State<'a>> {
        let stack: Option<Vec<Kont<'a>>> = self.stack.iter().map(copy).collect();
        Some(State {
            globals: self.globals.clone(),
            env: self.env.clone(),
            stack: stack?,
            control: self.control.clone(),
            rng: self.rng.clone(),
            inbox: self.inbox.clone(),
            journal: None,
        })
    }

    /// Journal `slot`'s value before it is written (when the journal is
    /// on).
    pub fn record(&mut self, slot: &Rc<RefCell<Slot<'a>>>) {
        if let Some(journal) = self.journal.as_mut() {
            journal.push((slot.clone(), slot.borrow().clone()));
        }
    }
}

/// A frame copied; `None` for a higher-order built-in in progress.
fn copy<'a>(k: &Kont<'a>) -> Option<Kont<'a>> {
    use Kont as K;
    Some(match k {
        K::Items { next } => K::Items { next: *next },
        K::Def { name } => K::Def { name },
        K::Show { span } => K::Show { span: *span },
        K::Bind {
            name,
            env,
            inner,
            body,
        } => K::Bind {
            name,
            env: env.clone(),
            inner: inner.clone(),
            body: *body,
        },
        K::Assign { slot, env, body } => K::Assign {
            slot: slot.clone(),
            env: env.clone(),
            body: *body,
        },
        K::Choose {
            then,
            other,
            env,
            span,
        } => K::Choose {
            then,
            other,
            env: env.clone(),
            span: *span,
        },
        other => return values(other),
    })
}

/// The frames of arrays, parts, axes and arguments copied.
fn values<'a>(k: &Kont<'a>) -> Option<Kont<'a>> {
    use Kont as K;
    Some(match k {
        K::Array {
            items,
            tuple,
            left,
            done,
            env,
        } => K::Array {
            items,
            tuple: *tuple,
            left: *left,
            done: done.clone(),
            env: env.clone(),
        },
        K::Part { index, span } => K::Part {
            index: *index,
            span: *span,
        },
        K::Axes { axes, arity, span } => K::Axes {
            axes,
            arity: *arity,
            span: *span,
        },
        K::Arg { x, env, span } => K::Arg {
            x,
            env: env.clone(),
            span: *span,
        },
        other => return calls(other),
    })
}

/// The frames of calls copied.
fn calls<'a>(k: &Kont<'a>) -> Option<Kont<'a>> {
    use Kont as K;
    Some(match k {
        K::Call { f, span } => K::Call {
            f: f.clone(),
            span: *span,
        },
        K::Pair { l, r, env, span } => K::Pair {
            l,
            r,
            env: env.clone(),
            span: *span,
        },
        K::PairRight { f, l, env, span } => K::PairRight {
            f: f.clone(),
            l,
            env: env.clone(),
            span: *span,
        },
        K::PairLeft { f, right, span } => K::PairLeft {
            f: f.clone(),
            right: right.clone(),
            span: *span,
        },
        K::ApplyTo { arg, span } => K::ApplyTo {
            arg: arg.clone(),
            span: *span,
        },
        K::Force { slot, expr, env } => K::Force {
            slot: slot.clone(),
            expr,
            env: env.clone(),
        },
        K::PrimArg { p, span } => K::PrimArg {
            p: p.clone(),
            span: *span,
        },
        K::UnitBody { body, env, span } => K::UnitBody {
            body,
            env: env.clone(),
            span: *span,
        },
        K::Called { span } => K::Called { span: *span },
        K::Kernel { .. } => return None,
        other => return guards(other),
    })
}

/// The frames of traps, cleanups and warnings copied.
fn guards<'a>(k: &Kont<'a>) -> Option<Kont<'a>> {
    use Kont as K;
    Some(match k {
        K::Trap {
            body,
            handler,
            runs,
            span,
        } => K::Trap {
            body: body.clone(),
            handler: handler.clone(),
            runs: *runs,
            span: *span,
        },
        K::Handling {
            body,
            handler,
            runs,
            span,
        } => K::Handling {
            body: body.clone(),
            handler: handler.clone(),
            runs: *runs,
            span: *span,
        },
        K::Ensure { cleanup, span } => K::Ensure {
            cleanup: cleanup.clone(),
            span: *span,
        },
        K::Cleaning { result, decided } => K::Cleaning {
            result: result.clone(),
            decided: decided.clone(),
        },
        K::Continuing {
            error,
            default,
            trap_at,
        } => K::Continuing {
            error: error.clone(),
            default: default.clone(),
            trap_at: *trap_at,
        },
        _ => return None,
    })
}
