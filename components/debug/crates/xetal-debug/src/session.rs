//! A debugging session: the machine run one transition at a time until
//! a stop wanted by the step in hand or a breakpoint.

use std::collections::{BTreeMap, HashMap};
use std::io::Write;

use xetal_arith::Rng;
use xetal_base::{Diagnostic, Span};
use xetal_core::Program;
use xetal_sources::Sources;
use xetal_step::Machine;
use xetal_travel::{Tally, Timeline};
use xetal_value::Value;

use crate::here::Place;
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

/// Where and why the session stopped, and the call depth there; after
/// going back, the files written after this point (they stay written).
#[derive(Debug, Clone, PartialEq)]
pub struct Stop {
    pub reason: Reason,
    pub place: Option<Place>,
    pub depth: usize,
    pub rewound: Vec<String>,
}

/// What the session keeps beside a checkpoint: its own state there.
#[derive(Clone)]
pub(crate) struct Book {
    pub(crate) hits: HashMap<(String, usize), usize>,
    pub(crate) last: Option<(Span, usize, usize)>,
    pub(crate) calls: usize,
    pub(crate) depth: usize,
    pub(crate) out: usize,
    pub(crate) mark: xetal_store::Mark,
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
    /// Transitions run so far (the step counter time travel counts by).
    pub(crate) now: usize,
    pub(crate) timeline: Timeline<'a, Book>,
    pub(crate) tally: Tally,
    /// Where a trip back began (the step and the store's counts), until
    /// it arrives.
    pub(crate) left: Option<(usize, xetal_store::Mark)>,
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
        // The output passes a gate, so a stretch run again after going
        // back prints nothing twice (leaked: it lives as long as `out`).
        let (gate, tally) = xetal_travel::gate(out);
        let out: &'o mut dyn Write = Box::leak(Box::new(gate));
        let mut machine = Machine::new(program, out, Rng::seeded(seed));
        machine.state().journal = Some(Vec::new());
        xetal_store::replay(0);
        let mut timeline = Timeline::new(1000, 1000);
        let book = || Book {
            hits: HashMap::new(),
            last: None,
            calls: 0,
            depth: 0,
            out: 0,
            mark: xetal_store::mark(),
        };
        timeline.take(0, machine.state(), book);
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
            now: 0,
            timeline,
            tally,
            left: None,
        }
    }

    /// Keep a checkpoint every `every` transitions, at most `max` (then
    /// every other one goes and they come half as often); by default
    /// every 1,000, at most 1,000.
    pub fn keep_checkpoints(&mut self, every: usize, max: usize) {
        let mut timeline = Timeline::new(every, max);
        let first = self.timeline.back_to(0);
        if let Some((step, state, book)) = first {
            timeline.take(step, &state, || book);
        }
        self.timeline = timeline;
    }

    /// How many checkpoints the session keeps now.
    pub fn checkpoints(&self) -> usize {
        self.timeline.count()
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
}
