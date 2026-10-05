//! System values and character codes (QD2, QD3, QD7): the read-only
//! values `[]A`, `[]D`, `[]AV` and `[]IO`, niladic built-ins read each
//! time they are used, and `[]U_CS` (characters to codes) and
//! `[]U_CHAR` (codes to characters), one static type each; the
//! clock: `[]TS` (the local time stamp) and `[]D_L` (a delay), read
//! from the host's clock (`xetal-clock`); and `[]S_IGNAL`, an error
//! of one's own (ER1).

mod clock;
mod codes;
mod signal;
mod values;

use xetal_base::{Diagnostic, Span};
use xetal_value::Value;

/// Call the quad `name` on `args`, if it is one of these.
pub fn call<'a>(
    name: &str,
    args: &[Value<'a>],
    span: Span,
) -> Option<Result<Value<'a>, Diagnostic>> {
    let result = match (name, args) {
        ("[]A", []) => Ok(values::text("ABCDEFGHIJKLMNOPQRSTUVWXYZ")),
        ("[]D", []) => Ok(values::text("0123456789")),
        ("[]AV", []) => Ok(values::atomic()),
        ("[]IO", []) => Ok(Value::Int(1)),
        ("[]TS", []) => clock::stamp(),
        ("[]D_L", [s]) => clock::delay(s),
        ("[]U_CS", [t]) => codes::codes(t),
        ("[]U_CHAR", [n]) => codes::chars(n),
        ("[]S_IGNAL", [code, message]) => signal::signal(code, message),
        _ => return None,
    };
    Some(result.map_err(|d| d.with_span(span)))
}
