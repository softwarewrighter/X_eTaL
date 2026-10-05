//! Types, type variables and schemes, with a readable display
//! (`Num a => a -> a -> a`).

use std::collections::HashMap;
use std::fmt;

use crate::Classes;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TypeVar(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Unit,
    Bool,
    Int,
    Float,
    Char,
    Var(TypeVar),
    Fn(Box<Type>, Box<Type>),
    /// An enclosed item of a nested array (A7, B14).
    Box(Box<Type>),
    /// A handler's outcome for a protected body of this type (ER2).
    Outcome(Box<Type>),
    /// A built-in nominal type, by name (QD6: `Color`, `Key`; ER2:
    /// `Error`).
    Named(&'static str),
}

/// The built-in nominal types (QD6, ER2); Saga 29 makes the enumerated
/// ones ordinary declarations.
pub const ENUMS: [&str; 4] = ["Color", "Key", "Error", "Event"];

/// A polymorphic type: `forall vars. ty`, with the variables that must
/// be numbers (`Num`) or usable as conditions (`Truthy`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scheme {
    pub vars: Vec<TypeVar>,
    /// The class constraints of quantified variables.
    pub classes: Vec<(TypeVar, Classes)>,
    pub ty: Type,
}

impl Type {
    /// Replace variables through `map`.
    pub fn rename(&self, map: &HashMap<TypeVar, Type>) -> Type {
        match self {
            Type::Var(v) => map.get(v).cloned().unwrap_or_else(|| self.clone()),
            Type::Fn(a, b) => Type::Fn(Box::new(a.rename(map)), Box::new(b.rename(map))),
            Type::Box(a) => Type::Box(Box::new(a.rename(map))),
            Type::Outcome(a) => Type::Outcome(Box::new(a.rename(map))),
            other => other.clone(),
        }
    }

    /// Variables in order of first appearance.
    pub fn vars(&self, out: &mut Vec<TypeVar>) {
        match self {
            Type::Var(v) if !out.contains(v) => out.push(*v),
            Type::Fn(a, b) => {
                a.vars(out);
                b.vars(out);
            }
            Type::Box(a) | Type::Outcome(a) => a.vars(out),
            _ => {}
        }
    }
}

/// Write `ty`, naming variables through `names`.
fn write(ty: &Type, names: &HashMap<TypeVar, String>, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match ty {
        Type::Unit => f.write_str("Unit"),
        Type::Bool => f.write_str("Bool"),
        Type::Int => f.write_str("Int"),
        Type::Float => f.write_str("Float"),
        Type::Char => f.write_str("Char"),
        Type::Named(name) => f.write_str(name),
        Type::Var(v) => match names.get(v) {
            Some(name) => f.write_str(name),
            None => write!(f, "t{}", v.0),
        },
        Type::Box(a) | Type::Outcome(a) => {
            f.write_str(if matches!(ty, Type::Box(_)) {
                "Box "
            } else {
                "Outcome "
            })?;
            match **a {
                Type::Fn(..) | Type::Box(_) | Type::Outcome(_) => {
                    f.write_str("(")?;
                    write(a, names, f)?;
                    f.write_str(")")
                }
                _ => write(a, names, f),
            }
        }
        Type::Fn(a, b) => {
            if matches!(**a, Type::Fn(..)) {
                f.write_str("(")?;
                write(a, names, f)?;
                f.write_str(")")?;
            } else {
                write(a, names, f)?;
            }
            f.write_str(" -> ")?;
            write(b, names, f)
        }
    }
}

fn letters(vars: &[TypeVar]) -> HashMap<TypeVar, String> {
    vars.iter()
        .enumerate()
        .map(|(i, v)| {
            let letter = char::from(b'a' + (i % 26) as u8);
            let name = if i < 26 {
                letter.to_string()
            } else {
                format!("{letter}{}", i / 26)
            };
            (*v, name)
        })
        .collect()
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut vars = Vec::new();
        self.vars(&mut vars);
        write(self, &letters(&vars), f)
    }
}

impl fmt::Display for Scheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut vars = Vec::new();
        self.ty.vars(&mut vars);
        let names = letters(&vars);
        let mut constraints: Vec<String> = Vec::new();
        for v in &vars {
            let name = names.get(v).cloned().unwrap_or_default();
            for class in self.class_of(*v).names() {
                constraints.push(format!("{class} {name}"));
            }
        }
        match constraints.len() {
            0 => {}
            1 => write!(f, "{} => ", constraints[0])?,
            _ => write!(f, "({}) => ", constraints.join(", "))?,
        }
        write(&self.ty, &names, f)
    }
}
