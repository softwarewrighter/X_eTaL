//! A code string's inside as a probe: the body of a lambda over the
//! names it uses (the macro's `## binds:` first, then every other plain
//! name it does not bind itself), bound to `_` so it can be checked
//! with the program's functions in scope.

use xetal_lex::{Token, TokenKind, lex};

use crate::args::Arg;

/// One inside, ready to check.
#[derive(Debug, Clone)]
pub(crate) struct Probe {
    /// The statement checked: `_ := { it y -> INSIDE\n}`.
    pub line: String,
    /// Where the inside starts in `line`, and its length.
    pub body: usize,
    pub len: usize,
    /// Where the inside's text starts in the file as written, and for
    /// each byte of it (and one past) the offset from there.
    pub at: usize,
    pub map: Vec<usize>,
    /// The macro reads its text itself (`## uses: parsed`).
    pub parsed: bool,
}

/// The probe for `a` (of `text`): a code string on a code side whose
/// inside lexes, names no mutable variable and does not mix `_l`/`_r`
/// with named parameters; otherwise none (the expansion is checked).
pub(crate) fn probe(text: &str, a: &Arg) -> Option<Probe> {
    if !(a.takes_code && a.code) {
        return None;
    }
    let (at, inside, map) = inside(&text[a.span.start..a.span.end]);
    let tokens = lex(&inside).ok()?;
    let mutable = tokens
        .iter()
        .any(|t| matches!(&t.kind, TokenKind::Var(v) if v.mutable));
    let shorthand = tokens
        .iter()
        .any(|t| matches!(t.kind, TokenKind::LamArg { .. }));
    let mut params = a.binds.clone();
    for name in free(&tokens) {
        if !params.contains(&name) {
            params.push(name);
        }
    }
    let head = match (shorthand, params.is_empty()) {
        _ if mutable => return None,
        (true, true) => "{ ".to_string(),
        (true, false) => return None,
        (false, true) => "{ @ -> ".to_string(),
        (false, false) => format!("{{ {} -> ", params.join(" ")),
    };
    let body = "_ := ".len() + head.len();
    Some(Probe {
        line: format!("_ := {head}{inside}\n}}"),
        body,
        len: inside.len(),
        at: a.span.start + at,
        map,
        parsed: a.parsed,
    })
}

/// The inside of the code string `written`: where its text starts in
/// `written`, the text with escapes resolved, and the byte map.
fn inside(written: &str) -> (usize, String, Vec<usize>) {
    let hashes = written[1..].len() - written[1..].trim_start_matches('#').len();
    let text = &written[2 + hashes..written.len() - 1 - hashes];
    match hashes {
        0 => {
            let (resolved, map) = xetal_strtext::unescape_mapped(text);
            (2, resolved, map)
        }
        _ => (2 + hashes, text.to_string(), (0..=text.len()).collect()),
    }
}

/// The plain names `tokens` read and do not bind: a name in a run of
/// names and brackets that ends at `:=` or `->` is bound there.
fn free(tokens: &[Token]) -> Vec<String> {
    let in_head = |k: &TokenKind| {
        matches!(
            k,
            TokenKind::Var(_)
                | TokenKind::LParen
                | TokenKind::RParen
                | TokenKind::Comma
                | TokenKind::Wild
        )
    };
    let (mut bound, mut read) = (Vec::new(), Vec::new());
    let mut i = 0;
    while i < tokens.len() {
        let end = (i..tokens.len())
            .find(|&j| !in_head(&tokens[j].kind))
            .unwrap_or(tokens.len());
        let binds = end > i
            && matches!(
                tokens.get(end).map(|t| &t.kind),
                Some(TokenKind::Assign | TokenKind::Arrow)
            );
        for t in &tokens[i..end.max(i + 1)] {
            if let TokenKind::Var(v) = &t.kind
                && v.ns.is_none()
            {
                let list = if binds { &mut bound } else { &mut read };
                list.push(v.name.clone());
            }
        }
        i = end.max(i + 1);
    }
    let mut out: Vec<String> = Vec::new();
    for name in read {
        if !bound.contains(&name) && !out.contains(&name) {
            out.push(name);
        }
    }
    out
}
