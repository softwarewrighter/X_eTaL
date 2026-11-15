//! Checkpoints along a run. Each holds a copy of the machine's state at
//! its step, what the debugger keeps beside it (`B`), and for the
//! interval after it every slot written there with the value it had at
//! the checkpoint (the first write's old value), so the slots, which
//! are shared and written in place, can be put back.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use xetal_step::{Journal, State};
use xetal_value::Slot;

type Undo<'a> = HashMap<*const RefCell<Slot<'a>>, (Rc<RefCell<Slot<'a>>>, Slot<'a>)>;

struct Checkpoint<'a, B> {
    step: usize,
    state: State<'a>,
    book: B,
    undo: Undo<'a>,
}

/// The checkpoints of a run, oldest first: one every `every`
/// transitions (when the state can be copied), at most `max`; past
/// that every other one goes and `every` doubles.
pub struct Timeline<'a, B> {
    points: Vec<Checkpoint<'a, B>>,
    every: usize,
    max: usize,
    due: usize,
}

impl<'a, B: Clone> Timeline<'a, B> {
    pub fn new(every: usize, max: usize) -> Self {
        Timeline {
            points: Vec::new(),
            every: every.max(1),
            max: max.max(2),
            due: 0,
        }
    }

    /// Keep a checkpoint at `step` if one is due and `state` can be
    /// copied; `book` gives what the debugger keeps beside it.
    pub fn take(&mut self, step: usize, state: &State<'a>, book: impl FnOnce() -> B) {
        if step < self.due {
            return;
        }
        let Some(state) = state.snapshot() else {
            return;
        };
        let book = book();
        let undo = Undo::new();
        self.points.push(Checkpoint {
            step,
            state,
            book,
            undo,
        });
        self.due = step + self.every;
        if self.points.len() > self.max {
            self.thin();
        }
    }

    /// The slot writes since the last checkpoint (kept: each slot's
    /// first old value).
    pub fn absorb(&mut self, journal: Journal<'a>) {
        let Some(last) = self.points.last_mut() else {
            return;
        };
        for (slot, old) in journal {
            last.undo.entry(Rc::as_ptr(&slot)).or_insert((slot, old));
        }
    }

    /// Back to the latest checkpoint at or before `step`: every slot
    /// written since is put back, later checkpoints are let go, and a
    /// copy of its state and book is given with its step.
    pub fn back_to(&mut self, step: usize) -> Option<(usize, State<'a>, B)> {
        let i = self.points.iter().rposition(|c| c.step <= step)?;
        for later in self.points[i..].iter_mut().rev() {
            for (_, (slot, old)) in later.undo.drain() {
                *slot.borrow_mut() = old;
            }
        }
        self.points.truncate(i + 1);
        let point = &self.points[i];
        self.due = point.step + self.every;
        Some((point.step, point.state.snapshot()?, point.book.clone()))
    }

    /// The checkpoints' steps, oldest first.
    pub fn steps(&self) -> Vec<usize> {
        self.points.iter().map(|c| c.step).collect()
    }

    /// How many checkpoints there are.
    pub fn count(&self) -> usize {
        self.points.len()
    }

    /// Every other checkpoint goes (the first stays), its slot writes
    /// joined to the one before; checkpoints come half as often.
    fn thin(&mut self) {
        let old = std::mem::take(&mut self.points);
        for (i, point) in old.into_iter().enumerate() {
            match (i % 2, self.points.last_mut()) {
                (1, Some(kept)) => {
                    for (key, entry) in point.undo {
                        kept.undo.entry(key).or_insert(entry);
                    }
                }
                _ => self.points.push(point),
            }
        }
        self.every *= 2;
    }
}
