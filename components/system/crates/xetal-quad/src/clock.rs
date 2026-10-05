//! The clock (QD2, QD3): `[]TS` and `[]D_L`, through the host's clock.

use xetal_array::Array;
use xetal_base::Diagnostic;
use xetal_value::{Value, to_value};

/// `[]TS`: the local time stamp, seven Ints.
pub(crate) fn stamp<'a>() -> Result<Value<'a>, Diagnostic> {
    let now = xetal_clock::now().map_err(|e| Diagnostic::new("no-clock", e))?;
    Ok(to_value(Array::vector(
        now.iter().map(|i| Value::Int(*i)).collect(),
    )))
}

/// `[]D_L s`: wait s seconds (an Int or a Float); the seconds waited.
pub(crate) fn delay<'a>(s: &Value<'a>) -> Result<Value<'a>, Diagnostic> {
    let seconds = match s {
        Value::Int(i) => *i as f64,
        Value::Float(x) => *x,
        other => {
            return Err(Diagnostic::new(
                "domain",
                format!("[]D_L takes a number of seconds, got {other}"),
            ));
        }
    };
    let code = if seconds.is_finite() && seconds >= 0.0 {
        "no-clock"
    } else {
        "domain"
    };
    let waited = xetal_clock::delay(seconds).map_err(|e| Diagnostic::new(code, e))?;
    Ok(Value::Float(waited))
}
