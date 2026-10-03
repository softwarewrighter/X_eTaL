//! `i_nner` (B6): the last axis of A paired with the first axis of B,
//! as in APL and J. Each pairing is reduced by a call of `r_/`, so it
//! folds and finds identities exactly as reduce does.

use std::rc::Rc;

use xetal_array::{Array, ArrayError, size};
use xetal_base::Diagnostic;
use xetal_kernel::{Kernel, all, apply, done, then};
use xetal_value::{Prim, Value, as_array};

use crate::items::finish;

/// `x f g i_nner y` (g, the nearest operand, pairs items; f reduces).
pub fn inner<'a>(
    g: &Value<'a>,
    f: &Value<'a>,
    x: &Value<'a>,
    y: &Value<'a>,
) -> Result<Kernel<'a, Value<'a>>, Diagnostic> {
    let (a, b) = (as_array(x), as_array(y));
    let n = paired(&a, &b)?;
    let (ra, rb) = (a.rank().max(1) - 1, b.rank().min(1));
    let shape = [&a.shape()[..ra], &b.shape()[rb..]].concat();
    size(&shape)?;
    let cols: usize = b.shape()[rb..].iter().product();
    let reduce = Value::Prim(Rc::new(Prim {
        name: "r_/",
        arity: 2,
        args: vec![f.clone()],
    }));
    let at = |t: &Array<Value<'a>>, k: usize| t.data()[if t.rank() == 0 { 0 } else { k }].clone();
    let mut cells = Vec::new();
    for i in 0..a.shape()[..ra].iter().product() {
        for j in 0..cols {
            let pairs =
                (0..n).map(|k| apply(g.clone(), vec![at(&a, i * n + k), at(&b, k * cols + j)]));
            let reduce = reduce.clone();
            cells.push(then(all(pairs.collect()), move |items| {
                Ok(apply(
                    reduce,
                    vec![Value::Array(Rc::new(Array::vector(items)))],
                ))
            }));
        }
    }
    Ok(then(all(cells), move |data| {
        Ok(done(finish("i_nner", Array::new(shape, data)?)?))
    }))
}

fn paired<T>(a: &Array<T>, b: &Array<T>) -> Result<usize, ArrayError> {
    match (a.shape().last(), b.shape().first()) {
        (Some(p), Some(q)) if p == q => Ok(*p),
        (Some(p), None) => Ok(*p),
        (None, Some(q)) => Ok(*q),
        (None, None) => Ok(1),
        _ => Err(ArrayError::Shape {
            left: a.shape().to_vec(),
            right: b.shape().to_vec(),
        }),
    }
}
