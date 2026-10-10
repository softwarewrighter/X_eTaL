//! `xetal lint`: list the lints, explain one, or lint a program and
//! report its findings (on `err`, as rustc does), failing when any
//! finding is denied.

use std::io::Write;

use xetal_base::Diagnostic;
use xetal_lint::{Level, all};

use crate::driver::{Finding, lint};
use crate::rule::command_line;

/// What was asked on the command line.
#[derive(Debug, Default)]
pub struct Request {
    /// The program: its name (a path, or `-e`) and text.
    pub input: Option<(String, String)>,
    /// `-A`, `-W` and `-D`: lints, groups or `all`.
    pub allow: Vec<String>,
    pub warn: Vec<String>,
    pub deny: Vec<String>,
    /// `--explain NAME`.
    pub explain: Option<String>,
    /// `--list`.
    pub list: bool,
}

/// Run the request; the text for standard output, or the error that
/// stops it (an unknown name, a program that does not load, or denied
/// findings: `lint-denied`).
pub fn command(req: &Request, err: &mut dyn Write) -> Result<String, Diagnostic> {
    if req.list {
        return Ok(list());
    }
    if let Some(name) = &req.explain {
        let found = xetal_lint::lint(name).map(|l| {
            format!(
                "{} ({}, {} by default)\n\n{}",
                l.name,
                l.group.name(),
                l.level.name(),
                l.explain
            )
        });
        return found.ok_or_else(|| {
            Diagnostic::new(
                "unknown-lint",
                format!("no lint named `{name}` (xetal lint --list)"),
            )
        });
    }
    let rules = command_line([&req.allow, &req.warn, &req.deny])?;
    let Some((name, text)) = &req.input else {
        return Err(Diagnostic::new(
            "no-input",
            "give source with -e EXPR or a FILE path",
        ));
    };
    report(&lint(name, text, &rules)?, err)
}

/// Every lint: name, group, default level and summary, one a line.
fn list() -> String {
    let rows = all().iter().map(|l| {
        format!(
            "{:<20} {:<12} {:<6} {}",
            l.name,
            l.group.name(),
            l.level.name(),
            l.summary
        )
    });
    let header = format!(
        "{:<20} {:<12} {:<6} {}",
        "LINT", "GROUP", "LEVEL", "SUMMARY"
    );
    std::iter::once(header)
        .chain(rows)
        .collect::<Vec<_>>()
        .join("\n")
}

/// Write the findings, each lint's first with a note on where to read
/// more, then a count; denied findings are an error.
fn report(findings: &[Finding], err: &mut dyn Write) -> Result<String, Diagnostic> {
    let mut noted = Vec::new();
    for f in findings {
        let _ = writeln!(err, "{f}");
        if !noted.contains(&f.lint.name) {
            noted.push(f.lint.name);
            let (name, group) = (f.lint.name, f.lint.group.name());
            let _ = writeln!(
                err,
                "  note: `{name}` is in group `{group}`; xetal lint --explain {name}"
            );
        }
    }
    let denied = findings.iter().filter(|f| f.level == Level::Deny).count();
    let warned = findings.len() - denied;
    if !findings.is_empty() {
        let _ = writeln!(err, "{warned} warning(s), {denied} error(s)");
    }
    match denied {
        0 => Ok(String::new()),
        n => Err(Diagnostic::new(
            "lint-denied",
            format!("{n} denied lint finding(s)"),
        )),
    }
}
