//! `"w1 w2" e_ach< "template"`: one copy of the template per word of
//! the left, `$w` in it replaced by the word; the copies are statements,
//! one per line (MC16).

use std::ops::Range;

use xetal_base::Diagnostic;

use crate::calls::Call;
use crate::system::Part;

/// The word placeholder in an `e_ach<` template; never valid code.
pub const WORD: &str = "$w";

pub(crate) fn each(call: &Call, left: &str, right: &str) -> Result<Vec<Part>, Diagnostic> {
    let fail =
        |code: &str, message: &str, span| Err(Diagnostic::new(code, message).with_span(span));
    if !call.statement {
        let message = "e_ach< writes statements: it stands as a statement of its own";
        return fail("misplaced-macro", message, call.token);
    }
    let words = words(left);
    if words.is_empty() {
        return fail(
            "bad-macro-argument",
            "e_ach< needs words on its left",
            call.left,
        );
    }
    let holes: Vec<usize> = right.match_indices(WORD).map(|(i, _)| i).collect();
    if holes.is_empty() {
        let message = "an e_ach< template names the word as $w: \"a b\" e_ach< \"u:$w := 1\"";
        return fail("bad-macro-argument", message, call.right);
    }
    let mut parts = Vec::new();
    for (k, word) in words.into_iter().enumerate() {
        if k > 0 {
            parts.push(Part::Glue("\n"));
        }
        parts.extend(copy(&holes, right.len(), word));
    }
    Ok(parts)
}

/// One copy of the template (length `len`, `$w` at `holes`).
fn copy(holes: &[usize], len: usize, word: Range<usize>) -> Vec<Part> {
    let mut parts = Vec::new();
    let mut at = 0;
    for &hole in holes {
        parts.push(Part::Right(at..hole));
        parts.push(Part::Left(word.clone()));
        at = hole + WORD.len();
    }
    parts.push(Part::Right(at..len));
    parts
}

/// The words of `text`: runs of characters other than white space.
fn words(text: &str) -> Vec<Range<usize>> {
    let mut out = Vec::new();
    let mut start = None;
    for (i, c) in text.char_indices().chain([(text.len(), ' ')]) {
        match (c.is_whitespace(), start) {
            (true, Some(s)) => {
                out.push(s..i);
                start = None;
            }
            (false, None) => start = Some(i),
            _ => {}
        }
    }
    out
}
