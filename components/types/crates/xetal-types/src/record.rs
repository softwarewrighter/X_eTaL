//! What inference records for elaboration (T6): the type of every
//! integer literal, the `Num` variables each generalized binding
//! quantifies, and the number types each use of a binding passes.

use std::collections::{HashMap, HashSet};

use xetal_base::NodeId;
use xetal_elab::Dicts;
use xetal_ty::{Scheme, Type, TypeVar};

use crate::infer::Infer;

/// The binding a use refers to: a value node, or a module definition
/// (whose value may not have been seen yet).
pub(crate) enum Binder {
    Node(NodeId),
    Global(String),
}

/// The number types a use passes: an instance of a scheme, or, for a
/// monomorphic use inside a binding group, the binder's own variables.
pub(crate) enum Args {
    Inst(Vec<Type>),
    Mono(Binder),
}

#[derive(Default)]
pub(crate) struct Record {
    lits: Vec<(NodeId, Type)>,
    args: Vec<(NodeId, Args)>,
    params: HashMap<NodeId, Vec<TypeVar>>,
    globals: HashMap<String, NodeId>,
    /// Variables some scheme quantifies; top-level defaulting skips them.
    pub quantified: HashSet<TypeVar>,
}

impl Record {
    pub(crate) fn use_of(&mut self, id: NodeId, args: Args) {
        if !matches!(&args, Args::Inst(ts) if ts.is_empty()) {
            self.args.push((id, args));
        }
    }

    pub(crate) fn generalized(&mut self, value: NodeId, scheme: &Scheme) {
        self.quantified.extend(scheme.vars.iter().copied());
        self.params.insert(value, scheme.num.clone());
    }

    pub(crate) fn global(&mut self, name: &str, value: NodeId) {
        self.globals.insert(name.to_string(), value);
    }
}

impl Infer {
    /// A polymorphic integer literal.
    pub(crate) fn int_literal(&mut self, id: NodeId) -> Type {
        let t = self.u.fresh_num();
        self.rec.lits.push((id, t.clone()));
        t
    }

    /// The recorded facts with every type resolved.
    pub(crate) fn dicts(&self) -> Dicts {
        let r = &self.rec;
        let binder_vars = |b: &Binder| {
            let id = match b {
                Binder::Node(id) => Some(*id),
                Binder::Global(name) => r.globals.get(name).copied(),
            };
            let vars = id.and_then(|id| r.params.get(&id)).cloned();
            vars.unwrap_or_default()
                .into_iter()
                .map(Type::Var)
                .collect()
        };
        let args = r.args.iter().filter_map(|(id, a)| {
            let ts: Vec<Type> = match a {
                Args::Inst(ts) => ts.clone(),
                Args::Mono(b) => binder_vars(b),
            };
            let ts: Vec<Type> = ts.iter().map(|t| self.u.resolve(t)).collect();
            (!ts.is_empty()).then_some((*id, ts))
        });
        Dicts {
            params: r
                .params
                .iter()
                .filter(|(_, vs)| !vs.is_empty())
                .map(|(k, v)| (*k, v.clone()))
                .collect(),
            args: args.collect(),
            lits: r
                .lits
                .iter()
                .map(|(id, t)| (*id, self.u.resolve(t)))
                .collect(),
        }
    }
}
