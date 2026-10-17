//! Number literals `[-]digits[.digits][e[-]digits]` (S8), the minus
//! sign, and a `^` touching the token before it: a literal exponent on a
//! value (`x^2`, D-1 to D-5) or a power on a function name (`f_^3`,
//! D-7). Strings are in `string`.

use xetal_base::Span;

use crate::cursor::Cursor;
use xetal_token::{ErrorKind, LexError};
use xetal_token::{Number, Symbol, TokenKind};

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
    float |= exponent(cur)?;
    if let Some(b) = cur.peek()
        && (b.is_ascii_alphanumeric() || b"_.@".contains(&b))
    {
        return Err(bad(
            Span::new(cur.pos, cur.pos + 1),
            "a number must be followed by a space, symbol or bracket",
        ));
    }
    value(cur.text(start), Span::new(start, cur.pos), float)
}

/// An exponent (S8): `e` or `E`, an optional `-`, then digits, all
/// touching the number; whether there was one.
fn exponent(cur: &mut Cursor) -> Result<bool, LexError> {
    if !matches!(cur.peek(), Some(b'e' | b'E')) {
        return Ok(false);
    }
    let (at, sign) = (cur.pos, usize::from(cur.peek_at(1) == Some(b'-')));
    if !cur.peek_at(1 + sign).is_some_and(|b| b.is_ascii_digit()) {
        let span = Span::new(at, at + 1 + sign);
        return Err(bad(span, "an exponent needs digits after the e"));
    }
    cur.pos += 1 + sign;
    cur.eat_while(|b| b.is_ascii_digit());
    Ok(true)
}

fn value(text: &str, span: Span, float: bool) -> Result<TokenKind, LexError> {
    let (number, message) = if float {
        let x = text.parse::<f64>().ok().filter(|x| x.is_finite());
        (x.map(Number::Float), "float literal is too large")
    } else {
        let n = text.parse::<i64>().ok();
        (
            n.map(Number::Int),
            "integer literal does not fit in 64 bits",
        )
    };
    number
        .map(TokenKind::Num)
        .ok_or_else(|| LexError::new(ErrorKind::NumberOutOfRange, span, message))
}

fn bad(span: Span, message: &str) -> LexError {
    LexError::new(ErrorKind::BadNumber, span, message)
}

/// `->`, a negative literal (only at a token boundary), or subtract.
pub(crate) fn minus(cur: &mut Cursor) -> Result<TokenKind, LexError> {
    if cur.peek_at(1) == Some(b'>') {
        cur.pos += 2;
        return Ok(TokenKind::Arrow);
    }
    if !cur.peek_at(1).is_some_and(|b| b.is_ascii_digit()) {
        cur.pos += 1;
        return Ok(TokenKind::Sym(Symbol::Minus));
    }
    if !cur.prev().is_none_or(|b| b" \t\r\n({[;,".contains(&b)) {
        return Err(LexError::new(
            ErrorKind::AmbiguousMinus,
            Span::new(cur.pos, cur.pos + 1),
            "`-` touching the previous token and a digit is ambiguous: \
             write `3 - 1` to subtract or `3 -1` for a negative literal",
        ));
    }
    let start = cur.pos;
    cur.pos += 1;
    lex_number(cur, start)
}

const ON_SYMBOL: &str =
    "a power goes on a function name (`f_^3`); for a symbol write `n '+ p_ower x`";
const COMPUTED: &str =
    "a power count must be a number literal; for a computed count write `n 'f_ p_ower x`";

/// The exponent or power at `^`, given the token it touches.
pub(crate) fn lex_exponent(cur: &mut Cursor, prev: &TokenKind) -> Result<TokenKind, LexError> {
    let caret = cur.pos;
    match prev {
        TokenKind::Func(_) => return function_power(cur),
        TokenKind::Sym(_) => return Err(LexError::at(ErrorKind::BadPower, caret, ON_SYMBOL)),
        TokenKind::Var(_)
        | TokenKind::Num(_)
        | TokenKind::RParen
        | TokenKind::LamArg { applied: false, .. } => {}
        _ => {
            return Err(LexError::at(
                ErrorKind::BadExponent,
                caret,
                "an exponent must touch a number, variable, lambda argument or `)`",
            ));
        }
    }
    cur.pos += 1;
    let start = cur.pos;
    if cur.peek() == Some(b'-') && cur.peek_at(1).is_some_and(|b| b.is_ascii_digit()) {
        cur.pos += 1;
    }
    match cur.peek() {
        Some(b'0'..=b'9') => match lex_number(cur, start)? {
            TokenKind::Num(n) => Ok(TokenKind::Exp(n)),
            other => Ok(other),
        },
        Some(b) if b.is_ascii_alphabetic() || b == b'_' => Err(LexError::at(
            ErrorKind::BadExponent,
            cur.pos,
            "an exponent must be a number literal; for a computed power write `x ^ n`",
        )),
        _ => Err(LexError::at(
            ErrorKind::BadExponent,
            caret,
            "`^` touching a value must be followed by a number literal",
        )),
    }
}

/// `f_^n`: a whole number n, 0 or more; `^-1` (the inverse) is reserved.
fn function_power(cur: &mut Cursor) -> Result<TokenKind, LexError> {
    let caret = cur.pos;
    cur.pos += 1;
    let start = cur.pos;
    if cur.peek() == Some(b'-') && cur.peek_at(1).is_some_and(|b| b.is_ascii_digit()) {
        cur.pos += 1;
    }
    let count = match cur.peek() {
        Some(b'0'..=b'9') => lex_number(cur, start)?,
        Some(b) if b.is_ascii_alphabetic() || b == b'_' => {
            return Err(LexError::at(ErrorKind::BadPower, cur.pos, COMPUTED));
        }
        _ => {
            let message = "`^` touching a function name must be followed by a count, as in `f_^3`";
            return Err(LexError::at(ErrorKind::BadPower, caret, message));
        }
    };
    let span = Span::new(caret, cur.pos);
    match count {
        TokenKind::Num(Number::Int(n)) if n >= 0 => Ok(TokenKind::Exp(Number::Int(n))),
        TokenKind::Num(Number::Int(-1)) => Err(LexError::new(
            ErrorKind::ReservedSuperscript,
            span,
            "`^-1`, the inverse of a function, is reserved",
        )),
        _ => Err(LexError::new(
            ErrorKind::BadPower,
            span,
            "a power count is a whole number, 0 or more",
        )),
    }
}
