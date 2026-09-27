//! Lexer errors; every one carries the span of the offending text.

use xetal_base::{Diagnostic, Span};

/// What went wrong; `code()` is the stable diagnostic code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    BadDecoration,
    BadAxis,
    NonCanonicalOrder,
    RedundantUnderline,
    BadLambdaArg,
    BadNamespace,
    AmbiguousMinus,
    BadNumber,
    NumberOutOfRange,
    UnexpectedChar,
    NonAscii,
}

impl ErrorKind {
    pub fn code(self) -> &'static str {
        match self {
            ErrorKind::BadDecoration => "bad-decoration",
            ErrorKind::BadAxis => "bad-axis",
            ErrorKind::NonCanonicalOrder => "non-canonical-order",
            ErrorKind::RedundantUnderline => "redundant-underline",
            ErrorKind::BadLambdaArg => "bad-lambda-arg",
            ErrorKind::BadNamespace => "bad-namespace",
            ErrorKind::AmbiguousMinus => "ambiguous-minus",
            ErrorKind::BadNumber => "bad-number",
            ErrorKind::NumberOutOfRange => "number-out-of-range",
            ErrorKind::UnexpectedChar => "unexpected-char",
            ErrorKind::NonAscii => "non-ascii",
        }
    }
}

/// A lexing failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexError {
    pub kind: ErrorKind,
    pub span: Span,
    pub message: String,
}

impl LexError {
    pub fn new(kind: ErrorKind, span: Span, message: impl Into<String>) -> Self {
        Self {
            kind,
            span,
            message: message.into(),
        }
    }

    /// An error covering the single byte at `pos`.
    pub fn at(kind: ErrorKind, pos: usize, message: impl Into<String>) -> Self {
        Self::new(kind, Span::new(pos, pos + 1), message)
    }

    pub fn code(&self) -> &'static str {
        self.kind.code()
    }
}

impl From<LexError> for Diagnostic {
    fn from(err: LexError) -> Self {
        Diagnostic::new(err.code(), err.message).with_span(err.span)
    }
}
