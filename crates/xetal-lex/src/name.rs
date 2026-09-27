//! Names: `[ns.]stem[^deriv][_[sub]]` in canonical order.

use xetal_base::Span;

use crate::cursor::Cursor;
use crate::error::{ErrorKind, LexError};
use crate::token::{Name, Sub, TokenKind};

pub(crate) fn is_symbol(b: u8) -> bool {
    b"+-*/=<>|".contains(&b)
}

pub(crate) fn lex_name(cur: &mut Cursor) -> Result<TokenKind, LexError> {
    let start = cur.pos;
    let (ns, stem, symbol) = stem_and_namespace(cur)?;
    let deriv = superscript(cur)?;
    let sub = subscript(cur, symbol || deriv.is_some())?;
    let name = Name {
        ns,
        stem,
        symbol,
        deriv,
        sub,
    };
    check_follow(cur, &name)?;
    if name.ns.is_some() && !name.is_function() {
        return Err(LexError::new(
            ErrorKind::BadNamespace,
            Span::new(start, cur.pos),
            "a namespace prefix applies only to a decorated function name",
        ));
    }
    Ok(TokenKind::Name(name))
}

type Stem = (Option<String>, String, bool);

fn stem_and_namespace(cur: &mut Cursor) -> Result<Stem, LexError> {
    if cur.peek().is_some_and(is_symbol) {
        let at = cur.pos;
        cur.pos += 1;
        return Ok((None, cur.text(at).to_string(), true));
    }
    let first = cur.eat_while(|b| b.is_ascii_alphanumeric()).to_string();
    if cur.peek() != Some(b'.') {
        return Ok((None, first, false));
    }
    if !cur.peek_at(1).is_some_and(|b| b.is_ascii_alphabetic()) {
        return Err(LexError::at(
            ErrorKind::BadNamespace,
            cur.pos,
            "a namespace prefix must be followed by a named function",
        ));
    }
    cur.pos += 1;
    let stem = cur.eat_while(|b| b.is_ascii_alphanumeric()).to_string();
    if cur.peek() == Some(b'.') {
        return Err(LexError::at(
            ErrorKind::BadNamespace,
            cur.pos,
            "nested namespaces are not supported",
        ));
    }
    Ok((Some(first), stem, false))
}

fn superscript(cur: &mut Cursor) -> Result<Option<String>, LexError> {
    if cur.peek() != Some(b'^') {
        return Ok(None);
    }
    let caret = cur.pos;
    cur.pos += 1;
    let word = cur.eat_while(|b| b.is_ascii_alphabetic()).to_string();
    if word.is_empty() {
        return Err(LexError::at(
            ErrorKind::BadDecoration,
            caret,
            "`^` must be followed by a derivation word such as `r`",
        ));
    }
    if cur.peek() == Some(b'^') {
        return Err(LexError::at(
            ErrorKind::BadDecoration,
            cur.pos,
            "only one superscript is allowed",
        ));
    }
    Ok(Some(word))
}

/// `already_function`: the stem is a symbol or carries a superscript, so
/// a bare underline would be redundant.
fn subscript(cur: &mut Cursor, already_function: bool) -> Result<Option<Sub>, LexError> {
    if cur.peek() != Some(b'_') {
        return Ok(None);
    }
    let underline = cur.pos;
    cur.pos += 1;
    match cur.peek() {
        Some(b'@') => {
            cur.pos += 1;
            Ok(Some(Sub::Niladic))
        }
        Some(b'0'..=b'9') => axes(cur).map(|a| Some(Sub::Axes(a))),
        Some(b'^') => Err(LexError::at(
            ErrorKind::NonCanonicalOrder,
            cur.pos,
            "the superscript comes before the underline: write `stem^sup_sub`",
        )),
        Some(b'_') => Err(LexError::at(
            ErrorKind::BadDecoration,
            cur.pos,
            "an underline cannot be doubled",
        )),
        Some(b) if b.is_ascii_alphabetic() => Err(LexError::at(
            ErrorKind::BadDecoration,
            cur.pos,
            "names cannot contain `_`; an underline ends the name",
        )),
        _ if already_function => Err(LexError::at(
            ErrorKind::RedundantUnderline,
            underline,
            "this name is already a function; drop the bare `_`",
        )),
        _ => Ok(Some(Sub::Bare)),
    }
}

fn axes(cur: &mut Cursor) -> Result<Vec<u8>, LexError> {
    let mut axes = Vec::new();
    while let Some(b @ b'0'..=b'9') = cur.peek() {
        let axis = b - b'0';
        if axis == 0 {
            return Err(LexError::at(
                ErrorKind::BadAxis,
                cur.pos,
                "axes are numbered from 1",
            ));
        }
        if axes.contains(&axis) {
            return Err(LexError::at(
                ErrorKind::BadAxis,
                cur.pos,
                "an axis may appear only once",
            ));
        }
        axes.push(axis);
        cur.pos += 1;
    }
    Ok(axes)
}

/// A name must end at a delimiter. Symbol stems without decoration may
/// touch anything (`1+2`); other names may not touch name characters.
fn check_follow(cur: &Cursor, name: &Name) -> Result<(), LexError> {
    let plain_symbol = name.symbol && name.deriv.is_none() && name.sub.is_none();
    match cur.peek() {
        _ if plain_symbol => Ok(()),
        Some(b'^') => Err(LexError::at(
            ErrorKind::NonCanonicalOrder,
            cur.pos,
            "the superscript comes before the underline: write `stem^sup_sub`",
        )),
        Some(b) if b.is_ascii_alphanumeric() || b"_.@".contains(&b) => Err(LexError::at(
            ErrorKind::BadDecoration,
            cur.pos,
            "unexpected character after a decorated name",
        )),
        _ => Ok(()),
    }
}
