//! Semantic classes for highlighting, from token kinds.

use xetal_lex::TokenKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Class {
    /// A system function or macro (`r_ev`, `u_se<`).
    Builtin,
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
