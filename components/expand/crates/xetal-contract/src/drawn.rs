//! What a `draws:` macro gives, read as a drawing (MC32): its text is
//! one string literal, whose value is the drawing (Equations' `t_ex<`
//! gives `"\\int_{0}^{1} ..."`, LaTeX); anything else is no drawing.

use xetal_lex::{TokenKind, lex};

/// The drawing in `output`, the text a `draws:` macro gave for a call:
/// the value of the one string literal it is; `None` for any other text.
pub fn drawing(output: &str) -> Option<String> {
    match lex(output).ok()?.as_slice() {
        [t] => match &t.kind {
            TokenKind::Str(s) => Some(s.clone()),
            _ => None,
        },
        _ => None,
    }
}
