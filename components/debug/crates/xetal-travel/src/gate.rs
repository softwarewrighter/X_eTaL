//! Output shown once: a run gone back and run forward again writes the
//! same bytes again, and the gate drops those already shown.

use std::cell::Cell;
use std::io::{Result, Write};
use std::rc::Rc;

/// A writer that passes on only bytes past the furthest point shown.
pub struct Gate<'o> {
    inner: &'o mut dyn Write,
    tally: Tally,
}

/// Where the run's output is (`at`, in bytes) and how far it has been
/// shown; shared with the gate.
#[derive(Clone, Default)]
pub struct Tally {
    at: Rc<Cell<usize>>,
    shown: Rc<Cell<usize>>,
}

impl Tally {
    /// The run's output so far, in bytes.
    pub fn at(&self) -> usize {
        self.at.get()
    }

    /// The run went back to where its output was `at` bytes.
    pub fn back(&self, at: usize) {
        self.at.set(at);
    }
}

/// A gate over `inner` and its tally.
pub fn gate(inner: &mut dyn Write) -> (Gate<'_>, Tally) {
    let tally = Tally::default();
    (
        Gate {
            inner,
            tally: tally.clone(),
        },
        tally,
    )
}

impl Write for Gate<'_> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let (at, shown) = (self.tally.at.get(), self.tally.shown.get());
        let skip = shown.saturating_sub(at).min(buf.len());
        self.inner.write_all(&buf[skip..])?;
        self.tally.at.set(at + buf.len());
        self.tally.shown.set(shown.max(at + buf.len()));
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }
}
