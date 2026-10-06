//! Events from the host (RS1): `[]E_VENT @` reads the next event's line
//! from standard input (in the live demo the worker's queue gives it
//! before this is reached), and the readers take an Event apart:
//! `[]E_KIND` its kind, `[]E_AT` its numbers, `[]E_KEY` its key.

use std::rc::Rc;

use xetal_base::Diagnostic;
use xetal_value::{Event, Value};

/// The event an Event value holds, or a domain error.
fn event<'v>(v: &'v Value<'_>) -> Result<&'v Rc<Event>, Diagnostic> {
    match v {
        Value::Event(e) => Ok(e),
        other => Err(Diagnostic::new(
            "domain",
            format!("expected an Event, got {other}"),
        )),
    }
}

/// `[]E_VENT @` without a queue: the next line of standard input as an
/// event; at the end of input, the event `end`.
pub(crate) fn next<'a>() -> Result<Value<'a>, Diagnostic> {
    let line = match xetal_store::read_line() {
        Ok(line) => line,
        Err(_) => return Ok(Value::Event(Rc::new(Event::end()))),
    };
    match Event::parse(&line) {
        Some(e) => Ok(Value::Event(Rc::new(e))),
        None => Err(Diagnostic::new(
            "bad-event",
            format!(
                "not an event: {line:?} (tick S, down X Y, move X Y, up X Y, click X Y, key NAME, choose A N, end)"
            ),
        )),
    }
}

/// `[]E_KIND e`: the kind as text; `[]E_AT e`: its numbers (`x y`, or
/// the seconds of a tick, or none); `[]E_KEY e`: its key.
pub(crate) fn read<'a>(what: &str, v: &Value<'a>) -> Result<Value<'a>, Diagnostic> {
    let e = event(v)?;
    Ok(match what {
        "[]E_KIND" => crate::values::text(e.kind),
        "[]E_AT" => xetal_value::to_value(xetal_array::Array::vector(
            e.at.iter().map(|x| Value::Float(*x)).collect(),
        )),
        _ => match e.key {
            Some(k) => Value::Tag("Key", k),
            None => {
                return Err(Diagnostic::new(
                    "domain",
                    format!("[]E_KEY: a {} event has no key", e.kind),
                ));
            }
        },
    })
}
