//! The surface AST. Every node carries the span of its source text.

use xetal_base::Span;
use xetal_lex::{FuncName, Number, Side, Symbol, Var};

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Bind {
        target: Target,
        value: Expr,
        span: Span,
    },
    Guard {
        cond: Expr,
        result: Expr,
        span: Span,
    },
    Expr(Expr),
}

/// A name that can be bound: a variable or a function name.
#[derive(Debug, Clone, PartialEq)]
pub enum Target {
    Var(Var),
    Func(FuncName),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    Num(Number),
    /// Adjacent numbers (each possibly with an exponent).
    Strand(Vec<Expr>),
    Var(Var),
    Arg(Side),
    Str(String),
    Unit,
    Pow {
        base: Box<Expr>,
        exp: Number,
    },
    /// A function passed as a value (`'f_`).
    Quote(Box<Fun>),
    Monadic {
        f: Box<Fun>,
        arg: Box<Expr>,
    },
    Dyadic {
        left: Box<Expr>,
        f: Box<Fun>,
        right: Box<Expr>,
    },
    /// A function standing alone as a value (a lambda, a train, a name).
    Fn(Box<Fun>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Fun {
    pub kind: FunKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FunKind {
    Name(FuncName),
    Sym(Symbol),
    /// `_l_` / `_r_`: apply the argument's value.
    Arg(Side),
    /// `(expr)_`: apply a computed function value.
    Apply(Box<Expr>),
    /// A quoted operand bound to the function on its right (F8).
    Operand {
        operand: Box<Fun>,
        f: Box<Fun>,
    },
    Lambda(Lambda),
    Train(Vec<Fun>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Lambda {
    pub params: Params,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Params {
    /// Shorthand using `_r` only.
    Right,
    /// Shorthand using `_l` and `_r`.
    LeftRight,
    /// `{ @ -> ... }`.
    Niladic,
    Named(Vec<Param>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub name: Target,
    pub lazy: bool,
    pub span: Span,
}

/// Uses of `_l` / `_r` belonging to this lambda (not nested ones).
pub(crate) fn stmt_args(stmt: &Stmt, acc: (bool, bool)) -> (bool, bool) {
    match stmt {
        Stmt::Bind { value, .. } => expr_args(value, acc),
        Stmt::Guard { cond, result, .. } => expr_args(result, expr_args(cond, acc)),
        Stmt::Expr(e) => expr_args(e, acc),
    }
}

fn expr_args(e: &Expr, (l, r): (bool, bool)) -> (bool, bool) {
    let side = |s: &Side| (l || *s == Side::Left, r || *s == Side::Right);
    match &e.kind {
        ExprKind::Arg(s) => side(s),
        ExprKind::Strand(items) => items.iter().fold((l, r), |acc, x| expr_args(x, acc)),
        ExprKind::Pow { base, .. } => expr_args(base, (l, r)),
        ExprKind::Quote(f) | ExprKind::Fn(f) => fun_args(f, (l, r)),
        ExprKind::Monadic { f, arg } => expr_args(arg, fun_args(f, (l, r))),
        ExprKind::Dyadic { left, f, right } => {
            expr_args(right, expr_args(left, fun_args(f, (l, r))))
        }
        _ => (l, r),
    }
}

fn fun_args(f: &Fun, (l, r): (bool, bool)) -> (bool, bool) {
    match &f.kind {
        FunKind::Arg(s) => (l || *s == Side::Left, r || *s == Side::Right),
        FunKind::Apply(e) => expr_args(e, (l, r)),
        FunKind::Operand { operand, f } => fun_args(f, fun_args(operand, (l, r))),
        FunKind::Train(fs) => fs.iter().fold((l, r), |acc, x| fun_args(x, acc)),
        _ => (l, r),
    }
}
