//! The move-to-front rule on runtime values, applying f through the
//! evaluator's [`Caller`].

use std::rc::Rc;

use xetal_base::{Diagnostic, Span};
use xetal_value::{Caller, Value, as_array, to_value};

use crate::move_axis;

type Out<'a> = Result<Value<'a>, Diagnostic>;

/// Built-ins whose left argument is data too: moving only the right
/// argument's axis would join mismatched arrays, so `_k` is refused.
const BOTH_DATA: &[&str] = &["c_at"];

/// `f_axes` applied to `args`, the last being the data (A6).
pub fn on_axes<'a>(
    axes: &[u8],
    f: &Value<'a>,
    args: &[Value<'a>],
    span: Span,
    c: &mut dyn Caller<'a>,
) -> Out<'a> {
    let (Some((data, controls)), [k]) = (args.split_last(), axes) else {
        return Err(axis_error(
            "several axes mean something only for rotate, reduce and scan",
        ));
    };
    if let Value::Prim(p) = f
        && BOTH_DATA.contains(&p.name)
    {
        let message = format!(
            "{}_{k} would move both arguments' axes; only the right one moves",
            p.name
        );
        return Err(axis_error(&message));
    }
    let (k, x) = (usize::from(*k), as_array(data));
    let rank = x.rank();
    if k > rank.max(1) {
        return Err(axis_error(&format!(
            "axis {k} does not exist in an argument of rank {rank}"
        )));
    }
    let moved = match k {
        1 => data.clone(),
        _ => to_value(move_axis(&x, k - 1, 0)),
    };
    let mut g = f.clone();
    for a in controls {
        g = c.call(&g, a.clone(), span)?;
    }
    back(c.call(&g, moved, span)?, k, rank)
}

/// Move the leading axis back to `k` when the rank is kept; a result
/// one rank lower consumed it.
fn back(result: Value<'_>, k: usize, rank: usize) -> Out<'_> {
    let r = as_array(&result);
    match r.rank() {
        _ if k == 1 => Ok(result),
        n if n == rank => Ok(Value::Array(Rc::new(move_axis(&r, 0, k - 1)))),
        n if n + 1 == rank => Ok(result),
        n => Err(axis_error(&format!(
            "a function under an axis subscript changed the rank from {rank} to {n}"
        ))),
    }
}

fn axis_error(message: &str) -> Diagnostic {
    Diagnostic::new("axis", message)
}
