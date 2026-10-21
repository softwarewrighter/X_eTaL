//! A text's findings as lines to show, and the text with their fixes.

use xetal_lint::Level;
use xetal_linter::{Finding, fixed, lint_with};
use xetal_macro::{FsLibraries, Libraries};

/// How a line is shown: a denied finding or a program that does not
/// load, a warning, or a note (the note after a lint's first finding,
/// the count).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Error,
    Warning,
    Note,
}

/// One line of the report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Line {
    pub kind: Kind,
    pub text: String,
}

/// The report: the lines as `xetal lint` prints them (a finding, the
/// note after a lint's first, then the count, which is always there),
/// or the error of a program that does not load; the text with every
/// fix the findings carry made, when one does; how many were made.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Report {
    pub lines: Vec<Line>,
    pub fixed: Option<String>,
    pub fixable: usize,
}

/// The report for `text`, reported as `name`, its libraries found
/// beside it (and the standard ones built in).
pub fn report(name: &str, text: &str) -> Report {
    report_with(name, text, &FsLibraries::from_env())
}

/// [`report`], its libraries found by `libs` (the live demo's come
/// from the browser's storage).
pub fn report_with(name: &str, text: &str, libs: &dyn Libraries) -> Report {
    let findings = match lint_with(name, text, &[], libs) {
        Ok(findings) => findings,
        Err(d) => {
            let line = Line {
                kind: Kind::Error,
                text: d.to_string(),
            };
            return Report {
                lines: vec![line],
                ..Report::default()
            };
        }
    };
    let (after, fixable) = fixed(text, &findings);
    Report {
        lines: lines(&findings),
        fixed: (fixable > 0).then_some(after),
        fixable,
    }
}

/// The findings as `xetal lint` reports them: each one, a note after
/// a lint's first, then the count.
fn lines(findings: &[Finding]) -> Vec<Line> {
    let mut out = Vec::new();
    let mut noted = Vec::new();
    for f in findings {
        let kind = match f.level {
            Level::Deny => Kind::Error,
            _ => Kind::Warning,
        };
        out.push(Line {
            kind,
            text: f.to_string(),
        });
        if !noted.contains(&f.lint.name) {
            noted.push(f.lint.name);
            let (name, group) = (f.lint.name, f.lint.group.name());
            let text =
                format!("  note: `{name}` is in group `{group}`; xetal lint --explain {name}");
            out.push(Line {
                kind: Kind::Note,
                text,
            });
        }
    }
    let denied = findings.iter().filter(|f| f.level == Level::Deny).count();
    let text = format!("{} warning(s), {denied} error(s)", findings.len() - denied);
    out.push(Line {
        kind: Kind::Note,
        text,
    });
    out
}
