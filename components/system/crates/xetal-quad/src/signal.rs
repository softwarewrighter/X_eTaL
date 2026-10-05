//! An error of one's own (ER1): `"code" []S_IGNAL "message"`.

use xetal_base::Diagnostic;
use xetal_value::{Value, as_array};

/// The characters of a text value.
fn text(v: &Value<'_>) -> Result<String, Diagnostic> {
    as_array(v)
        .data()
        .iter()
        .map(|c| match c {
            Value::Char(c) => Ok(*c),
            other => Err(Diagnostic::new(
                "domain",
                format!("expected text, got {other}"),
            )),
        })
        .collect()
}

/// Whether `code` is spelled as xetal's own error codes are: lowercase
/// letters, digits and hyphens between them.
fn well_formed(code: &str) -> bool {
    let ok = |c: char| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-';
    !code.is_empty() && code.chars().all(ok) && !code.starts_with('-') && !code.ends_with('-')
}

/// `"code" []S_IGNAL "message"`: always an error, `error[code]: message`.
pub(crate) fn signal<'a>(code: &Value<'a>, message: &Value<'a>) -> Result<Value<'a>, Diagnostic> {
    let (code, message) = (text(code)?, text(message)?);
    if !well_formed(&code) {
        let what = format!("an error code is lowercase letters, digits and hyphens, got {code:?}");
        return Err(Diagnostic::new("bad-code", what));
    }
    Err(Diagnostic::new(code, message))
}
