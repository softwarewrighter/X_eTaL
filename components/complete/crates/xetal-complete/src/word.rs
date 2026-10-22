//! The word at the cursor, which situation it is in, and the
//! completion for it.

use crate::directive::directive;
use crate::library::{exports, libraries};
use crate::names::{arguments, names};

/// One candidate: what the list shows, what is written in place of the
/// word, and what it is (a type, a doc line, a kind).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidate {
    pub label: String,
    pub insert: String,
    pub detail: String,
}

/// The candidates for `text[start..end]`, the word being typed (the
/// cursor is at `end`); accepting one writes its `insert` over it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Completion {
    pub start: usize,
    pub end: usize,
    pub candidates: Vec<Candidate>,
}

/// What the host knows: the saved files (`Name.xtl`, `Name.xtlm`), and
/// a library's text by name (the standard ones and the saved ones).
pub struct Context<'a> {
    pub files: &'a [String],
    pub library: &'a dyn Fn(&str) -> Option<String>,
}

/// Characters a word is made of: a name with its namespace and marks,
/// or a system function.
fn is_word(c: char) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, '_' | ':' | '!' | '<' | '[' | ']')
}

/// The completion at `cursor` (a byte offset) in `text`, if there is
/// one: none when nothing is being typed, or nothing matches.
pub fn complete(text: &str, cursor: usize, ctx: &Context) -> Option<Completion> {
    let before = text.get(..cursor)?;
    let line_start = before.rfind('\n').map_or(0, |i| i + 1);
    let line = &before[line_start..];
    if let Some((start, candidates)) = directive(line) {
        return finish(line_start + start, cursor, candidates);
    }
    if let Some((start, prefix)) = in_use_string(line) {
        return finish(line_start + start, cursor, libraries(prefix, ctx));
    }
    let start = line
        .rfind(|c: char| !is_word(c))
        .map_or(0, |i| i + c_len(line, i));
    let word = &line[start..];
    let start = line_start + start;
    if word.is_empty() {
        return None;
    }
    let candidates = match word.split_once(':') {
        Some((alias, rest)) if !alias.is_empty() && alias != "u" => exports(text, alias, rest, ctx),
        _ if word == "_" => arguments(before),
        _ => names(text, word),
    };
    finish(start, cursor, candidates)
}

/// The byte length of the character at `i`.
fn c_len(s: &str, i: usize) -> usize {
    s[i..].chars().next().map_or(1, char::len_utf8)
}

/// The start of a library name being typed in the string after
/// `u_se<`, and that prefix: `"x:" u_se< "St` gives `St`.
fn in_use_string(line: &str) -> Option<(usize, &str)> {
    let quote = line.rfind('"')?;
    let (head, prefix) = (line[..quote].trim_end(), &line[quote + 1..]);
    if !head.ends_with("u_se<") || prefix.contains('"') {
        return None;
    }
    Some((quote + 1, prefix))
}

fn finish(start: usize, end: usize, mut candidates: Vec<Candidate>) -> Option<Completion> {
    candidates.sort_by(|a, b| a.label.cmp(&b.label));
    candidates.dedup_by(|a, b| a.label == b.label);
    (!candidates.is_empty()).then_some(Completion {
        start,
        end,
        candidates,
    })
}
