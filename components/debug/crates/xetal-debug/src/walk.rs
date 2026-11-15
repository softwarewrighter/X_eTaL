//! The walk: the machine run one transition at a time, each counted and
//! checkpointed now and then, until a stop the step wants, a breakpoint,
//! the end, a wait or an error; and the way back: to a checkpoint, then
//! forward again (time travel, docs/debugger-plan.md section 3).

use xetal_base::Span;
use xetal_step::Status;

use crate::here::{Kind, Place, depth, next, place};
use crate::session::{Book, Debugger, Reason, Stop};

/// A point the walk passed where it may stop.
pub(crate) struct Passed {
    pub(crate) kind: Kind,
    pub(crate) depth: usize,
    pub(crate) here: Option<Place>,
    /// A breakpoint broke here.
    pub(crate) hit: bool,
}

impl Debugger<'_, '_> {
    /// Transitions until `wanted` (or a breakpoint) at a statement or
    /// an application, the end, a wait or an error.
    pub(crate) fn go(&mut self, wanted: impl Fn(Kind, usize) -> bool, why: Reason) -> Stop {
        match self.walk(|_, p| p.hit || wanted(p.kind, p.depth)) {
            Err(ended) => *ended,
            Ok(p) => {
                self.stopped();
                let reason = match p.hit && why != Reason::Entry {
                    true => Reason::Breakpoint,
                    false => why,
                };
                let (place, depth) = (p.here, p.depth);
                let rewound = Vec::new();
                Stop {
                    reason,
                    place,
                    depth,
                    rewound,
                }
            }
        }
    }

    /// Transitions until `until` (given the step and the point) says
    /// stop at a point, or the run ends (`Err`, its stop).
    pub(crate) fn walk(
        &mut self,
        mut until: impl FnMut(usize, &Passed) -> bool,
    ) -> Result<Passed, Box<Stop>> {
        loop {
            let moved = self.machine.view().control.is_some();
            let status = self.machine.run(1);
            if moved {
                self.now += 1;
                let state = self.machine.state();
                if let Some(journal) = state.journal.as_mut().map(std::mem::take) {
                    self.timeline.absorb(journal);
                }
                let book = || Book {
                    hits: self.hits.clone(),
                    last: self.last,
                    calls: self.calls,
                    depth: self.depth,
                    out: self.tally.at(),
                    mark: xetal_store::mark(),
                };
                self.timeline.take(self.now, state, book);
            }
            match status {
                Ok(Status::Running) => {}
                Ok(Status::Done) => return Err(Box::new(self.ended(Reason::End))),
                Ok(_) => return Err(Box::new(self.ended(Reason::Waiting))),
                Err(d) => return Err(Box::new(self.ended(Reason::Error(d)))),
            }
            let view = self.machine.view();
            let ((kind, span), d) = (next(&view), depth(&view));
            let kind = self.fresh(kind, span, d);
            if kind == Kind::Other {
                continue;
            }
            let here = span.map(|s| place(self.sources, s));
            self.at = span;
            let hit = kind == Kind::Statement && here.as_ref().is_some_and(|p| self.breaks(p));
            let passed = Passed {
                kind,
                depth: d,
                here,
                hit,
            };
            if until(self.now, &passed) {
                return Ok(passed);
            }
        }
    }

    /// `kind`, unless it is a statement continuing the last one (inside
    /// its span, at its depth, in the same call): then `Other`.
    pub(crate) fn fresh(&mut self, kind: Kind, span: Option<Span>, d: usize) -> Kind {
        if d != self.depth {
            (self.depth, self.calls) = (d, self.calls + 1);
        }
        let (Kind::Statement, Some(s)) = (kind, span) else {
            return kind;
        };
        let inside = |(l, ld, lc): (Span, usize, usize)| {
            ld == d && lc == self.calls && l.start <= s.start && s.end <= l.end
        };
        if self.last.is_some_and(inside) {
            return Kind::Other;
        }
        self.last = Some((s, d, self.calls));
        kind
    }

    pub(crate) fn ended(&mut self, reason: Reason) -> Stop {
        self.at = None;
        Stop {
            reason,
            place: None,
            depth: 0,
            rewound: Vec::new(),
        }
    }

    /// Back to the latest checkpoint at or before `step`: the machine,
    /// the slots, the session's own state, the output's place and the
    /// store's counts as they were there.
    pub(crate) fn rewind_to(&mut self, step: usize) {
        let state = self.machine.state();
        if let Some(journal) = state.journal.as_mut().map(std::mem::take) {
            self.timeline.absorb(journal);
        }
        let Some((at, mut state, book)) = self.timeline.back_to(step) else {
            return;
        };
        state.journal = Some(Vec::new());
        *self.machine.state() = state;
        (self.now, self.hits, self.last) = (at, book.hits, book.last);
        (self.calls, self.depth) = (book.calls, book.depth);
        self.tally.back(book.out);
        xetal_store::rewind(book.mark);
    }

    /// The step of the last point before now that `wanted` accepts,
    /// searched back a checkpoint's stretch at a time; `None` when there
    /// is none (the session is then at the run's first checkpoint).
    pub(crate) fn seek(&mut self, wanted: impl Fn(&Passed) -> bool) -> Option<usize> {
        let mut end = self.now;
        self.left.get_or_insert((end, xetal_store::mark()));
        let starts: Vec<usize> = self
            .timeline
            .steps()
            .into_iter()
            .filter(|s| *s < end)
            .collect();
        for start in starts.into_iter().rev() {
            self.rewind_to(start);
            let mut found = None;
            let _ = self.walk(|now, p| {
                if now < end && wanted(p) {
                    found = Some(now);
                }
                now >= end
            });
            if found.is_some() {
                return found;
            }
            end = start + 1;
        }
        None
    }
}
