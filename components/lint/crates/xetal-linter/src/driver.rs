//! Linting one program: load it, run every lint pass, keep what is in
//! the program's own file, and give each finding the level its rules
//! resolve to (allowed findings are dropped).

use std::fmt;

use xetal_base::Diagnostic;
use xetal_lint::{Level, Lint};
use xetal_program::Loaded;

use crate::directive::directives;
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
    pub message: String,
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
/// program that does not load is an error.
pub fn lint(name: &str, text: &str, cli: &[Rule]) -> Result<Vec<Finding>, Diagnostic> {
    let loaded = match xetal_program::is_library_named(name, text) {
        true => xetal_program::load_library(name, text)?,
        false => xetal_program::load(name, text)?,
    };
    let written = directives(text);
    let mut all = cli.to_vec();
    all.extend(rules(&written, text, &loaded.program, &loaded.sources));
    let mut raw: Vec<Raw> = unknown(&written)
        .into_iter()
        .map(|(line, col, message)| ("unknown-lint", line, col, message))
        .collect();
    raw.extend(placed(&loaded));
    let mut out: Vec<Finding> = raw
        .into_iter()
        .filter_map(|r| leveled(r, name, &all))
        .collect();
    out.sort_by(|a, b| (a.line, a.col, a.lint.name).cmp(&(b.line, b.col, b.lint.name)));
    out.dedup();
    Ok(out)
}

/// A finding before its level: lint name, line, column and message.
type Raw = (&'static str, usize, usize, String);

/// What the lint passes find in the program's own file, placed there.
fn placed(loaded: &Loaded) -> Vec<Raw> {
    let mut out = Vec::new();
    for d in xetal_lint::lints(&loaded.program) {
        let at = loaded.sources.locate(d.span.map_or(0, |s| s.start));
        if at.index == 0 {
            out.extend(xetal_lint::lint(&d.code).map(|l| (l.name, at.line, at.col, d.message)));
        }
    }
    out
}

/// `raw` at the level its rules give it; `None` when allowed.
fn leveled((code, line, col, message): Raw, name: &str, rules: &[Rule]) -> Option<Finding> {
    let lint = xetal_lint::lint(code)?;
    let level = resolve(lint, line, rules);
    let file = name.to_string();
    (level != Level::Allow).then_some(Finding {
        lint,
        level,
        file,
        line,
        col,
        message,
    })
}
