//! The machine and its run loop: a slice is a number of transitions,
//! each one step of evaluation or one frame given its value.

use std::collections::HashMap;
use std::io::Write;

use xetal_arith::Rng;
use xetal_base::{Diagnostic, Span};
use xetal_core::Program;
use xetal_value::Value;

use xetal_frame::{Control, Inbox, Kont, State};

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
    /// The run's state: what is in hand, the pending frames, the
    /// bindings, the generator, the inbox.
    pub(crate) state: State<'a>,
    pub(crate) out: &'o mut dyn Write,
    /// When set, given each top-level value instead of printing it.
    pub(crate) keep: Option<&'o mut dyn FnMut(&Value<'a>)>,
    /// Called with each top-level item's span just before it runs.
    pub(crate) before: Option<&'o mut (dyn FnMut(Span) + Send)>,
}

impl<'a, 'o> Machine<'a, 'o> {
    /// A run of `program`, printing to `out`, `r_oll!` drawing from `rng`.
    pub fn new(program: &'a Program, out: &'o mut dyn Write, rng: Rng) -> Self {
        let state = State {
            globals: HashMap::new(),
            env: None,
            stack: vec![Kont::Items { next: 0 }],
            control: Some(Control::Return(Value::Unit)),
            rng,
            inbox: Inbox::default(),
            journal: None,
        };
        Machine {
            program,
            state,
            out,
            keep: None,
            before: None,
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

    /// The run's state, to start from a paused run's bindings (a
    /// debugger's watch runs in the frame where the program stopped),
    /// or to copy and put back (a debugger's checkpoints).
    pub fn state(&mut self) -> &mut State<'a> {
        &mut self.state
    }

    /// Run at most `budget` transitions.
    pub fn run(&mut self, budget: usize) -> Result<Status, Diagnostic> {
        for _ in 0..budget {
            if self.state.inbox.waiting {
                return Ok(self.waited());
            }
            let Some(control) = self.state.control.take() else {
                return Ok(Status::Done);
            };
            match self.transition(control) {
                Ok(next) => self.state.control = next,
                Err(e) => self.state.control = Some(self.catch(e)?),
            }
        }
        Ok(
            match (self.state.inbox.waiting, self.state.control.is_some()) {
                (true, _) => self.waited(),
                (false, true) => Status::Running,
                (false, false) => Status::Done,
            },
        )
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
                if self.state.stack.len() >= MAX_FRAMES {
                    return Err(Diagnostic::new("stack-overflow", "recursion is too deep")
                        .with_span(e.span));
                }
                self.eval(e, &env).map(Some)
            }
            Control::Return(v) => self.resume(v),
        }
    }

    /// The state as it is between transitions, read-only (a debugger
    /// reads it; a run without one never calls this).
    pub fn view(&self) -> xetal_frame::View<'_, 'a> {
        xetal_frame::View {
            control: self.state.control.as_ref(),
            stack: &self.state.stack,
            env: &self.state.env,
            globals: &self.state.globals,
        }
    }
}
