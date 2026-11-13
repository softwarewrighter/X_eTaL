//! Tokens and lexer errors: what the lexer (`xetal-lex`, which
//! re-exports them) produces from raw ASCII source.

mod error;
mod fresh;
mod groups;
mod text;
mod token;

pub use error::{ErrorKind, LexError};
pub use fresh::is_fresh;
pub use groups::{Bracket, applied_parens, bracket_depths};
pub use token::{FuncName, Number, SYSTEM, Side, Symbol, Token, TokenKind, Var, ns_text};
