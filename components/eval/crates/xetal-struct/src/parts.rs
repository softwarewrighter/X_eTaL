//! Partition (B14, APL2's partition): major cells cut into pieces by
//! keys, one per cell; and the shape check `r_equireShape` (type-check-macros
//! approach 8).

use xetal_array::{Array, ArrayError};
use xetal_base::Diagnostic;

use crate::cells::cell_len;

/// The pieces of `a`: a new piece starts where the key increases, and
/// a cell with key 0 is dropped (it also ends the piece before it).
pub fn partition<T: Clone>(keys: &[usize], a: &Array<T>) -> Result<Vec<Array<T>>, ArrayError> {
    let (len, cell) = (a.shape()[0], cell_len(a));
    if keys.len() != len {
        let (left, right) = (vec![keys.len()], a.shape().to_vec());
        return Err(ArrayError::Shape { left, right });
    }
    let mut starts: Vec<(usize, usize)> = Vec::new();
    let mut previous = 0;
    for (i, &k) in keys.iter().enumerate() {
        match k {
            0 => {}
            _ if k > previous || starts.is_empty() => starts.push((i, i + 1)),
            _ => {
                if let Some(last) = starts.last_mut() {
                    last.1 = i + 1;
                }
            }
        }
        previous = k;
    }
    starts
        .into_iter()
        .map(|(from, to)| {
            let mut shape = a.shape().to_vec();
            shape[0] = to - from;
            Ok(Array::new(shape, a.data()[from * cell..to * cell].to_vec())?.with_kind(a.kind()))
        })
        .collect()
}

/// `r_equireShape`: nothing when `a`'s shape is `expected`, else
/// `shape-mismatch` naming both, as a macro's emitted check reports it.
pub fn check_shape<T>(expected: &[i64], a: &Array<T>) -> Result<(), Diagnostic> {
    let same =
        expected.len() == a.rank() && expected.iter().zip(a.shape()).all(|(e, d)| *e == *d as i64);
    let words = |d: Vec<String>| d.join(" ");
    let found = words(a.shape().iter().map(ToString::to_string).collect());
    let wanted = words(expected.iter().map(ToString::to_string).collect());
    let message = format!("shape {found}, expected {wanted}");
    match same {
        true => Ok(()),
        false => Err(Diagnostic::new("shape-mismatch", message)),
    }
}
