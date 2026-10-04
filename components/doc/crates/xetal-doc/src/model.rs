//! The model: files, the items defined in them, and what each uses.

use xetal_doccom::{Doc, Example};

/// One source file: a program, a library, a macro library or the
/// system macro library.
#[derive(Debug, Clone, PartialEq)]
pub struct DocFile {
    pub name: String,
    pub kind: &'static str,
    pub doc: Option<Doc>,
    pub items: Vec<Item>,
}

/// A definition, as written in its own file.
#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub name: String,
    pub kind: &'static str,
    pub public: bool,
    pub ty: String,
    pub line: usize,
    pub section: Option<String>,
    pub doc: Option<Doc>,
    pub source: String,
    pub uses: Vec<Use>,
}

/// A use of another item: as written here, and where it is defined.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Use {
    pub written: String,
    pub file: String,
    pub item: String,
}

impl Item {
    /// The item's examples (none without a doc comment).
    pub fn examples(&self) -> &[Example] {
        self.doc.as_ref().map_or(&[], |d| d.examples.as_slice())
    }
}

/// What a name written in its own file is: a macro (`<` at its end), a
/// function (an underlined letter), or a value.
pub(crate) fn kind_of(written: &str) -> &'static str {
    let base = written.rsplit_once(':').map_or(written, |(_, b)| b);
    match (base.ends_with('<'), base.contains('_')) {
        (true, _) => "macro",
        (false, true) => "function",
        _ => "value",
    }
}
