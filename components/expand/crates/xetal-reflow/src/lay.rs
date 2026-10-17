//! The layout walk over the tokens of an expansion.

use std::ops::Range;

use xetal_lex::{Token, TokenKind, lex};
use xetal_mapped::Mapped;

use crate::blocks::{Open, closes, long, opens, partners};

/// The columns a laid-out line may reach.
pub const WIDTH: usize = 80;

/// `text` laid out where a line of it is longer than [`WIDTH`]; the
/// spaces and line breaks put in map to `call`, the macro call the text
/// came from. Text that does not lex, or fits, comes back as it is.
pub fn laid_out(text: &Mapped, call: Range<usize>) -> Mapped {
    let src = text.text();
    if !src.lines().any(|l| l.chars().count() > WIDTH) {
        return text.clone();
    }
    let Ok(tokens) = lex(src) else {
        return text.clone();
    };
    let partners = partners(&tokens);
    let mut lay = Lay {
        text,
        tokens: &tokens,
        call,
        out: Mapped::default(),
        col: 0,
        stack: Vec::new(),
        at: 0,
    };
    for (i, partner) in partners.iter().enumerate() {
        lay.token(i, *partner);
    }
    lay.out.push(&text.slice(lay.at..src.len()));
    lay.out
}

/// An open bracket on the stack: what it is, whether its block is laid
/// out (long), and the indent of lines inside it.
pub(crate) struct Frame {
    pub(crate) open: Open,
    pub(crate) laid: bool,
    pub(crate) indent: usize,
}

pub(crate) struct Lay<'a> {
    pub(crate) text: &'a Mapped,
    pub(crate) tokens: &'a [Token],
    pub(crate) call: Range<usize>,
    pub(crate) out: Mapped,
    /// The column the output's current line has reached.
    pub(crate) col: usize,
    pub(crate) stack: Vec<Frame>,
    /// How far into `text` the output has consumed.
    pub(crate) at: usize,
}

impl Lay<'_> {
    fn token(&mut self, i: usize, partner: Option<usize>) {
        let t = &self.tokens[i];
        let gap = &self.text.text()[self.at..t.span.start];
        let width = t.span.end - t.span.start;
        match self.separator(i, gap, width) {
            Some(sep) => self.glue(&sep),
            None => self.copy(self.at..t.span.start),
        }
        // In a laid-out block a `;` or a line break the macro wrote is
        // dropped: the statement's end is the line break put in.
        let ends = matches!(t.kind, TokenKind::Semi | TokenKind::Newline);
        if ends && self.in_laid_block() {
            self.at = t.span.end;
            return;
        }
        self.copy(t.span.start..t.span.end);
        self.frames(i, partner);
    }

    /// The line break and indent to put before token `i` instead of
    /// `gap`, when one belongs there.
    fn separator(&self, i: usize, gap: &str, width: usize) -> Option<String> {
        if gap.contains('#') || gap.contains('\n') {
            return None;
        }
        if matches!(self.tokens[i].kind, TokenKind::Newline) && self.in_laid_block() {
            return None;
        }
        let t = &self.tokens[i];
        let prev = i.checked_sub(1).map(|p| &self.tokens[p].kind);
        let top = self.stack.last()?;
        if top.laid && top.open == Open::Brace {
            let ended = matches!(
                prev,
                Some(TokenKind::Arrow | TokenKind::Semi | TokenKind::Newline)
            );
            if matches!(t.kind, TokenKind::RBrace) {
                return Some(line(top.indent - 2));
            }
            if ended && !matches!(t.kind, TokenKind::Newline) {
                return Some(line(top.indent));
            }
        }
        let fits = self.col + gap.chars().count() + width <= WIDTH;
        if !fits && !gap.is_empty() && top.laid && top.open == Open::Paren {
            return Some(line(top.indent));
        }
        None
    }

    fn frames(&mut self, i: usize, partner: Option<usize>) {
        let t = &self.tokens[i];
        if let Some(open) = opens(&t.kind) {
            let laid = partner.is_some_and(|c| long(self.text.text(), self.tokens, i, c, WIDTH))
                && open != Open::Bracket;
            // A block's lines sit two in from the enclosing block's,
            // whatever parentheses surround it; a continuation line
            // inside parentheses, two in from the innermost frame's.
            let indent = match open {
                Open::Brace => self.block_indent() + 2,
                _ => self.stack.last().map_or(0, |f| f.indent) + 2,
            };
            self.stack.push(Frame { open, laid, indent });
        } else if closes(&t.kind) {
            self.stack.pop();
        }
    }

    /// The indent of the innermost block's lines (0 outside any).
    fn block_indent(&self) -> usize {
        self.stack
            .iter()
            .rev()
            .find(|f| f.open == Open::Brace)
            .map_or(0, |f| f.indent)
    }

    fn in_laid_block(&self) -> bool {
        self.stack
            .last()
            .is_some_and(|f| f.laid && f.open == Open::Brace)
    }
}

fn line(indent: usize) -> String {
    format!("\n{}", " ".repeat(indent))
}
