//! Where the machine is, between two transitions: about to evaluate a
//! statement, an application, or something smaller; at what call depth;
//! and at which file, line and column, through the program's source map
//! (a stop inside a macro's expansion is placed at the call).

use xetal_base::Span;
use xetal_core::Kind as Form;
use xetal_sources::Sources;
use xetal_step::{Control, Kont, View};

/// What the machine is about to evaluate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// A statement: a top-level item, a binding's value, a lambda's
    /// body or the statement after a binding (a guard's result is part
    /// of the guard's statement).
    Statement,
    /// An application (`f x`, `x f y`): a stop when stepping in.
    Application,
    /// Anything else (a value returned, a literal, a name).
    Other,
}

/// A stop's place in the files as written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Place {
    pub file: String,
    pub line: usize,
    pub col: usize,
}

/// What the next transition evaluates, and its span.
pub(crate) fn next(view: &View) -> (Kind, Option<Span>) {
    let Some(Control::Eval(e, _)) = view.control else {
        return (Kind::Other, None);
    };
    // A binding goes straight on to its value, and a lambda only makes
    // a closure: neither is a stop of its own.
    if matches!(
        e.kind,
        Form::Let { .. } | Form::Set { .. } | Form::Lam { .. }
    ) {
        return (Kind::Other, Some(e.span));
    }
    let statement = matches!(
        view.stack.last(),
        Some(
            Kont::Def { .. }
                | Kont::Show { .. }
                | Kont::Bind { .. }
                | Kont::Assign { .. }
                | Kont::Called { .. }
        )
    );
    let kind = match (statement, &e.kind) {
        (true, _) => Kind::Statement,
        (false, Form::App(..) | Form::App2 { .. }) => Kind::Application,
        _ => Kind::Other,
    };
    (kind, Some(e.span))
}

/// How many calls are in progress.
pub(crate) fn depth(view: &View) -> usize {
    view.stack
        .iter()
        .filter(|k| matches!(k, Kont::Called { .. }))
        .count()
}

/// The place of `span` in the files as written.
pub(crate) fn place(sources: &Sources, span: Span) -> Place {
    let at = sources.locate(span.start);
    Place {
        file: at.file.to_string(),
        line: at.line,
        col: at.col,
    }
}
