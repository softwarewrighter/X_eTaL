//! Hygiene (MC30): a name a macro binds around text copied from its
//! call's arguments (a parameter of a lambda the macro wrote, or a
//! binding inside one) is renamed to a fresh name, `g1:t`, so the
//! user's text never sees it; names the macro declares with
//! `## binds:` (anaphora) stay as written.

use std::cell::Cell;

use xetal_base::Diagnostic;
use xetal_lex::{Token, TokenKind, is_fresh, lex};
use xetal_mapped::Mapped;

/// `m` (a macro's expansion) with the binders it introduced around the
/// call's text renamed, numbered from `fresh`; `binds` are kept.
pub(crate) fn hygiene(m: &Mapped, binds: &[String], fresh: &Cell<usize>) -> Mapped {
    let Ok(tokens) = lex(m.text()) else {
        return m.clone();
    };
    let added = |t: &Token| !m.copied(t.span.start);
    let raw = |t: &Token| m.text()[t.span.start..t.span.end].to_string();
    let mut renamed: Vec<Option<String>> = vec![None; tokens.len()];
    for (i, t) in tokens.iter().enumerate() {
        if t.kind != TokenKind::LBrace || !added(t) {
            continue;
        }
        let end = closing(&tokens, i);
        let inner = &tokens[i + 1..end];
        let code = |t: &Token| !added(t) && !matches!(t.kind, TokenKind::Str(_));
        if !inner.iter().any(code) {
            continue;
        }
        for key in binders(inner).into_iter().filter(|b| added(b)).map(raw) {
            let todo: Vec<usize> = (i + 1..end)
                .filter(|&k| renamed[k].is_none() && added(&tokens[k]) && raw(&tokens[k]) == key)
                .collect();
            if binds.contains(&key) || todo.is_empty() {
                continue;
            }
            fresh.set(fresh.get() + 1);
            for k in todo {
                renamed[k] = Some(format!("g{}:{key}", fresh.get()));
            }
        }
    }
    rebuild(m, &tokens, &renamed)
}

/// The index of the `}` closing the `{` at `open` (or the end).
fn closing(tokens: &[Token], open: usize) -> usize {
    let mut depth = 0;
    for (k, t) in tokens.iter().enumerate().skip(open) {
        match t.kind {
            TokenKind::LBrace | TokenKind::LParen | TokenKind::LBracket => depth += 1,
            TokenKind::RBrace | TokenKind::RParen | TokenKind::RBracket => {
                depth -= 1;
                if depth == 0 {
                    return k;
                }
            }
            _ => {}
        }
    }
    tokens.len()
}

/// The plain names a lambda body (`inner`, between its braces) binds:
/// its parameters before `->`, and its own top-level local bindings.
fn binders(inner: &[Token]) -> Vec<&Token> {
    let plain = |t: &Token| match &t.kind {
        TokenKind::Var(v) => v.ns.is_none(),
        TokenKind::Func(f) => f.ns.is_none(),
        _ => false,
    };
    let arrow = inner.iter().position(|t| t.kind == TokenKind::Arrow);
    // Parameters are names, `@`, `~` and tuple patterns (TU11).
    let params = arrow.filter(|&a| {
        inner[..a].iter().all(|t| {
            plain(t)
                || matches!(
                    t.kind,
                    TokenKind::Unit
                        | TokenKind::Lazy
                        | TokenKind::LParen
                        | TokenKind::RParen
                        | TokenKind::Comma
                        | TokenKind::Wild
                )
        })
    });
    let mut out: Vec<&Token> = params.map_or(Vec::new(), |a| {
        inner[..a].iter().filter(|t| plain(t)).collect()
    });
    let (mut depth, mut start) = (0, true);
    for (k, t) in inner.iter().enumerate() {
        match t.kind {
            TokenKind::LBrace | TokenKind::LParen | TokenKind::LBracket => depth += 1,
            TokenKind::RBrace | TokenKind::RParen | TokenKind::RBracket => depth -= 1,
            _ => {}
        }
        let next = inner.get(k + 1).map(|n| &n.kind);
        if depth == 0 && start && plain(t) && next == Some(&TokenKind::Assign) {
            out.push(t);
        }
        if depth == 1 && start && t.kind == TokenKind::LParen {
            out.extend(pattern_names(inner, k).into_iter().filter(|t| plain(t)));
        }
        start = depth == 0
            && matches!(
                t.kind,
                TokenKind::Semi | TokenKind::Newline | TokenKind::Arrow
            );
    }
    out
}

/// The names of a tuple pattern binding (`(a, (b, _)) := ...`, TU11)
/// whose `(` is `inner[open]`; none when no `:=` follows its `)`.
fn pattern_names(inner: &[Token], open: usize) -> Vec<&Token> {
    let close = open + closing(&inner[open..], 0);
    match inner.get(close + 1).map(|t| &t.kind) {
        Some(TokenKind::Assign) => inner[open..close].iter().collect(),
        _ => Vec::new(),
    }
}

/// `m` with the tokens given a new spelling replaced (each mapped to
/// where the token came from).
fn rebuild(m: &Mapped, tokens: &[Token], renamed: &[Option<String>]) -> Mapped {
    let (mut out, mut at) = (Mapped::default(), 0);
    for (t, new) in tokens.iter().zip(renamed) {
        if let Some(new) = new {
            out.push(&m.slice(at..t.span.start));
            let from = m.span(t.span);
            out.glue(new, from.start..from.end);
            at = t.span.end;
        }
    }
    out.push(&m.slice(at..m.text().len()));
    out
}

/// A program may not write the macro phase's fresh namespaces.
pub(crate) fn unwritten(tokens: &[Token]) -> Result<(), Diagnostic> {
    let ns = |t: &Token| match &t.kind {
        TokenKind::Var(v) => v.ns.clone(),
        TokenKind::Func(f) => f.ns.clone(),
        _ => None,
    };
    match tokens.iter().find(|t| ns(t).is_some_and(|n| is_fresh(&n))) {
        None => Ok(()),
        Some(t) => {
            let message = "g and a number (g1:) is the namespace of names a macro binds; a program cannot write it";
            Err(Diagnostic::new("reserved-namespace", message).with_span(t.span))
        }
    }
}
