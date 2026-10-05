//! Characters and their codes (QD7): `[]U_CS` and `[]U_CHAR`, item by
//! item, keeping the shape.

use xetal_base::Diagnostic;
use xetal_value::{Value, as_array, to_value};

/// `[]U_CS t`: the code of each character of `t`.
pub(crate) fn codes<'a>(t: &Value<'a>) -> Result<Value<'a>, Diagnostic> {
    let codes = as_array(t).map(|c| match c {
        Value::Char(c) => Ok(Value::Int(i64::from(u32::from(*c)))),
        other => Err(Diagnostic::new(
            "domain",
            format!("[]U_CS takes characters, got {other}"),
        )),
    })?;
    Ok(to_value(codes))
}

/// `[]U_CHAR n`: the character of each code of `n`, 0 to 127.
pub(crate) fn chars<'a>(n: &Value<'a>) -> Result<Value<'a>, Diagnostic> {
    let chars = as_array(n).map(|v| match v {
        Value::Int(i) if (0..128).contains(i) => Ok(Value::Char(char::from(*i as u8))),
        other => Err(Diagnostic::new(
            "domain",
            format!("[]U_CHAR takes codes 0 to 127, got {other}"),
        )),
    })?;
    Ok(to_value(chars))
}
