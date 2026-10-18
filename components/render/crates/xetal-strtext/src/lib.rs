//! A string literal's text (ST1's escapes `\"` `\\` `\n` `\t`): resolved,
//! with a map back to where each byte was written, and escaped again.
//! The code-string drawing (xetal-render) uses both directions so a
//! drawn inside is placed on the bytes it came from.

/// `written` (the text between a string's quotes) with its escapes
/// resolved, and for each byte of the result (and one past its end) the
/// offset in `written` it came from.
pub fn unescape_mapped(written: &str) -> (String, Vec<usize>) {
    let (mut out, mut map) = (String::new(), Vec::new());
    let mut chars = written.char_indices().peekable();
    while let Some((at, c)) = chars.next() {
        let resolved = match c {
            '\\' => match chars.next().map(|(_, e)| e) {
                Some('n') => '\n',
                Some('t') => '\t',
                Some(other) => other,
                None => '\\',
            },
            c => c,
        };
        map.extend(std::iter::repeat_n(at, resolved.len_utf8()));
        out.push(resolved);
    }
    map.push(written.len());
    (out, map)
}

/// `text` with `\` and `"` escaped, so it can stand between quotes.
pub fn escape(text: &str) -> String {
    text.replace('\\', "\\\\").replace('"', "\\\"")
}

/// The byte offset of the first `"` in `body` that no `\` escapes.
pub fn unescaped_quote(body: &str) -> Option<usize> {
    let mut escaped = false;
    for (i, b) in body.bytes().enumerate() {
        match (escaped, b) {
            (false, b'"') => return Some(i),
            (false, b'\\') => escaped = true,
            _ => escaped = false,
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_resolve_with_their_places() {
        let (text, map) = unescape_mapped(r#"a\"b\\c\n"#);
        assert_eq!(text, "a\"b\\c\n");
        assert_eq!(map, [0, 1, 3, 4, 6, 7, 9]);
        assert_eq!(escape(&text), r#"a\"b\\c"#.to_string() + "\n");
    }

    #[test]
    fn the_closing_quote_is_the_first_unescaped_one() {
        assert_eq!(unescaped_quote(r#"a\"b"c"#), Some(4));
        assert_eq!(unescaped_quote(r#"a\\"b"#), Some(3));
        assert_eq!(unescaped_quote("abc"), None);
    }
}
