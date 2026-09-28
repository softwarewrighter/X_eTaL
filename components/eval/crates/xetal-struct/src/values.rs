//! Value helpers for the structural built-ins: integers and fills.

use xetal_array::Array;
use xetal_base::Diagnostic;
use xetal_value::Value;

pub(crate) use xetal_value::{as_array, as_vector, to_value};

/// Integers (Bool counts as 1 / 0, T1), keeping the shape.
pub(crate) fn ints(v: &Value<'_>) -> Result<Array<i64>, Diagnostic> {
    as_array(v).map(|x| match x {
        Value::Int(i) => Ok(*i),
        Value::Bool(b) => Ok(i64::from(*b)),
        other => Err(Diagnostic::new(
            "not-an-integer",
            format!("expected integers, got {other}"),
        )),
    })
}

/// The fill for padding (B10), taken from the items: 0, 0.0 or a space.
pub(crate) fn fill<'a>(a: &Array<Value<'a>>) -> Option<Value<'a>> {
    Some(match a.data().first()? {
        Value::Int(_) => Value::Int(0),
        Value::Bool(_) => Value::Bool(false),
        Value::Float(_) => Value::Float(0.0),
        Value::Char(_) => Value::Char(' '),
        _ => return None,
    })
}
