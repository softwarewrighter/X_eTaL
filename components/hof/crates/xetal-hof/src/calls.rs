//! Dispatch: the higher-order built-ins on runtime values.

use xetal_base::{Diagnostic, Span};
use xetal_value::{Caller, Value};

use crate::fold::{reduce, scan};

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
        _ => return None,
    };
    Some(result.map_err(|d| match d.span {
        Some(_) => d,
        None => d.with_span(span),
    }))
}
