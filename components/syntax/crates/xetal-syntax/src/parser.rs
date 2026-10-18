//! The parser state, the entry point, and moving through the tokens:
//! newlines as whitespace inside brackets, and the bracket nesting limit.

use xetal_base::{Diagnostic, Span};
use xetal_lex::{Token, TokenKind};

/// The deepest bracket nesting the parser accepts.
pub const MAX_NESTING: usize = 64;

pub(crate) fn err(code: &str, span: Span, message: impl Into<String>) -> Diagnostic {
    Diagnostic::new(code, message).with_span(span)
}

pub(crate) struct Parser {
    pub(crate) tokens: Vec<Token>,
    pub(crate) pos: usize,
    pub(crate) end: usize,
    /// Inside `( )` and `[ ]` a newline is whitespace; elsewhere it
    /// separates statements (S2).
    pub(crate) newline_is_space: Vec<bool>,
    /// Open brackets around the current point.
    pub(crate) nesting: usize,
}

impl Parser {
    /// The next token, skipping newlines where they are whitespace.
    pub(crate) fn peek(&mut self) -> Option<&Token> {
        if self.newline_is_space.last() == Some(&true) {
            while self
                .tokens
                .get(self.pos)
                .is_some_and(|t| t.kind == TokenKind::Newline)
            {
                self.pos += 1;
            }
        }
        self.tokens.get(self.pos)
    }

    pub(crate) fn next(&mut self) -> Option<Token> {
        let token = self.peek().cloned();
        if token.is_some() {
            self.pos += 1;
        }
        token
    }

    /// Enter a bracket; too many open brackets are `too-deep`.
    pub(crate) fn enter(&mut self, open: Span) -> Result<(), Diagnostic> {
        self.nesting += 1;
        if self.nesting > MAX_NESTING {
            self.nesting -= 1;
            return Err(err(
                "too-deep",
                open,
                format!("brackets nest more than {MAX_NESTING} levels deep"),
            ));
        }
        Ok(())
    }

    pub(crate) fn leave(&mut self) {
        self.nesting -= 1;
    }

    /// The span of the next token, or an empty span at the end.
    pub(crate) fn here(&mut self) -> Span {
        let end = self.end;
        self.peek().map_or(Span::new(end, end), |t| t.span)
    }
}
