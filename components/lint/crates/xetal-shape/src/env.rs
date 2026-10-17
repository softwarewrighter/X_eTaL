//! The names in scope and what the lint knows of each, innermost last.

use crate::ext::{Ext, Fact};

#[derive(Default)]
pub(crate) struct Env {
    names: Vec<(String, Fact)>,
    bindings: usize,
}

impl Env {
    /// What is known of `name` here; nothing when it is not bound.
    pub fn lookup(&self, name: &str) -> Fact {
        let found = self.names.iter().rev().find(|(n, _)| n == name);
        found.map(|(_, f)| f.clone()).unwrap_or_default()
    }

    /// Bind `name` to `fact` (shadowing): a single number the lint
    /// cannot see becomes an extent named for this binding. A mutable
    /// name (`X!`) is bound to nothing known, since it may change.
    pub fn bind(&mut self, name: &str, mut fact: Fact) {
        self.bindings += 1;
        if name.ends_with('!') {
            fact = Fact::default();
        } else if fact.value.is_none() && fact.shape.as_ref().is_some_and(Vec::is_empty) {
            fact.value = Some(vec![Ext::Name(name.to_string(), self.bindings)]);
        }
        self.names.push((name.to_string(), fact));
    }

    /// How many names are bound, to [`Env::reset`] to after a scope.
    pub fn mark(&self) -> usize {
        self.names.len()
    }

    pub fn reset(&mut self, mark: usize) {
        self.names.truncate(mark);
    }
}
