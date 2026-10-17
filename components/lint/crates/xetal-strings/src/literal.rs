//! The string literals of a text, from its tokens: each one's form,
//! place, body as written and value; and the check every fix passes,
//! that a spelling lexes to exactly the same value.

use xetal_base::Span;
use xetal_lex::{Token, TokenKind, lex};

use crate::form::Form;

/// One string literal as written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Literal<'a> {
    pub form: Form,
    /// The whole literal, delimiters included.
    pub span: Span,
    /// The text between the delimiters, as written (escapes and all).
    pub body: &'a str,
    /// Where `body` starts in the text.
    pub body_at: usize,
    /// The string it means, as the lexer gives it.
    pub value: String,
}

impl Literal<'_> {
    /// The literal written in form `to` with the same body.
    pub fn respelled(&self, to: Form) -> String {
        format!("{}{}{}", to.open(), self.body, to.close())
    }

    /// The edits that write it in form `to`: its delimiters only.
    pub fn delimiters(&self, to: Form) -> Vec<(Span, String)> {
        let (start, end) = (self.span.start, self.span.end);
        let close = self.form.close().len();
        vec![
            (Span::new(start, self.body_at), to.open()),
            (Span::new(end.saturating_sub(close), end), to.close()),
        ]
    }
}

/// The string literals of `text`, in order; none when it does not lex.
pub fn literals(text: &str) -> Vec<Literal<'_>> {
    let tokens = lex(text).unwrap_or_default();
    tokens.iter().filter_map(|t| literal(text, t)).collect()
}

/// `token` as a literal of `text`, when it is a string of a known form.
fn literal<'a>(text: &'a str, token: &Token) -> Option<Literal<'a>> {
    let TokenKind::Str(value) = &token.kind else {
        return None;
    };
    let written = text.get(token.span.start..token.span.end)?;
    let form = Form::of(written)?;
    let open = form.open().len();
    let body = written.get(open..written.len().checked_sub(form.close().len())?)?;
    Some(Literal {
        form,
        span: token.span,
        body,
        body_at: token.span.start + open,
        value: value.clone(),
    })
}

/// Whether `spelling` is one string literal, all of it, whose value is
/// `value`.
pub fn holds(spelling: &str, value: &str) -> bool {
    match lex(spelling).as_deref() {
        Ok(
            [
                Token {
                    kind: TokenKind::Str(s),
                    span,
                },
            ],
        ) => s == value && span.start == 0 && span.end == spelling.len(),
        _ => false,
    }
}
