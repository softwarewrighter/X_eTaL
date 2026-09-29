//! Glyphs for the lambda arguments `_l` and `_r` (I3).

/// Lambda arguments: `_l` and `_r` as subscript l and r (U+2097,
/// U+1D63).
pub const LAMBDA_ARGS: [(char, char); 2] = [('l', '\u{2097}'), ('r', '\u{1d63}')];

/// The glyph for lambda argument `side` (`l` or `r`).
pub fn lambda_glyph(side: char) -> Option<char> {
    LAMBDA_ARGS
        .iter()
        .find(|(s, _)| *s == side)
        .map(|(_, g)| *g)
}

/// The side a lambda-argument glyph stands for.
pub fn from_lambda_glyph(c: char) -> Option<char> {
    LAMBDA_ARGS.iter().find(|(_, g)| *g == c).map(|(s, _)| *s)
}
