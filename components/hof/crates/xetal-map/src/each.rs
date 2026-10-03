//! `e_ach` (B6) and `m_ap` (B14). Dyadic each is currying: when f applied to the items
//! gives functions, `f e_ach A` is a pending item-wise application
//! (`#each`), and [`zip`] applies it to the next argument item by item.
//! Each is a kernel (D50): one call of f per item, in order.

use std::rc::Rc;

use xetal_array::{Array, ArrayError};
use xetal_base::Diagnostic;
use xetal_kernel::{Kernel, all, apply, done, then};
use xetal_value::{Prim, Value, as_array, to_value};

use crate::items::{finish, is_function, takes_two};

type Out<'a> = Result<Kernel<'a, Value<'a>>, Diagnostic>;

/// f applied to each item, the results in the items' shape.
fn calls<'a>(f: &Value<'a>, x: &Value<'a>) -> (Vec<usize>, Kernel<'a, Vec<Value<'a>>>) {
    let items = as_array(x);
    let calls = items
        .data()
        .iter()
        .map(|item| apply(f.clone(), vec![item.clone()]));
    (items.shape().to_vec(), all(calls.collect()))
}

pub fn each<'a>(f: &Value<'a>, x: &Value<'a>) -> Out<'a> {
    let (shape, results) = calls(f, x);
    let f = f.clone();
    Ok(then(results, move |data| {
        let results = Array::new(shape, data)?;
        let pending = match results.data().first() {
            Some(first) => is_function(first),
            None => takes_two(&f),
        };
        Ok(done(match pending {
            true => Value::Prim(Rc::new(Prim {
                name: "#each",
                arity: 2,
                args: vec![to_value(results)],
            })),
            false => finish("e_ach", results)?,
        }))
    }))
}

pub fn map<'a>(f: &Value<'a>, x: &Value<'a>) -> Out<'a> {
    let (shape, results) = calls(f, x);
    Ok(then(results, move |data| {
        let boxed = data.into_iter().map(|v| Value::Boxed(Rc::new(v))).collect();
        Ok(done(to_value(Array::new(shape, boxed)?)))
    }))
}

pub fn zip<'a>(fs: &Value<'a>, y: &Value<'a>) -> Out<'a> {
    let (fs, ys) = (as_array(fs), as_array(y));
    let shape = match (fs.rank(), ys.rank()) {
        (0, _) => ys.shape().to_vec(),
        (_, 0) => fs.shape().to_vec(),
        _ if fs.shape() == ys.shape() => fs.shape().to_vec(),
        _ => {
            let (left, right) = (fs.shape().to_vec(), ys.shape().to_vec());
            return Err(ArrayError::Shape { left, right }.into());
        }
    };
    let at = |a: &Array<Value<'a>>, i: usize| a.data()[if a.rank() == 0 { 0 } else { i }].clone();
    let n = shape.iter().product();
    let calls = (0..n)
        .map(|i| apply(at(&fs, i), vec![at(&ys, i)]))
        .collect();
    Ok(then(all(calls), move |data| {
        Ok(done(finish("e_ach", Array::new(shape, data)?)?))
    }))
}
