//! Membership by hashing (ask G10): when both sides hold only numbers
//! and characters, each item becomes a key that is equal exactly when
//! `=` says the items are equal, so a long vector against a long vector
//! costs one pass over each instead of every pair.

use std::collections::HashSet;

use xetal_value::Value;

/// What `=` compares: a number that is a whole number within 2^53 as
/// that integer (an Int, a Bool, an integral Float), another Float by
/// its bits, a Char by its code.
#[derive(PartialEq, Eq, Hash)]
enum Key {
    Whole(i64),
    Float(u64),
    Char(char),
}

/// 2^53: past it a Float no longer holds every integer, and an Int is
/// compared with a Float after rounding.
const EXACT: i64 = 1 << 53;

fn key(v: &Value<'_>) -> Option<Key> {
    match v {
        Value::Int(i) => Some(Key::Whole(*i)),
        Value::Bool(b) => Some(Key::Whole(i64::from(*b))),
        Value::Char(c) => Some(Key::Char(*c)),
        Value::Float(x) if x.is_nan() => None,
        Value::Float(x) if x.fract() == 0.0 && x.abs() <= EXACT as f64 => {
            Some(Key::Whole(*x as i64))
        }
        Value::Float(x) => Some(Key::Float(x.to_bits())),
        _ => None,
    }
}

/// Whether each item of `xs` is among `ys`, or None where hashing would
/// not agree with `=` (the caller compares instead).
pub(crate) fn member(xs: &[Value<'_>], ys: &[Value<'_>]) -> Option<Vec<bool>> {
    let float = |v: &Value<'_>| matches!(v, Value::Float(_));
    let big = |v: &Value<'_>| matches!(v, Value::Int(i) if i.abs() > EXACT);
    let floats = xs.iter().any(float) || ys.iter().any(float);
    if floats && (xs.iter().any(big) || ys.iter().any(big)) {
        return None;
    }
    let found: HashSet<Key> = ys.iter().map(key).collect::<Option<_>>()?;
    let wanted: Vec<Key> = xs.iter().map(key).collect::<Option<_>>()?;
    Some(wanted.iter().map(|k| found.contains(k)).collect())
}
