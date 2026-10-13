//! Hygiene by choice (MC30): a name a macro writes stays as written; a
//! macro that wants a name of its own asks for a fresh one with
//! `[]G_ENSYM` (numbered by the expander, see [`crate::MacroCall`]). A
//! name a macro declares it binds for its call's text (`## binds: it`,
//! an anaphor) is the macro's: the call's text cannot bind it again.
//! A program cannot write the fresh namespace (`g1:`).

use xetal_base::Diagnostic;
use xetal_lex::{Token, TokenKind, is_fresh, lex};
use xetal_mapped::Mapped;

use crate::calls::Call;

/// An error when the text of `call` (`left`, `right`) binds one of the
/// anaphora its macro declares (`binds`): a lambda's parameter, a local
/// binding or a pattern's name, at any depth.
pub(crate) fn anaphora(
    call: &Call,
    (left, right): (&Mapped, &Mapped),
    binds: &[String],
) -> Result<(), Diagnostic> {
    for (side, span) in [(left, call.left), (right, call.right)] {
        if let Some(name) = bound(side.text()).into_iter().find(|n| binds.contains(n)) {
            let message = format!(
                "{name} is bound by {} for this text (## binds: {name}); the text cannot bind it again: use another name",
                call.name
            );
            return Err(Diagnostic::new("shadows-anaphor", message).with_span(span));
        }
    }
    Ok(())
}

/// Every name `text` binds: its own top-level bindings and those of each
/// lambda in it. Text that does not lex binds nothing.
fn bound(text: &str) -> Vec<String> {
    let Ok(tokens) = lex(text) else {
        return Vec::new();
    };
    let raw = |t: &Token| text[t.span.start..t.span.end].to_string();
    let mut out: Vec<String> = binders(&tokens).into_iter().map(raw).collect();
    for (i, t) in tokens.iter().enumerate() {
        if t.kind == TokenKind::LBrace {
            let end = closing(&tokens, i);
            out.extend(binders(&tokens[i + 1..end]).into_iter().map(raw));
        }
    }
    out
}

/// The index of the `}` closing the `{` at `open` (or the end).
fn closing(tokens: &[Token], open: usize) -> usize {
    let mut depth = 0;
    for (k, t) in tokens.iter().enumerate().skip(open) {
        match t.kind {
            TokenKind::LBrace | TokenKind::LParen | TokenKind::LBracket => depth += 1,
            TokenKind::RBrace | TokenKind::RParen | TokenKind::RBracket => {
                depth -= 1;
                if depth == 0 {
                    return k;
                }
            }
            _ => {}
        }
    }
    tokens.len()
}

/// The plain names a lambda body (`inner`, between its braces) binds:
/// its parameters before `->`, and its own top-level local bindings.
fn binders(inner: &[Token]) -> Vec<&Token> {
    let plain = |t: &Token| match &t.kind {
        TokenKind::Var(v) => v.ns.is_none(),
        TokenKind::Func(f) => f.ns.is_none(),
        _ => false,
    };
    let arrow = inner.iter().position(|t| t.kind == TokenKind::Arrow);
    // Parameters are names, `@`, `~` and tuple patterns (TU11).
    let params = arrow.filter(|&a| {
        inner[..a].iter().all(|t| {
            plain(t)
                || matches!(
                    t.kind,
                    TokenKind::Unit
                        | TokenKind::Lazy
                        | TokenKind::LParen
                        | TokenKind::RParen
                        | TokenKind::Comma
                        | TokenKind::Wild
                )
        })
    });
    let mut out: Vec<&Token> = params.map_or(Vec::new(), |a| {
        inner[..a].iter().filter(|t| plain(t)).collect()
    });
    let (mut depth, mut start) = (0, true);
    for (k, t) in inner.iter().enumerate() {
        match t.kind {
            TokenKind::LBrace | TokenKind::LParen | TokenKind::LBracket => depth += 1,
            TokenKind::RBrace | TokenKind::RParen | TokenKind::RBracket => depth -= 1,
            _ => {}
        }
        let next = inner.get(k + 1).map(|n| &n.kind);
        if depth == 0 && start && plain(t) && next == Some(&TokenKind::Assign) {
            out.push(t);
        }
        if depth == 1 && start && t.kind == TokenKind::LParen {
            out.extend(pattern_names(inner, k).into_iter().filter(|t| plain(t)));
        }
        start = depth == 0
            && matches!(
                t.kind,
                TokenKind::Semi | TokenKind::Newline | TokenKind::Arrow
            );
    }
    out
}

/// The names of a tuple pattern binding (`(a, (b, _)) := ...`, TU11)
/// whose `(` is `inner[open]`; none when no `:=` follows its `)`.
fn pattern_names(inner: &[Token], open: usize) -> Vec<&Token> {
    let close = open + closing(&inner[open..], 0);
    match inner.get(close + 1).map(|t| &t.kind) {
        Some(TokenKind::Assign) => inner[open..close].iter().collect(),
        _ => Vec::new(),
    }
}

/// A program may not write the macro phase's fresh namespaces.
pub(crate) fn unwritten(tokens: &[Token]) -> Result<(), Diagnostic> {
    let ns = |t: &Token| match &t.kind {
        TokenKind::Var(v) => v.ns.clone(),
        TokenKind::Func(f) => f.ns.clone(),
        _ => None,
    };
    match tokens.iter().find(|t| ns(t).is_some_and(|n| is_fresh(&n))) {
        None => Ok(()),
        Some(t) => {
            let message = "g and a number (g1:) is the namespace of names a macro binds; a program cannot write it";
            Err(Diagnostic::new("reserved-namespace", message).with_span(t.span))
        }
    }
}
