//! `i_nner` (B6): the last axis of A paired with the first axis of B,
//! as in APL and J. Each pairing is reduced by a call of `r_/`, so it
//! folds and finds identities exactly as reduce does; with first-order
//! built-ins for both operands it is computed at once, folding as
//! reduce does (right to left).

use std::rc::Rc;

use xetal_array::{Array, ArrayError, size};
use xetal_base::Diagnostic;
use xetal_kernel::{Direct, Kernel, all, apply, done, then};
use xetal_value::{Prim, Value, as_array};

use crate::items::finish;

/// `x f g i_nner y` (g, the nearest operand, pairs items; f reduces).
pub fn inner<'a>(
    g: &Value<'a>,
    f: &Value<'a>,
    x: &Value<'a>,
    y: &Value<'a>,
    direct: &mut dyn Direct<'a>,
) -> Result<Kernel<'a, Value<'a>>, Diagnostic> {
    let (a, b) = (as_array(x), as_array(y));
    let n = paired(&a, &b)?;
    let (ra, rb) = (a.rank().max(1) - 1, b.rank().min(1));
    let shape = [&a.shape()[..ra], &b.shape()[rb..]].concat();
    size(&shape)?;
    let (rows, cols) = (
        a.shape()[..ra].iter().product::<usize>(),
        b.shape()[rb..].iter().product::<usize>(),
    );
    let at = |t: &Array<Value<'a>>, k: usize| t.data()[if t.rank() == 0 { 0 } else { k }].clone();
    let pair = |i: usize, j: usize, k: usize| [at(&a, i * n + k), at(&b, k * cols + j)];
    let cells = (0..rows).flat_map(|i| (0..cols).map(move |j| (i, j)));
    let data = match n > 0 && direct.takes(g, 2) && direct.takes(f, 2) {
        true => return at_once((g, f), cells, n, pair, direct, shape),
        false => cells.map(|(i, j)| cell(g, f, (0..n).map(|k| pair(i, j, k)))),
    };
    Ok(then(all(data.collect()), move |data| {
        Ok(done(finish("i_nner", Array::new(shape, data)?)?))
    }))
}

/// One cell through the evaluator: g on each pair, then `f r_/`.
fn cell<'a>(
    g: &Value<'a>,
    f: &Value<'a>,
    pairs: impl Iterator<Item = [Value<'a>; 2]>,
) -> Kernel<'a, Value<'a>> {
    let reduce = Value::Prim(Rc::new(Prim {
        name: "r_/",
        arity: 2,
        args: vec![f.clone()],
    }));
    let items = pairs.map(|[x, y]| apply(g.clone(), vec![x, y]));
    then(all(items.collect()), move |items| {
        Ok(apply(
            reduce,
            vec![Value::Array(Rc::new(Array::vector(items)))],
        ))
    })
}

/// Every cell with built-in operands, each call made at once in the
/// kernel's order: the pairs left to right, then the right fold.
fn at_once<'a>(
    (g, f): (&Value<'a>, &Value<'a>),
    cells: impl Iterator<Item = (usize, usize)>,
    n: usize,
    pair: impl Fn(usize, usize, usize) -> [Value<'a>; 2],
    direct: &mut dyn Direct<'a>,
    shape: Vec<usize>,
) -> Result<Kernel<'a, Value<'a>>, Diagnostic> {
    let mut data = Vec::new();
    for (i, j) in cells {
        let items = (0..n)
            .map(|k| direct.call(g, &pair(i, j, k)))
            .collect::<Result<Vec<_>, _>>()?;
        let Some((last, rest)) = items.split_last() else {
            return Err(Diagnostic::new("internal", "i_nner at once with no pairs"));
        };
        let mut acc = last.clone();
        for item in rest.iter().rev() {
            acc = direct.call(f, &[item.clone(), acc])?;
        }
        data.push(acc);
    }
    Ok(done(finish("i_nner", Array::new(shape, data)?)?))
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
