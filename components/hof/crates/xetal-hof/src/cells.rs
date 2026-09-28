//! Major cells: splitting a value along its leading axis and joining
//! cells back into one value.

use xetal_array::Array;
use xetal_base::Diagnostic;
use xetal_value::{Value, as_array, as_vector, to_value};

/// The major cells of `x` (a scalar is one cell), and their shape.
pub fn major_cells<'a>(x: &Value<'a>) -> (Vec<Value<'a>>, Vec<usize>) {
    let a = as_vector(x);
    let shape = a.shape()[1..].to_vec();
    let len = shape.iter().product::<usize>();
    let cells = (0..a.shape()[0])
        .map(|i| to_value(cell(&shape, &a.data()[i * len..(i + 1) * len])))
        .collect();
    (cells, shape)
}

fn cell<'a>(shape: &[usize], items: &[Value<'a>]) -> Array<Value<'a>> {
    match Array::new(shape.to_vec(), items.to_vec()) {
        Ok(a) => a,
        Err(_) => Array::vector(items.to_vec()),
    }
}

/// Cells of one `shape` joined along a new leading axis.
pub fn join<'a>(cells: &[Value<'a>], shape: &[usize]) -> Result<Value<'a>, Diagnostic> {
    let mut data = Vec::with_capacity(cells.len() * shape.iter().product::<usize>());
    for c in cells {
        let a = as_array(c);
        if a.shape() != shape {
            return Err(Diagnostic::new(
                "shape-mismatch",
                format!(
                    "each result must be {}, got {}",
                    named(shape),
                    named(a.shape())
                ),
            ));
        }
        data.extend_from_slice(a.data());
    }
    let full = [&[cells.len()], shape].concat();
    Ok(to_value(Array::new(full, data)?))
}

/// `a scalar` or `shape 2 3`.
fn named(shape: &[usize]) -> String {
    match shape {
        [] => "a scalar".into(),
        _ => {
            let dims: Vec<String> = shape.iter().map(ToString::to_string).collect();
            format!("shape {}", dims.join(" "))
        }
    }
}
