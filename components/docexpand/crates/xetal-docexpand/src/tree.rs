//! The expansions of a text: each call with what its macro gave, and
//! the calls in that text in turn.

use std::collections::HashMap;

use xetal_base::Span;
use xetal_expand::{Call, DEPTH, calls};
use xetal_lex::{Token, TokenKind, lex};

use crate::record::{Key, quoted};

/// One macro call and its expansion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expansion {
    /// The macro as the call writes it (`i_f<`, `g:w_hen<`).
    pub name: String,
    /// Where the call is in its text, and its line there (from 1).
    pub span: Span,
    pub line: usize,
    /// The call as written.
    pub call: String,
    /// What replaced it: the macro's text (parenthesized inside an
    /// expression, as the macro phase substitutes it).
    pub text: String,
    /// The calls in that text, expanded in turn.
    pub nested: Vec<Expansion>,
}

/// The expansions of the calls in `text`, from what the calls gave.
pub(crate) fn tree(text: &str, seen: &HashMap<Key, String>, depth: usize) -> Vec<Expansion> {
    let Ok(tokens) = lex(text) else {
        return Vec::new();
    };
    let found = calls(&tokens).unwrap_or_default();
    found
        .iter()
        .filter(|_| depth < DEPTH)
        .filter_map(|c| one(text, &tokens, c, seen, depth))
        .collect()
}

fn one(
    text: &str,
    tokens: &[Token],
    c: &Call,
    seen: &HashMap<Key, String>,
    depth: usize,
) -> Option<Expansion> {
    let side = |span: Span| match tokens.iter().find(|t| t.span == span).map(|t| &t.kind) {
        Some(TokenKind::Str(s)) => quoted(s),
        _ => "@".to_string(),
    };
    let key = (
        c.statement,
        format!("{} {} {}", side(c.left), c.name, side(c.right)),
    );
    let given = seen.get(&key)?;
    let shown = match c.statement {
        true => given.clone(),
        false => format!("({given})"),
    };
    Some(Expansion {
        name: c.name.clone(),
        span: c.span,
        line: text[..c.span.start].matches('\n').count() + 1,
        call: text[c.span.start..c.span.end].to_string(),
        nested: tree(&shown, seen, depth + 1),
        text: shown,
    })
}
