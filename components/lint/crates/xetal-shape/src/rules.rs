//! The shape rules of the catalog (`shape` in builtins.toml): what a
//! built-in's result is, and what it surely refuses.

use xetal_base::{Diagnostic, Span};

use crate::ext::{Ext, Fact, Shape, differ, same, show};

/// A monadic built-in's result (reduce: given its operand).
pub(crate) fn monadic(rule: &str, x: Fact) -> Fact {
    match rule {
        "scalar" => Fact::shaped(x.shape),
        "count" => Fact::shaped(Some(Vec::new())),
        "reduce" => Fact::shaped(x.shape.filter(|s| !s.is_empty()).map(|s| s[1..].to_vec())),
        "shape" => Fact {
            shape: x.shape.as_ref().map(|s| vec![Ext::Num(s.len() as i64)]),
            value: x.shape,
            parts: None,
        },
        _ => Fact::default(),
    }
}

/// A dyadic built-in's result, reporting a sure failure.
pub(crate) fn dyadic(rule: &str, [l, r]: [Fact; 2], span: Span, out: &mut Vec<Diagnostic>) -> Fact {
    match rule {
        "scalar" => Fact::shaped(items(l.shape, r.shape, span, out)),
        "reshape" => Fact::shaped(l.value),
        "cat" => {
            let scalar = |f: &Fact| f.shape.as_ref().is_some_and(|s| s.len() <= 1);
            let both = scalar(&l) && scalar(&r);
            let value = l.value.zip(r.value).filter(|_| both);
            let value = value.map(|(a, b)| [a, b].concat());
            let shape = value.as_ref().map(|v| vec![Ext::Num(v.len() as i64)]);
            Fact {
                shape,
                value,
                parts: None,
            }
        }
        "check" => check(l.value, r, span, out),
        _ => Fact::default(),
    }
}

/// Item by item: a scalar extends, other shapes must be equal; so
/// when it runs, one side known not to be a scalar gives the shape.
fn items(
    a: Option<Shape>,
    b: Option<Shape>,
    span: Span,
    out: &mut Vec<Diagnostic>,
) -> Option<Shape> {
    let array = |s: &Option<Shape>| s.clone().filter(|s| !s.is_empty());
    let (a, b) = match (a, b) {
        (Some(a), Some(b)) => (a, b),
        (a, b) => return array(&a).or(array(&b)),
    };
    if a.is_empty() || b.is_empty() {
        return Some(if a.is_empty() { b } else { a });
    }
    if differ(&a, &b) {
        let message = format!(
            "these shapes differ: {} and {}, so this fails at run time (item by item, only a single value extends)",
            show(&a),
            show(&b)
        );
        out.push(Diagnostic::warning("shape-mismatch", message).with_span(span));
        return None;
    }
    Some(a)
}

/// `r_equireShape`: the right's shape against the left's extents; after it,
/// the value has the checked shape.
fn check(want: Option<Vec<Ext>>, x: Fact, span: Span, out: &mut Vec<Diagnostic>) -> Fact {
    if let (Some(want), Some(have)) = (&want, &x.shape) {
        let (want_s, have_s) = (show(want), show(have));
        if differ(want, have) {
            let message =
                format!("this check fails at run time: the shape here is {have_s}, not {want_s}");
            out.push(Diagnostic::warning("shape-mismatch", message).with_span(span));
        } else if same(want, have) {
            let message = format!(
                "this check always passes: the shape here is already {have_s}, so it can go"
            );
            out.push(Diagnostic::warning("shape-check-proven", message).with_span(span));
        }
    }
    Fact {
        shape: want.or(x.shape),
        value: x.value,
        parts: None,
    }
}

/// `A f g i_nner B`: A's last axis is paired with B's first; a scalar
/// on either side extends.
pub(crate) fn inner([l, r]: [Fact; 2], span: Span, out: &mut Vec<Diagnostic>) -> Fact {
    let (Some(a), Some(b)) = (l.shape, r.shape) else {
        return Fact::default();
    };
    let (Some(last), Some(first)) = (a.last(), b.first()) else {
        return Fact::default();
    };
    if differ(&vec![last.clone()], &vec![first.clone()]) {
        let message = format!(
            "the inner product pairs the left's last axis ({}) with the right's first ({}): they differ, so this fails at run time",
            show(&vec![last.clone()]),
            show(&vec![first.clone()])
        );
        out.push(Diagnostic::warning("shape-mismatch", message).with_span(span));
        return Fact::default();
    }
    Fact::shaped(Some([&a[..a.len() - 1], &b[1..]].concat()))
}
