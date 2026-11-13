//! Applied groups (F5, drawn by D-11): a parenthesized expression whose
//! `)` touches an apply `_`, `(expr)_ x`. Views draw both of its
//! parentheses underlined and the `_` as nothing.

use crate::{Token, TokenKind};

/// For each token of `tokens`, whether it is a parenthesis of an
/// applied group: the `)` before an apply `_` and the `(` it closes
/// (matched with a stack over every kind of bracket). The `)` is
/// marked even when no `(` matches it, since its `_` is not drawn.
pub fn applied_parens(tokens: &[Token]) -> Vec<bool> {
    let mut marks = vec![false; tokens.len()];
    let mut open: Vec<(usize, &TokenKind)> = Vec::new();
    for (i, t) in tokens.iter().enumerate() {
        match t.kind {
            TokenKind::LParen | TokenKind::LBrace | TokenKind::LBracket => open.push((i, &t.kind)),
            TokenKind::RParen | TokenKind::RBrace | TokenKind::RBracket => {
                let opened = open.pop();
                let applied = tokens
                    .get(i + 1)
                    .is_some_and(|n| n.kind == TokenKind::Apply);
                if applied && t.kind == TokenKind::RParen {
                    marks[i] = true;
                    if let Some((j, TokenKind::LParen)) = opened {
                        marks[j] = true;
                    }
                }
            }
            _ => {}
        }
    }
    marks
}

/// A bracket's place: its depth (the outermost pair is 0, every kind of
/// bracket counting), the same for both of a pair; or a stray with no
/// pair (D-12).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bracket {
    Depth(usize),
    Unmatched,
}

/// For each token of `tokens`, its [`Bracket`] if it is one. A closer
/// that does not close the innermost open bracket is a stray, and so is
/// an opener never closed.
pub fn bracket_depths(tokens: &[Token]) -> Vec<Option<Bracket>> {
    let mut out: Vec<Option<Bracket>> = vec![None; tokens.len()];
    let mut open: Vec<(usize, TokenKind)> = Vec::new();
    for (i, t) in tokens.iter().enumerate() {
        let closes = match t.kind {
            TokenKind::LParen | TokenKind::LBrace | TokenKind::LBracket => {
                open.push((i, t.kind.clone()));
                continue;
            }
            TokenKind::RParen => TokenKind::LParen,
            TokenKind::RBrace => TokenKind::LBrace,
            TokenKind::RBracket => TokenKind::LBracket,
            _ => continue,
        };
        out[i] = Some(Bracket::Unmatched);
        if open.last().is_some_and(|(_, k)| *k == closes) {
            let (j, _) = open.pop().unwrap_or((i, closes));
            out[i] = Some(Bracket::Depth(open.len()));
            out[j] = out[i];
        }
    }
    for (j, _) in open {
        out[j] = Some(Bracket::Unmatched);
    }
    out
}
