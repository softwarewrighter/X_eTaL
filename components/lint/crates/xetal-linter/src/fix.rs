//! Machine-applicable suggestions: a pass's fix (a span in the combined
//! text) made an edit to the file as written, when it can be made
//! surely, and edits applied to a text.

use std::io::Write;
use std::path::Path;
use std::process::Command;

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
/// in that file as written (a macro's expansion, an import), or whose
/// text there is not what the pass saw.
pub(crate) fn edit(fix: &Fix, sources: &Sources) -> Option<Edit> {
    let (from, last) = (fix.span.start, fix.span.end.checked_sub(1)?);
    let (a, b) = (sources.locate(from), sources.locate(last));
    let exact = a.to == a.offset + 1 && b.to == b.offset + 1;
    if a.index != 0 || b.index != 0 || !exact {
        return None;
    }
    let written = sources.written_text(0).get(a.offset..=b.offset)?;
    let seen = sources.combined().get(fix.span.start..fix.span.end)?;
    (written == seen).then(|| Edit::new(a.offset, b.offset + 1, fix.replacement.clone()))
}

/// Line and column just past the end of `span`, which starts in the
/// program's file: its start when the end is not written there.
pub(crate) fn ended(sources: &Sources, span: xetal_base::Span) -> (usize, usize) {
    let (start, last) = (sources.locate(span.start), span.end.saturating_sub(1));
    let last = sources.locate(last.max(span.start));
    match last.index == 0 && last.offset >= start.offset {
        true => (last.line, last.col + 1),
        false => (start.line, start.col),
    }
}

/// `text` with `fixes` made, and how many: each fix's edits together or
/// none, earliest fix first; a fix with an edit overlapping one already
/// kept (or outside the text) is left out. Applied from the end, so the
/// offsets stay true.
pub fn apply(text: &str, fixes: &[Vec<Edit>]) -> (String, usize) {
    let mut sorted: Vec<&Vec<Edit>> = fixes.iter().filter(|f| !f.is_empty()).collect();
    sorted.sort_by_key(|f| f.iter().map(|e| (e.start, e.end)).min());
    let (mut kept, mut n): (Vec<&Edit>, usize) = (Vec::new(), 0);
    for fix in sorted {
        let clear = |e: &Edit| kept.iter().all(|k| k.end <= e.start || e.end <= k.start);
        if fix
            .iter()
            .all(|e| clear(e) && text.get(e.start..e.end).is_some())
        {
            kept.extend(fix);
            n += 1;
        }
    }
    kept.sort_by_key(|e| std::cmp::Reverse(e.start));
    let mut out = text.to_string();
    for e in kept {
        out.replace_range(e.start..e.end, &e.replacement);
    }
    (out, n)
}

/// `text` with the fixes of `findings` made, and how many.
pub fn fixed(text: &str, findings: &[Finding]) -> (String, usize) {
    let fixes: Vec<Vec<Edit>> = findings.iter().map(|f| f.fix.clone()).collect();
    apply(text, &fixes)
}

/// `--fix`: make the findings' edits in file `name` (written only when
/// the result still loads), say how many on `err`, and lint what is
/// left. An error that stops the program from loading but carries a
/// fix of its own (a code string on a text side, `string-kind`, B2) is
/// made the same way, so the file loads and is linted after. Text
/// given with `-e` has no file to rewrite, and a file git cannot give
/// back as it was is not rewritten unless `allow_dirty`.
pub(crate) fn fix_file(
    name: &str,
    text: &str,
    rules: &[Rule],
    allow_dirty: bool,
    err: &mut dyn Write,
) -> Result<Vec<Finding>, Diagnostic> {
    if name == "-e" {
        let message = "--fix rewrites a file: give a FILE path, not -e";
        return Err(Diagnostic::new("fix-needs-file", message));
    }
    let (after, n) = match lint(name, text, rules) {
        Ok(findings) => fixed(text, &findings),
        Err(d) if !d.fix().is_empty() => {
            let edits = d
                .fix()
                .iter()
                .map(|f| Edit::new(f.span.start, f.span.end, f.replacement.clone()));
            apply(text, &[edits.collect()])
        }
        Err(d) => return Err(d),
    };
    let left = lint(name, &after, rules)?;
    if n > 0 && !allow_dirty && !restorable(name) {
        let message = format!(
            "{name} is not committed as it is (changed, untracked, or outside a git repository), so --fix could not be undone: commit it first, or pass --allow-dirty"
        );
        return Err(Diagnostic::new("fix-dirty", message));
    }
    if n > 0 {
        std::fs::write(name, &after)
            .map_err(|e| Diagnostic::new("io", format!("cannot write {name}: {e}")))?;
    }
    let _ = writeln!(err, "fixed {n} finding(s) in {name}");
    Ok(left)
}

/// Whether git can give file `name` back as it is now: tracked, and the
/// same as committed. Not when git is missing, or the file is outside a
/// repository, ignored, untracked, or changed (staged or not).
fn restorable(name: &str) -> bool {
    let path = Path::new(name);
    let dir = path.parent().filter(|d| !d.as_os_str().is_empty());
    let (dir, file) = (dir.unwrap_or(Path::new(".")), path.file_name());
    let git = |args: &[&str]| {
        let mut git = Command::new("git");
        git.arg("-C").arg(dir).args(args).arg("--").args(file);
        git.output().is_ok_and(|o| o.status.success())
    };
    file.is_some() && git(&["ls-files", "--error-unmatch"]) && git(&["diff", "--quiet", "HEAD"])
}
