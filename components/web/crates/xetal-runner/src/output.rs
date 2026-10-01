//! What the page shows of a run as its events arrive: the output
//! appended a line at a time, pictures and errors as they come, and
//! whether it is still running.

use std::rc::Rc;

use xetal_play::Run;
use yew::Reducible;

use crate::Event;

/// The run shown (none: the types are shown instead) and whether it is
/// still going.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Output {
    pub run: Option<Run>,
    pub running: bool,
}

/// What happens to it.
pub enum Action {
    /// A run begins: empty output, running.
    Start,
    /// Something the run did, as it happened.
    Event(Event),
    /// A run on the page (one that reads the keyboard) ended with this.
    Finished(Run),
    /// Stopped by hand: what was shown stays, and says so.
    Stop,
    /// Back to showing the types.
    Clear,
}

impl Reducible for Output {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Action) -> Rc<Self> {
        let mut next = (*self).clone();
        let run = next.run.get_or_insert_with(Run::default);
        match action {
            Action::Start => (next.run, next.running) = (Some(Run::default()), true),
            Action::Event(Event::Out(line)) => run.out += &format!("{line}\n"),
            Action::Event(Event::Err(line)) => run.err += &format!("{line}\n"),
            Action::Event(Event::Picture(svg)) => run.pictures.push(svg),
            Action::Event(Event::Wrote(..) | Event::Ready) => {}
            Action::Event(Event::Done) => next.running = false,
            Action::Finished(done) => (next.run, next.running) = (Some(done), false),
            Action::Stop => {
                run.err += "stopped\n";
                next.running = false;
            }
            Action::Clear => return Rc::new(Output::default()),
        }
        Rc::new(next)
    }
}
