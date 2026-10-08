//! Tuples (TU1): parts separated by commas directly inside parentheses.
//! A group with a comma at its own level is a tuple, one without is
//! grouping, so `(x)` and `(x, y)` each have one reading (rule 3).

use xetal_base::{Diagnostic, Span};
use xetal_lex::{Token, TokenKind};

use crate::parser::{Parser, err};
use crate::stmt::target;
use xetal_ast::{Expr, ExprKind, Stmt, Target};

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

    /// `(a, b) := value` at the start of a statement: a pattern binding
    /// (TU3), recognized by `:=` right after the closing parenthesis.
    pub(crate) fn pattern_binding(&mut self) -> Result<Option<Stmt>, Diagnostic> {
        let mut depth = 0usize;
        let mut close = None;
        for (k, t) in self.tokens.iter().enumerate().skip(self.pos) {
            match t.kind {
                TokenKind::LParen => depth += 1,
                TokenKind::RParen if depth == 1 => {
                    close = Some(k);
                    break;
                }
                TokenKind::RParen => depth = depth.saturating_sub(1),
                _ if depth == 0 => break,
                _ => {}
            }
        }
        let Some(close) = close else { return Ok(None) };
        if self
            .tokens
            .get(close + 1)
            .is_none_or(|t| t.kind != TokenKind::Assign)
        {
            return Ok(None);
        }
        let start = self.tokens[self.pos].span;
        // Inside parentheses a newline is whitespace (S2).
        let tokens: Vec<Token> = self.tokens[self.pos..=close]
            .iter()
            .filter(|t| t.kind != TokenKind::Newline)
            .cloned()
            .collect();
        let target = pattern(&tokens, &mut 0, false, start)?;
        let whole = start.join(self.tokens[close].span);
        distinct(&target, &mut Vec::new()).map_err(|d| d.with_span(whole))?;
        let assign = self.tokens[close + 1].span;
        self.pos = close + 2;
        if self.expr_is_empty() {
            return Err(err(
                "missing-value",
                assign,
                "a binding needs a value after `:=`",
            ));
        }
        let value = self.expr()?;
        let span = start.join(value.span);
        Ok(Some(Stmt::Bind {
            target,
            value,
            span,
        }))
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
        TokenKind::LParen => {
            let mut parts = vec![pattern(tokens, i, true, t.span)?];
            loop {
                match tokens.get(*i).map(|t| &t.kind) {
                    Some(TokenKind::Comma) => {
                        *i += 1;
                        parts.push(pattern(tokens, i, true, t.span)?);
                    }
                    Some(TokenKind::RParen) if parts.len() > 1 => {
                        *i += 1;
                        return Ok(Target::Tuple(parts));
                    }
                    Some(TokenKind::RParen) => {
                        return Err(err(
                            "bad-pattern",
                            t.span,
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
        _ => target(t).ok_or_else(|| {
            err(
                "bad-pattern",
                t.span,
                "a tuple pattern is names, `_` and parentheses, separated by commas",
            )
        }),
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
