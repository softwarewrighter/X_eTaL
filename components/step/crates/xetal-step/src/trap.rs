//! Catching errors (ER2): `[]T_RAP` and `[]E_NSURE` are frames on the
//! machine's stack. An error unwinds to the nearest one: a trap calls
//! its handler with the `Error`, and the handler's outcome recovers
//! with a value, runs the body again or lets the error go on; an
//! ensure runs its cleanup, then the body's result goes on as it was.

use std::rc::Rc;

use xetal_base::{Diagnostic, Span};
use xetal_value::{Outcome, Slot, Value};

use crate::machine::Machine;
use xetal_frame::{Control, Kont, err};

/// How many times a trap runs its body before a retry gives up and the
/// error goes on (a handler that always retries).
const MAX_RUNS: usize = 1000;

impl<'a> Machine<'a, '_> {
    /// `[]T_RAP` or `[]E_NSURE` fully applied: push the frame, run the body.
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
            _ => return None,
        };
        self.stack.push(frame);
        Some(self.apply(args[0].clone(), Slot::Value(Value::Unit), span))
    }

    /// An error raised: unwind to the nearest trap or ensure and act;
    /// with none, the run ends with the error.
    pub(crate) fn catch(&mut self, e: Diagnostic) -> Result<Control<'a>, Diagnostic> {
        while let Some(k) = self.stack.pop() {
            match k {
                Kont::Force { slot, expr, env } => *slot.borrow_mut() = Slot::Thunk(expr, env),
                Kont::Trap {
                    body,
                    handler,
                    runs,
                    span,
                } => {
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
                    self.stack.push(Kont::Cleaning { result: Err(e) });
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
    /// (Trap, Ensure), the handler answered (Handling) or the cleanup
    /// ran (Cleaning).
    pub(crate) fn resume_trap(
        &mut self,
        k: Kont<'a>,
        v: Value<'a>,
    ) -> Result<Option<Control<'a>>, Diagnostic> {
        Ok(Some(match k {
            Kont::Trap { .. } => Control::Return(v),
            Kont::Ensure { cleanup, span } => {
                self.stack.push(Kont::Cleaning { result: Ok(v) });
                self.apply(cleanup, Slot::Value(Value::Unit), span)?
            }
            Kont::Cleaning { result } => match result {
                Ok(v) => Control::Return(v),
                Err(e) => self.catch(e)?,
            },
            Kont::Handling {
                body,
                handler,
                runs,
                span,
            } => self.outcome(body, handler, runs, span, v)?,
            _ => return Err(Diagnostic::new("internal", "not a trap frame")),
        }))
    }

    /// Act on a handler's outcome.
    fn outcome(
        &mut self,
        body: Value<'a>,
        handler: Value<'a>,
        runs: usize,
        span: Span,
        v: Value<'a>,
    ) -> Result<Control<'a>, Diagnostic> {
        let Value::Outcome(outcome) = v else {
            return Err(err(
                "domain",
                span,
                "a handler answers []R_ECOVER, []R_ETRY or []H_ALT",
            ));
        };
        match &*outcome {
            Outcome::Recover(value) => Ok(Control::Return(value.clone())),
            Outcome::Halt(e) => self.catch((**e).clone()),
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
