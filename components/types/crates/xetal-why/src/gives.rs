//! What a built-in gives, read from its signature in the catalog.

use xetal_core::{Expr, Kind};

/// A built-in's type as the catalog writes it (`Num a => a -> a -> a`).
pub fn signature(name: &str) -> Option<&'static str> {
    xetal_catalog::find(name)
        .map(|b| b.sig)
        .filter(|s| !s.is_empty())
}

/// For an application of a built-in whose result is always one type,
/// what to say of it: "`d_iv` always gives a Float".
pub fn gives(e: &Expr) -> Option<String> {
    let f = match &e.kind {
        Kind::App(f, _) | Kind::App2 { f, .. } => f,
        _ => return None,
    };
    let Kind::Prim(name) = &f.kind else {
        return None;
    };
    let result = signature(name)?.rsplit("->").next()?.trim();
    let concrete = result.chars().next().is_some_and(char::is_uppercase)
        && result.chars().all(char::is_alphanumeric);
    concrete.then(|| format!("`{name}` always gives {}", article(result)))
}

/// A type named with its article: "an Int", "a Float".
pub(crate) fn article(ty: &str) -> String {
    match ty.starts_with(['A', 'E', 'I', 'O', 'U']) {
        true => format!("an {ty}"),
        false => format!("a {ty}"),
    }
}
