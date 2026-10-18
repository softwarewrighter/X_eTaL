//! Comments by count of `#` (S9) as the display draws them (I4): a
//! line's `##` doc comment and `###` section heading get markers of
//! their own, one glyph per `#` so columns are kept; any other comment
//! (an end-of-line remark, `##x`, `####`) is a lone lamp.

/// The kind of a comment, for its marker and its style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommentKind {
    /// `#`, and any comment that is not one of the others.
    Remark,
    /// `##` alone or before a space, first on its line.
    Doc,
    /// `### ` first on its line.
    Heading,
}

const LAMP: &str = "\u{235d}";
const DOC: &str = "\u{235d}\u{2502}";
const HEADING: &str = "\u{235d}\u{2501}\u{2501}";

/// The kind of comment `text` (from its `#` to the end of its line),
/// given whether only whitespace comes before it on its line.
pub fn comment_kind(text: &str, line_start: bool) -> CommentKind {
    if !line_start {
        return CommentKind::Remark;
    }
    let hashes = text.bytes().take_while(|&b| b == b'#').count();
    match (hashes, text.as_bytes().get(hashes)) {
        (2, None | Some(b' ' | b'\r' | b'\n')) => CommentKind::Doc,
        (3, Some(b' ')) => CommentKind::Heading,
        _ => CommentKind::Remark,
    }
}

/// The glyphs drawn for a comment's marker and the bytes of raw text
/// (`#`s) they stand for.
pub fn comment_marker(kind: CommentKind) -> (&'static str, usize) {
    match kind {
        CommentKind::Remark => (LAMP, 1),
        CommentKind::Doc => (DOC, 2),
        CommentKind::Heading => (HEADING, 3),
    }
}

/// The `#`s a decorated marker at the start of `text` (just after its
/// lamp) stands for beyond the lamp's own, and the bytes it takes: a
/// light vertical before a space or the end is a doc comment, two heavy
/// horizontals before a space a heading; anything else is the remark's
/// own text.
pub(crate) fn marker_after_lamp(text: &str) -> Option<(&'static str, usize)> {
    let ends = |rest: &str| rest.is_empty() || rest.starts_with([' ', '\r', '\n']);
    if let Some(rest) = text.strip_prefix('\u{2502}') {
        return ends(rest).then_some(("#", '\u{2502}'.len_utf8()));
    }
    let rest = text.strip_prefix("\u{2501}\u{2501}")?;
    rest.starts_with(' ')
        .then_some(("##", 2 * '\u{2501}'.len_utf8()))
}
