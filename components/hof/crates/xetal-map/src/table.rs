//! `t_able`: f between every item of x and every item of y, f fixed
//! once per left item (one call), then applied to each right item; a
//! kernel (D50), or computed at once for a first-order built-in f.

use std::rc::Rc;

use xetal_array::{Array, size};
use xetal_base::Diagnostic;
use xetal_kernel::{Direct, Kernel, all, apply, done, then};
use xetal_value::{Value, as_array};

use crate::items::finish;

pub fn table<'a>(
    f: &Value<'a>,
    x: &Value<'a>,
    y: &Value<'a>,
    direct: &mut dyn Direct<'a>,
) -> Result<Kernel<'a, Value<'a>>, Diagnostic> {
    let (xs, ys) = (as_array(x), Rc::new(as_array(y)));
    let shape = [xs.shape(), ys.shape()].concat();
    size(&shape)?;
    if direct.takes(f, 2) {
        return at_once(f, &xs, &ys, shape, direct);
    }
    let rows = xs.data().iter().map(|a| {
        let ys = ys.clone();
        then(apply(f.clone(), vec![a.clone()]), move |row| {
            Ok(all(ys
                .data()
                .iter()
                .map(|b| apply(row.clone(), vec![b.clone()]))
                .collect()))
        })
    });
    Ok(then(all(rows.collect()), move |rows| {
        let data = rows.into_iter().flatten().collect();
        Ok(done(finish("t_able", Array::new(shape, data)?)?))
    }))
}

/// The table with a built-in f, each call made at once, in the
/// kernel's order (row by row).
fn at_once<'a>(
    f: &Value<'a>,
    xs: &Array<Value<'a>>,
    ys: &Array<Value<'a>>,
    shape: Vec<usize>,
    direct: &mut dyn Direct<'a>,
) -> Result<Kernel<'a, Value<'a>>, Diagnostic> {
    let mut data = Vec::with_capacity(xs.data().len() * ys.data().len());
    for a in xs.data() {
        for b in ys.data() {
            data.push(direct.call(f, &[a.clone(), b.clone()])?);
        }
    }
    Ok(done(finish("t_able", Array::new(shape, data)?)?))
}
