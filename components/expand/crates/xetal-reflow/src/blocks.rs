//! The brackets of a token list: which closes which, and which blocks
//! are long enough to lay out.

use xetal_lex::{Token, TokenKind};

/// What a bracket token opens.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Open {
    Brace,
    Paren,
    Bracket,
}

pub(crate) fn opens(kind: &TokenKind) -> Option<Open> {
    match kind {
        TokenKind::LBrace => Some(Open::Brace),
        TokenKind::LParen => Some(Open::Paren),
        TokenKind::LBracket => Some(Open::Bracket),
        _ => None,
    }
}

pub(crate) fn closes(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::RBrace | TokenKind::RParen | TokenKind::RBracket
    )
}

/// For each token, the index of the token that closes it (an opening
/// bracket) or None; unbalanced text leaves the rest None.
pub(crate) fn partners(tokens: &[Token]) -> Vec<Option<usize>> {
    let mut out = vec![None; tokens.len()];
    let mut open = Vec::new();
    for (i, t) in tokens.iter().enumerate() {
        if opens(&t.kind).is_some() {
            open.push(i);
        } else if closes(&t.kind)
            && let Some(o) = open.pop()
        {
            out[o] = Some(i);
        }
    }
    out
}

/// Whether the text from token `i` to its partner spans more than
/// `width` columns on one line (a block worth laying out).
pub(crate) fn long(text: &str, tokens: &[Token], i: usize, close: usize, width: usize) -> bool {
    let inside = &text[tokens[i].span.start..tokens[close].span.end];
    inside.lines().any(|l| l.chars().count() > width)
}
