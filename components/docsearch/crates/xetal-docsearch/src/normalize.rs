//! Types as search compares them (Hoogle-like): without constraints,
//! split into names and runs of symbols, each type variable (a name
//! starting with a lowercase letter) renamed `a`, `b`, ... in the order
//! it first appears. search.js normalizes a typed query the same way.

/// `ty` normalized: `Num b => b -> b -> b` is `a -> a -> a`.
pub fn normalize(ty: &str) -> String {
    let body = ty.split_once("=>").map_or(ty, |(_, b)| b);
    let mut seen: Vec<String> = Vec::new();
    let shown: Vec<String> = tokens(body)
        .into_iter()
        .map(|t| match t.starts_with(|c: char| c.is_ascii_lowercase()) {
            true => rename(&mut seen, t),
            false => t,
        })
        .collect();
    shown.join(" ")
}

/// A type variable's new name: by the order the variables appear.
fn rename(seen: &mut Vec<String>, var: String) -> String {
    let at = seen.iter().position(|v| *v == var).unwrap_or_else(|| {
        seen.push(var);
        seen.len() - 1
    });
    char::from(b'a' + (at % 26) as u8).to_string()
}

/// Runs of letters, digits and `_`, and runs of other characters that
/// are not white space.
fn tokens(text: &str) -> Vec<String> {
    let word = |c: char| c.is_ascii_alphanumeric() || c == '_';
    let mut out: Vec<String> = Vec::new();
    let mut last: Option<bool> = None;
    for c in text.chars() {
        if c.is_whitespace() {
            last = None;
            continue;
        }
        match (last, out.last_mut()) {
            (Some(w), Some(t)) if w == word(c) => t.push(c),
            _ => out.push(c.to_string()),
        }
        last = Some(word(c));
    }
    out
}
