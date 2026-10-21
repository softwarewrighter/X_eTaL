//! Linting one program: load it, run every lint pass, keep what is in
//! the program's own file, and give each finding the level its rules
//! resolve to (allowed findings are dropped).

use std::fmt;

use xetal_base::Diagnostic;
use xetal_ir::Program;
use xetal_lint::{Level, Lint};
use xetal_macro::{FsLibraries, Libraries};
use xetal_program::{Loaded, expanded_with, is_library};
use xetal_sources::Sources;
use xetal_strings::line_col;

use crate::directive::directives;
use crate::fix::{Edit, edit, ended};
use crate::rule::{Rule, resolve};
use crate::scope::{rules, unknown};

/// One reported lint, placed in the program's file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    pub lint: &'static Lint,
    pub level: Level,
    pub file: String,
    pub line: usize,
    pub col: usize,
    /// Where the finding's span ends, just past it (as an editor marks
    /// a range); the start when it ends outside the file as written.
    pub end_line: usize,
    pub end_col: usize,
    pub message: String,
    /// The edits that fix it, made together; empty when they cannot all
    /// be made surely.
    pub fix: Vec<Edit>,
}

impl fmt::Display for Finding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let level = if self.level == Level::Deny {
            "error"
        } else {
            "warning"
        };
        let (code, message) = (self.lint.name, &self.message);
        write!(
            f,
            "{level}[{code}]: {message} at {}:{}:{}",
            self.file, self.line, self.col
        )
    }
}

/// The findings for program `text`, reported as `name`, under the
/// command line's `cli` rules and the text's own `# lint:` directives;
/// in file order. A library is loaded as it is when imported. A
/// program that does not load is an error. Libraries are found beside
/// the file (and the standard ones built in).
pub fn lint(name: &str, text: &str, cli: &[Rule]) -> Result<Vec<Finding>, Diagnostic> {
    lint_with(name, text, cli, &FsLibraries::from_env())
}

/// [`lint`], its libraries found by `libs` (the live demo's come from
/// the browser's storage); a text naming no `l:` is a library when its
/// expansion names one (ask E7), as `is_library_named` decides.
pub fn lint_with(
    name: &str,
    text: &str,
    cli: &[Rule],
    libs: &dyn Libraries,
) -> Result<Vec<Finding>, Diagnostic> {
    let library = is_library(text)
        || !text.starts_with("#!") && expanded_with(name, text, libs).is_ok_and(|e| is_library(&e));
    let loaded = match library {
        true => xetal_program::load_library_with(name, text, libs)?,
        false => xetal_program::load_with(name, text, libs)?,
    };
    let written = directives(text);
    let mut all = cli.to_vec();
    all.extend(rules(&written, (text, 0), &loaded.program, &loaded.sources));
    let mut raw: Vec<Raw> = unknown(&written)
        .into_iter()
        .map(|(line, col, message)| ("unknown-lint", (line, col, line, col), message, Vec::new()))
        .collect();
    raw.extend(placed(&loaded));
    raw.extend(as_written(text, xetal_strings::strings(text)));
    let code = xetal_codecheck::checks(name, text, &loaded.sources, library);
    raw.extend(as_written(text, code));
    let mut out: Vec<Finding> = raw
        .into_iter()
        .filter_map(|r| leveled(r, name, &all))
        .collect();
    out.sort_by(|a, b| (a.line, a.col, a.lint.name).cmp(&(b.line, b.col, b.lint.name)));
    out.dedup();
    Ok(out)
}

/// A finding before its level: lint name, place, message, edit.
type Raw = (&'static str, Place, String, Vec<Edit>);

/// Line and column of a finding's start, then of just past its end.
type Place = (usize, usize, usize, usize);

/// What the lint passes find in the program's own file, placed there.
fn placed(loaded: &Loaded) -> Vec<Raw> {
    let mut out = Vec::new();
    let mut found = xetal_lint::lints(&loaded.program);
    found.extend(xetal_typed::typed(&loaded.program));
    found.extend(xetal_origin::captures(&loaded.program, &loaded.sources));
    found.extend(xetal_shape::shapes(&loaded.program));
    for d in xetal_macrolint::merged(found, &loaded.program, &loaded.sources) {
        let d = xetal_origin::placed(d, &loaded.sources);
        let at = loaded.sources.locate(d.span.map_or(0, |s| s.start));
        if at.index == 0 {
            let end = d
                .span
                .map_or((at.line, at.col), |s| ended(&loaded.sources, s));
            let fix = d
                .fix()
                .iter()
                .map(|f| edit(f, &loaded.sources))
                .collect::<Option<_>>();
            let fix = fix.unwrap_or_default();
            let lint = xetal_lint::lint(&d.code);
            let place = (at.line, at.col, end.0, end.1);
            out.extend(lint.map(|l| (l.name, place, d.message, fix)));
        }
    }
    out
}

/// Findings on the file as written, `text` (the string lints, the code
/// string checks): their spans and fixes are offsets in it already.
fn as_written(text: &str, found: Vec<Diagnostic>) -> Vec<Raw> {
    let place = |s: xetal_base::Span| {
        let ((line, col), end) = (line_col(text, s.start), line_col(text, s.end));
        (line, col, end.0, end.1)
    };
    let raw = |d: Diagnostic| {
        let lint = xetal_lint::lint(&d.code)?;
        let fix = d.fix().iter();
        let fix: Vec<Edit> = fix
            .map(|f| Edit::new(f.span.start, f.span.end, f.replacement.clone()))
            .collect();
        Some((lint.name, place(d.span?), d.message, fix))
    };
    found.into_iter().filter_map(raw).collect()
}

/// `raw` at the level its rules give it; `None` when allowed.
fn leveled((code, place, message, fix): Raw, name: &str, rules: &[Rule]) -> Option<Finding> {
    let (line, col, end_line, end_col) = place;
    let lint = xetal_lint::lint(code)?;
    let level = resolve(lint, line, rules);
    let file = name.to_string();
    (level != Level::Allow).then_some(Finding {
        lint,
        level,
        file,
        line,
        col,
        end_line,
        end_col,
        message,
        fix,
    })
}

/// The warnings `xetal run` prints (spans in the combined text), less
/// those a `# lint:` comment in the file each is in allows (S12): a
/// library can allow its own. A warning is never more than a warning
/// there, whatever its level (L7); one without a span or a catalog
/// entry is kept.
pub fn honored(sources: &Sources, program: &Program, warnings: Vec<Diagnostic>) -> Vec<Diagnostic> {
    let mut by_file: Vec<(usize, Vec<Rule>)> = Vec::new();
    let mut allowed = |w: &Diagnostic| {
        let (Some(span), Some(lint)) = (w.span, xetal_lint::lint(&w.code)) else {
            return false;
        };
        let at = sources.locate(span.start);
        let known = by_file.iter().position(|(f, _)| *f == at.index);
        let i = known.unwrap_or_else(|| {
            let text = sources.written_text(at.index);
            let found = rules(&directives(text), (text, at.index), program, sources);
            by_file.push((at.index, found));
            by_file.len() - 1
        });
        resolve(lint, at.line, &by_file[i].1) == Level::Allow
    };
    warnings.into_iter().filter(|w| !allowed(w)).collect()
}
