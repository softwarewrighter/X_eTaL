//! `xetal migrate`: a library's bare top-level functions written as
//! `h:` (PN2), with every use that refers to them; a lambda's own local
//! of the same name and bare variables are left alone. Only the names
//! change, so the file parses as before apart from them (rule 10).

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
    for (t, bound) in tokens.iter().zip(locals(&tokens)) {
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
