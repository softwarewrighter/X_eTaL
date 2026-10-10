//! `# lint:` comments: `# lint: LEVEL NAME...` at the end of a line
//! covers that line, on a line of its own the statement below it;
//! `# lint-file: LEVEL NAME...` covers the whole file. LEVEL is allow,
//! warn or deny; a NAME is a lint, a group or `all` (docs/lint.md).

use xetal_lint::Level;

/// One directive as written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Directive {
    /// Its line (from 1) and the column of its `#` (from 1).
    pub line: usize,
    pub col: usize,
    /// Alone on its line (covers the statement below).
    pub own_line: bool,
    /// `lint-file:` (covers the whole file).
    pub file: bool,
    /// The level word as written, and the names after it.
    pub level: String,
    pub names: Vec<String>,
}

impl Directive {
    /// The level, if the word is allow, warn or deny.
    pub fn level(&self) -> Option<Level> {
        Level::parse(&self.level)
    }
}

/// Every directive in `text`, in order.
pub fn directives(text: &str) -> Vec<Directive> {
    let mut out = Vec::new();
    for (i, line) in text.lines().enumerate() {
        let Some(at) = comment(line) else { continue };
        let Some((file, rest)) = parse(&line[at + 1..]) else {
            continue;
        };
        let mut words = rest
            .split(|c: char| c.is_whitespace() || c == ',')
            .filter(|w| !w.is_empty())
            .map(str::to_string);
        out.push(Directive {
            line: i + 1,
            col: line[..at].chars().count() + 1,
            own_line: line[..at].trim().is_empty(),
            file,
            level: words.next().unwrap_or_default(),
            names: words.collect(),
        });
    }
    out
}

/// The byte index of the `#` starting `line`'s comment, outside any
/// string (strings are single-line, escapes `\"` and `\\`).
fn comment(line: &str) -> Option<usize> {
    let (mut in_string, mut escaped) = (false, false);
    for (i, b) in line.bytes().enumerate() {
        match (in_string, escaped, b) {
            (true, true, _) => escaped = false,
            (true, false, b'\\') => escaped = true,
            (_, false, b'"') => in_string = !in_string,
            (false, _, b'#') => return Some(i),
            _ => {}
        }
    }
    None
}

/// After a single `#`: `lint:` or `lint-file:` and the rest; `##`
/// (documentation) and `###` (headings) are never directives.
fn parse(after_hash: &str) -> Option<(bool, &str)> {
    let text = after_hash.trim_start();
    if after_hash.starts_with('#') {
        return None;
    }
    let file = text.strip_prefix("lint-file:").map(|rest| (true, rest));
    file.or_else(|| text.strip_prefix("lint:").map(|rest| (false, rest)))
}
