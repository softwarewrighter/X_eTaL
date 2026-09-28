//! Pure conversions between runtime values and generic arrays.

use std::rc::Rc;

use xetal_array::Array;
use xetal_base::Diagnostic;
use xetal_value::Value;

/// A value as an array: a scalar is rank 0 (one item).
pub(crate) fn as_array<'a>(v: &Value<'a>) -> Array<Value<'a>> {
    match v {
        Value::Array(a) => (**a).clone(),
        s => Array::scalar(s.clone()),
    }
}

/// A value as something with a leading axis: a scalar is a 1-vector.
pub(crate) fn as_vector<'a>(v: &Value<'a>) -> Array<Value<'a>> {
    match v {
        Value::Array(a) => (**a).clone(),
        s => Array::vector(vec![s.clone()]),
    }
}

/// An array as a value: rank 0 is its one item, a scalar.
pub(crate) fn to_value(a: Array<Value<'_>>) -> Value<'_> {
    match a.rank() {
        0 => a.data()[0].clone(),
        _ => Value::Array(Rc::new(a)),
    }
}

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
