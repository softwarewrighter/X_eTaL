//! The main scanning loop: whitespace, punctuation, lambda arguments and
//! the negative-literal rule; names and numbers are delegated.

use xetal_base::Span;

use crate::cursor::Cursor;
use crate::error::{ErrorKind, LexError};
use crate::token::{Side, Token, TokenKind};
use crate::{name, number};

/// Lex `src` into tokens. Spaces, tabs and carriage returns separate
/// tokens; each `\n` is a `Newline` token.
pub fn lex(src: &str) -> Result<Vec<Token>, LexError> {
    let mut cur = Cursor::new(src);
    let mut tokens = Vec::new();
    loop {
        cur.eat_while(|b| matches!(b, b' ' | b'\t' | b'\r'));
        let start = cur.pos;
        let Some(byte) = cur.peek() else {
            return Ok(tokens);
        };
        let kind = next_token(&mut cur, byte)?;
        tokens.push(Token {
            kind,
            span: Span::new(start, cur.pos),
        });
    }
}

fn next_token(cur: &mut Cursor, byte: u8) -> Result<TokenKind, LexError> {
    if let Some(kind) = punct(byte) {
        cur.pos += 1;
        return Ok(kind);
    }
    match byte {
        b'0'..=b'9' => number::lex_number(cur, cur.pos),
        b'-' if cur.peek_at(1).is_some_and(|b| b.is_ascii_digit()) => minus_literal(cur),
        b'_' => lambda_arg(cur),
        b if name::is_symbol(b) || b.is_ascii_alphabetic() => name::lex_name(cur),
        _ => Err(unexpected(cur)),
    }
}

fn punct(byte: u8) -> Option<TokenKind> {
    Some(match byte {
        b'\n' => TokenKind::Newline,
        b';' => TokenKind::Semi,
        b'@' => TokenKind::Unit,
        b'(' => TokenKind::LParen,
        b')' => TokenKind::RParen,
        b'{' => TokenKind::LBrace,
        b'}' => TokenKind::RBrace,
        b'[' => TokenKind::LBracket,
        b']' => TokenKind::RBracket,
        _ => return None,
    })
}

/// `-` before a digit is a negative literal only at a token boundary
/// (start, whitespace, an opener or `;`); touching a previous token it
/// is ambiguous.
fn minus_literal(cur: &mut Cursor) -> Result<TokenKind, LexError> {
    let boundary = match cur.prev() {
        None => true,
        Some(b) => b" \t\r\n({[;".contains(&b),
    };
    if !boundary {
        return Err(LexError::new(
            ErrorKind::AmbiguousMinus,
            Span::new(cur.pos, cur.pos + 1),
            "`-` touching the previous token and a digit is ambiguous: \
             write `3 - 1` to subtract or `3 -1` for a negative literal",
        ));
    }
    let start = cur.pos;
    cur.pos += 1;
    number::lex_number(cur, start)
}

fn lambda_arg(cur: &mut Cursor) -> Result<TokenKind, LexError> {
    let start = cur.pos;
    cur.pos += 1;
    let side = match cur.peek() {
        Some(b'l') => Side::Left,
        Some(b'r') => Side::Right,
        _ => {
            cur.eat_while(|b| b.is_ascii_alphanumeric() || b == b'@');
            return Err(bad_arg(Span::new(start, cur.pos.max(start + 1))));
        }
    };
    cur.pos += 1;
    match cur.peek() {
        Some(b) if b.is_ascii_alphanumeric() => {
            cur.eat_while(|b| b.is_ascii_alphanumeric());
            Err(bad_arg(Span::new(start, cur.pos)))
        }
        Some(b'_' | b'^' | b'.' | b'@') => Err(bad_arg(Span::new(cur.pos, cur.pos + 1))),
        _ => Ok(TokenKind::LamArg(side)),
    }
}

fn bad_arg(span: Span) -> LexError {
    LexError::new(
        ErrorKind::BadLambdaArg,
        span,
        "lambda arguments are exactly `_l` and `_r`, undecorated",
    )
}

fn unexpected(cur: &Cursor) -> LexError {
    let span = cur.char_span();
    if cur.peek().is_some_and(|b| !b.is_ascii()) {
        LexError::new(
            ErrorKind::NonAscii,
            span,
            "source is ASCII; Unicode appears only in rendered output",
        )
    } else {
        LexError::new(ErrorKind::UnexpectedChar, span, "unexpected character")
    }
}
