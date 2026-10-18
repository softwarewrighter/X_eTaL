//! The shape of `builtins.toml`.

use std::collections::BTreeMap;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Catalog {
    pub group: Vec<Group>,
    /// Absent once every listed built-in is implemented.
    #[serde(default)]
    pub later: Vec<Later>,
}

/// Implemented built-ins: `[name, arity, signature]` rows, with the
/// shape rule of the group (`shape`) or of one entry (`shapes`).
#[derive(Deserialize)]
pub struct Group {
    pub rule: String,
    pub entries: Vec<(String, usize, String)>,
    #[serde(default)]
    pub shape: String,
    #[serde(default)]
    pub shapes: BTreeMap<String, String>,
}

/// Built-ins that exist but arrive with a later saga.
#[derive(Deserialize)]
pub struct Later {
    pub rule: String,
    pub names: Vec<String>,
}
