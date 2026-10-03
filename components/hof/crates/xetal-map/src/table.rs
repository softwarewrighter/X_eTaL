//! `t_able`: f between every item of x and every item of y, f fixed
//! once per left item (one call), then applied to each right item; a
//! kernel (D50).

use std::rc::Rc;

use xetal_array::{Array, size};
use xetal_base::Diagnostic;
use xetal_kernel::{Kernel, all, apply, done, then};
use xetal_value::{Value, as_array};

use crate::items::finish;

pub fn table<'a>(
    f: &Value<'a>,
    x: &Value<'a>,
    y: &Value<'a>,
) -> Result<Kernel<'a, Value<'a>>, Diagnostic> {
    let (xs, ys) = (as_array(x), Rc::new(as_array(y)));
    let shape = [xs.shape(), ys.shape()].concat();
    size(&shape)?;
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
