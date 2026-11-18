//! The windows of `n_wise` and their reductions.

use xetal_array::{Array, ArrayError};
use xetal_base::Diagnostic;
use xetal_kernel::{Kernel, apply, done, fold, then};
use xetal_value::{Value, as_array, major_cells, to_value};

type Out<'a> = Result<Kernel<'a, Value<'a>>, Diagnostic>;

/// f reduced over each window of `n` major cells of `x`; `unit` gives
/// f's identity shaped like one cell, for windows of none (n of 0).
pub fn n_wise<'a>(
    f: &Value<'a>,
    n: &Value<'a>,
    x: &Value<'a>,
    unit: impl FnOnce(&[usize]) -> Result<Value<'a>, Diagnostic>,
) -> Out<'a> {
    let Value::Int(n) = n else {
        return Err(Diagnostic::new("domain", "a window is a whole number"));
    };
    let (cells, shape) = major_cells(x);
    let (len, width) = (cells.len(), n.unsigned_abs() as usize);
    if width > len + 1 {
        let message = format!("a window of {width} over {len} items");
        return Err(Diagnostic::new("length", message));
    }
    let count = len + 1 - width;
    if width == 0 {
        let one = unit(&shape)?;
        return Ok(done(joined(vec![one; count], &shape)?));
    }
    let windows: Vec<Vec<Value<'a>>> = (0..count)
        .map(|i| {
            let mut w = cells[i..i + width].to_vec();
            if *n < 0 {
                w.reverse();
            }
            w
        })
        .collect();
    let f = f.clone();
    let results = fold(Vec::new(), windows, move |mut got: Vec<Value<'a>>, w| {
        then(reduced(&f, w), move |y| {
            got.push(y);
            Ok(done(got))
        })
    });
    Ok(then(results, move |got| Ok(done(joined(got, &shape)?))))
}

/// One window folded from the right: c1 f (c2 f (... f cn)).
fn reduced<'a>(f: &Value<'a>, mut w: Vec<Value<'a>>) -> Kernel<'a, Value<'a>> {
    let last = w.pop().unwrap_or(Value::Unit);
    w.reverse();
    let f = f.clone();
    fold(last, w, move |acc, cell| apply(f.clone(), vec![cell, acc]))
}

/// The results along a new leading axis; each must have one shape.
fn joined<'a>(got: Vec<Value<'a>>, cell: &[usize]) -> Result<Value<'a>, Diagnostic> {
    let shape = got.first().map(|v| as_array(v).shape().to_vec());
    let shape = shape.unwrap_or_else(|| cell.to_vec());
    let mut data = Vec::new();
    for v in &got {
        let a = as_array(v);
        if a.shape() != shape {
            let (left, right) = (shape.clone(), a.shape().to_vec());
            return Err(ArrayError::Shape { left, right }.into());
        }
        data.extend_from_slice(a.data());
    }
    let full = [&[got.len()], &shape[..]].concat();
    Ok(to_value(Array::new(full, data)?))
}
