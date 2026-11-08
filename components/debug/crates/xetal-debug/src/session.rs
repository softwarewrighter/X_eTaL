//! A debugging session: the machine run one transition at a time until
//! a stop wanted by the step in hand or a breakpoint.

use std::collections::{BTreeMap, HashMap};
use std::io::Write;

use xetal_arith::Rng;
use xetal_base::{Diagnostic, Span};
use xetal_core::Program;
use xetal_sources::Sources;
use xetal_step::Machine;
use xetal_value::Value;

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

/// A breakpoint on a line, with an optional condition (an expression
/// evaluated where it stops: it breaks when the value is true) and an
/// optional hit condition (`3` or `>= 3`, `== 3`, `> 3`, `% 3`: which
/// hits break, counted from 1).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Breakpoint {
    pub line: usize,
    pub condition: Option<String>,
    pub hit: Option<String>,
}

/// A program being debugged.
pub struct Debugger<'a, 'o> {
    pub(crate) machine: Machine<'a, 'o>,
    pub(crate) sources: &'a Sources,
    pub(crate) breakpoints: BTreeMap<(String, usize), Breakpoint>,
    pub(crate) hits: HashMap<(String, usize), usize>,
    /// Values shown at this stop with children, by id (id 1 is the
    /// first): a front end opens one by its id, a page at a time.
    pub(crate) refs: Vec<Value<'a>>,
    /// Each watch's lowered program, kept for the session.
    pub(crate) programs: HashMap<String, &'a Program>,
    pub(crate) watches: Vec<String>,
    /// (stop number, watch, summary) at every stop.
    pub(crate) history: Vec<(usize, String, String)>,
    pub(crate) stops: usize,
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
            breakpoints: BTreeMap::new(),
            hits: HashMap::new(),
            refs: Vec::new(),
            programs: HashMap::new(),
            watches: Vec::new(),
            history: Vec::new(),
            stops: 0,
            at: None,
            last: None,
            calls: 0,
            depth: 0,
        }
    }

    /// The breakpoints of `file`, by line (replacing its earlier ones).
    pub fn set_breakpoints(&mut self, file: &str, lines: &[usize]) {
        let plain = |line: &usize| Breakpoint {
            line: *line,
            ..Breakpoint::default()
        };
        self.set_conditional(file, lines.iter().map(plain).collect());
    }

    /// The breakpoints of `file`, with conditions and hit conditions;
    /// their hit counts start again.
    pub fn set_conditional(&mut self, file: &str, points: Vec<Breakpoint>) {
        self.breakpoints.retain(|(f, _), _| f != file);
        self.hits.retain(|(f, _), _| f != file);
        for b in points {
            self.breakpoints.insert((file.to_string(), b.line), b);
        }
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
