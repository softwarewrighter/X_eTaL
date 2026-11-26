//! Where a range must be its items (step 202): a built-in that cannot
//! keep it, an array's items, a guard, a kernel's operand result and a
//! printed value; `too-large` there for a range past MAX_ITEMS.

use std::borrow::Cow;
use std::rc::Rc;

use xetal_base::{Diagnostic, Span};
use xetal_value::{Value, range_array};

/// `v` with any range in it (at the top or in a tuple) made into its
/// items; borrowed when there is none.
pub fn dense<'v, 'a>(v: &'v Value<'a>) -> Result<Cow<'v, Value<'a>>, Diagnostic> {
    Ok(match v {
        Value::Range(p) => Cow::Owned(Value::Array(Rc::new(range_array(p)?))),
        Value::Tuple(parts) if parts.iter().any(lazy) => {
            let parts: Result<Vec<_>, Diagnostic> =
                parts.iter().map(|p| Ok(dense(p)?.into_owned())).collect();
            Cow::Owned(Value::Tuple(parts?.into()))
        }
        other => Cow::Borrowed(other),
    })
}

/// Arguments with every range made into its items: the same vector,
/// not copied, when none is a range (every call passes here).
pub fn dense_args<'a>(args: Vec<Value<'a>>, span: Span) -> Result<Vec<Value<'a>>, Diagnostic> {
    if !args.iter().any(lazy) {
        return Ok(args);
    }
    let all: Result<Vec<_>, Diagnostic> = args.iter().map(|a| Ok(dense(a)?.into_owned())).collect();
    all.map_err(|e| e.with_span(span))
}

/// One value with any range in it made into its items.
pub fn dense_one<'a>(v: Value<'a>, span: Span) -> Result<Value<'a>, Diagnostic> {
    if !lazy(&v) {
        return Ok(v);
    }
    dense(&v)
        .map(Cow::into_owned)
        .map_err(|e| e.with_span(span))
}

/// Whether `v` holds a range where `dense` would make it items.
fn lazy(v: &Value<'_>) -> bool {
    match v {
        Value::Range(_) => true,
        Value::Tuple(parts) => parts.iter().any(lazy),
        _ => false,
    }
}

/// A guard's condition (`c ? a; b`), a range made into its items first.
pub fn guard(v: Value<'_>, span: Span) -> Result<bool, Diagnostic> {
    xetal_arith::truth(&dense_one(v, span)?, span)
}
