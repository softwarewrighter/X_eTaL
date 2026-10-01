//! The structural built-ins on runtime values (B4, B5, B10): each arm
//! converts its arguments and hands them to a generic kernel.

use xetal_array::{Array, size};
use xetal_base::{Diagnostic, Span};
use xetal_value::Value;

use crate::values::{as_array, as_vector, counts, fill, ints, to_value};
use crate::{cat, drop, first, replicate, reshape, select, take};

type Out<'a> = Result<Value<'a>, Diagnostic>;

/// Call the structural built-in `name`, if it is one.
pub fn call<'a>(name: &str, args: &[Value<'a>], span: Span) -> Option<Out<'a>> {
    let result = match (name, args) {
        ("s_hape", [x]) => Ok(shape(x)),
        ("t_ally", [x]) => Ok(Value::Int(as_vector(x).shape()[0] as i64)),
        ("r_avel", [x]) => Ok(to_value(Array::vector(as_array(x).data().to_vec()))),
        ("f_irst", [x]) => first(&as_vector(x)).map(to_value).map_err(Into::into),
        ("r_ange", [n]) => range(n, 1),
        ("o_ffsets", [n]) => range(n, 0),
        ("r_eshape", [s, x]) => dims(s).and_then(|d| Ok(to_value(reshape(d, as_array(x).data())?))),
        ("t_ake", [n, x]) => count(n).and_then(|k| {
            let a = as_vector(x);
            Ok(to_value(take(k, &a, fill(&a))?))
        }),
        ("d_rop", [n, x]) => count(n).map(|k| to_value(drop(k, &as_vector(x)))),
        ("s_elect", [i, x]) => ints(i).and_then(|i| Ok(to_value(select(&i, &as_vector(x))?))),
        ("r_eplicate", [c, x]) => {
            let a = as_vector(x);
            counts(c, a.shape()[0]).and_then(|k| Ok(to_value(replicate(&k, &a)?)))
        }
        ("c_at", [a, b]) => join(a, b),
        _ => return None,
    };
    Some(result.map_err(|d| match d.span {
        Some(_) => d,
        None => d.with_span(span),
    }))
}

fn shape<'a>(x: &Value<'a>) -> Value<'a> {
    let dims = as_array(x)
        .shape()
        .iter()
        .map(|d| Value::Int(*d as i64))
        .collect();
    to_value(Array::vector(dims))
}

/// A single integer: a count or a position.
fn count(n: &Value<'_>) -> Result<i64, Diagnostic> {
    let n = ints(n)?;
    match n.data() {
        [k] if n.rank() == 0 => Ok(*k),
        _ => Err(Diagnostic::new("not-a-scalar", "expected a single integer")),
    }
}

/// A shape: integers of zero or more.
fn dims(s: &Value<'_>) -> Result<Vec<usize>, Diagnostic> {
    let bad = |d: &i64| Diagnostic::new("domain", format!("a shape cannot hold {d}"));
    ints(s)?
        .data()
        .iter()
        .map(|d| usize::try_from(*d).map_err(|_| bad(d)))
        .collect()
}

/// `r_ange n` (from 1, A5) and `o_ffsets n` (from 0, B5).
fn range<'a>(n: &Value<'a>, from: i64) -> Out<'a> {
    let k = count(n)?;
    let len = usize::try_from(k)
        .map_err(|_| Diagnostic::new("domain", format!("a count cannot be {k}")))?;
    size(&[len])?;
    Ok(to_value(Array::vector(
        (from..from + k).map(Value::Int).collect(),
    )))
}

/// `c_at`: a scalar extends to one major cell of the other (B10).
fn join<'a>(a: &Value<'a>, b: &Value<'a>) -> Out<'a> {
    let (a, b) = (as_array(a), as_array(b));
    let cell = |x: &Array<Value<'a>>, other: &Array<Value<'a>>| match (x.rank(), other.rank()) {
        (0, r) if r > 1 => reshape(other.shape()[1..].to_vec(), x.data()),
        _ => Ok(x.clone()),
    };
    Ok(to_value(cat(&cell(&a, &b)?, &cell(&b, &a)?)?))
}
