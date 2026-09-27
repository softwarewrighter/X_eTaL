//! Token types and the stable one-line dump format used by `xetal lex`.

use std::fmt;

use xetal_base::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Name(Name),
    LamArg(Side),
    Num(Number),
    Unit,
    Semi,
    Newline,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
}

/// A possibly decorated name: `[ns.]stem[^deriv][_[sub]]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name {
    pub ns: Option<String>,
    pub stem: String,
    /// The stem is a symbol (`+ - * / = < > |`), always a function.
    pub symbol: bool,
    /// Superscript derivation word, spelled as written (`r`, `reduce`).
    pub deriv: Option<String>,
    pub sub: Option<Sub>,
}

/// What follows the underline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sub {
    /// Bare underline: "this name is a function".
    Bare,
    /// Axis subscript, one digit per axis (1-based).
    Axes(Vec<u8>),
    /// `_@`: niladic application sugar.
    Niladic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Number {
    Int(i64),
    Float(f64),
}

impl Name {
    /// Class comes from decoration only: a plain named stem is a noun.
    pub fn is_function(&self) -> bool {
        self.symbol || self.deriv.is_some() || self.sub.is_some()
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.is_function() {
            return write!(f, "Noun({})", self.stem);
        }
        write!(f, "Func({}", self.stem)?;
        if let Some(ns) = &self.ns {
            write!(f, ", ns={ns}")?;
        }
        if let Some(deriv) = &self.deriv {
            write!(f, ", deriv={deriv}")?;
        }
        match &self.sub {
            Some(Sub::Axes(axes)) => {
                let list: Vec<String> = axes.iter().map(u8::to_string).collect();
                write!(f, ", axes=[{}]", list.join(","))?;
            }
            Some(Sub::Niladic) => write!(f, ", niladic")?,
            Some(Sub::Bare) | None => {}
        }
        write!(f, ")")
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Int(n) => write!(f, "{n}"),
            Number::Float(x) => write!(f, "{x:?}"),
        }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fixed = match self {
            TokenKind::Name(name) => return write!(f, "{name}"),
            TokenKind::Num(n) => return write!(f, "Num({n})"),
            TokenKind::LamArg(Side::Left) => "LamArg(l)",
            TokenKind::LamArg(Side::Right) => "LamArg(r)",
            TokenKind::Unit => "Unit",
            TokenKind::Semi => "Semi",
            TokenKind::Newline => "Newline",
            TokenKind::LParen => "LParen",
            TokenKind::RParen => "RParen",
            TokenKind::LBrace => "LBrace",
            TokenKind::RBrace => "RBrace",
            TokenKind::LBracket => "LBracket",
            TokenKind::RBracket => "RBracket",
        };
        f.write_str(fixed)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{} {}", self.span.start, self.span.end, self.kind)
    }
}
