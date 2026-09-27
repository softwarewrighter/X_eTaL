//! Number literals: `[-]digits[.digits]`.

use xetal_base::Span;

use crate::cursor::Cursor;
use crate::error::{ErrorKind, LexError};
use crate::token::{Number, TokenKind};

/// Lex a number whose text starts at `start` (a `-` already consumed, if
/// any); the cursor is at the first digit.
pub(crate) fn lex_number(cur: &mut Cursor, start: usize) -> Result<TokenKind, LexError> {
    cur.eat_while(|b| b.is_ascii_digit());
    let mut float = false;
    if cur.peek() == Some(b'.') {
        if !cur.peek_at(1).is_some_and(|b| b.is_ascii_digit()) {
            return Err(bad(
                Span::new(start, cur.pos + 1),
                "a `.` must be followed by digits",
            ));
        }
        cur.pos += 1;
        cur.eat_while(|b| b.is_ascii_digit());
        float = true;
    }
    if let Some(b) = cur.peek()
        && (b.is_ascii_alphanumeric() || b"_^.@".contains(&b))
    {
        return Err(bad(
            Span::new(cur.pos, cur.pos + 1),
            "a number must be followed by a space, symbol or bracket",
        ));
    }
    value(cur.text(start), Span::new(start, cur.pos), float)
}

fn value(text: &str, span: Span, float: bool) -> Result<TokenKind, LexError> {
    let number = if float {
        text.parse::<f64>().ok().map(Number::Float)
    } else {
        text.parse::<i64>().ok().map(Number::Int)
    };
    number.map(TokenKind::Num).ok_or_else(|| {
        LexError::new(
            ErrorKind::NumberOutOfRange,
            span,
            "integer literal does not fit in 64 bits",
        )
    })
}

fn bad(span: Span, message: &str) -> LexError {
    LexError::new(ErrorKind::BadNumber, span, message)
}
