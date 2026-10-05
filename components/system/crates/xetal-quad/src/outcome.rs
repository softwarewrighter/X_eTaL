//! A caught error and a handler's outcome (ER2): what a handler reads
//! from the `Error` it is given, and the outcome it answers with. The
//! trap itself is the machine's (`xetal-step`), since it unwinds.

use std::rc::Rc;

use xetal_base::Diagnostic;
use xetal_value::{Outcome, Value};

/// The characters of `s` as a Char vector.
fn text<'a>(s: &str) -> Value<'a> {
    xetal_value::to_value(xetal_array::Array::vector(
        s.chars().map(Value::Char).collect(),
    ))
}

/// The error a handler was given, or a domain error.
fn error<'v>(v: &'v Value<'_>) -> Result<&'v Rc<Diagnostic>, Diagnostic> {
    match v {
        Value::Error(e) => Ok(e),
        other => Err(Diagnostic::new(
            "domain",
            format!("expected an Error, got {other}"),
        )),
    }
}

/// `[]E_CODE e`, `[]E_MESSAGE e`, `[]E_WHERE e` (the span as errors
/// print it, `8..26`).
pub(crate) fn read<'a>(what: &str, e: &Value<'a>) -> Result<Value<'a>, Diagnostic> {
    let e = error(e)?;
    Ok(match what {
        "[]E_CODE" => text(&e.code),
        "[]E_MESSAGE" => text(&e.message),
        _ => text(
            &e.span
                .map_or(String::new(), |s| format!("{}..{}", s.start, s.end)),
        ),
    })
}

/// `[]R_ECOVER v`, `[]R_ETRY e`, `[]H_ALT e`.
pub(crate) fn outcome<'a>(what: &str, v: &Value<'a>) -> Result<Value<'a>, Diagnostic> {
    let outcome = match what {
        "[]R_ECOVER" => Outcome::Recover(v.clone()),
        "[]R_ETRY" => error(v).map(|_| Outcome::Retry)?,
        _ => Outcome::Halt(error(v)?.clone()),
    };
    Ok(Value::Outcome(Rc::new(outcome)))
}
