//! Selecting from a progression, and where an Int stands in one.

use xetal_progression::Progression;
use xetal_value::Value;

pub(crate) fn call<'a>(name: &str, args: &[Value<'a>]) -> Option<Value<'a>> {
    match (name, args) {
        ("s_elect", [Value::Int(i), Value::Range(p)]) => item(p, *i).map(Value::Int),
        ("s_elect", [Value::Range(q), Value::Range(p)]) => picked(p, q).map(Value::Range),
        ("m_ember?", [Value::Int(x), Value::Range(p)]) => {
            Some(Value::Bool(p.position(*x).is_some()))
        }
        ("i_ndexOf", [Value::Range(p), Value::Int(x)]) => {
            let at = p.position(*x).unwrap_or(p.count());
            i64::try_from(at + 1).ok().map(Value::Int)
        }
        _ => None,
    }
}

/// Item `i` of `p`, 1-origin (A5).
fn item(p: &Progression, i: i64) -> Option<i64> {
    p.at(usize::try_from(i.checked_sub(1)?).ok()?)
}

/// `q s_elect p` when every index in q is in p: a progression again.
fn picked(p: &Progression, q: &Progression) -> Option<Progression> {
    if q.count() == 0 {
        return Progression::new(0, 1, 0);
    }
    let first = item(p, q.start())?;
    item(p, q.at(q.count() - 1)?)?;
    Progression::new(first, p.step().checked_mul(q.step())?, q.count())
}
