//! What a copying macro left out of its text: every name and number
//! token of an argument must be a token of the expansion (punctuation
//! and separators are free, a name may gain a namespace, a number
//! written as an exponent is that number). A token inside one of the
//! expansion's string literals counts too: a macro hands its text on
//! to another macro as a string, or quotes it into a message. An
//! expansion that calls `e_rror<` refuses the call and is not checked.

use xetal_base::Diagnostic;
use xetal_lex::{Token, TokenKind, lex};

use crate::{Contract, Uses};

/// `unused-argument` when `contract` copies and a name or number token
/// of an argument (`args`: left, right; none for `@`) is not a token of
/// `expansion`; nothing for a parsing macro or text that does not lex.
pub fn unused(
    contract: &Contract,
    written: &str,
    args: &[Option<&str>; 2],
    expansion: &str,
) -> Option<Diagnostic> {
    if contract.uses == Uses::Parsed {
        return None;
    }
    let tokens = lex(expansion).ok()?;
    if tokens.iter().any(|t| refuses(t)) {
        return None;
    }
    let given = words(&tokens, expansion);
    let sides = [("left", args[0]), ("right", args[1])];
    sides.iter().find_map(|(side, arg)| {
        let arg = (*arg)?;
        let tokens = lex(arg).ok()?;
        let missing = tokens
            .iter()
            .filter_map(|t| counted(t, arg))
            .find(|word| !given.contains(word))?;
        let message = format!(
            "{written} did not use {missing} of the text on its {side}; a macro that copies its text writes every name and number of it (## uses: copied)"
        );
        Some(Diagnostic::new("unused-argument", message).with_note(format!("macro-place: {side}")))
    })
}

/// The words of `tokens`, and of the strings among them: as code when
/// a string lexes, else its runs of name characters (a message).
fn words(tokens: &[Token], text: &str) -> Vec<String> {
    let mut out: Vec<String> = tokens.iter().filter_map(|t| counted(t, text)).collect();
    for inner in tokens.iter().filter_map(|t| match &t.kind {
        TokenKind::Str(s) => Some(s),
        _ => None,
    }) {
        match lex(inner) {
            Ok(tokens) => out.extend(words(&tokens, inner)),
            Err(_) => out.extend(loose(inner)),
        }
    }
    out
}

/// The runs of name characters of a text that is not code, each
/// without a namespace (`ns:name` splits at the colon).
fn loose(text: &str) -> Vec<String> {
    text.split(|c: char| !(c.is_ascii_alphanumeric() || "_!.<".contains(c)))
        .filter(|w| !w.is_empty())
        .map(String::from)
        .collect()
}

/// A call of the system macro `e_rror<`: the expansion refuses the call.
fn refuses(t: &Token) -> bool {
    matches!(&t.kind, TokenKind::Func(f) if f.ns.is_none() && f.spelled() == "e_rror<")
}

/// The word a token counts as: a name without its namespace, a
/// number's text; nothing for punctuation, strings and separators.
fn counted(t: &Token, text: &str) -> Option<String> {
    match &t.kind {
        TokenKind::Var(v) => Some(format!("{}{}", v.name, if v.mutable { "!" } else { "" })),
        TokenKind::Func(f) => Some(f.spelled()),
        TokenKind::Num(_) => Some(text[t.span.start..t.span.end].to_string()),
        TokenKind::Exp(_) => Some(
            text[t.span.start..t.span.end]
                .trim_start_matches('^')
                .to_string(),
        ),
        _ => None,
    }
}
