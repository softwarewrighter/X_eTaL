//! The macro calls a file's expansion came from: where each was written,
//! the macro as the call writes it, the names it declares it binds for
//! the call's text (`## binds:`, MC30) and whether it parses its
//! arguments (`## uses: parsed`, MC31). Lints read them to tell code a
//! macro wrote from code copied from the call (docs/lint.md).

use std::ops::Range;

use crate::Sources;

/// A macro call written in file `file` over bytes `span` (as written).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Site {
    pub file: usize,
    pub span: Range<usize>,
    /// The macro as written there (`x:w_ith<`, `t_ry<`).
    pub name: String,
    /// The names it declares it binds for the call's text.
    pub binds: Vec<String>,
    /// It reads a notation (`## uses: parsed`): its text is not the
    /// call's code, even where it repeats it.
    pub parsed: bool,
    /// Which sides it takes text on (`## takes:`), none declared.
    pub takes: Option<(bool, bool)>,
    /// Which of those take X_eTaL (`## takes: @ code`, ST5).
    pub code: (bool, bool),
}

impl Sources {
    /// Record a macro call (expanded in turn: one inside an expansion is
    /// recorded at its outermost call, after it).
    pub fn add_site(&mut self, site: Site) {
        self.sites.push(site);
    }

    /// Every call recorded, outermost first.
    pub fn sites(&self) -> &[Site] {
        &self.sites
    }

    /// The call written in file `file` over byte `offset`: the first
    /// recorded there (the outermost), with the binds of every call
    /// expanded from it, parsed if any of them parses.
    pub fn site(&self, file: usize, offset: usize) -> Option<Site> {
        let over = |s: &&Site| s.file == file && s.span.contains(&offset);
        let mut found = self.sites.iter().filter(over);
        let mut site = found.next()?.clone();
        for s in found {
            site.binds.extend(s.binds.iter().cloned());
            site.parsed |= s.parsed;
        }
        Some(site)
    }
}
