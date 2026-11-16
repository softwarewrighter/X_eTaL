//! `xetal migrate`: a library's bare top-level functions written as
//! `h:` (PN2), with every use that refers to them; a lambda's own local
//! of the same name and bare variables are left alone; inside a format
//! template (`f_ormat<`, `p_anic<`, `t_odo<`) the code in braces is
//! renamed too. Only the names change, so the file parses as before
//! apart from them (rule 10).

use std::collections::HashSet;

use xetal_base::Diagnostic;
use xetal_lex::{TokenKind, lex};

use crate::imports::statements;
use crate::names::locals;
use crate::rename::name;

/// `text` with its bare top-level functions renamed to `h:`.
pub fn migrate(text: &str) -> Result<String, Diagnostic> {
    let tokens = lex(text)?;
    let mut helpers = HashSet::new();
    for statement in statements(&tokens) {
        let defines = matches!(statement.get(1).map(|t| &t.kind), Some(TokenKind::Assign));
        if let (true, Some(first)) = (defines, statement.first())
            && let (TokenKind::Func(_), Some((None, key))) = (&first.kind, name(first))
        {
            helpers.insert(key);
        }
    }
    let mut out = String::with_capacity(text.len() + helpers.len() * 8);
    let mut at = 0;
    for (i, (t, bound)) in tokens.iter().zip(locals(&tokens)).enumerate() {
        if matches!(t.kind, TokenKind::Str(_)) && i > 0 && formats(&tokens[i - 1].kind) {
            out.push_str(&text[at..t.span.start]);
            out.push_str(&holes(&text[t.span.start..t.span.end], &helpers));
            at = t.span.end;
            continue;
        }
        let ours = matches!(t.kind, TokenKind::Func(_))
            && name(t).is_some_and(|(ns, key)| ns.is_none() && helpers.contains(&key));
        if ours && !bound {
            out.push_str(&text[at..t.span.start]);
            out.push_str("h:");
            at = t.span.start;
        }
    }
    out.push_str(&text[at..]);
    Ok(out)
}

/// Whether a token is a macro whose string is a format template.
fn formats(kind: &TokenKind) -> bool {
    matches!(kind, TokenKind::Func(f) if f.ns.is_none()
        && ["f_ormat<", "p_anic<", "t_odo<"].contains(&f.spelled().as_str()))
}

/// A template string token written `raw` with the helpers in its holes
/// (`{...}`, not `{{`) written `h:`; the rest as written.
fn holes(raw: &str, helpers: &HashSet<String>) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut rest = raw;
    while let Some(open) = rest.find('{') {
        out.push_str(&rest[..=open]);
        rest = &rest[open + 1..];
        if let Some(after) = rest.strip_prefix('{') {
            out.push('{');
            rest = after;
            continue;
        }
        let close = rest.find('}').unwrap_or(rest.len());
        let inner = &rest[..close];
        out.push_str(&rename(inner, helpers));
        rest = &rest[close..];
    }
    out.push_str(rest);
    out
}

/// `code` with the bare uses of `helpers` written `h:`.
fn rename(code: &str, helpers: &HashSet<String>) -> String {
    let Ok(tokens) = lex(code) else {
        return code.to_string();
    };
    let mut out = String::with_capacity(code.len() + 8);
    let mut at = 0;
    for t in &tokens {
        let ours = matches!(t.kind, TokenKind::Func(_))
            && name(t).is_some_and(|(ns, key)| ns.is_none() && helpers.contains(&key));
        if ours {
            out.push_str(&code[at..t.span.start]);
            out.push_str("h:");
            at = t.span.start;
        }
    }
    out.push_str(&code[at..]);
    out
}
