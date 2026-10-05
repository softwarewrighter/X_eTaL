//! The lines fed to a run (D50, RS1): what `[]R_EAD`, `[]K_EY` and
//! `[]E_VENT` take from a queue the host fills, and what the run waits
//! for when the queue is empty. Without a queue (the command line)
//! those built-ins read standard input themselves.

use std::collections::VecDeque;
use std::rc::Rc;

use xetal_base::Diagnostic;
use xetal_value::{Event, Value};

/// What a waiting run wants next.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Wants {
    /// A line typed, for `[]R_EAD`.
    Line,
    /// A key's name, for `[]K_EY`.
    Key,
    /// An event's line, for `[]E_VENT`.
    Event,
}

impl Wants {
    /// What the built-in `name` reads from the inbox, if it reads it.
    pub fn of(name: &str) -> Option<Wants> {
        match name {
            "[]R_EAD" => Some(Wants::Line),
            "[]K_EY" => Some(Wants::Key),
            "[]E_VENT" => Some(Wants::Event),
            _ => None,
        }
    }
}

/// The queue of lines fed to a run, and whether the run waits on it.
#[derive(Debug, Default)]
pub struct Inbox {
    queue: Option<VecDeque<String>>,
    pub waiting: bool,
    pub wants: Option<Wants>,
}

impl Inbox {
    /// Read from a queue (fed with [`Inbox::feed`]) instead of standard
    /// input: a run that needs a line before one is fed waits.
    pub fn open(&mut self) {
        self.queue = Some(VecDeque::new());
    }

    /// A line typed (without its newline); a waiting run can go on.
    pub fn feed(&mut self, line: String) {
        self.queue
            .get_or_insert_with(Default::default)
            .push_back(line);
        self.waiting = false;
    }

    /// The next line as what the built-in wants: `None` without a queue
    /// (the built-in reads standard input), `Some(None)` when the queue
    /// is empty (the run now waits), else the value, or the error of a
    /// line that is not what was wanted.
    pub fn next<'a>(&mut self, wants: Wants) -> Option<Option<Result<Value<'a>, Diagnostic>>> {
        let queue = self.queue.as_mut()?;
        Some(match queue.pop_front() {
            Some(line) => Some(value_of(wants, &line)),
            None => {
                self.waiting = true;
                self.wants = Some(wants);
                None
            }
        })
    }
}

/// A fed line as the value a built-in wants.
pub fn value_of<'a>(wants: Wants, line: &str) -> Result<Value<'a>, Diagnostic> {
    Ok(match wants {
        Wants::Line => Value::Array(Rc::new(xetal_array::Array::vector(
            line.chars().map(Value::Char).collect(),
        ))),
        Wants::Key => Value::Tag("Key", xetal_value::key_named(line).unwrap_or(0)),
        Wants::Event => match Event::parse(line) {
            Some(e) => Value::Event(Rc::new(e)),
            None => {
                let what = format!(
                    "not an event: {line:?} (tick S, down X Y, move X Y, up X Y, click X Y, key NAME, end)"
                );
                return Err(Diagnostic::new("bad-event", what));
            }
        },
    })
}
