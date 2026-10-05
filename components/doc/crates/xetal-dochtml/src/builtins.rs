//! What each built-in does, from docs/reference/builtins.ref (the
//! source of docs/reference.md): the description under its `== name`
//! line, without its examples.

/// The reference's source, built in.
const REFERENCE: &str = include_str!("../../../../../docs/reference/builtins.ref");

/// The description of the built-in `name`, if the reference has one
/// (its lines joined; `code` in backquotes as written).
pub fn described(name: &str) -> Option<String> {
    let mut lines = REFERENCE.lines();
    lines.find(|l| l.strip_prefix("== ") == Some(name))?;
    let text: Vec<&str> = lines
        .take_while(|l| !l.starts_with("== "))
        .filter(|l| !l.starts_with('>') && !l.starts_with("!>") && !l.starts_with(";;"))
        .collect();
    let text = text.join(" ").trim().to_string();
    (!text.is_empty()).then_some(text)
}
