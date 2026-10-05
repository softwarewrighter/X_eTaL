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

/// `default []W_ARN ("code"; "message")` (ER3): the error of the pair (a 2-item `Box Char`, a strand of two strings),
/// which the machine raises resumably with the default. Here it is
/// always the error: the machine catches it before it would end the
/// run, and a `[]W_ARN` that reaches no trap is an error like a signal.
pub(crate) fn warn<'a>(what: &Value<'a>) -> Result<Value<'a>, Diagnostic> {
    let items = as_array(what);
    let items = items.data();
    let [code, message] = items else {
        let what = format!("[]W_ARN takes (code; message), got {} items", items.len());
        return Err(Diagnostic::new("domain", what));
    };
    let open = |v: &Value<'a>| match v {
        Value::Boxed(b) => b.as_ref().clone(),
        other => other.clone(),
    };
    signal(&open(code), &open(message))
}
