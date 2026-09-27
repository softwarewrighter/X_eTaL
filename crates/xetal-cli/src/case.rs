//! Spec-case files (`spec/**/*.case`): parsing, rendering (for
//! `XETAL_BLESS=1`) and checking a case against the CLI stages.
//!
//! A case file is a sequence of `== NAME` headers, each followed by the
//! section body. Only `SOURCE` is required. Lines before the first
//! header may be blank or `#` comments.

use std::fmt;

/// The sections a case file may contain, in canonical order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Section {
    Source,
    Tokens,
    Surface,
    Canonical,
    Core,
    Type,
    Result,
    Error,
    Status,
}

impl Section {
    pub const ALL: [Section; 9] = [
        Section::Source,
        Section::Tokens,
        Section::Surface,
        Section::Canonical,
        Section::Core,
        Section::Type,
        Section::Result,
        Section::Error,
        Section::Status,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Section::Source => "SOURCE",
            Section::Tokens => "TOKENS",
            Section::Surface => "SURFACE",
            Section::Canonical => "CANONICAL",
            Section::Core => "CORE",
            Section::Type => "TYPE",
            Section::Result => "RESULT",
            Section::Error => "ERROR",
            Section::Status => "STATUS",
        }
    }

    pub fn from_name(name: &str) -> Option<Section> {
        Section::ALL.into_iter().find(|s| s.name() == name)
    }

    /// The `xetal` subcommand whose output this section pins, if any.
    pub fn stage(self) -> Option<&'static str> {
        match self {
            Section::Tokens => Some("lex"),
            Section::Surface => Some("parse"),
            Section::Canonical => Some("fmt"),
            Section::Core => Some("core"),
            Section::Type => Some("type"),
            Section::Result | Section::Error => Some("eval"),
            Section::Source | Section::Status => None,
        }
    }
}

/// Whether a case is expected to pass (`active`) or still fail (`pending`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Active,
    Pending,
}

/// A malformed case file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaseError {
    /// 1-based line number, or 0 for whole-file problems.
    pub line: usize,
    pub message: String,
}

impl fmt::Display for CaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.line == 0 {
            write!(f, "{}", self.message)
        } else {
            write!(f, "line {}: {}", self.line, self.message)
        }
    }
}

impl std::error::Error for CaseError {}

fn case_err(line: usize, message: impl Into<String>) -> CaseError {
    CaseError {
        line,
        message: message.into(),
    }
}

/// A parsed case file. Sections keep their file order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaseFile {
    pub preamble: Vec<String>,
    pub sections: Vec<(Section, String)>,
}

impl CaseFile {
    pub fn parse(text: &str) -> Result<CaseFile, CaseError> {
        let mut preamble = Vec::new();
        let mut sections: Vec<(Section, String)> = Vec::new();
        let mut body: Vec<&str> = Vec::new();
        let mut current: Option<Section> = None;

        for (idx, raw) in text.lines().enumerate() {
            let line_no = idx + 1;
            let line = raw.strip_suffix('\r').unwrap_or(raw);
            if let Some(rest) = line.strip_prefix("==") {
                let name = rest.trim();
                let section = Section::from_name(name)
                    .ok_or_else(|| case_err(line_no, format!("unknown section `{name}`")))?;
                if let Some(prev) = current.take() {
                    sections.push((prev, join_body(&body)));
                }
                if sections.iter().any(|(s, _)| *s == section) {
                    return Err(case_err(
                        line_no,
                        format!("duplicate section `{}`", section.name()),
                    ));
                }
                current = Some(section);
                body.clear();
            } else if current.is_some() {
                body.push(line);
            } else if line.trim().is_empty() || line.starts_with('#') {
                preamble.push(line.to_string());
            } else {
                return Err(case_err(
                    line_no,
                    "text before the first `== SECTION` header must be a `#` comment",
                ));
            }
        }
        if let Some(prev) = current {
            sections.push((prev, join_body(&body)));
        }
        while preamble.last().is_some_and(|l| l.trim().is_empty()) {
            preamble.pop();
        }

        let case = CaseFile { preamble, sections };
        case.validate()?;
        Ok(case)
    }

    fn validate(&self) -> Result<(), CaseError> {
        match self.get(Section::Source) {
            None => return Err(case_err(0, "missing `== SOURCE` section")),
            Some(src) if src.trim().is_empty() => {
                return Err(case_err(0, "empty `== SOURCE` section"));
            }
            Some(_) => {}
        }
        if self.get(Section::Result).is_some() && self.get(Section::Error).is_some() {
            return Err(case_err(0, "`RESULT` and `ERROR` are mutually exclusive"));
        }
        self.status_checked().map(|_| ())
    }

    fn status_checked(&self) -> Result<Status, CaseError> {
        let Some(text) = self.get(Section::Status) else {
            return Ok(Status::Active);
        };
        match text.split_whitespace().next() {
            Some("active") => Ok(Status::Active),
            Some("pending") => Ok(Status::Pending),
            other => Err(case_err(
                0,
                format!(
                    "STATUS must start with `active` or `pending`, found `{}`",
                    other.unwrap_or("")
                ),
            )),
        }
    }

