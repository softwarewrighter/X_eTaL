//! String literals: `"..."` with the escapes `\"` `\\` `\n` `\t`, single-line
//! (ST1, ST2), and raw strings `r"..."` and `r#"..."#` (ST4), Rust's
//! rules: the text verbatim (no escapes), newlines kept; with one or
//! more `#` it ends only at a `"` followed by as many `#`.

use xetal_base::Span;

use crate::cursor::Cursor;
use xetal_token::{ErrorKind, LexError, TokenKind};

/// A single-line ASCII string with the escapes `\"` `\\` `\n` `\t`.
pub(crate) fn lex_string(cur: &mut Cursor) -> Result<TokenKind, LexError> {
    let start = cur.pos;
    cur.pos += 1;
    let mut text = String::new();
    loop {
        let c = match cur.peek() {
            None | Some(b'\n') => return Err(unterminated(start, cur.pos)),
            Some(b'"') => {
                cur.pos += 1;
                return Ok(TokenKind::Str(text));
            }
            Some(b'\\') => match cur.peek_at(1) {
                Some(b'"') => '"',
                Some(b'\\') => '\\',
                Some(b'n') => '\n',
                Some(b't') => '\t',
                None | Some(b'\n') => return Err(unterminated(start, cur.pos)),
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

fn unterminated(start: usize, pos: usize) -> LexError {
    LexError::new(
        ErrorKind::BadString,
        Span::new(start, pos),
        "unterminated string (strings are single-line; use \\n)",
    )
}

/// The number of `#` when a raw string starts at the cursor (`r`, any
/// `#`, then `"`); `None` when it does not.
pub(crate) fn opening(cur: &Cursor) -> Option<usize> {
    if cur.peek() != Some(b'r') {
        return None;
    }
    let hashes = (1..).take_while(|&i| cur.peek_at(i) == Some(b'#')).count();
    (cur.peek_at(1 + hashes) == Some(b'"')).then_some(hashes)
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
