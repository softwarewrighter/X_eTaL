//! Glyph tables for the decorated display.

/// U+0332 COMBINING LOW LINE, drawn under the underlined letter.
pub const UNDERLINE: char = '\u{332}';

/// Superscript digits 0-9.
const SUPERSCRIPT_DIGITS: [char; 10] = [
    '\u{2070}', '\u{b9}', '\u{b2}', '\u{b3}', '\u{2074}', '\u{2075}', '\u{2076}', '\u{2077}',
    '\u{2078}', '\u{2079}',
];

/// U+207B SUPERSCRIPT MINUS.
pub const SUPERSCRIPT_MINUS: char = '\u{207b}';

pub fn subscript_digit(d: u8) -> char {
    char::from_u32(0x2080 + u32::from(d)).unwrap_or('?')
}

/// The ASCII digit a subscript digit glyph stands for.
pub fn from_subscript(c: char) -> Option<char> {
    let n = u32::from(c).checked_sub(0x2080)?;
    (n <= 9).then(|| char::from_digit(n, 10)).flatten()
}

/// The superscript form of an exponent's text (`-12`), if every
/// character has one; a decimal point has none.
pub fn superscript_text(text: &str) -> Option<String> {
    text.chars()
        .map(|c| match c {
            '-' => Some(SUPERSCRIPT_MINUS),
            d => d.to_digit(10).map(|d| SUPERSCRIPT_DIGITS[d as usize]),
        })
        .collect()
}

/// The ASCII character a superscript glyph stands for.
pub fn from_superscript(c: char) -> Option<char> {
    if c == SUPERSCRIPT_MINUS {
        return Some('-');
    }
    let d = SUPERSCRIPT_DIGITS.iter().position(|s| *s == c)?;
    char::from_digit(u32::try_from(d).ok()?, 10)
}
