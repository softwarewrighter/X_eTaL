//! Finding the imports of one file: a top-level statement of exactly
//! `"alias:" u_se< "Library"` (MC3); any other use of a macro is an
//! error from the table (MC8 rows 3, 4, 5, 13, 14, 15).

use xetal_base::{Diagnostic, Span};
use xetal_lex::{Token, TokenKind, lex};

/// One import statement.
#[derive(Debug, Clone)]
pub(crate) struct Import {
    pub alias: String,
    pub spec: String,
    /// The whole statement, removed from the program text.
    pub span: Span,
}

fn fail(code: &str, span: Span, message: impl Into<String>) -> Diagnostic {
    Diagnostic::new(code, message).with_span(span)
}

/// The imports of `text`; text that does not lex has none (the parser
/// reports it later).
pub(crate) fn imports(text: &str) -> Result<Vec<Import>, Diagnostic> {
    let Ok(tokens) = lex(text) else {
        return Ok(Vec::new());
    };
    let mut found = Vec::new();
    for statement in statements(&tokens) {
        for (i, t) in statement.iter().enumerate() {
            if let TokenKind::Func(f) = &t.kind
                && f.is_macro()
            {
                found.push(import(statement, i, t, &f.spelled())?);
            }
        }
    }
    Ok(found)
}

/// Top-level statements: tokens between newlines and `;` outside
/// brackets. A macro inside brackets makes the whole bracket part of
/// its statement, so it is caught as misplaced.
fn statements(tokens: &[Token]) -> Vec<&[Token]> {
    let (mut out, mut start, mut depth) = (Vec::new(), 0, 0i32);
    for (i, t) in tokens.iter().enumerate() {
        match t.kind {
            TokenKind::LParen | TokenKind::LBrace | TokenKind::LBracket => depth += 1,
            TokenKind::RParen | TokenKind::RBrace | TokenKind::RBracket => depth -= 1,
            TokenKind::Newline | TokenKind::Semi if depth <= 0 => {
                out.push(&tokens[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    out.push(&tokens[start..]);
    out
}

/// The import in `statement`, whose token `at` is the macro `name`.
fn import(
    statement: &[Token],
    at: usize,
    macro_token: &Token,
    name: &str,
) -> Result<Import, Diagnostic> {
    let whole = Span::new(
        statement[0].span.start,
        statement[statement.len() - 1].span.end,
    );
    if name != "u_se<" {
        return Err(fail(
            "unknown-macro",
            macro_token.span,
            format!("there is no macro {name}; the only one is u_se<"),
        ));
    }
    match (
        at,
        statement.len(),
        statement.first().map(|t| &t.kind),
        statement.last().map(|t| &t.kind),
    ) {
        (0, _, _, _) => Err(fail(
            "missing-alias",
            whole,
            "u_se< needs an alias on its left: \"c:\" u_se< \"Library\"",
        )),
        (1, 3, Some(TokenKind::Str(alias)), Some(TokenKind::Str(spec))) => {
            valid_alias(alias, statement[0].span)?;
            Ok(Import {
                alias: alias.clone(),
                spec: spec.clone(),
                span: whole,
            })
        }
        (1, 3, _, _) => Err(fail(
            "bad-import",
            whole,
            "both sides of u_se< are strings: \"c:\" u_se< \"Library\"",
        )),
        _ => Err(fail(
            "misplaced-macro",
            macro_token.span,
            "u_se< is a statement of its own at the top level of a file",
        )),
    }
}

/// An alias is lowercase letters and a colon, and not `u:` or `l:`.
fn valid_alias(alias: &str, span: Span) -> Result<(), Diagnostic> {
    let letters = alias.strip_suffix(':').unwrap_or("");
    if letters.is_empty() || !letters.bytes().all(|b| b.is_ascii_lowercase()) {
        return Err(fail(
            "bad-alias",
            span,
            format!("{alias:?} is not an alias: write lowercase letters and a colon, like \"c:\""),
        ));
    }
    if letters == "u" || letters == "l" {
        return Err(fail(
            "reserved-alias",
            span,
            "u: and l: cannot be aliases (u: is the program, l: a library itself)",
        ));
    }
    Ok(())
}
