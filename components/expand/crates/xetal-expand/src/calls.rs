//! Finding the macro calls in a text: `"left" n_ame< "right"`, each
//! known, written with a string on each side, and standing either as
//! a statement of its own or inside an expression (MC12).

use xetal_base::{Diagnostic, Span};
use xetal_lex::{Token, TokenKind};

use crate::system::{SYSTEM, known};

/// One macro call.
#[derive(Debug, Clone)]
pub(crate) struct Call {
    pub name: String,
    /// The macro's name token, the two string tokens and the whole call.
    pub token: Span,
    pub left: Span,
    pub right: Span,
    pub span: Span,
    /// It is a statement of its own (else part of an expression).
    pub statement: bool,
}

/// The calls among `tokens`, in order (`u_se<` is left for the imports,
/// and a macro being defined, `m:n_ame< := ...`, for the names phase).
pub(crate) fn calls(tokens: &[Token]) -> Result<Vec<Call>, Diagnostic> {
    let (mut found, mut open) = (Vec::new(), Vec::new());
    for (i, t) in tokens.iter().enumerate() {
        match &t.kind {
            TokenKind::LParen | TokenKind::LBrace | TokenKind::LBracket => open.push(&t.kind),
            TokenKind::RParen | TokenKind::RBrace | TokenKind::RBracket => drop(open.pop()),
            TokenKind::Func(f) if f.is_macro() => {
                let name = match &f.ns {
                    Some(ns) => format!("{ns}:{}", f.spelled()),
                    None => f.spelled(),
                };
                let defines = matches!(tokens.get(i + 1).map(|t| &t.kind), Some(TokenKind::Assign));
                if name != "u_se<" && !defines {
                    let braced = matches!(open.last(), None | Some(TokenKind::LBrace));
                    found.push(call(tokens, i, name, braced)?);
                }
            }
            _ => {}
        }
    }
    Ok(found)
}

/// The call whose macro is token `i`.
fn call(tokens: &[Token], i: usize, name: String, braced: bool) -> Result<Call, Diagnostic> {
    let token = tokens[i].span;
    if !name.contains(':') && !known(&name) {
        let message = format!("there is no macro {name}; the system macros are {SYSTEM}");
        return Err(Diagnostic::new("unknown-macro", message).with_span(token));
    }
    let kind = |k: Option<usize>| k.and_then(|k| tokens.get(k)).map(|t| &t.kind);
    let (before, after) = (kind(i.checked_sub(2)), kind(Some(i + 2)));
    shape(
        &name,
        [before, kind(i.checked_sub(1)), kind(Some(i + 1)), after],
    )
    .map_err(|message| Diagnostic::new("bad-macro-call", message).with_span(token))?;
    let (left, right) = (tokens[i - 1].span, tokens[i + 1].span);
    Ok(Call {
        name,
        token,
        left,
        right,
        span: Span::new(left.start, right.end),
        statement: braced && starts(before) && ends(after),
    })
}

/// The tokens around a call (two before, two after the macro) must be
/// one string on each side and no more strings or calls beyond them.
fn shape(name: &str, around: [Option<&TokenKind>; 4]) -> Result<(), String> {
    let [before, left, right, after] = around;
    let strings = matches!(
        (left, right),
        (Some(TokenKind::Str(_)), Some(TokenKind::Str(_)))
    );
    let more = matches!(before, Some(TokenKind::Str(_)))
        || matches!(after, Some(TokenKind::Str(_)))
        || matches!(after, Some(TokenKind::Func(f)) if f.is_macro());
    match strings && !more {
        true => Ok(()),
        false => Err(format!(
            "{name} takes one string on each side: \"left\" {name} \"right\""
        )),
    }
}

/// A statement may start after these (or at the start of the text).
fn starts(before: Option<&TokenKind>) -> bool {
    matches!(
        before,
        None | Some(TokenKind::Newline | TokenKind::Semi | TokenKind::LBrace | TokenKind::Arrow)
    )
}

/// A statement may end before these (or at the end of the text).
fn ends(after: Option<&TokenKind>) -> bool {
    matches!(
        after,
        None | Some(TokenKind::Newline | TokenKind::Semi | TokenKind::RBrace)
    )
}
