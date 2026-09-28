//! Dispatch: the higher-order built-ins on runtime values.

use xetal_base::{Diagnostic, Span};
use xetal_value::{Caller, Value};

use crate::fold::{reduce, scan};
use xetal_map::{each, inner, table, zip};

type Out<'a> = Result<Value<'a>, Diagnostic>;

/// Call the higher-order built-in `name` with its arguments, applying
/// operands through `c`, if it is one.
pub fn call<'a>(
    name: &str,
    args: &[Value<'a>],
    span: Span,
    c: &mut dyn Caller<'a>,
) -> Option<Out<'a>> {
    let result = match (name, args) {
        ("r_/", [f, x]) => reduce(f, x, span, c),
        ("s_\\", [f, x]) => scan(f, x, span, c),
        ("e_ach", [f, x]) => each(f, x, span, c),
        ("#each", [fs, y]) => zip(fs, y, span, c),
        ("t_able", [f, x, y]) => table(f, x, y, span, c),
        ("i_nner", [g, f, x, y]) => inner(g, f, x, y, span, c),
        ("c_ompose", [g, f, x]) => c
            .call(g, x.clone(), span)
            .and_then(|gx| c.call(f, gx, span)),
        ("s_wap", [f, x, y]) => c.call2(f, y.clone(), x.clone(), span),
        _ => return None,
    };
    Some(result.map_err(|d| match d.span {
        Some(_) => d,
        None => d.with_span(span),
    }))
}
