//! Glyph tables (docs/design.md section 8).

/// U+0332 COMBINING LOW LINE, drawn under each stem character.
pub const UNDERLINE: char = '\u{332}';

/// Modifier letters for the superscript alphabet, `a..z`. `q` has no
/// Unicode superscript form, so it is `None`; a word using it (or any
/// uppercase letter) is shown raw.
const SUPERSCRIPT: [Option<char>; 26] = [
    Some('\u{1d43}'), // a
    Some('\u{1d47}'), // b
    Some('\u{1d9c}'), // c
    Some('\u{1d48}'), // d
    Some('\u{1d49}'), // e
    Some('\u{1da0}'), // f
    Some('\u{1d4d}'), // g
    Some('\u{2b0}'),  // h
    Some('\u{2071}'), // i
    Some('\u{2b2}'),  // j
    Some('\u{1d4f}'), // k
    Some('\u{2e1}'),  // l
    Some('\u{1d50}'), // m
    Some('\u{207f}'), // n
    Some('\u{1d52}'), // o
    Some('\u{1d56}'), // p
    None,             // q
    Some('\u{2b3}'),  // r
    Some('\u{2e2}'),  // s
    Some('\u{1d57}'), // t
    Some('\u{1d58}'), // u
    Some('\u{1d5b}'), // v
    Some('\u{2b7}'),  // w
    Some('\u{2e3}'),  // x
    Some('\u{2b8}'),  // y
    Some('\u{1dbb}'), // z
];

/// The superscript form of a whole derivation word, if every letter has one.
pub fn superscript_word(word: &str) -> Option<String> {
    word.bytes()
        .map(|b| match b {
            b'a'..=b'z' => SUPERSCRIPT[usize::from(b - b'a')],
            _ => None,
        })
        .collect()
}

/// The ASCII letter a superscript glyph stands for.
pub fn from_superscript(c: char) -> Option<char> {
    let idx = SUPERSCRIPT.iter().position(|s| *s == Some(c))?;
    u8::try_from(idx).ok().map(|i| char::from(b'a' + i))
}

pub fn subscript_digit(d: u8) -> char {
    char::from_u32(0x2080 + u32::from(d)).unwrap_or('?')
}

/// The ASCII digit a subscript digit glyph stands for.
pub fn from_subscript(c: char) -> Option<char> {
    let n = u32::from(c).checked_sub(0x2080)?;
    (n <= 9).then(|| char::from_digit(n, 10)).flatten()
}
