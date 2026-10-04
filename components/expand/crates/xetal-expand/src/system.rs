//! The system macros' expansions, as parts: text the macro writes and
//! ranges of its two arguments (MC14-MC16).

use std::ops::Range;

use xetal_base::Diagnostic;
use xetal_lex::{TokenKind, lex};

use crate::calls::Call;
use crate::each::each;

/// The system macros, as an error message lists them.
pub const SYSTEM: &str = "u_se<, i_f<, u_nless< and e_ach<";

/// A piece of an expansion.
#[derive(Debug, Clone)]
pub(crate) enum Part {
    /// Text the macro writes.
    Glue(&'static str),
    /// Bytes of the left or right argument (the string's inside).
    Left(Range<usize>),
    Right(Range<usize>),
}

pub(crate) fn known(name: &str) -> bool {
    matches!(name, "i_f<" | "u_nless<" | "e_ach<")
}

/// The expansion of `call`, whose arguments are `left` and `right`.
pub(crate) fn parts(call: &Call, left: &str, right: &str) -> Result<Vec<Part>, Diagnostic> {
    let inner = match call.name.as_str() {
        "u_nless<" => unless(left, right),
        "i_f<" => when(call, left, right)?,
        _ => return each(call, left, right),
    };
    Ok(match call.statement {
        true => inner,
        false => [vec![Part::Glue("(")], inner, vec![Part::Glue(")")]].concat(),
    })
}

/// `"c" u_nless< "b"`: run b (statements) unless c holds; the value is `@`.
fn unless(left: &str, right: &str) -> Vec<Part> {
    vec![
        Part::Glue("{ @ -> ("),
        Part::Left(trim(left, 0..left.len())),
        Part::Glue(") ? @; "),
        Part::Right(trim(right, 0..right.len())),
        Part::Glue("; @ } @"),
    ]
}

/// `"c" i_f< "a; b"`: the value of a when c holds, else of b; only the
/// one chosen is evaluated.
fn when(call: &Call, left: &str, right: &str) -> Result<Vec<Part>, Diagnostic> {
    let branches = branches(right).filter(|b| b.len() == 2 && b.iter().all(|r| !r.is_empty()));
    let Some([then, otherwise]) = branches.as_deref() else {
        let message =
            "i_f< takes two expressions on its right, separated by ;: \"c\" i_f< \"a; b\"";
        return Err(Diagnostic::new("bad-macro-argument", message).with_span(call.right));
    };
    Ok(vec![
        Part::Glue("{ @ -> ("),
        Part::Left(trim(left, 0..left.len())),
        Part::Glue(") ? "),
        Part::Right(then.clone()),
        Part::Glue("; "),
        Part::Right(otherwise.clone()),
        Part::Glue(" } @"),
    ])
}

/// `text` split at `;` and newlines outside brackets, each part trimmed
/// (none when it does not lex).
fn branches(text: &str) -> Option<Vec<Range<usize>>> {
    let (mut out, mut start, mut depth) = (Vec::new(), 0, 0i32);
    for t in lex(text).ok()? {
        match t.kind {
            TokenKind::LParen | TokenKind::LBrace | TokenKind::LBracket => depth += 1,
            TokenKind::RParen | TokenKind::RBrace | TokenKind::RBracket => depth -= 1,
            TokenKind::Semi | TokenKind::Newline if depth == 0 => {
                out.push(trim(text, start..t.span.start));
                start = t.span.end;
            }
            _ => {}
        }
    }
    out.push(trim(text, start..text.len()));
    Some(out)
}

/// `range` of `text` without the white space at its ends.
pub(crate) fn trim(text: &str, range: Range<usize>) -> Range<usize> {
    let piece = &text[range.clone()];
    let start = range.start + (piece.len() - piece.trim_start().len());
    let end = range.end - (piece.len() - piece.trim_end().len());
    start..end.max(start)
}
