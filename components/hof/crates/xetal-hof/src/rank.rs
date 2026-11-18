//! The rank form (A10): `k 'f r_ank x` applies f to each rank-k cell of
//! x, one call at a time (a kernel, D50), and mixes the results along
//! x's other axes as `m_ix` does.

use std::rc::Rc;

use xetal_array::Array;
use xetal_base::Diagnostic;
use xetal_kernel::{Kernel, apply, done, fold, then};
use xetal_value::{Value, as_array, to_value};

/// f on each rank-`k` cell of `x`.
pub fn rank<'a>(
    f: &Value<'a>,
    k: &Value<'a>,
    x: &Value<'a>,
) -> Result<Kernel<'a, Value<'a>>, Diagnostic> {
    let Value::Int(k) = k else {
        return Err(Diagnostic::new("domain", "a rank is a whole number"));
    };
    let a = as_array(x);
    let r = a.rank() as i64;
    let cell = if *k >= 0 { (*k).min(r) } else { (r + k).max(0) } as usize;
    let split = a.rank() - cell;
    let (frame, shape) = (a.shape()[..split].to_vec(), a.shape()[split..].to_vec());
    let len: usize = shape.iter().product();
    let cells: Vec<Value<'a>> = (0..frame.iter().product::<usize>())
        .map(|i| piece(&a, &shape, i * len, len))
        .collect::<Result<_, _>>()?;
    let f = f.clone();
    let results = fold(Vec::new(), cells, move |mut got: Vec<Value<'a>>, c| {
        then(apply(f.clone(), vec![c]), move |y| {
            got.push(Value::Boxed(Rc::new(y)));
            Ok(done(got))
        })
    });
    Ok(then(results, move |got| {
        let boxes = to_value(Array::new(frame, got)?.with_kind(xetal_array::Kind::Box));
        Ok(done(xetal_assemble::mix(&boxes)?))
    }))
}

/// The cell of `shape` starting at item `at`.
fn piece<'a>(
    a: &Array<Value<'a>>,
    shape: &[usize],
    at: usize,
    len: usize,
) -> Result<Value<'a>, Diagnostic> {
    let items = a.data()[at..at + len].to_vec();
    Ok(to_value(
        Array::new(shape.to_vec(), items)?.with_kind(a.kind()),
    ))
}
