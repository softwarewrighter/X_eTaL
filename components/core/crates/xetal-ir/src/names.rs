//! Where each binding's name is written (D152): a lint that renames
//! needs the name's own span, which no Core node has (a binding's span
//! covers the whole statement, a lambda's the whole lambda).

use xetal_base::{NodeId, Span};

use crate::ir::{Expr, Item, Kind, Param, Program};

/// A name written at `span`, bound by `node`: for a `:=` (top-level or
/// local) the node is the bound value; for a lambda parameter
/// (`param`) it is the `Lam` that binds it. A lambda bound by `:=` is
/// both, so the two are told apart by `param`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NameSpan {
    pub node: NodeId,
    pub param: bool,
    pub span: Span,
}

impl Program {
    /// Where the name a `:=` binds to the value `node` is written, if
    /// recorded: not for `_ :=` or a tuple pattern.
    pub fn name_span(&self, node: NodeId) -> Option<Span> {
        self.find_name(node, false)
    }

    /// Where the parameter of the lambda `node` is written, if
    /// recorded: not for the shorthand's `_r` and `_l`, `@`, or a
    /// tuple pattern.
    pub fn param_span(&self, node: NodeId) -> Option<Span> {
        self.find_name(node, true)
    }

    fn find_name(&self, node: NodeId, param: bool) -> Option<Span> {
        let found = self
            .names
            .iter()
            .find(|n| n.node == node && n.param == param);
        found.map(|n| n.span)
    }

    /// The edits that rename `old` to `new` everywhere it is written: each
    /// binding of it and each use. `None` when that cannot be done
    /// surely: `new` is already a name here (it would be captured), or
    /// some binding or use of `old` is not written as `old` (a macro
    /// made it). Every binding of `old` is renamed, shadowed or not, so
    /// what each use refers to does not change.
    pub fn renamed(&self, old: &str, new: &str) -> Option<Vec<(Span, String)>> {
        let mut spans = Vec::new();
        for item in &self.items {
            let (name, value) = match item {
                Item::Def { name, value }
                | Item::Let { name, value, .. }
                | Item::Set { name, value } => (name.as_str(), value),
                Item::Eval(value) => ("", value),
            };
            if name == new {
                return None;
            }
            if name == old {
                spans.push(self.name_span(value.id)?);
            }
            self.spans_in(value, (old, new), &mut spans)?;
        }
        Some(spans.into_iter().map(|s| (s, new.to_string())).collect())
    }

    /// Add to `spans` where `old` is bound or used in `e`; `None` as
    /// for [`Program::renamed`].
    fn spans_in(&self, e: &Expr, (old, new): (&str, &str), spans: &mut Vec<Span>) -> Option<()> {
        let (name, at) = match &e.kind {
            Kind::Var(n) | Kind::Global(n) => {
                let exact = e.span.end - e.span.start == n.len();
                (n, exact.then_some(e.span))
            }
            Kind::Let { name, value, .. } | Kind::Set { name, value, .. } => {
                (name, self.name_span(value.id))
            }
            Kind::Lam {
                param: Param::Name(p),
                ..
            } => (p, self.param_span(e.id)),
            _ => (&String::new(), None),
        };
        if name == new {
            return None;
        }
        if name == old {
            spans.push(at?);
        }
        e.children()
            .into_iter()
            .try_for_each(|c| self.spans_in(c, (old, new), spans))
    }
}

/// The first place `name` is written in `src` within `within`, or `None`
/// when it is not (a name a macro made, a fresh one).
pub fn written_at(src: &str, within: Span, name: &str) -> Option<Span> {
    let text = src.get(within.start..within.end)?;
    let start = within.start + text.find(name)?;
    (!name.is_empty()).then(|| Span::new(start, start + name.len()))
}
