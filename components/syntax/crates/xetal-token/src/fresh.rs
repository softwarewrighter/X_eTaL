//! The fresh namespaces of hygiene (MC30).

/// A namespace the macro phase gives a name a macro binds (hygiene):
/// `g` and a number (`g1:t`), reserved, so no program writes one.
pub fn is_fresh(ns: &str) -> bool {
    ns.len() > 1 && ns.starts_with('g') && ns[1..].bytes().all(|b| b.is_ascii_digit())
}
