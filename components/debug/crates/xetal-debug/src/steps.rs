//! The step commands: over, in, out and continue, each a rule for
//! where the walk stops next.

use crate::here::Kind;
use crate::session::{Debugger, Reason, Stop};

impl Debugger<'_, '_> {
    /// Over: the next statement at this call depth or outside it; in:
    /// the next statement or application anywhere; out: the next
    /// statement outside this call; continue: a breakpoint or the end.
    pub fn step_over(&mut self) -> Stop {
        let d = self.depth();
        self.go(
            move |k, depth| k == Kind::Statement && depth <= d,
            Reason::Step,
        )
    }

    pub fn step_in(&mut self) -> Stop {
        self.go(|k, _| k != Kind::Other, Reason::Step)
    }

    pub fn step_out(&mut self) -> Stop {
        let d = self.depth();
        self.go(
            move |k, depth| k == Kind::Statement && depth < d,
            Reason::Step,
        )
    }

    pub fn resume(&mut self) -> Stop {
        self.go(|_, _| false, Reason::Step)
    }
}
