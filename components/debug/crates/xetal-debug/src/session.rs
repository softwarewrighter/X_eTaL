//! A debugging session: the machine run one transition at a time until
//! a stop wanted by the step in hand or a breakpoint.

use std::collections::BTreeSet;
use std::io::Write;

use xetal_arith::Rng;
use xetal_base::{Diagnostic, Span};
use xetal_core::Program;
use xetal_sources::Sources;
use xetal_step::Machine;

use crate::here::{Place, depth};
use crate::stack::{Frame, frames};

/// Why the session stopped.
#[derive(Debug, Clone, PartialEq)]
pub enum Reason {
    /// Before the first statement.
    Entry,
    /// A step finished.
    Step,
    /// A breakpoint's line was reached.
    Breakpoint,
    /// The program waits for a line or a key it reads.
    Waiting,
    /// The program has finished.
    End,
    /// The program failed.
    Error(Diagnostic),
}

/// Where and why the session stopped, and the call depth there.
#[derive(Debug, Clone, PartialEq)]
pub struct Stop {
    pub reason: Reason,
    pub place: Option<Place>,
    pub depth: usize,
}

/// A program being debugged.
pub struct Debugger<'a, 'o> {
    pub(crate) machine: Machine<'a, 'o>,
    pub(crate) sources: &'a Sources,
    pub(crate) breakpoints: BTreeSet<(String, usize)>,
    pub(crate) at: Option<Span>,
    /// The last statement seen, its depth, and the call it was in
    /// (`calls` counts each change of depth): a statement inside it in
    /// the same call (a guard's result) continues it.
    pub(crate) last: Option<(Span, usize, usize)>,
    pub(crate) calls: usize,
    pub(crate) depth: usize,
}

impl<'a, 'o> Debugger<'a, 'o> {
    /// A session over `program` (its places from `sources`), printing
    /// to `out`, `r_oll!` drawing from `seed`; nothing has run yet.
    pub fn new(
        program: &'a Program,
        sources: &'a Sources,
        out: &'o mut dyn Write,
        seed: u64,
    ) -> Self {
        let machine = Machine::new(program, out, Rng::seeded(seed));
        Debugger {
            machine,
            sources,
            breakpoints: BTreeSet::new(),
            at: None,
            last: None,
            calls: 0,
            depth: 0,
        }
    }

    /// The breakpoints of `file`, by line (replacing its earlier ones).
    pub fn set_breakpoints(&mut self, file: &str, lines: &[usize]) {
        self.breakpoints.retain(|(f, _)| f != file);
        self.breakpoints
            .extend(lines.iter().map(|l| (file.to_string(), *l)));
    }

    /// Run to the first statement.
    pub fn start(&mut self) -> Stop {
        self.go(|_, _| true, Reason::Entry)
    }

    /// The call stack at the stop, innermost first.
    pub fn stack(&self) -> Vec<Frame> {
        frames(&self.machine.view(), self.sources, self.at)
    }

    pub(crate) fn depth(&self) -> usize {
        depth(&self.machine.view())
    }
}
