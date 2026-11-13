//! Lexer: raw ASCII source to tokens with byte spans, following
//! docs/lang-choices.md. An underlined letter (`_` directly after it)
//! makes a name a function name; a leading `ns:` names its namespace.

mod cursor;
mod literal;
mod name;
mod quad;
mod scan;
mod string;

pub use scan::lex;
pub use xetal_token::{
    Bracket, ErrorKind, FuncName, LexError, Number, SYSTEM, Side, Symbol, Token, TokenKind, Var,
    applied_parens, bracket_depths, is_fresh, ns_text,
};
