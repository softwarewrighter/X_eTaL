//! Tuples (TU1): parts separated by commas directly inside parentheses.
//! A group with a comma at its own level is a tuple, one without is
//! grouping, so `(x)` and `(x, y)` each have one reading (rule 3).

use xetal_base::{Diagnostic, Span};
use xetal_lex::TokenKind;

use crate::parser::{Parser, err};
use xetal_ast::{Expr, ExprKind};

impl Parser {
    /// The parts after the first, while a comma follows: each comma must
    /// be followed by a part (`(1,)` and `(1,,2)` are errors).
    pub(crate) fn tuple_parts(&mut self, first: Expr) -> Result<Vec<Expr>, Diagnostic> {
        let mut parts = vec![first];
        while self.peek().is_some_and(|t| t.kind == TokenKind::Comma) {
            let comma = self.next().expect("checked").span;
            if self.expr_is_empty() {
                return Err(missing_part(comma, "after"));
            }
            parts.push(self.expr()?);
        }
        Ok(parts)
    }

    /// The tuple of the parts, spanning the parentheses.
    pub(crate) fn tuple(parts: Vec<Expr>, span: Span) -> Expr {
        Expr::new(ExprKind::Tuple(parts), span)
    }

    /// The error for a statement that ends at a token it cannot take: a
    /// comma outside parentheses gets its own message (TU1).
    pub(crate) fn end_of_statement(&mut self, message: &str) -> Diagnostic {
        let span = self.here();
        if self.peek().is_some_and(|t| t.kind == TokenKind::Comma) {
            return err(
                "comma-outside-tuple",
                span,
                "a comma separates the parts of a tuple, inside parentheses: (a, b)",
            );
        }
        err("unexpected-token", span, message)
    }
}

/// A tuple part missing next to a comma.
pub(crate) fn missing_part(comma: Span, side: &str) -> Diagnostic {
    err(
        "empty-tuple-part",
        comma,
        format!("a tuple part is missing {side} the comma"),
    )
}
