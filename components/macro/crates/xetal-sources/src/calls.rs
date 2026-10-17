//! The macro calls a file's expansion came from: where each was written,
//! the macro as the call writes it, and the names it declares it binds
//! for the call's text (`## binds:`, MC30). Lints read them to tell code
//! a macro wrote from code copied from the call (docs/lint.md).

use crate::Sources;

/// A macro call written in file `file` at byte `at` (as written).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Site {
    pub file: usize,
    pub at: usize,
    /// The macro as written there (`x:w_ith<`, `t_ry<`).
    pub name: String,
    /// The names it declares it binds for the call's text.
    pub binds: Vec<String>,
}

impl Sources {
    /// Record a macro call (expanded in turn: one inside an expansion is
    /// recorded at its outermost call, after it).
    pub fn add_site(&mut self, site: Site) {
        self.sites.push(site);
    }

    /// The call written in file `file` at byte `at`: the first recorded
    /// there (the outermost), with the binds of every call expanded
    /// from it.
    pub fn site(&self, file: usize, at: usize) -> Option<Site> {
        let mut found = self.sites.iter().filter(|s| s.file == file && s.at == at);
        let mut site = found.next()?.clone();
        found.for_each(|s| site.binds.extend(s.binds.iter().cloned()));
        Some(site)
    }
}
