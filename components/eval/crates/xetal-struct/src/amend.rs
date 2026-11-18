//! `s a_mend (p, v)` (A8): a new array, s with its major cells at the
//! 1-origin positions p replaced by v's cells in order, or by v itself
//! when it is a single cell.

use xetal_array::{Array, ArrayError};
use xetal_base::Diagnostic;
use xetal_value::Value;

use crate::values::{as_array, as_vector, ints, to_value};

/// Amend `s` by the tuple `pv` of positions and values.
pub(crate) fn amend<'a>(s: &Value<'a>, pv: &Value<'a>) -> Result<Value<'a>, Diagnostic> {
    let Value::Tuple(parts) = pv else {
        return Err(Diagnostic::new("type", "a_mend takes (positions, values)"));
    };
    let [p, v] = &parts[..] else {
        return Err(Diagnostic::new("type", "a_mend takes (positions, values)"));
    };
    let (a, at, v) = (as_vector(s), ints(p)?, as_array(v));
    let (n, cell) = (a.shape()[0], &a.shape()[1..]);
    let len: usize = cell.iter().product();
    let one = v.shape() == cell;
    if !one && v.shape() != [&[at.data().len()], cell].concat() {
        let left = [&[at.data().len()], cell].concat();
        let right = v.shape().to_vec();
        return Err(ArrayError::Shape { left, right }.into());
    }
    let mut data = a.data().to_vec();
    for (k, &i) in at.data().iter().enumerate() {
        if i < 1 || i as usize > n {
            return Err(ArrayError::Index { index: i, len: n }.into());
        }
        let from = if one { 0 } else { k * len };
        let to = (i as usize - 1) * len;
        data[to..to + len].clone_from_slice(&v.data()[from..from + len]);
    }
    let out = Array::new(a.shape().to_vec(), data)?.with_kind(a.kind());
    Ok(to_value(out))
}
