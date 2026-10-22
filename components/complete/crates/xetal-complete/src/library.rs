//! Libraries: their names in the string after `u_se<`, and the exports
//! of one the text imports, after its alias.

use crate::names::{defined, offered};
use crate::word::{Candidate, Context};

/// The standard libraries, the standard macro libraries and the saved
/// files whose names start with `prefix` (case aside).
pub(crate) fn libraries(prefix: &str, ctx: &Context) -> Vec<Candidate> {
    let lower = prefix.to_lowercase();
    let one = |name: &str, what: &str| Candidate {
        label: name.to_string(),
        insert: name.to_string(),
        detail: what.to_string(),
    };
    let standard = xetal_libs::LIBRARIES
        .iter()
        .map(|(n, _)| one(n, "standard library"));
    let macros = xetal_libs::MACROS
        .iter()
        .map(|(n, _)| one(n, "standard macro library"));
    let files = ctx.files.iter().filter_map(|f| {
        let name = f.strip_suffix(".xtl").or_else(|| f.strip_suffix(".xtlm"))?;
        Some(one(name, "your file"))
    });
    standard
        .chain(macros)
        .chain(files)
        .filter(|c| c.label.to_lowercase().starts_with(&lower))
        .collect()
}

/// The library `text` imports as `alias` (`"x:" u_se< "Stats"`), if any.
fn imported(text: &str, alias: &str) -> Option<String> {
    let want = format!("\"{alias}:\"");
    text.lines().find_map(|line| {
        let rest = line.trim_start().strip_prefix(&want)?.trim_start();
        let rest = rest.strip_prefix("u_se<")?.trim_start();
        let name = rest.strip_prefix('"')?;
        Some(name.split('"').next()?.to_string())
    })
}

/// The first line of the `##` doc comment block just above line number
/// `line` (from 1), if there is one.
pub(crate) fn doc_above(lines: &[&str], line: usize) -> Option<String> {
    let block: Vec<&&str> = lines[..line.saturating_sub(1)]
        .iter()
        .rev()
        .take_while(|l| l.trim_start().starts_with("##"))
        .collect();
    let first = block.last()?;
    Some(first.trim().trim_start_matches('#').trim().to_string())
}

/// The exports of the library `alias` names, as `alias:name`, matching
/// `rest` (what is typed after the colon); each with its doc line when
/// the definition has one, else the library's name.
pub(crate) fn exports(text: &str, alias: &str, rest: &str, ctx: &Context) -> Vec<Candidate> {
    let Some(library) = imported(text, alias) else {
        return Vec::new();
    };
    let Some(source) = (ctx.library)(&library) else {
        return Vec::new();
    };
    let lines: Vec<&str> = source.lines().collect();
    defined(&source)
        .into_iter()
        .filter_map(|(name, line)| {
            let export = name
                .strip_prefix("l:")
                .or_else(|| name.strip_prefix("m:"))?;
            let doc = doc_above(&lines, line);
            let aliased = format!("{alias}:{export}");
            offered(&aliased, &format!("{alias}:{rest}")).then(|| Candidate {
                label: aliased.clone(),
                insert: aliased,
                detail: doc.unwrap_or_else(|| library.clone()),
            })
        })
        .collect()
}
