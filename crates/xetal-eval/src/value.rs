//! Runtime values and the persistent environment.

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use xetal_core::{Expr, Param};

#[derive(Debug, Clone)]
pub enum Value<'a> {
    Int(i64),
    Float(f64),
    Bool(bool),
    Unit,
    Closure(Rc<Closure<'a>>),
    Prim(Rc<Prim<'a>>),
}

#[derive(Debug)]
pub struct Closure<'a> {
    pub param: &'a Param,
    pub lazy: bool,
    pub body: &'a Expr,
    pub env: Env<'a>,
}

/// A built-in, possibly partially applied.
#[derive(Debug)]
pub struct Prim<'a> {
    pub name: &'static str,
    pub arity: usize,
    pub args: Vec<Value<'a>>,
}

/// A variable's storage: shared so closures see `!` updates (M2).
#[derive(Debug)]
pub enum Slot<'a> {
    Value(Value<'a>),
    /// A lazy argument not yet needed (E1).
    Thunk(&'a Expr, Env<'a>),
    /// Being forced right now (a cycle means infinite recursion).
    Forcing,
    /// A recursive binding before its value exists.
    Empty,
}

#[derive(Debug)]
pub struct Frame<'a> {
    pub name: &'a str,
    pub slot: Rc<RefCell<Slot<'a>>>,
    pub next: Env<'a>,
}

/// A persistent (shared-tail) list of frames.
pub type Env<'a> = Option<Rc<Frame<'a>>>;

pub fn extend<'a>(env: &Env<'a>, name: &'a str, slot: Slot<'a>) -> Env<'a> {
    Some(Rc::new(Frame {
        name,
        slot: Rc::new(RefCell::new(slot)),
        next: env.clone(),
    }))
}

pub fn lookup<'a>(env: &Env<'a>, name: &str) -> Option<Rc<RefCell<Slot<'a>>>> {
    let mut frame = env.as_ref();
    while let Some(f) = frame {
        if f.name == name {
            return Some(f.slot.clone());
        }
        frame = f.next.as_ref();
    }
    None
}

impl fmt::Display for Value<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{i}"),
            Value::Float(x) => write!(f, "{x:?}"),
            Value::Bool(b) => f.write_str(if *b { "1" } else { "0" }),
            Value::Unit => f.write_str("@"),
            Value::Closure(_) | Value::Prim(_) => f.write_str("<function>"),
        }
    }
}
