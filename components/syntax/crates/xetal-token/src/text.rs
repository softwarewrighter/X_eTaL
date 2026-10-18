//! A string token's text, whichever way the string was written.

use crate::TokenKind;

impl TokenKind {
    /// The text of a string token, plain, raw or code: what a string
    /// literal's value is, whichever way it was written.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            TokenKind::Str(s) | TokenKind::Code(s) => Some(s),
            _ => None,
        }
    }
}
