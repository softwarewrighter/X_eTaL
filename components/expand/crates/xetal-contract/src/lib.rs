//! A macro's contract (MC31), read from the `##` doc block directly
//! above its definition: `## binds:` the names it binds for the text of
//! its call (anaphora, MC30); `## takes:` the shape of call it takes
//! (`text text`, `@ text`, `text @`, `@ @`), checked before it runs;
//! `## uses: copied` (the default) or `## uses: parsed`, whether every
//! name and number of its arguments must appear in its expansion.

mod doc;
mod takes;
mod uses;

pub use doc::contracts_of;
pub use takes::shape;
pub use uses::unused;

/// What a macro declares about its calls.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Contract {
    /// The names it binds for its call's text (`## binds: it`).
    pub binds: Vec<String>,
    /// Which sides take text (`## takes: @ text` is `(false, true)`);
    /// none declared, any shape.
    pub takes: Option<(bool, bool)>,
    /// How it treats its arguments.
    pub uses: Uses,
}

/// Whether a macro copies its arguments into its text or parses them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Uses {
    /// Every name and number of each argument appears in the expansion.
    #[default]
    Copied,
    /// The macro reads a notation and answers for it (the Parsing library).
    Parsed,
}
