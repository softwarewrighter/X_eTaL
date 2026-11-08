//! A value's children, a page at a time: an array's major cells (a
//! matrix's rows, a vector's items), a tuple's parts, a box's content.

use std::rc::Rc;

use xetal_array::Array;
use xetal_value::Value;

/// How many children a value has (0: a leaf).
pub fn count(v: &Value) -> usize {
    match v {
        Value::Array(a) if a.rank() == 0 => 0,
        Value::Array(a) => a.shape()[0],
        Value::Tuple(parts) => parts.len(),
        Value::Boxed(_) => 1,
        _ => 0,
    }
}

/// Children `start..start + count` (clipped), each named by its place:
/// `[1]` for a major cell (1-origin, as `s_elect` counts), `.1` for a
/// tuple's part, `>` for a box's content.
pub fn children<'a>(v: &Value<'a>, start: usize, count: usize) -> Vec<(String, Value<'a>)> {
    let end = start.saturating_add(count).min(self::count(v));
    (start..end)
        .filter_map(|i| Some((label(v, i), child(v, i)?)))
        .collect()
}

fn label(v: &Value, i: usize) -> String {
    match v {
        Value::Tuple(_) => format!(".{}", i + 1),
        Value::Boxed(_) => ">".into(),
        _ => format!("[{}]", i + 1),
    }
}

/// Child `i`: a vector's item, a higher rank's major cell (an array of
/// the remaining axes), a tuple's part, a box's content.
fn child<'a>(v: &Value<'a>, i: usize) -> Option<Value<'a>> {
    match v {
        Value::Array(a) if a.rank() == 1 => a.data().get(i).cloned(),
        Value::Array(a) => {
            let rest = a.shape()[1..].to_vec();
            let size: usize = rest.iter().product();
            let items = a.data().get(i * size..(i + 1) * size)?.to_vec();
            Array::new(rest, items)
                .ok()
                .map(|c| Value::Array(Rc::new(c)))
        }
        Value::Tuple(parts) => parts.get(i).cloned(),
        Value::Boxed(inner) => Some((**inner).clone()),
        _ => None,
    }
}
