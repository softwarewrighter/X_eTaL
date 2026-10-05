//! The links in a source, and where each item is used.

use std::collections::BTreeMap;

use xetal_base::Span;
use xetal_lex::{Token, TokenKind, lex};

use xetal_doc::Expansion;

use crate::scope::Scopes;
use crate::{Resolver, Target};

/// A name (its bytes in the source) and what it names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Link {
    pub span: Span,
    pub target: Target,
}

/// The links in `text`, written in file `file`: every name that names
/// something documented or built in, and each import's library string.
/// Text that does not lex has none.
pub fn links(r: &Resolver, file: usize, text: &str) -> Vec<Link> {
    let tokens = lex(text).unwrap_or_default();
    let mut scopes = Scopes::default();
    let mut out = Vec::new();
    for i in 0..tokens.len() {
        let target = match imported(r, file, &tokens, i) {
            Some(target) => Some(target),
            None => scopes
                .visit(&tokens, i)
                .and_then(|name| r.resolve(file, &name)),
        };
        let span = tokens[i].span;
        out.extend(target.map(|target| Link { span, target }));
    }
    out
}

/// Token `i` is the library string of `"a:" u_se< "Name"`: the file
/// that import found (the library, else the macro library).
fn imported(r: &Resolver, file: usize, tokens: &[Token], i: usize) -> Option<Target> {
    let TokenKind::Str(spec) = &tokens[i].kind else {
        return None;
    };
    let before = &tokens[i.checked_sub(1)?].kind;
    let is_use = matches!(before, TokenKind::Func(f) if f.ns.is_none() && f.spelled() == "u_se<");
    let import = r.imports(file).find(|m| &m.spec == spec);
    let found = import.filter(|_| is_use)?.files.first()?;
    r.file_index(found).map(Target::File)
}

/// Where each item is used: (file, line from 1) for every link to it
/// in the documented files' sources and in their macro expansions (at
/// the line of the call written in the file), in file and line order.
pub fn uses(r: &Resolver) -> BTreeMap<Target, Vec<(usize, usize)>> {
    let mut found: Vec<(Target, (usize, usize))> = Vec::new();
    for (file, f) in r.files.iter().enumerate() {
        for link in links(r, file, &f.text) {
            let line = f.text[..link.span.start].matches('\n').count() + 1;
            found.push((link.target, (file, line)));
        }
        let mut pending: Vec<(usize, &Expansion)> =
            f.expansions.iter().map(|e| (e.line, e)).collect();
        while let Some((line, e)) = pending.pop() {
            found.extend(
                links(r, file, &e.text)
                    .into_iter()
                    .map(|l| (l.target, (file, line))),
            );
            pending.extend(e.nested.iter().map(|n| (line, n)));
        }
    }
    let mut out: BTreeMap<Target, Vec<(usize, usize)>> = BTreeMap::new();
    for (target, place) in found {
        out.entry(target).or_default().push(place);
    }
    out.retain(|t, _| matches!(t, Target::Item { .. }));
    out.values_mut().for_each(|places| {
        places.sort();
        places.dedup();
    });
    out
}
