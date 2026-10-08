//! Which `##` block documents what: the file's first block (its header,
//! whatever follows it), the block directly above a definition (unless
//! that is the header), and the `###` section a line falls under.

use crate::Doc;

/// The text of a doc comment line without its `##` (and one space), or
/// None for any other line, a `###` section title included.
fn doc_line(line: &str) -> Option<&str> {
    let rest = line.trim_start().strip_prefix("##")?;
    match rest.chars().next() {
        None => Some(""),
        Some(' ') => Some(&rest[1..]),
        Some(_) => None,
    }
}

/// The doc of the definition starting on `line` (from 1): the block of
/// `##` lines directly above it, unless that block is the file's header.
pub fn doc_above(text: &str, line: usize) -> Option<Doc> {
    let lines: Vec<&str> = text.lines().collect();
    let end = line.checked_sub(1)?.min(lines.len());
    let start = (0..end)
        .rev()
        .take_while(|&i| doc_line(lines[i]).is_some())
        .last()?;
    if header(&lines).is_some_and(|(top, _)| top == start) {
        return None;
    }
    Some(block(&lines[start..end]))
}

/// The file's own doc: its header, the `##` block at the top of the file
/// (after any `#!` line and blank lines), whatever follows it.
pub fn file_doc(text: &str) -> Option<Doc> {
    let lines: Vec<&str> = text.lines().collect();
    let (start, len) = header(&lines)?;
    Some(block(&lines[start..start + len]))
}

/// The first lines (from 1) of the `##` blocks that document nothing:
/// neither the header nor directly above one of `items` (the lines,
/// from 1, where definitions and macro calls start).
pub fn stray_blocks(text: &str, items: &[usize]) -> Vec<usize> {
    let lines: Vec<&str> = text.lines().collect();
    let top = header(&lines).map(|(start, _)| start);
    let mut strays = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let len = lines[i..]
            .iter()
            .take_while(|l| doc_line(l).is_some())
            .count();
        if len > 0 && Some(i) != top && !items.contains(&(i + len + 1)) {
            strays.push(i + 1);
        }
        i += len.max(1);
    }
    strays
}

/// Where the header is: its first line and its length.
fn header(lines: &[&str]) -> Option<(usize, usize)> {
    let start = lines
        .iter()
        .position(|l| !(l.trim().is_empty() || l.starts_with("#!")))?;
    doc_line(lines[start])?;
    let len = lines[start..]
        .iter()
        .take_while(|l| doc_line(l).is_some())
        .count();
    Some((start, len))
}

/// The doc of a run of `##` lines.
fn block(lines: &[&str]) -> Doc {
    let text: Vec<&str> = lines.iter().filter_map(|l| doc_line(l)).collect();
    Doc::from_lines(&text)
}

/// The title of the `### Title` section line `line` (from 1) falls
/// under, if any.
pub fn section_at(text: &str, line: usize) -> Option<String> {
    text.lines()
        .take(line.saturating_sub(1))
        .filter_map(|l| l.trim_start().strip_prefix("### "))
        .last()
        .map(|t| t.trim().to_string())
}
