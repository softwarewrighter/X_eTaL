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
