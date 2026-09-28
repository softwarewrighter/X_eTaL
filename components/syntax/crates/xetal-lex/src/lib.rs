//! Lexer: raw ASCII source to tokens with byte spans, following
//! docs/lang-choices.md. An underlined letter (`_` directly after it)
//! makes a name a function name; a leading `ns:` names its namespace.

mod cursor;
mod error;
mod literal;
mod name;
mod scan;
mod token;

pub use error::{ErrorKind, LexError};
pub use scan::lex;
pub use token::{FuncName, Number, Side, Symbol, Token, TokenKind, Var};
