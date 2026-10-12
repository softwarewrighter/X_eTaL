//! `xetal lint`: list the lints, explain one, or lint a program and
//! report its findings (on `err`, as rustc does; one JSON object a line
//! on `out` with `--format json`), failing when any finding is denied.

use std::io::Write;

use xetal_base::Diagnostic;
use xetal_lint::{Level, all};

use crate::driver::{Finding, lint};
use crate::fix::fix_file;
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
    /// `--fix`: make the findings' sure edits in the file.
    pub fix: bool,
    /// `--format json`: the findings as JSON lines on standard output.
    pub json: bool,
}

/// Run the request; the text for standard output, or the error that
/// stops it (an unknown name, a program that does not load, `--fix`
/// without a file, or denied findings: `lint-denied`).
pub fn command(
    req: &Request,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> Result<String, Diagnostic> {
    if req.list {
        return Ok(list());
    }
    if let Some(name) = &req.explain {
        return explain(name);
    }
    let rules = command_line([&req.allow, &req.warn, &req.deny])?;
    let Some((name, text)) = &req.input else {
        return Err(Diagnostic::new(
            "no-input",
            "give source with -e EXPR or a FILE path",
        ));
    };
    let findings = match req.fix {
        true => fix_file(name, text, &rules, err)?,
        false => lint(name, text, &rules)?,
    };
    match req.json {
        true => json(&findings, out),
        false => report(&findings, err),
    }
}

/// What lint `name` checks, why, and an example.
fn explain(name: &str) -> Result<String, Diagnostic> {
    let found = xetal_lint::lint(name).map(|l| {
        let (group, level) = (l.group.name(), l.level.name());
        format!("{} ({group}, {level} by default)\n\n{}", l.name, l.explain)
    });
    found.ok_or_else(|| {
        Diagnostic::new(
            "unknown-lint",
            format!("no lint named `{name}` (xetal lint --list)"),
        )
    })
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
    if !findings.is_empty() {
        let warned = findings.len() - denied;
        let _ = writeln!(err, "{warned} warning(s), {denied} error(s)");
    }
    refused(denied)
}

/// Each finding as one JSON object on its own line (for editors: the
/// Emacs mode, the live demo); denied findings are still an error.
fn json(findings: &[Finding], out: &mut dyn Write) -> Result<String, Diagnostic> {
    for f in findings {
        let fix = f.fix.as_ref().map_or("null".to_string(), |e| {
            let r = json_string(&e.replacement);
            format!(
                r#"{{"start":{},"end":{},"replacement":{r}}}"#,
                e.start, e.end
            )
        });
        let _ = writeln!(
            out,
            r#"{{"lint":{},"group":{},"level":{},"file":{},"line":{},"col":{},"message":{},"fix":{fix}}}"#,
            json_string(f.lint.name),
            json_string(f.lint.group.name()),
            json_string(f.level.name()),
            json_string(&f.file),
            f.line,
            f.col,
            json_string(&f.message),
        );
    }
    refused(findings.iter().filter(|f| f.level == Level::Deny).count())
}

/// `text` as a JSON string, quoted and escaped.
pub fn json_string(text: &str) -> String {
    let mut s = String::from('"');
    for c in text.chars() {
        match c {
            '"' => s.push_str("\\\""),
            '\\' => s.push_str("\\\\"),
            '\n' => s.push_str("\\n"),
            '\t' => s.push_str("\\t"),
            '\r' => s.push_str("\\r"),
            c if (c as u32) < 0x20 => s.push_str(&format!("\\u{:04x}", c as u32)),
            c => s.push(c),
        }
    }
    s.push('"');
    s
}

/// The command's outcome when `denied` findings were denied.
fn refused(denied: usize) -> Result<String, Diagnostic> {
    match denied {
        0 => Ok(String::new()),
        n => Err(Diagnostic::new(
            "lint-denied",
            format!("{n} denied lint finding(s)"),
        )),
    }
}
