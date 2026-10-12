//! Machine-applicable suggestions: a pass's fix (a span in the combined
//! text) made an edit to the file as written, when it can be made
//! surely, and edits applied to a text.

use std::io::Write;

use xetal_base::{Diagnostic, Fix};
use xetal_sources::Sources;

use crate::driver::{Finding, lint};
use crate::rule::Rule;

/// Write `replacement` in place of bytes `start..end` of the file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edit {
    pub start: usize,
    pub end: usize,
    pub replacement: String,
}

impl Edit {
    pub fn new(start: usize, end: usize, replacement: impl Into<String>) -> Self {
        let replacement = replacement.into();
        Edit {
            start,
            end,
            replacement,
        }
    }
}

/// `fix` as an edit to the program's own file (number 0), or `None`
/// when it cannot be made surely: a span that is not wholly and exactly
/// in that file as written (a macro's expansion, an import), or that
/// does not hold a whole lambda `{ ... }` there.
pub(crate) fn edit(fix: &Fix, sources: &Sources) -> Option<Edit> {
    let (from, last) = (fix.span.start, fix.span.end.checked_sub(1)?);
    let (a, b) = (sources.locate(from), sources.locate(last));
    let exact = a.to == a.offset + 1 && b.to == b.offset + 1;
    if a.index != 0 || b.index != 0 || !exact {
        return None;
    }
    let text = sources.written_text(0);
    let written = text.get(a.offset..=b.offset)?;
    let whole = written.starts_with('{') && written.ends_with('}');
    whole.then(|| Edit::new(a.offset, b.offset + 1, fix.replacement.clone()))
}

/// `text` with `edits` made, and how many: earliest first, an edit that
/// overlaps one already kept is left out; applied from the end, so the
/// offsets stay true.
pub fn apply(text: &str, edits: &[Edit]) -> (String, usize) {
    let mut sorted: Vec<&Edit> = edits.iter().collect();
    sorted.sort_by_key(|e| (e.start, e.end));
    let mut kept: Vec<&Edit> = Vec::new();
    for e in sorted {
        let fits = kept.last().is_none_or(|k| k.end <= e.start);
        if fits && text.get(e.start..e.end).is_some() {
            kept.push(e);
        }
    }
    let mut out = text.to_string();
    for e in kept.iter().rev() {
        out.replace_range(e.start..e.end, &e.replacement);
    }
    (out, kept.len())
}

/// `text` with the edits of `findings` made, and how many.
pub fn fixed(text: &str, findings: &[Finding]) -> (String, usize) {
    let edits: Vec<Edit> = findings.iter().filter_map(|f| f.fix.clone()).collect();
    apply(text, &edits)
}

/// `--fix`: make the findings' edits in file `name` (written only when
/// the result still loads), say how many on `err`, and lint what is
/// left. Text given with `-e` has no file to rewrite.
pub(crate) fn fix_file(
    name: &str,
    text: &str,
    rules: &[Rule],
    err: &mut dyn Write,
) -> Result<Vec<Finding>, Diagnostic> {
    if name == "-e" {
        let message = "--fix rewrites a file: give a FILE path, not -e";
        return Err(Diagnostic::new("fix-needs-file", message));
    }
    let (after, n) = fixed(text, &lint(name, text, rules)?);
    let left = lint(name, &after, rules)?;
    if n > 0 {
        std::fs::write(name, &after)
            .map_err(|e| Diagnostic::new("io", format!("cannot write {name}: {e}")))?;
    }
    let _ = writeln!(err, "fixed {n} finding(s) in {name}");
    Ok(left)
}
