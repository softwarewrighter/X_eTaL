//! Lexer: raw ASCII source to tokens with byte spans.
//!
//! Decoration decides a name's class: a plain stem is a noun; an
//! underline, subscript or superscript (or a symbol stem) makes it a
//! function. See docs/design.md section 2.

mod cursor;
mod error;
mod name;
mod number;
mod scan;
mod token;

pub use error::{ErrorKind, LexError};
pub use scan::lex;
pub use token::{Name, Number, Side, Sub, Token, TokenKind};

/// Pipeline stage name used in diagnostics and by the CLI.
pub const STAGE: &str = "lex";
