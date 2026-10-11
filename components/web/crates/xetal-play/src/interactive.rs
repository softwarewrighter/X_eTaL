//! A run the page steps (D50, Saga 25): started once, then taken a
//! slice at a time, so the page stays responsive and can stop it; at
//! `[]R_EAD` with no line typed it waits, and a fed line resumes it at
//! that very call, as web-sw-tos steps its emulator.
//!
//! The machine borrows the program for the whole run, which outlives
//! any one call, so the loaded program and the output are kept for the
//! life of the process: in the live demo each run has a worker of its
//! own, ended with the run.

use std::io::Write;
use std::sync::{Arc, Mutex};

use xetal_arith::Rng;
use xetal_base::{Diagnostic, Span};
use xetal_program::Loaded;
use xetal_step::{Machine, Status};

use crate::cells::Cells;
use crate::engine::{Run, finish, ready};

/// Where a run is after a slice.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Step {
    /// More to do.
    Running,
    /// Waiting for a typed line ([`Interactive::feed`]).
    Waiting,
    /// Waiting for one key, fed by its name (`Up`, `a`).
    WaitingKey,
    /// Waiting for an event's line (`tick 0.016`, `down 120 80`; RS1).
    WaitingEvent,
    /// Finished: its warnings and error, and the pictures it showed.
    Done(Run),
}

/// A program running a slice at a time.
pub struct Interactive {
    loaded: &'static Loaded,
    machine: Machine<'static, 'static>,
    warnings: Vec<Diagnostic>,
    /// A notebook's cells, the rest shown when the run ends.
    cells: Option<Arc<Mutex<Cells>>>,
}

impl Interactive {
    /// Load and check `src`, ready to run, rolling from `seed` and
    /// printing to `out`; a program that does not load or check, or a
    /// library (nothing to run), is a finished run instead.
    pub fn start(src: &str, seed: u64, out: Box<dyn Write>) -> Result<Interactive, Run> {
        if xetal_program::is_library(src) {
            return Err(crate::engine::run(src, seed));
        }
        let loaded: &'static Loaded = Box::leak(Box::new(ready(src)?));
        Ok(Self::begin(loaded, seed, out, None))
    }

    /// `src` run as a notebook (statements up to `upto`, all with None):
    /// `cell` is given each statement's source before it runs, and the
    /// run waits for typed lines as [`Interactive::start`]'s does.
    pub fn notebook(
        src: &str,
        seed: u64,
        upto: Option<usize>,
        cell: Box<dyn FnMut(&str) + Send>,
        out: Box<dyn Write>,
    ) -> Result<Interactive, Run> {
        let text = crate::cells::reach(src, upto)?;
        if xetal_program::is_library(&text) {
            return Err(crate::engine::run(&text, seed));
        }
        let loaded: &'static Loaded = Box::leak(Box::new(ready(&text)?));
        let cells = Arc::new(Mutex::new(Cells::new(&text, cell)));
        let shown = cells.clone();
        let before = move |span: Span| {
            if let Ok(mut c) = shown.lock() {
                c.before(loaded, span);
            }
        };
        let before: &'static mut (dyn FnMut(Span) + Send) = Box::leak(Box::new(before));
        let mut run = Self::begin(loaded, seed, out, Some(before));
        run.cells = Some(cells);
        Ok(run)
    }

    fn begin(
        loaded: &'static Loaded,
        seed: u64,
        out: Box<dyn Write>,
        before: Option<&'static mut (dyn FnMut(Span) + Send)>,
    ) -> Interactive {
        xetal_store::take_shown();
        let out: &'static mut dyn Write = Box::leak(out);
        let warnings = xetal_lint::warnings(&loaded.program);
        let machine = Machine::new(&loaded.program, out, Rng::seeded(seed))
            .hooks(None, before)
            .waiting_for_input();
        Interactive {
            loaded,
            machine,
            warnings,
            cells: None,
        }
    }

    /// Run at most `budget` transitions.
    pub fn step(&mut self, budget: usize) -> Step {
        let result = match self.machine.run(budget) {
            Ok(Status::Running) => return Step::Running,
            Ok(Status::Waiting) => return Step::Waiting,
            Ok(Status::WaitingKey) => return Step::WaitingKey,
            Ok(Status::WaitingEvent) => return Step::WaitingEvent,
            Ok(Status::Done) => {
                if let Some(Ok(mut c)) = self.cells.as_ref().map(|c| c.lock()) {
                    c.rest();
                }
                Ok(())
            }
            Err(e) => Err(self.loaded.program.annotate(e)),
        };
        Step::Done(finish(
            self.loaded,
            std::mem::take(&mut self.warnings),
            result,
        ))
    }

    /// A line typed (without its newline) for the waiting `[]R_EAD`, or
    /// a key's name for the waiting `[]K_EY`.
    pub fn feed(&mut self, line: String) {
        self.machine.feed(line);
    }
}
