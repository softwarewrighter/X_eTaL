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
    let (mut line, mut start, mut seen) = (1, 0, 0);
    let mut out = Vec::new();
    for at in comments(text) {
        for (i, b) in text.bytes().enumerate().take(at).skip(seen) {
            if b == b'\n' {
                (line, start) = (line + 1, i + 1);
            }
        }
        seen = at;
        let end = text[at..].find('\n').map_or(text.len(), |n| at + n);
        out.extend(directive(&text[start..end], at - start, line));
    }
    out
}

/// The directive in the comment at byte `at` of `line` (number `n`).
fn directive(line: &str, at: usize, n: usize) -> Option<Directive> {
    let (file, rest) = parse(&line[at + 1..])?;
    let mut words = rest
        .split(|c: char| c.is_whitespace() || c == ',')
        .filter(|w| !w.is_empty())
        .map(str::to_string);
    Some(Directive {
        line: n,
        col: line[..at].chars().count() + 1,
        own_line: line[..at].trim().is_empty(),
        file,
        level: words.next().unwrap_or_default(),
        names: words.collect(),
    })
}

/// The byte index of each comment's `#`: the first `#` in a gap
/// between the lexer's tokens. Every string form (plain, raw, and any
/// later one) is a token, so a `#` or `"` inside one is never read
/// here, and a comment ends at its line's `Newline` token. Text that
/// does not lex has none; it does not load either.
fn comments(text: &str) -> Vec<usize> {
    let Ok(tokens) = xetal_lex::lex(text) else {
        return Vec::new();
    };
    let ends = std::iter::once(0).chain(tokens.iter().map(|t| t.span.end));
    let starts = tokens.iter().map(|t| t.span.start);
    let gaps = ends.zip(starts.chain(std::iter::once(text.len())));
    gaps.filter_map(|(a, b)| text[a..b].find('#').map(|i| a + i))
        .collect()
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
