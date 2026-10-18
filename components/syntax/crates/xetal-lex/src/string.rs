//! String literals: `"..."` with the escapes `\"` `\\` `\n` `\t`, single-line
//! (ST1, ST2), and raw strings `r"..."` and `r#"..."#` (ST4), Rust's
//! rules: the text verbatim (no escapes), newlines kept; with one or
//! more `#` it ends only at a `"` followed by as many `#`. Code strings
//! `c"..."` (escapes, any number of lines) and `c#"..."#` (verbatim) are
//! the same text as a `Code` token (ST5).

use xetal_base::Span;

use crate::cursor::Cursor;
use xetal_token::{ErrorKind, LexError, TokenKind};

/// A single-line ASCII string with the escapes `\"` `\\` `\n` `\t`.
pub(crate) fn lex_string(cur: &mut Cursor) -> Result<TokenKind, LexError> {
    let start = cur.pos;
    cur.pos += 1;
    escaped(cur, start, false).map(TokenKind::Str)
}

/// The body of an escaped string from the cursor to its closing `"`;
/// `lines` lets it span lines (a code string), else a newline is the
/// error of an unterminated string.
fn escaped(cur: &mut Cursor, start: usize, lines: bool) -> Result<String, LexError> {
    let mut text = String::new();
    loop {
        let c = match cur.peek() {
            None => return Err(unterminated(start, cur.pos, lines)),
            Some(b'\n') if !lines => return Err(unterminated(start, cur.pos, lines)),
            Some(b'"') => {
                cur.pos += 1;
                return Ok(text);
            }
            Some(b'\\') => match cur.peek_at(1) {
                Some(b'"') => '"',
                Some(b'\\') => '\\',
                Some(b'n') => '\n',
                Some(b't') => '\t',
                None | Some(b'\n') => return Err(unterminated(start, cur.pos, lines)),
                Some(_) => {
                    return Err(LexError::new(
                        ErrorKind::BadString,
                        Span::new(cur.pos, cur.pos + 2),
                        "unknown escape: use \\\" \\\\ \\n or \\t",
                    ));
                }
            },
            // Unicode is allowed in a string (and a comment), nowhere else.
            Some(b) if !b.is_ascii() => {
                let span = cur.char_span();
                cur.pos = span.end;
                text.push_str(cur.text(span.start));
                continue;
            }
            Some(b) => char::from(b),
        };
        cur.pos += if matches!(cur.peek(), Some(b'\\')) {
            2
        } else {
            1
        };
        text.push(c);
    }
}

fn unterminated(start: usize, pos: usize, code: bool) -> LexError {
    let message = match code {
        true => "unterminated code string: it ends with `\"`",
        false => "unterminated string (strings are single-line; use \\n)",
    };
    LexError::new(ErrorKind::BadString, Span::new(start, pos), message)
}

/// The letter (`r` raw, `c` code) and number of `#` when a prefixed
/// string starts at the cursor (the letter, any `#`, then `"`); `None`
/// when it does not.
pub(crate) fn opening(cur: &Cursor) -> Option<(u8, usize)> {
    let letter = cur.peek().filter(|b| matches!(b, b'r' | b'c'))?;
    let hashes = (1..).take_while(|&i| cur.peek_at(i) == Some(b'#')).count();
    (cur.peek_at(1 + hashes) == Some(b'"')).then_some((letter, hashes))
}

/// Lex a code string opened by `c`, `hashes` `#` and `"`: with no `#`
/// the usual escapes and any number of lines; with `#` verbatim, as a
/// raw string.
pub(crate) fn lex_code(cur: &mut Cursor, hashes: usize) -> Result<TokenKind, LexError> {
    if hashes > 0 {
        return lex_raw(cur, hashes).map(|t| match t {
            TokenKind::Str(s) => TokenKind::Code(s),
            other => other,
        });
    }
    let start = cur.pos;
    cur.pos += 2;
    escaped(cur, start, true).map(TokenKind::Code)
}

/// Lex a raw string opened by `r`, `hashes` `#` and `"`. Bytes are
/// compared one by one: no byte of a multi-byte character is `"` or `#`.
pub(crate) fn lex_raw(cur: &mut Cursor, hashes: usize) -> Result<TokenKind, LexError> {
    let start = cur.pos;
    cur.pos += hashes + 2;
    let body = cur.pos;
    let closes = |cur: &Cursor| (1..=hashes).all(|k| cur.peek_at(k) == Some(b'#'));
    while let Some(b) = cur.peek() {
        if b == b'"' && closes(cur) {
            let text = cur.text(body).to_string();
            cur.pos += 1 + hashes;
            return Ok(TokenKind::Str(text));
        }
        cur.pos += 1;
    }
    let message = format!(
        "unterminated raw string: it ends with \"{}",
        "#".repeat(hashes)
    );
    Err(LexError::new(
        ErrorKind::BadString,
        Span::new(start, cur.pos),
        message,
    ))
}
