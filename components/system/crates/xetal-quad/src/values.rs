//! The read-only system values (QD2).

use xetal_array::Array;
use xetal_value::{Value, to_value};

/// A Char vector of `s`.
pub(crate) fn text<'a>(s: &str) -> Value<'a> {
    to_value(Array::vector(s.chars().map(Value::Char).collect()))
}

/// `[]AV`: every character, ASCII 0 to 127 (source is ASCII).
pub(crate) fn atomic<'a>() -> Value<'a> {
    to_value(Array::vector(
        (0u8..128).map(|b| Value::Char(char::from(b))).collect(),
    ))
}
