//! `m_ix` (A9, APL2's first-axis mix): an array of boxes as one array,
//! the boxes' axes first and the items' after, each item raised to the
//! greatest rank by leading axes of length 1 and padded at the end of
//! each axis with its kind's fill.

use std::rc::Rc;

use xetal_array::{Array, Kind};
use xetal_base::Diagnostic;
use xetal_value::Value;

use xetal_value::{as_array, to_value};

/// Mix the boxes of `x`.
pub fn mix<'a>(x: &Value<'a>) -> Result<Value<'a>, Diagnostic> {
    let outer = as_array(x);
    let items: Vec<Array<Value<'a>>> = outer
        .data()
        .iter()
        .map(|v| match v {
            Value::Boxed(inner) => Ok(as_array(inner)),
            other => Err(Diagnostic::new(
                "type",
                format!("m_ix takes boxes, got {other}"),
            )),
        })
        .collect::<Result<_, _>>()?;
    let rank = items.iter().map(|a| a.rank()).max().unwrap_or(0);
    let shapes: Vec<Vec<usize>> = items.iter().map(|a| raised(a.shape(), rank)).collect();
    let most: Vec<usize> = (0..rank)
        .map(|k| shapes.iter().map(|s| s[k]).max().unwrap_or(0))
        .collect();
    let kind = items.iter().map(|a| a.kind()).find(|k| *k != Kind::Number);
    let kind = kind.unwrap_or(Kind::Number);
    let mut data = Vec::new();
    for (a, shape) in items.iter().zip(&shapes) {
        padded(a.data(), shape, &most, &fill(kind), &mut data);
    }
    let shape = [outer.shape(), &most[..]].concat();
    Ok(to_value(Array::new(shape, data)?.with_kind(kind)))
}

/// `shape` with leading axes of length 1 up to `rank`.
fn raised(shape: &[usize], rank: usize) -> Vec<usize> {
    let mut s = vec![1; rank - shape.len()];
    s.extend_from_slice(shape);
    s
}

/// The fill of a kind (T9): 0, a space, an empty box.
fn fill<'a>(kind: Kind) -> Value<'a> {
    match kind {
        Kind::Number => Value::Int(0),
        Kind::Char => Value::Char(' '),
        Kind::Box => {
            let empty = Array::vector(Vec::new()).with_kind(Kind::Number);
            Value::Boxed(Rc::new(to_value(empty)))
        }
    }
}

/// The items of an array of `shape` laid out in `most`, padded with
/// `fill`, appended to `out`.
fn padded<'a>(
    items: &[Value<'a>],
    shape: &[usize],
    most: &[usize],
    fill: &Value<'a>,
    out: &mut Vec<Value<'a>>,
) {
    let Some((&n, rest)) = most.split_first() else {
        out.push(items.first().cloned().unwrap_or_else(|| fill.clone()));
        return;
    };
    let len: usize = shape[1..].iter().product();
    for i in 0..n {
        if i < shape[0] {
            padded(&items[i * len..(i + 1) * len], &shape[1..], rest, fill, out);
        } else {
            padded(&[], &vec![0; rest.len()], rest, fill, out);
        }
    }
}
