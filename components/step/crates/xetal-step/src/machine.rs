//! The machine and its run loop: a slice is a number of transitions,
//! each one step of evaluation or one frame given its value.

use std::collections::HashMap;
use std::io::Write;

use xetal_arith::Rng;
use xetal_base::{Diagnostic, Span};
use xetal_core::Program;
use xetal_value::{Env, Value};

use xetal_frame::{Control, Inbox, Kont, Wants};

/// Pending frames allowed before reporting `stack-overflow` (the depth a
/// runaway recursion reaches; deep finite recursion is well within it).
const MAX_FRAMES: usize = 2_000_000;

/// Where a run is after a slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// More to do: run another slice.
    Running,
    /// Stopped at `[]R_EAD` until a line is fed (with an input queue).
    Waiting,
    /// Stopped at `[]K_EY` until a key's name is fed.
    WaitingKey,
    /// Stopped at `[]E_VENT` until an event's line is fed (RS1).
    WaitingEvent,
    /// Every item has run.
    Done,
}

/// A program being evaluated: its state is data, so it can stop between
/// any two transitions and go on later.
pub struct Machine<'a, 'o> {
    pub(crate) program: &'a Program,
    pub(crate) globals: HashMap<String, Value<'a>>,
    /// The top level's bindings so far.
    pub(crate) env: Env<'a>,
    pub(crate) stack: Vec<Kont<'a>>,
    pub(crate) control: Option<Control<'a>>,
    pub(crate) out: &'o mut dyn Write,
    pub(crate) rng: Rng,
    /// When set, given each top-level value instead of printing it.
    pub(crate) keep: Option<&'o mut dyn FnMut(&Value<'a>)>,
    /// Called with each top-level item's span just before it runs.
    pub(crate) before: Option<&'o mut (dyn FnMut(Span) + Send)>,
    /// Lines fed for `[]R_EAD`, `[]K_EY` and `[]E_VENT`, and what the
    /// run waits for; without a queue those read standard input.
    pub(crate) inbox: Inbox,
}

impl<'a, 'o> Machine<'a, 'o> {
    /// A run of `program`, printing to `out`, `r_oll!` drawing from `rng`.
    pub fn new(program: &'a Program, out: &'o mut dyn Write, rng: Rng) -> Self {
        Machine {
            program,
            globals: HashMap::new(),
            env: None,
            stack: vec![Kont::Items { next: 0 }],
            control: Some(Control::Return(Value::Unit)),
            out,
            rng,
            keep: None,
            before: None,
            inbox: Inbox::default(),
        }
    }

    /// Give each top-level value to `keep` instead of printing it, and
    /// call `before` with each item's span before it runs.
    pub fn hooks(
        mut self,
        keep: Option<&'o mut dyn FnMut(&Value<'a>)>,
        before: Option<&'o mut (dyn FnMut(Span) + Send)>,
    ) -> Self {
        self.keep = keep;
        self.before = before;
        self
    }

    /// Run at most `budget` transitions.
    pub fn run(&mut self, budget: usize) -> Result<Status, Diagnostic> {
        for _ in 0..budget {
            if self.inbox.waiting {
                return Ok(self.waited());
            }
            let Some(control) = self.control.take() else {
                return Ok(Status::Done);
            };
            match self.transition(control) {
                Ok(next) => self.control = next,
                Err(e) => self.control = Some(self.catch(e)?),
            }
        }
        Ok(match (self.inbox.waiting, self.control.is_some()) {
            (true, _) => self.waited(),
            (false, true) => Status::Running,
            (false, false) => Status::Done,
        })
    }

    /// Run to the end (or until it waits for a line).
    pub fn finish(&mut self) -> Result<(), Diagnostic> {
        while self.run(usize::MAX)? == Status::Running {}
        Ok(())
    }

    /// One transition; `None` when the program has finished.
    pub(crate) fn transition(
        &mut self,
        control: Control<'a>,
    ) -> Result<Option<Control<'a>>, Diagnostic> {
        match control {
            Control::Eval(e, env) => {
                if self.stack.len() >= MAX_FRAMES {
                    return Err(Diagnostic::new("stack-overflow", "recursion is too deep")
                        .with_span(e.span));
                }
                self.eval(e, &env).map(Some)
            }
            Control::Return(v) => self.resume(v),
        }
    }

    /// What the run waits for.
    fn waited(&self) -> Status {
        match self.inbox.wants {
            Some(Wants::Key) => Status::WaitingKey,
            Some(Wants::Event) => Status::WaitingEvent,
            _ => Status::Waiting,
        }
    }
}
