//! Programs and statements: bindings (S1), pattern bindings (TU3),
//! guards (G1), separators (S2, S3).

use xetal_base::Diagnostic;
use xetal_lex::{Token, TokenKind};

use crate::parser::{Parser, err};
use xetal_ast::{Program, Stmt, Target};

impl Parser {
    pub(crate) fn program(&mut self) -> Result<Program, Diagnostic> {
        let stmts = self.statements(false)?;
        if self.peek().is_some() {
            return Err(self.end_of_statement("unexpected token"));
        }
        Ok(Program { stmts })
    }

    /// Statements separated by newlines or `;`, up to the end or `}`.
    pub(crate) fn statements(&mut self, in_lambda: bool) -> Result<Vec<Stmt>, Diagnostic> {
        let mut stmts = Vec::new();
        loop {
            match self.peek().map(|t| &t.kind) {
                Some(TokenKind::Newline | TokenKind::Semi) => {
                    self.pos += 1;
                }
                None | Some(TokenKind::RBrace) => return Ok(stmts),
                Some(_) => {
                    stmts.push(self.statement(in_lambda)?);
                    match self.peek().map(|t| &t.kind) {
                        None | Some(TokenKind::Newline | TokenKind::Semi | TokenKind::RBrace) => {}
                        Some(_) => {
                            return Err(self.end_of_statement("expected the end of the statement"));
                        }
                    }
                }
            }
        }
    }

    fn statement(&mut self, in_lambda: bool) -> Result<Stmt, Diagnostic> {
        if self.peek().is_some_and(|t| t.kind == TokenKind::LParen)
            && let Some(binding) = self.pattern_binding()?
        {
            return Ok(binding);
        }
        let is_binding = self
            .tokens
            .get(self.pos + 1)
            .is_some_and(|t| t.kind == TokenKind::Assign)
            && self.tokens.get(self.pos).and_then(target).is_some();
        if is_binding {
            let name = self.next().expect("checked");
            let assign = self.next().expect("checked");
            let target = target(&name).expect("checked");
            if self.expr_is_empty() {
                return Err(err(
                    "missing-value",
                    assign.span,
                    "a binding needs a value after `:=`",
                ));
            }
            let value = self.expr()?;
            let span = name.span.join(value.span);
            return Ok(Stmt::Bind {
                target,
                value,
                span,
            });
        }
        let cond = self.expr()?;
        if self.peek().is_some_and(|t| t.kind == TokenKind::Guard) {
            let guard = self.next().expect("checked");
            if !in_lambda {
                return Err(err(
                    "guard-outside-lambda",
                    guard.span,
                    "a guard `?` belongs inside a lambda",
                ));
            }
            let result = self.expr()?;
            let span = cond.span.join(result.span);
            return Ok(Stmt::Guard { cond, result, span });
        }
        Ok(Stmt::Expr(cond))
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
        let target = crate::tuple::pattern(&tokens, &mut 0, false, start)?;
        let whole = start.join(self.tokens[close].span);
        crate::tuple::distinct(&target, &mut Vec::new()).map_err(|d| d.with_span(whole))?;
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

/// A bindable name.
pub(crate) fn target(token: &Token) -> Option<Target> {
    match &token.kind {
        TokenKind::Var(v) => Some(Target::Var(v.clone())),
        TokenKind::Func(f) => Some(Target::Func(f.clone())),
        _ => None,
    }
}
