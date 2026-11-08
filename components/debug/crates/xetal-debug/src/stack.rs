//! The call stack at a stop (each call in progress, innermost first,
//! named as its call writes it, placed where it was written), and the
//! variables there: scopes, values shown with ids, children by page.

use xetal_base::Span;
use xetal_inspect::bindings;
use xetal_sources::Sources;
use xetal_step::{Control, Kont, View};
use xetal_value::Value;

use crate::here::{Place, place};
use crate::session::Debugger;

/// One call in progress: what was called (the function's name as the
/// call writes it), and where the machine is in it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    pub name: String,
    pub place: Place,
}

/// The frames, innermost first: the stop itself, then each call site
/// out to the top level.
pub(crate) fn frames(view: &View, sources: &Sources, at: Option<Span>) -> Vec<Frame> {
    let calls: Vec<Span> = view
        .stack
        .iter()
        .filter_map(|k| match k {
            Kont::Called { span } => Some(*span),
            _ => None,
        })
        .collect();
    let mut out = Vec::new();
    let mut here = at;
    for call in calls.iter().rev() {
        if let Some(span) = here {
            out.push(Frame {
                name: text(sources, *call),
                place: place(sources, span),
            });
        }
        here = Some(*call);
    }
    if let Some(span) = here {
        out.push(Frame {
            name: "(top level)".into(),
            place: place(sources, span),
        });
    }
    out
}

/// What a call called: the first word of its text (`u:f_`), at most
/// 40 characters.
fn text(sources: &Sources, span: Span) -> String {
    let all = sources.combined();
    let raw = all.get(span.start..span.end.min(all.len())).unwrap_or("");
    let word = raw.split_whitespace().next().unwrap_or("(a call)");
    match word.chars().count() > 40 {
        true => format!("{}...", word.chars().take(37).collect::<String>()),
        false => word.to_string(),
    }
}

/// A variable (or a watch, or a child of either) as shown: its name, a
/// one-line summary of its value, its type, how many children it has
/// and the id they are opened by (0 when it has none).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Var {
    pub name: String,
    pub value: String,
    pub type_name: String,
    pub children: usize,
    pub id: usize,
    /// Whether the value reads as true (a breakpoint's condition).
    pub truthy: bool,
}

/// A group of variables: the stopped frame's locals, the top level's
/// bindings, the libraries' exports by alias.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scope {
    pub name: String,
    pub vars: Vec<Var>,
}

impl<'a> Debugger<'a, '_> {
    /// The scopes at the stop: Locals (when stopped in a call), Top
    /// level (bindings and `u:` definitions), Libraries (by alias).
    pub fn scopes(&mut self) -> Vec<Scope> {
        let view = self.machine.view();
        let here = match view.control {
            Some(Control::Eval(_, env)) => env.clone(),
            _ => None,
        };
        let (top, globals) = (view.env.clone(), view.globals.clone());
        let locals = bindings(&here, &top);
        let mut tops = bindings(&top, &None);
        let mut libs = Vec::new();
        let mut names: Vec<&String> = globals.keys().collect();
        names.sort();
        for name in names {
            let item = (name.clone(), Ok(globals[name].clone()));
            match name.split_once(':') {
                Some((ns, _)) if ns != "u" => libs.push(item),
                _ => tops.push(item),
            }
        }
        let mut scope = |name: &str, vars: Vec<(String, Result<Value<'a>, String>)>| Scope {
            name: name.into(),
            vars: vars.into_iter().map(|(n, v)| self.shown(&n, v)).collect(),
        };
        let mut out = vec![
            scope("Locals", locals),
            scope("Top level", tops),
            scope("Libraries", libs),
        ];
        out.retain(|s| !s.vars.is_empty());
        out
    }

    /// Children `start..start + count` of the value shown with `id`.
    pub fn children(&mut self, id: usize, start: usize, count: usize) -> Vec<Var> {
        let Some(v) = id.checked_sub(1).and_then(|i| self.refs.get(i)).cloned() else {
            return Vec::new();
        };
        xetal_inspect::children(&v, start, count)
            .into_iter()
            .map(|(name, child)| self.var(&name, child))
            .collect()
    }

    /// A value as shown, given an id when it has children.
    pub(crate) fn var(&mut self, name: &str, v: Value<'a>) -> Var {
        let children = xetal_inspect::count(&v);
        let (value, type_name) = (xetal_inspect::summary(&v), xetal_inspect::type_name(&v));
        let truthy = xetal_inspect::truthy(&v);
        let id = match children {
            0 => 0,
            _ => {
                self.refs.push(v);
                self.refs.len()
            }
        };
        Var {
            name: name.into(),
            value,
            type_name,
            children,
            id,
            truthy,
        }
    }

    fn shown(&mut self, name: &str, v: Result<Value<'a>, String>) -> Var {
        match v {
            Ok(v) => self.var(name, v),
            Err(note) => Var {
                name: name.into(),
                value: note,
                type_name: String::new(),
                children: 0,
                id: 0,
                truthy: false,
            },
        }
    }
}
