//! What each directive covers: the whole file, its own line, or the
//! top-level statement that starts on the next line of code (all of its
//! lines, found from the statement's span in Core).

use xetal_ir::{Item, Program};
use xetal_sources::Sources;

use crate::directive::Directive;
use crate::rule::{Rule, Scope, known};

/// The rules `directives` set, written in `text`, the program's file
/// number `file`; those with an unknown level or name are left out
/// (see [`unknown`]).
pub fn rules(
    directives: &[Directive],
    (text, file): (&str, usize),
    program: &Program,
    sources: &Sources,
) -> Vec<Rule> {
    let statements = statements(program, sources, file);
    let mut out = Vec::new();
    for (order, d) in directives.iter().enumerate() {
        let Some(level) = d.level() else { continue };
        let (scope, lines) = match (d.file, d.own_line) {
            (true, _) => (Scope::File, 1..=usize::MAX),
            (false, false) => (Scope::Statement, d.line..=d.line),
            (false, true) => (Scope::Statement, below(d.line, text, &statements)),
        };
        for target in d.names.iter().filter(|n| known(n)) {
            let target = target.clone();
            out.push(Rule {
                scope,
                target,
                level,
                lines: lines.clone(),
                order,
            });
        }
    }
    out
}

/// The directives' mistakes: `(line, col, message)` for a level that is
/// not allow, warn or deny, a directive naming nothing, or an unknown name.
pub fn unknown(directives: &[Directive]) -> Vec<(usize, usize, String)> {
    let mut out = Vec::new();
    for d in directives {
        if d.level().is_none() {
            let message = format!(
                "`{}` is not a lint level: write allow, warn or deny",
                d.level
            );
            out.push((d.line, d.col, message));
        } else if d.names.is_empty() {
            out.push((
                d.line,
                d.col,
                "this directive names no lint or group".to_string(),
            ));
        }
        for name in d.names.iter().filter(|n| !known(n)) {
            let message = format!("no lint or group named `{name}` (xetal lint --list)");
            out.push((d.line, d.col, message));
        }
    }
    out
}

/// The lines of the statement starting on the first line of code after
/// `line` (only that line if no statement starts there).
fn below(
    line: usize,
    text: &str,
    statements: &[(usize, usize)],
) -> std::ops::RangeInclusive<usize> {
    let code = text
        .lines()
        .enumerate()
        .skip(line)
        .find(|(_, l)| !l.trim().is_empty() && !l.trim_start().starts_with('#'))
        .map_or(line + 1, |(i, _)| i + 1);
    let end = statements
        .iter()
        .filter(|&&(start, _)| start == code)
        .map(|&(_, end)| end)
        .max();
    code..=end.unwrap_or(code)
}

/// The first and last lines of each top-level statement written in the
/// program's file number `file` (0: the program; then its libraries).
fn statements(program: &Program, sources: &Sources, file: usize) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    for item in &program.items {
        let (Item::Def { value, .. }
        | Item::Let { value, .. }
        | Item::Set { value, .. }
        | Item::Eval(value)) = item;
        let span = value.span;
        let (start, end) = (
            sources.locate(span.start),
            sources.locate(span.end.max(span.start + 1) - 1),
        );
        if start.index == file && end.index == file {
            out.push((start.line, end.line));
        }
    }
    out
}
