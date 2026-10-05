//! The entries of the search index, and the index as a script.

use xetal_doc::DocFile;
use xetal_dochtml::{anchor, page};

use crate::normalize;

/// One thing to find: its name, kind, type (and the type normalized),
/// the file it is in, where its documentation is, its doc's first line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub name: String,
    pub kind: String,
    pub ty: String,
    pub norm: String,
    pub file: String,
    pub href: String,
    pub about: String,
}

/// Every item of `files`, then every built-in of the catalog.
pub fn entries(files: &[DocFile]) -> Vec<Entry> {
    let items = files.iter().flat_map(|f| {
        f.items.iter().map(move |i| Entry {
            name: i.name.clone(),
            kind: i.kind.to_string(),
            norm: normalize(&i.ty),
            ty: i.ty.clone(),
            file: f.name.rsplit('/').next().unwrap_or(&f.name).to_string(),
            href: format!("{}.html#{}", page(&f.name), anchor(&i.name)),
            about: i.doc.as_ref().map_or(String::new(), |d| first(&d.text)),
        })
    });
    let builtins = xetal_catalog::BUILTINS.iter().filter(|b| b.implemented);
    let builtins = builtins.map(|b| Entry {
        name: b.name.to_string(),
        kind: "built-in".into(),
        ty: b.sig.to_string(),
        norm: normalize(b.sig),
        file: String::new(),
        href: format!("builtins.html#{}", anchor(b.name)),
        about: xetal_dochtml::described(b.name).map_or(b.rule.to_string(), |d| first(&d)),
    });
    items.chain(builtins).collect()
}

/// The first line of a doc's prose (its first sentence, at most).
fn first(text: &str) -> String {
    let line = text.lines().next().unwrap_or("");
    let end = line.find(". ").map_or(line.len(), |i| i + 1);
    line[..end].to_string()
}

/// The index as a script defining `window.XETAL_DOC_INDEX`: one array
/// per entry, `[name, kind, type, normalized type, file, href, about]`.
pub fn script(entries: &[Entry]) -> String {
    let rows: Vec<String> = entries
        .iter()
        .map(|e| {
            let fields = [&e.name, &e.kind, &e.ty, &e.norm, &e.file, &e.href, &e.about];
            let quoted: Vec<String> = fields.iter().map(|f| string(f)).collect();
            format!("[{}]", quoted.join(","))
        })
        .collect();
    format!("window.XETAL_DOC_INDEX = [\n{}\n];\n", rows.join(",\n"))
}

/// A JavaScript string literal (JSON's rules).
fn string(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '<' => out.push_str("\\u003c"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}
