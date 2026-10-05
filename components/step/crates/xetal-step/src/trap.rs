//! Catching errors (ER2, ER3): `[]T_RAP` and `[]E_NSURE` are frames on
//! the machine's stack. An error unwinds to the nearest one: a trap
//! calls its handler with the `Error`, and the handler's outcome
//! recovers with a value, runs the body again or lets the error go
//! on; an ensure runs its cleanup, then the body's result goes on as
//! it was. A warning (`[]W_ARN`) unwinds nothing: the nearest trap's
//! handler runs above the stack as it stands, and `[]C_ONTINUE` goes
//! on from the warning with its default.

use std::rc::Rc;

use xetal_base::{Diagnostic, Span};
use xetal_value::{Outcome, Slot, Value};

use crate::machine::Machine;
use xetal_frame::{Control, Kont, err};

/// How many times a trap runs its body before a retry gives up and the
/// error goes on (a handler that always retries).
const MAX_RUNS: usize = 1000;

impl<'a> Machine<'a, '_> {
    /// `[]T_RAP`, `[]E_NSURE` or `[]W_ARN` fully applied: push the frame
    /// and run the body, or raise the warning resumably.
    pub(crate) fn trapping(
        &mut self,
        name: &str,
        args: &[Value<'a>],
        span: Span,
    ) -> Option<Result<Control<'a>, Diagnostic>> {
        let frame = match (name, args) {
            ("[]T_RAP", [body, handler]) => Kont::Trap {
                body: body.clone(),
                handler: handler.clone(),
                runs: 1,
                span,
            },
            ("[]E_NSURE", [_, cleanup]) => Kont::Ensure {
                cleanup: cleanup.clone(),
                span,
            },
            ("[]W_ARN", [default, _]) => {
                let raised = xetal_prim::call(name, args, span, self.out, &mut self.rng);
                return Some(match raised {
                    Ok(v) => Ok(Control::Return(v)),
                    Err(e) => self.resumable(Rc::new(e), default.clone(), self.stack.len()),
                });
            }
            _ => return None,
        };
        self.stack.push(frame);
        Some(self.apply(args[0].clone(), Slot::Value(Value::Unit), span))
    }

    /// A warning raised, or halted by a handler: the handler of the
    /// nearest trap below `below` runs above the stack, which stays as
    /// it is; with no trap left, the warning is an ordinary error.
    fn resumable(
        &mut self,
        error: Rc<Diagnostic>,
        default: Value<'a>,
        below: usize,
    ) -> Result<Control<'a>, Diagnostic> {
        let trap = self.stack[..below]
            .iter()
            .enumerate()
            .rev()
            .find_map(|(i, k)| match k {
                Kont::Trap { handler, span, .. } => Some(Ok((i, handler.clone(), *span))),
                Kont::Items { .. } => Some(Err(())),
                _ => None,
            });
        let Some(Ok((trap_at, handler, span))) = trap else {
            return self.catch((*error).clone());
        };
        self.stack.push(Kont::Continuing {
            error: error.clone(),
            default,
            trap_at,
        });
        self.apply(handler, Slot::Value(Value::Error(error)), span)
    }

    /// An error raised: unwind to the nearest trap or ensure and act;
    /// with none, the run ends with the error.
    pub(crate) fn catch(&mut self, e: Diagnostic) -> Result<Control<'a>, Diagnostic> {
        self.unwind(e, None)
    }

    /// Unwind to the nearest trap or ensure. At a trap, the handler
    /// runs, unless a warning's handler already `decided` the outcome.
    fn unwind(
        &mut self,
        e: Diagnostic,
        decided: Option<Rc<Outcome<'a>>>,
    ) -> Result<Control<'a>, Diagnostic> {
        while let Some(k) = self.stack.pop() {
            match k {
                Kont::Force { slot, expr, env } => *slot.borrow_mut() = Slot::Thunk(expr, env),
                Kont::Trap {
                    body,
                    handler,
                    runs,
                    span,
                } => {
                    if let Some(outcome) = decided {
                        return self.outcome(body, handler, runs, span, &outcome);
                    }
                    let error = Value::Error(Rc::new(e));
                    self.stack.push(Kont::Handling {
                        body,
                        handler: handler.clone(),
                        runs,
                        span,
                    });
                    return self.apply(handler, Slot::Value(error), span);
                }
                Kont::Ensure { cleanup, span } => {
                    self.stack.push(Kont::Cleaning {
                        result: Err(e),
                        decided,
                    });
                    return self.apply(cleanup, Slot::Value(Value::Unit), span);
                }
                Kont::Items { .. } => break,
                _ => {}
            }
        }
        self.stack.clear();
        Err(e)
    }

    /// A trap's or ensure's frame given a value: the body finished
    /// (Trap, Ensure), the handler answered (Handling, Continuing) or
    /// the cleanup ran (Cleaning).
    pub(crate) fn resume_trap(
        &mut self,
        k: Kont<'a>,
        v: Value<'a>,
    ) -> Result<Option<Control<'a>>, Diagnostic> {
        Ok(Some(match k {
            Kont::Trap { .. } => Control::Return(v),
            Kont::Ensure { cleanup, span } => {
                self.stack.push(Kont::Cleaning {
                    result: Ok(v),
                    decided: None,
                });
                self.apply(cleanup, Slot::Value(Value::Unit), span)?
            }
            Kont::Cleaning { result, decided } => match result {
                Ok(v) => Control::Return(v),
                Err(e) => self.unwind(e, decided)?,
            },
            Kont::Handling {
                body,
                handler,
                runs,
                span,
            } => self.outcome(body, handler, runs, span, &*answered(v, span)?)?,
            Kont::Continuing {
                error,
                default,
                trap_at,
            } => {
                let outcome = answered(v, error.span.unwrap_or_default())?;
                match &*outcome {
                    Outcome::Continue(_) => Control::Return(default),
                    Outcome::Halt(e) => self.resumable(e.clone(), default, trap_at)?,
                    _ => self.unwind((*error).clone(), Some(outcome))?,
                }
            }
            _ => return Err(Diagnostic::new("internal", "not a trap frame")),
        }))
    }

    /// Act on a handler's outcome at its trap.
    fn outcome(
        &mut self,
        body: Value<'a>,
        handler: Value<'a>,
        runs: usize,
        span: Span,
        outcome: &Outcome<'a>,
    ) -> Result<Control<'a>, Diagnostic> {
        match outcome {
            Outcome::Recover(value) => Ok(Control::Return(value.clone())),
            Outcome::Halt(e) => self.catch((**e).clone()),
            Outcome::Continue(e) => {
                let what = format!(
                    "error[{}] was raised by []S_IGNAL, not []W_ARN, so it has no value to go on with",
                    e.code
                );
                let at = e.span.unwrap_or(span);
                self.catch(err("not-resumable", at, what))
            }
            Outcome::Retry if runs >= MAX_RUNS => {
                let message = format!("the body was retried {MAX_RUNS} times");
                self.catch(err("retry-limit", span, message))
            }
            Outcome::Retry => {
                self.stack.push(Kont::Trap {
                    body: body.clone(),
                    handler,
                    runs: runs + 1,
                    span,
                });
                self.apply(body, Slot::Value(Value::Unit), span)
            }
        }
    }
}

/// The outcome a handler answered, or a domain error.
fn answered<'a>(v: Value<'a>, span: Span) -> Result<Rc<Outcome<'a>>, Diagnostic> {
    match v {
        Value::Outcome(outcome) => Ok(outcome),
        _ => Err(err(
            "domain",
            span,
            "a handler answers []R_ECOVER, []R_ETRY, []H_ALT or []C_ONTINUE",
        )),
    }
}
