//! Semantic classes for highlighting, from token kinds.

use xetal_lex::TokenKind;

use crate::Segment;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Class {
    /// A system function (`r_ev`).
    Builtin,
    /// A macro, a name ending in `<` (`u_se<`, MC2).
    Macro,
    /// A function in the program's namespace (`u:s_quare`).
    UserFunc,
    /// A function from an imported library (`c:K_`).
    LibFunc,
    Variable,
    /// `_l`, `_r` (and applied, `_l_`).
    LambdaArg,
    Number,
    Exponent,
    String,
    /// A symbol function (`+`, `<=`).
    Symbol,
    /// `'`, which passes a function as a value.
    Quote,
    /// Brackets, `:=`, `->`, `;`, `?`, `~` and `_` (apply).
    Punct,
    Unit,
    Comment,
    /// Whitespace and newlines.
    Space,
    /// Text that does not lex; shown as typed.
    Error,
}

/// The class of a token.
pub(crate) fn classify(kind: &TokenKind) -> Class {
    match kind {
        TokenKind::Func(f) if f.mark == Some('<') => Class::Macro,
        TokenKind::Func(f) => match f.ns.as_deref() {
            None => Class::Builtin,
            Some("u") => Class::UserFunc,
            Some(_) => Class::LibFunc,
        },
        TokenKind::Var(_) => Class::Variable,
        TokenKind::LamArg { .. } => Class::LambdaArg,
        TokenKind::Num(_) => Class::Number,
        TokenKind::Exp(_) => Class::Exponent,
        TokenKind::Str(_) => Class::String,
        TokenKind::Sym(_) => Class::Symbol,
        TokenKind::Quote => Class::Quote,
        TokenKind::Unit => Class::Unit,
        TokenKind::Newline => Class::Space,
        _ => Class::Punct,
    }
}

/// A quote takes the class of the function it quotes (`'+`, `'r_/`,
/// `'u:p_lus`), so an operand reads as one unit; before anything else
/// (a lambda, a train) it keeps its own class.
pub(crate) fn quotes_take_function_class(segments: &mut [Segment]) {
    for i in 0..segments.len() {
        if segments[i].class != Class::Quote {
            continue;
        }
        let next = segments[i + 1..]
            .iter()
            .find(|s| s.class != Class::Space)
            .map(|s| s.class);
        if let Some(c @ (Class::Symbol | Class::Builtin | Class::UserFunc | Class::LibFunc)) = next
        {
            segments[i].class = c;
        }
    }
}