    pub fn get(&self, section: Section) -> Option<&str> {
        self.sections
            .iter()
            .find(|(s, _)| *s == section)
            .map(|(_, body)| body.as_str())
    }

    pub fn source(&self) -> &str {
        self.get(Section::Source).unwrap_or("")
    }

    pub fn status(&self) -> Status {
        self.status_checked().unwrap_or(Status::Active)
    }

    /// Replace a section body, or insert the section in canonical order.
    pub fn set(&mut self, section: Section, body: String) {
        if let Some(slot) = self.sections.iter_mut().find(|(s, _)| *s == section) {
            slot.1 = body;
            return;
        }
        let rank = |s: Section| Section::ALL.iter().position(|x| *x == s);
        let at = self
            .sections
            .iter()
            .position(|(s, _)| rank(*s) > rank(section))
            .unwrap_or(self.sections.len());
        self.sections.insert(at, (section, body));
    }

    pub fn render(&self) -> String {
        let mut out = String::new();
        for line in &self.preamble {
            out.push_str(line);
            out.push('\n');
        }
        for (section, body) in &self.sections {
            out.push_str("== ");
            out.push_str(section.name());
            out.push('\n');
            if !body.is_empty() {
                out.push_str(body);
                out.push('\n');
            }
        }
        out
    }
}

fn join_body(lines: &[&str]) -> String {
    let end = lines
        .iter()
        .rposition(|l| !l.trim().is_empty())
        .map_or(0, |i| i + 1);
    lines[..end].join("\n")
}

/// Output of running one CLI stage on a case source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StageOutput {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

/// Result of checking one section.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    Pass,
    /// The stage is not implemented yet.
    Unsupported,
    /// The stage ran but disagreed; `actual` is what it produced and
    /// `blessable` is true when `actual` may replace the expectation.
    Mismatch {
        actual: String,
        blessable: bool,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Check {
    pub section: Section,
    pub expected: String,
    pub outcome: Outcome,
}

/// Run every stage-backed section of `case` through `run(stage, source)`.
pub fn check_case(case: &CaseFile, mut run: impl FnMut(&str, &str) -> StageOutput) -> Vec<Check> {
    let mut checks = Vec::new();
    for (section, expected) in &case.sections {
        let Some(stage) = section.stage() else {
            continue;
        };
        let out = run(stage, case.source());
        let want_error = *section == Section::Error;
        let text = if want_error { &out.stderr } else { &out.stdout };
        let actual = text.trim_end().to_string();
        let outcome = if out.stderr.starts_with("error[unsupported]") {
            Outcome::Unsupported
        } else if out.success != want_error && actual == *expected {
            Outcome::Pass
        } else {
            Outcome::Mismatch {
                blessable: out.success != want_error,
                actual: if out.success != want_error {
                    actual
                } else {
                    format!("{}{}", out.stdout.trim_end(), out.stderr.trim_end())
                },
            }
        };
        checks.push(Check {
            section: *section,
            expected: expected.clone(),
            outcome,
        });
    }
    checks
}

/// Decide whether a case's checks are acceptable for its status.
pub fn verdict(status: Status, checks: &[Check]) -> Result<(), String> {
    if checks.is_empty() {
        return Err("case checks nothing: add at least one expectation section".into());
    }
    let all_pass = checks.iter().all(|c| c.outcome == Outcome::Pass);
    match status {
        Status::Active if all_pass => Ok(()),
        Status::Active => {
            let failed: Vec<String> = checks
                .iter()
                .filter(|c| c.outcome != Outcome::Pass)
                .map(describe)
                .collect();
            Err(failed.join("\n"))
        }
        Status::Pending if all_pass => Err(
            "pending case unexpectedly passes: flip STATUS to active in a deliberate commit".into(),
        ),
        Status::Pending => Ok(()),
    }
}

fn describe(check: &Check) -> String {
    let name = check.section.name();
    match &check.outcome {
        Outcome::Pass => format!("{name}: ok"),
        Outcome::Unsupported => format!(
            "{name}: unsupported (stage `{}` not implemented)",
            check.section.stage().unwrap_or("?")
        ),
        Outcome::Mismatch { actual, .. } => format!(
            "{name}: mismatch\n--- expected\n{}\n--- actual\n{actual}",
            check.expected
        ),
    }
}

/// Rewrite mismatching expectations of an active case with actual
/// output. Returns true if anything changed. Pending cases are never
/// blessed: their expectations are hand-written targets.
pub fn bless(case: &mut CaseFile, checks: &[Check]) -> bool {
    if case.status() == Status::Pending {
        return false;
    }
    let mut changed = false;
    for check in checks {
        if let Outcome::Mismatch {
            actual,
            blessable: true,
        } = &check.outcome
        {
            case.set(check.section, actual.clone());
            changed = true;
        }
    }
    changed
}

#[cfg(test)]
mod tests;
