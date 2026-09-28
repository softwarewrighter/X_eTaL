//! Parser: tokens to a surface AST with spans (docs/lang-choices.md).
//!
//! The grammar is decided by tokens alone and reads right to left: a
//! function takes everything to its right; in `x f y` the left argument
//! is the single value immediately left of `f`; a quoted function
//! directly left of a function is its operand. Every input therefore has
//! at most one reading, and shapes near those rules are rejected with a
//! specific error (the ambiguity corpus in `spec/ambiguity/`).

mod ast;
mod block;
mod expr;
mod item;
mod show;
mod stmt;

pub use ast::{
    Expr, ExprKind, Fun, FunKind, Lambda, MAX_DEPTH, Param, Params, Program, Stmt, Target,
};

/// The deepest bracket nesting the parser accepts.
pub const MAX_NESTING: usize = 64;

use xetal_base::{Diagnostic, Span};
use xetal_lex::{Token, TokenKind, lex};

/// Pipeline stage name used in diagnostics and by the CLI.
pub const STAGE: &str = "parse";

/// Lex and parse a whole program.
pub fn parse(src: &str) -> Result<Program, Diagnostic> {
    let tokens = lex(src)?;
    let mut parser = Parser {
        tokens,
        pos: 0,
        end: src.len(),
        newline_is_space: vec![false],
        nesting: 0,
    };
    parser.program()
}

pub(crate) fn err(code: &str, span: Span, message: impl Into<String>) -> Diagnostic {
    Diagnostic::new(code, message).with_span(span)
}

pub(crate) struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    end: usize,
    /// Inside `( )` and `[ ]` a newline is whitespace; elsewhere it
    /// separates statements (S2).
    newline_is_space: Vec<bool>,
    /// Open brackets around the current point.
    nesting: usize,
}

impl Parser {
    /// The next token, skipping newlines where they are whitespace.
    fn peek(&mut self) -> Option<&Token> {
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

    fn next(&mut self) -> Option<Token> {
        let token = self.peek().cloned();
        if token.is_some() {
            self.pos += 1;
        }
        token
    }

    /// Enter a bracket; too many open brackets are `too-deep`.
    fn enter(&mut self, open: Span) -> Result<(), Diagnostic> {
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

    fn leave(&mut self) {
        self.nesting -= 1;
    }

    /// The span of the next token, or an empty span at the end.
    fn here(&mut self) -> Span {
        let end = self.end;
        self.peek().map_or(Span::new(end, end), |t| t.span)
    }
}
