//! Level rules and how they resolve, as rustc and clippy resolve
//! `-A`/`-W`/`-D` and `#[allow]`: a rule names a lint, a group or
//! `all`, sets a level, and covers some lines of the program. For a
//! finding, the covering rule from the narrowest scope wins, then the
//! most specific target (lint over group over `all`), then the last.

use std::ops::RangeInclusive;

use xetal_base::Diagnostic;
use xetal_lint::{Group, Level, Lint};

/// Where a rule was set, widest first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Scope {
    CommandLine,
    File,
    Statement,
}

/// One level setting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    pub scope: Scope,
    pub target: String,
    pub level: Level,
    /// The program lines covered (from 1).
    pub lines: RangeInclusive<usize>,
    /// Later rules win ties (source order; on the command line allow,
    /// then warn, then deny).
    pub order: usize,
}

/// Whether `target` names a lint, a group or `all`.
pub fn known(target: &str) -> bool {
    specificity(target).is_some()
}

/// The command line's `-A`, `-W` and `-D` lists as rules over the whole
/// program; an unknown name is an error, never silently ignored.
pub fn command_line(lists: [&[String]; 3]) -> Result<Vec<Rule>, Diagnostic> {
    let levels = [Level::Allow, Level::Warn, Level::Deny];
    let mut rules = Vec::new();
    for (order, (list, level)) in lists.into_iter().zip(levels).enumerate() {
        for target in list {
            if !known(target) {
                let message = format!("no lint or group named `{target}` (xetal lint --list)");
                return Err(Diagnostic::new("unknown-lint", message));
            }
            let lines = 1..=usize::MAX;
            let (scope, target) = (Scope::CommandLine, target.clone());
            rules.push(Rule {
                scope,
                target,
                level,
                lines,
                order,
            });
        }
    }
    Ok(rules)
}

/// The level of `lint` at `line`: its default unless a rule covers it.
pub fn resolve(lint: &Lint, line: usize, rules: &[Rule]) -> Level {
    rules
        .iter()
        .filter(|r| r.lines.contains(&line))
        .filter_map(|r| applies(&r.target, lint).map(|s| (r.scope, s, r.order, r.level)))
        .max_by_key(|&(scope, spec, order, _)| (scope, spec, order))
        .map_or(lint.level, |(.., level)| level)
}

/// How specific `target` is when it selects `lint`; `None` if it does not.
fn applies(target: &str, lint: &Lint) -> Option<u8> {
    let spec = specificity(target)?;
    let hit = target == "all" || target == lint.group.name() || target == lint.name;
    hit.then_some(spec)
}

fn specificity(target: &str) -> Option<u8> {
    if target == "all" {
        Some(0)
    } else if Group::ALL.iter().any(|g| g.name() == target) {
        Some(1)
    } else {
        xetal_lint::lint(target).map(|_| 2)
    }
}
