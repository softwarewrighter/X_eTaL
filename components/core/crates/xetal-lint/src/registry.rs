//! The lint catalog: every lint's name, group, default level and
//! explanation, listed once; the driver, `--list` and `--explain` read
//! it, so a lint cannot exist without its documentation.

/// What a lint is about, as clippy groups its lints; a group name
/// selects all of its lints (`xetal lint -D style`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Group {
    /// Code that is wrong or useless.
    Correctness,
    /// Code that is most likely wrong or misleading.
    Suspicious,
    /// Code that does something simple in a complex way.
    Complexity,
    /// Code that departs from the style guide (lang-choices section 16).
    Style,
    /// Stricter checks, off by default.
    Pedantic,
    /// Forms that still work but are on their way out.
    Deprecated,
}

/// What happens when a lint fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Allow,
    Warn,
    Deny,
}

/// One lint: its stable name (the diagnostic code), group, default
/// level, a one-line summary and the text `--explain` prints.
#[derive(Debug, PartialEq, Eq)]
pub struct Lint {
    pub name: &'static str,
    pub group: Group,
    pub level: Level,
    pub summary: &'static str,
    pub explain: &'static str,
}

impl Group {
    pub const ALL: [Group; 6] = [
        Group::Correctness,
        Group::Suspicious,
        Group::Complexity,
        Group::Style,
        Group::Pedantic,
        Group::Deprecated,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Group::Correctness => "correctness",
            Group::Suspicious => "suspicious",
            Group::Complexity => "complexity",
            Group::Style => "style",
            Group::Pedantic => "pedantic",
            Group::Deprecated => "deprecated",
        }
    }
}

impl Level {
    pub fn name(self) -> &'static str {
        match self {
            Level::Allow => "allow",
            Level::Warn => "warn",
            Level::Deny => "deny",
        }
    }

    /// `allow`, `warn` or `deny`.
    pub fn parse(text: &str) -> Option<Level> {
        [Level::Allow, Level::Warn, Level::Deny]
            .into_iter()
            .find(|l| l.name() == text)
    }
}

/// The lint named `name`.
pub fn lint(name: &str) -> Option<&'static Lint> {
    crate::catalog::LINTS.iter().find(|l| l.name == name)
}

/// Every lint, in catalog order.
pub fn all() -> &'static [Lint] {
    crate::catalog::LINTS
}
