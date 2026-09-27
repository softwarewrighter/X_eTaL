//! Render tests (one test binary so helpers are shared).

mod inverse;
mod latex;
mod props;
mod unicode;

/// Combining low line, the underline decoration.
pub const UL: char = '\u{332}';

/// Underline every char of `stem`.
pub fn ul(stem: &str) -> String {
    stem.chars().flat_map(|c| [c, UL]).collect()
}
