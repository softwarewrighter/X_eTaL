//! The names a macro deliberately binds for the text of its call
//! (anaphora, MC30): a line `## binds: it` (one or more names) in the
//! `##` doc block directly above the macro's definition.

use std::collections::HashMap;

/// Per macro defined in `text` (a macro library), by its name with the
/// mark (`w_ith<`), the names its doc block declares it binds.
pub fn binds_of(text: &str) -> HashMap<String, Vec<String>> {
    let (mut out, mut pending) = (HashMap::new(), Vec::new());
    for line in text.lines() {
        if let Some(doc) = line.strip_prefix("##") {
            if let Some(names) = doc.trim_start().strip_prefix("binds:") {
                pending.extend(names.split_whitespace().map(String::from));
            }
            continue;
        }
        if let Some(name) = defined(line).filter(|_| !pending.is_empty()) {
            out.insert(name, std::mem::take(&mut pending));
        }
        pending.clear();
    }
    out
}

/// The macro `line` defines (`m:w_ith< := ...` gives `w_ith<`).
fn defined(line: &str) -> Option<String> {
    let (head, _) = line.split_once(":=")?;
    let (_, name) = head.trim().split_once(':')?;
    name.ends_with('<').then(|| name.to_string())
}
