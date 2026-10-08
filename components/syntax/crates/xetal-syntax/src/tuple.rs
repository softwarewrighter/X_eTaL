//! Tuples (TU1): parts separated by commas directly inside parentheses.
//! A group with a comma at its own level is a tuple, one without is
//! grouping, so `(x)` and `(x, y)` each have one reading (rule 3).

use xetal_base::{Diagnostic, Span};
use xetal_lex::{Token, TokenKind};

use crate::parser::{Parser, err};
use crate::stmt::target;
use xetal_ast::{Expr, ExprKind, Target};

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

/// The pattern at `tokens[*i..]`: a name, `_` (only inside a tuple
/// pattern, TU4), or `(p, p, ...)` of two or more parts (TU3). `at` is
/// where an unfinished pattern is reported.
pub(crate) fn pattern(
    tokens: &[Token],
    i: &mut usize,
    inner: bool,
    at: Span,
) -> Result<Target, Diagnostic> {
    let Some(t) = tokens.get(*i) else {
        return Err(err(
            "bad-pattern",
            at,
            "a tuple pattern needs its closing parenthesis",
        ));
    };
    *i += 1;
    match &t.kind {
        TokenKind::Wild if inner => Ok(Target::Wild),
        TokenKind::Wild => Err(err(
            "bad-pattern",
            t.span,
            "`_` ignores a part of a tuple pattern: write a name here",
        )),
        TokenKind::LParen => parts(tokens, i, t.span, at),
        _ => target(t).ok_or_else(|| {
            err(
                "bad-pattern",
                t.span,
                "a tuple pattern is names, `_` and parentheses, separated by commas",
            )
        }),
    }
}

/// The parts of a parenthesized pattern, after its `(` at `open`: two
/// or more patterns separated by commas, then `)`.
fn parts(tokens: &[Token], i: &mut usize, open: Span, at: Span) -> Result<Target, Diagnostic> {
    let mut parts = vec![pattern(tokens, i, true, open)?];
    loop {
        match tokens.get(*i).map(|t| &t.kind) {
            Some(TokenKind::Comma) => {
                *i += 1;
                parts.push(pattern(tokens, i, true, open)?);
            }
            Some(TokenKind::RParen) if parts.len() > 1 => {
                *i += 1;
                return Ok(Target::Tuple(parts));
            }
            Some(TokenKind::RParen) => {
                return Err(err(
                    "bad-pattern",
                    open,
                    "a tuple pattern has two or more parts: (a, b)",
                ));
            }
            _ => {
                let span = tokens.get(*i).map_or(at, |t| t.span);
                return Err(err(
                    "bad-pattern",
                    span,
                    "a tuple pattern is names, `_` and parentheses, separated by commas",
                ));
            }
        }
    }
}

/// Every name a pattern binds, checked to appear once (`seen` holds the
/// names of the patterns before it).
pub(crate) fn distinct(t: &Target, seen: &mut Vec<Target>) -> Result<(), Diagnostic> {
    match t {
        Target::Tuple(parts) => parts.iter().try_for_each(|p| distinct(p, seen)),
        Target::Wild => Ok(()),
        name if seen.contains(name) => Err(Diagnostic::new(
            "bad-pattern",
            "a name appears twice in a pattern",
        )),
        name => {
            seen.push(name.clone());
            Ok(())
        }
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
