//! Expressions evaluated where the program stopped: watches, a
//! breakpoint's condition, and the hit conditions that pick which hits
//! break. A watch runs in a machine of its own, started from the paused
//! frame's bindings, with effects refused and a budget of transitions.

use std::collections::HashMap;
use xetal_arith::Rng;

use std::rc::Rc;
use xetal_core::{Item, Program};
use xetal_step::{Control, Machine, Status};

use xetal_value::{Env, Frame, Slot, Value};

use crate::here::Place;
use crate::session::Debugger;
use crate::stack::Var;
use xetal_inspect::own;

/// Transitions a watch may take before it is given up.
const BUDGET: usize = 1_000_000;

impl<'a> Debugger<'a, '_> {
    /// `text` evaluated in the frame where the program stopped.
    pub fn evaluate(&mut self, text: &str) -> Result<Var, String> {
        if text.contains('!') || text.contains("[]") {
            return Err("a watch cannot have effects (a ! function or a system name)".into());
        }
        let (env, globals, locals) = self.frame();
        let program = self.program(text, &locals)?;
        let mut kept: Vec<Value<'a>> = Vec::new();
        let status = {
            let (mut sink, mut keep) = (std::io::sink(), |v: &Value<'a>| kept.push(v.clone()));
            let machine = Machine::new(program, &mut sink, Rng::seeded(0)).within(env, globals);
            let mut machine = machine.hooks(Some(&mut keep), None);
            machine.run(BUDGET)
        };
        match status {
            Ok(Status::Done) => kept
                .last()
                .map(|v| self.var(text, v.clone()))
                .ok_or_else(|| "no value".into()),
            Ok(Status::Running) => Err(format!("still running after {BUDGET} steps")),
            Ok(_) => Err("a watch cannot read input".into()),
            Err(d) => Err(format!("error[{}]: {}", d.code, d.message)),
        }
    }

    /// Watch `text` at every stop from now on.
    pub fn add_watch(&mut self, text: &str) {
        if !self.watches.iter().any(|w| w == text) {
            self.watches.push(text.into());
        }
    }

    /// The values `text` had, as (stop number, summary), from the stop
    /// it was added at.
    pub fn watch_history(&self, text: &str) -> Vec<(usize, String)> {
        let of = |(n, w, s): &(usize, String, String)| (w == text).then(|| (*n, s.clone()));
        self.history.iter().filter_map(of).collect()
    }

    /// The stopped frame: its own bindings laid over the top level as
    /// it is now (a closure's env holds the top level of when it was
    /// made), the globals, and each local's placeholder for checking.
    fn frame(&self) -> (Env<'a>, HashMap<String, Value<'a>>, Vec<(String, String)>) {
        let view = self.machine.view();
        let (mut env, mut locals) = (view.env.clone(), Vec::new());
        if let Some(Control::Eval(_, here)) = view.control {
            for f in own(here, view.env).into_iter().rev() {
                if let Slot::Value(v) = &*f.slot.borrow()
                    && let Some(p) = xetal_watch::placeholder(v)
                    && !f.name.starts_with('#')
                {
                    locals.push((f.name.to_string(), p));
                }
                let frame = Frame {
                    name: f.name,
                    slot: f.slot.clone(),
                    next: env,
                };
                env = Some(Rc::new(frame));
            }
        }
        (env, view.globals.clone(), locals)
    }

    /// `text` checked against the program with `locals` in scope, once
    /// for each set of local types, and kept for the session.
    fn program(&mut self, text: &str, locals: &[(String, String)]) -> Result<&'a Program, String> {
        let key = format!("{text}\u{0}{locals:?}");
        if let Some(p) = self.programs.get(&key) {
            return Ok(p);
        }
        let (name, main) = (self.sources.name(0), self.sources.written_text(0));
        let expr = xetal_watch::checked(name, main, locals, text)?;
        let program = Program {
            items: vec![Item::Eval(expr)],
            notes: Vec::new(),
            names: Vec::new(),
        };
        let program: &'a Program = Box::leak(Box::new(program));
        self.programs.insert(key, program);
        Ok(program)
    }

    /// Whether the breakpoint at `here` breaks now: counted, its hit
    /// condition met, its condition true (or failing, which stops too).
    pub(crate) fn breaks(&mut self, here: &Place) -> bool {
        let key = (here.file.clone(), here.line);
        let Some(point) = self.breakpoints.get(&key).cloned() else {
            return false;
        };
        let n = self.hits.entry(key).and_modify(|n| *n += 1).or_insert(1);
        if !point.hit.as_deref().is_none_or(|h| xetal_watch::hit(h, *n)) {
            return false;
        }
        let Some(condition) = point.condition else {
            return true;
        };
        let value = self.evaluate(&condition);
        value.is_err() || value.is_ok_and(|v| v.truthy)
    }

    /// A stop is made: values from the last stop are let go, and every
    /// watch's value is recorded.
    pub(crate) fn stopped(&mut self) {
        self.refs.clear();
        self.stops += 1;
        for text in self.watches.clone() {
            let shown = match self.evaluate(&text) {
                Ok(v) => v.value,
                Err(e) => e,
            };
            self.history.push((self.stops, text, shown));
        }
    }
}
