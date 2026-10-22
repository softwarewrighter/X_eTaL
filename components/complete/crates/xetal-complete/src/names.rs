//! Names: the built-ins of the catalog, the names the text defines, and
//! the lambda arguments.

use xetal_catalog::BUILTINS;

use crate::word::Candidate;

/// A name without its underlines, so `ran` finds `r_ange` and `r_a`
/// does too.
fn plain(name: &str) -> String {
    name.replace('_', "")
}

/// Whether `name` is offered for the `prefix` typed: the same letters
/// so far, underlines aside, and not the whole name already.
pub(crate) fn offered(name: &str, prefix: &str) -> bool {
    name != prefix && (name.starts_with(prefix) || plain(name).starts_with(&plain(prefix)))
}

/// The built-ins and the text's own definitions matching `prefix`.
pub(crate) fn names(text: &str, prefix: &str) -> Vec<Candidate> {
    let mut out: Vec<Candidate> = BUILTINS
        .iter()
        .filter(|b| b.implemented && offered(b.name, prefix))
        .map(|b| Candidate {
            label: b.name.to_string(),
            insert: b.name.to_string(),
            detail: b.sig.to_string(),
        })
        .collect();
    out.extend(defined(text).into_iter().filter_map(|(name, line)| {
        offered(&name, prefix).then(|| Candidate {
            insert: name.clone(),
            label: name,
            detail: format!("defined on line {line}"),
        })
    }));
    out
}

/// The names `text` binds with `:=`, each with its line number (from
/// 1): `u:k_eep := ...`, `x := 2`, `l:m_ean := ...`.
pub(crate) fn defined(text: &str) -> Vec<(String, usize)> {
    let is_name = |c: char| c.is_ascii_alphanumeric() || matches!(c, '_' | ':' | '!' | '<');
    text.lines()
        .enumerate()
        .filter_map(|(i, line)| {
            let line = line.trim_start();
            let end = line.find(|c: char| !is_name(c)).unwrap_or(line.len());
            let (name, rest) = line.split_at(end);
            let bound = rest.trim_start().starts_with(":=");
            (bound
                && name
                    .chars()
                    .next()
                    .is_some_and(|c| c.is_ascii_alphabetic() || c == '_'))
            .then(|| (name.to_string(), i + 1))
        })
        .collect()
}

/// After a lone `_` inside a block: the lambda arguments.
pub(crate) fn arguments(before: &str) -> Vec<Candidate> {
    let opened = before.matches('{').count() > before.matches('}').count();
    if !opened {
        return Vec::new();
    }
    [
        ("_l", "the left argument (drawn as alpha)"),
        ("_r", "the right argument (drawn as omega)"),
        ("_u", "a parameter nothing reads (a parameter list only)"),
    ]
    .iter()
    .map(|(name, what)| Candidate {
        label: name.to_string(),
        insert: name.to_string(),
        detail: what.to_string(),
    })
    .collect()
}
