//! Reduce and scan along the leading axis (B6). Reduce is a right fold
//! (`'- r_/ 1 2 3` is 1 - (2 - 3)), taken from the last cell in one
//! pass; item k of a scan is the reduce of the first k cells. Both are
//! kernels (D50).

use std::rc::Rc;

use xetal_base::Diagnostic;
use xetal_kernel::{Kernel, apply, done, fold, then};
use xetal_value::Value;

use crate::cells::join;
use crate::identity::{associative, identity};
use xetal_value::major_cells;

type Out<'a> = Result<Kernel<'a, Value<'a>>, Diagnostic>;

/// `f r_/ x`; an empty leading axis gives f's identity (B6).
pub(crate) fn reduce<'a>(f: &Value<'a>, x: &Value<'a>) -> Out<'a> {
    let (cells, shape) = major_cells(x);
    match cells.split_last() {
        Some((last, rest)) => Ok(right_fold(f, last.clone(), rest.to_vec())),
        None => Ok(done(identity(f, &shape)?)),
    }
}

/// `f` folded from `last` leftwards over `rest`: cell f (cell f ...).
fn right_fold<'a>(f: &Value<'a>, last: Value<'a>, rest: Vec<Value<'a>>) -> Kernel<'a, Value<'a>> {
    let f = f.clone();
    fold(last, rest.into_iter().rev(), move |acc, cell| {
        apply(f.clone(), vec![cell, acc])
    })
}

/// `f s_\ x`: the prefix reductions, with the shape of `x`.
pub(crate) fn scan<'a>(f: &Value<'a>, x: &Value<'a>) -> Out<'a> {
    let (cells, shape) = major_cells(x);
    let (running, scalar) = (associative(f, x), !matches!(x, Value::Array(_)));
    let (n, cells, f) = (cells.len(), Rc::new(cells), f.clone());
    let prefixes = fold(
        Vec::with_capacity(n),
        0..n,
        move |out: Vec<Value<'a>>, k| {
            let cell = cells[k].clone();
            let next = match (k, out.last()) {
                (0, _) => done(cell),
                (_, Some(prev)) if running => apply(f.clone(), vec![prev.clone(), cell]),
                _ => right_fold(&f, cell, cells[..k].to_vec()),
            };
            then(next, move |v| {
                let mut out = out;
                out.push(v);
                Ok(done(out))
            })
        },
    );
    Ok(then(prefixes, move |out| {
        Ok(done(match scalar {
            false => join(&out, &shape)?,
            true => out
                .into_iter()
                .next()
                .ok_or_else(|| Diagnostic::new("internal", "an empty scan of a scalar"))?,
        }))
    }))
}
