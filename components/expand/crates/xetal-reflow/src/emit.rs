//! Writing the laid-out text: tokens and kept gaps copied with their
//! map, the line breaks put in glued to the call.

use std::ops::Range;

use crate::lay::Lay;

impl Lay<'_> {
    pub(crate) fn copy(&mut self, range: Range<usize>) {
        let piece = self.text.slice(range.clone());
        self.advance(piece.text());
        self.out.push(&piece);
        self.at = range.end;
    }

    pub(crate) fn glue(&mut self, sep: &str) {
        self.advance(sep);
        self.out.glue(sep, self.call.clone());
        self.at = self.tokens_gap_end();
    }

    pub(crate) fn advance(&mut self, s: &str) {
        match s.rfind('\n') {
            Some(n) => self.col = s[n + 1..].chars().count(),
            None => self.col += s.chars().count(),
        }
    }

    /// Where the gap just replaced ended: the next token's start.
    pub(crate) fn tokens_gap_end(&self) -> usize {
        self.tokens
            .iter()
            .find(|t| t.span.start >= self.at)
            .map_or(self.text.text().len(), |t| t.span.start)
    }
}
