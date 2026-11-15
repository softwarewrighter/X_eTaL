//! The step commands: over, in, out and continue, each a rule for
//! where the walk stops next; and back: step back, reverse continue and
//! run to step k, each a trip to a checkpoint and forward again.

use crate::here::{Kind, depth};
use crate::session::{Debugger, Reason, Stop};

impl Debugger<'_, '_> {
    /// Over: the next statement at this call depth or outside it; in:
    /// the next statement or application anywhere; out: the next
    /// statement outside this call; continue: a breakpoint or the end.
    pub fn step_over(&mut self) -> Stop {
        let d = depth(&self.machine.view());
        self.go(
            move |k, depth| k == Kind::Statement && depth <= d,
            Reason::Step,
        )
    }

    pub fn step_in(&mut self) -> Stop {
        self.go(|k, _| k != Kind::Other, Reason::Step)
    }

    pub fn step_out(&mut self) -> Stop {
        let d = depth(&self.machine.view());
        self.go(
            move |k, depth| k == Kind::Statement && depth < d,
            Reason::Step,
        )
    }

    pub fn resume(&mut self) -> Stop {
        self.go(|_, _| false, Reason::Step)
    }

    /// Back to the last statement before this stop at this call depth
    /// or outside it (the reverse of step over); the first statement
    /// when there is none.
    pub fn step_back(&mut self) -> Stop {
        let d = depth(&self.machine.view());
        let target = self.seek(|p| p.kind == Kind::Statement && p.depth <= d);
        self.run_to(target.unwrap_or(0))
    }

    /// Back to the last breakpoint hit before this stop; the first
    /// statement when there is none.
    pub fn reverse_continue(&mut self) -> Stop {
        let target = self.seek(|p| p.hit);
        self.run_to(target.unwrap_or(0))
    }

    /// To the first stop at or after transition `step`, back or forward;
    /// going back, the stop names the files written after it (they stay
    /// written, and running forward does not write them again).
    pub fn run_to(&mut self, step: usize) -> Stop {
        let (was, before) = self.left.take().unwrap_or((self.now, xetal_store::mark()));
        if step <= self.now {
            self.rewind_to(step.saturating_sub(1));
        }
        let mut stop = match self.walk(|now, _| now >= step) {
            Err(ended) => *ended,
            Ok(p) => {
                self.stopped();
                let reason = if p.hit {
                    Reason::Breakpoint
                } else {
                    Reason::Step
                };
                let rewound = Vec::new();
                Stop {
                    reason,
                    place: p.here,
                    depth: p.depth,
                    rewound,
                }
            }
        };
        if step < was {
            stop.rewound = xetal_store::written(xetal_store::mark(), before);
        }
        stop
    }
}
