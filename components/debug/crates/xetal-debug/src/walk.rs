//! The walk: the machine run one transition at a time until a stop the
//! step wants, a breakpoint, the end, a wait or an error.

use xetal_base::Span;
use xetal_step::Status;

use crate::here::{Kind, depth, next, place};
use crate::session::{Debugger, Reason, Stop};

impl Debugger<'_, '_> {
    /// Transitions until `wanted` (or a breakpoint) at a statement or
    /// an application, the end, a wait or an error.
    pub(crate) fn go(&mut self, wanted: impl Fn(Kind, usize) -> bool, why: Reason) -> Stop {
        loop {
            match self.machine.run(1) {
                Ok(Status::Running) => {}
                Ok(Status::Done) => return self.ended(Reason::End),
                Ok(_) => return self.ended(Reason::Waiting),
                Err(d) => return self.ended(Reason::Error(d)),
            }
            let view = self.machine.view();
            let ((kind, span), d) = (next(&view), depth(&view));
            let kind = self.fresh(kind, span, d);
            if kind == Kind::Other {
                self.at = self.at.or(span);
                continue;
            }
            let here = span.map(|s| place(self.sources, s));
            let hit = kind == Kind::Statement
                && here
                    .as_ref()
                    .is_some_and(|p| self.breakpoints.contains(&(p.file.clone(), p.line)));
            if hit || wanted(kind, d) {
                self.at = span;
                let reason = if hit && why != Reason::Entry {
                    Reason::Breakpoint
                } else {
                    why
                };
                return Stop {
                    reason,
                    place: here,
                    depth: d,
                };
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
        }
    }
}
