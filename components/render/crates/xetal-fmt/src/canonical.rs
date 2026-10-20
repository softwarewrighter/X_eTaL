//! The canonical printer over the surface tree. It reparses to the same
//! tree: `parse(fmt(parse x)) == parse x`.

use xetal_base::Diagnostic;
use xetal_lex::{Number, Side};
use xetal_syntax::{
    Expr, ExprKind, Fun, FunKind, Lambda, Params, Stmt, Target, parse, parse_body,
};

use crate::names::{func_text, target_text, var_text};
use crate::string::{code_text, string_text};

/// Format lexable, parsable source canonically.
pub fn canonical(src: &str) -> Result<String, Diagnostic> {
    let program = parse(src)?;
    let printer = Printer { src };
    let lines: Vec<String> = program.stmts.iter().map(|s| printer.stmt(s)).collect();
    Ok(lines.join("\n"))
}

/// The canonical form of `text`, a lambda's body standing alone (a code
/// string's inside, ST5): its statements on lines where it was written
/// on lines, else joined by `; `; none when it does not parse.
pub(crate) fn body(text: &str) -> Option<String> {
    let stmts = parse_body(text).ok()?;
    let printer = Printer { src: text };
    let sep = if text.contains('\n') { "\n" } else { "; " };
    let lines: Vec<String> = stmts.iter().map(|s| printer.stmt(s)).collect();
    Some(lines.join(sep))
}

/// The source being formatted, for what the tree does not keep (the
/// form a raw string was written in).
struct Printer<'a> {
    src: &'a str,
}

impl Printer<'_> {
    fn stmt(&self, s: &Stmt) -> String {
        match s {
            Stmt::Bind { target, value, .. } => {
                format!("{} := {}", target_text(target), self.expr(value, true))
            }
            Stmt::Guard { cond, result, .. } => {
                format!("{} ? {}", self.expr(cond, true), self.expr(result, true))
            }
            // A macro call that is the whole statement stays one (the
            // expander gives it statements: an import, a definition).
            Stmt::Expr(e) => {
                let call = match &e.kind {
                    ExprKind::Monadic { f, .. } | ExprKind::Dyadic { f, .. } => f,
                    _ => return self.expr(e, true),
                };
                let statement = matches!(&call.kind, FunKind::Name(n) if n.is_macro());
                self.expr(e, !statement)
            }
        }
    }

    /// `wrap` puts an application in parentheses (off only where the
    /// caller supplies them, as in `(expr)_`).
    fn expr(&self, e: &Expr, wrap: bool) -> String {
        let group = |text: String| if wrap { format!("({text})") } else { text };
        let joined = |items: &[Expr], sep: &str| {
            let parts: Vec<String> = items.iter().map(|x| self.expr(x, true)).collect();
            parts.join(sep)
        };
        match &e.kind {
            ExprKind::Num(Number::Int(i)) => i.to_string(),
            ExprKind::Num(Number::Float(x)) => format!("{x:?}"),
            ExprKind::Strand(items) => joined(items, " "),
            ExprKind::Tuple(items) => format!("({})", joined(items, ", ")),
            ExprKind::Var(v) => var_text(v),
            ExprKind::Arg(side) => if *side == Side::Left { "_l" } else { "_r" }.into(),
            ExprKind::Str(s) => string_text(self.src, e.span, s),
            ExprKind::Unit => "@".into(),
            ExprKind::Pow { base, exp } => {
                let base_text = match base.kind {
                    ExprKind::Num(_)
                    | ExprKind::Var(_)
                    | ExprKind::Arg(_)
                    | ExprKind::Monadic { .. }
                    | ExprKind::Dyadic { .. } => self.expr(base, true),
                    _ => format!("({})", self.expr(base, true)),
                };
                let exp_text = self.expr(&Expr::new(ExprKind::Num(*exp), base.span), true);
                format!("{base_text}^{exp_text}")
            }
            ExprKind::Quote(f) => format!("'{}", self.fun(f)),
            ExprKind::Monadic { f, arg } => group(format!("{} {}", self.fun(f), self.side(arg, f))),
            ExprKind::Dyadic { left, f, right } => group(format!(
                "{} {} {}",
                self.side(left, f),
                self.fun(f),
                self.side(right, f)
            )),
            ExprKind::Fn(f) => self.fun(f),
        }
    }

    /// An argument of `f`: a code string on a macro's side is code, its
    /// inside in canonical form (ST5); any other as usual.
    fn side(&self, e: &Expr, f: &Fun) -> String {
        match (&e.kind, &f.kind) {
            (ExprKind::Str(s), FunKind::Name(n)) if n.is_macro() => code_text(self.src, e.span, s),
            _ => self.expr(e, true),
        }
    }

    fn fun(&self, f: &Fun) -> String {
        match &f.kind {
            FunKind::Name(n) => func_text(n),
            FunKind::Sym(s) => s.text().into(),
            FunKind::Arg(side) => if *side == Side::Left { "_l_" } else { "_r_" }.into(),
            FunKind::Apply(e) => format!("({})_", self.expr(e, false)),
            FunKind::Operand { operand, f } => format!("'{} {}", self.fun(operand), self.fun(f)),
            FunKind::Power { f, count } => format!("{}^{count}", self.fun(f)),
            FunKind::Lambda(l) => self.lambda(l),
            FunKind::Train(fs) => {
                let parts: Vec<String> = fs.iter().map(|f| self.fun(f)).collect();
                format!("[{}]", parts.join(" "))
            }
        }
    }

    fn lambda(&self, l: &Lambda) -> String {
        let params = match &l.params {
            Params::Right | Params::LeftRight => String::new(),
            Params::Niladic => "@ -> ".into(),
            Params::Named(ps) => {
                let names: Vec<String> = ps
                    .iter()
                    .map(|p| {
                        let name = match &p.name {
                            None => "@".to_string(),
                            Some(Target::Wild) => "_u".to_string(),
                            Some(name) => target_text(name),
                        };
                        format!("{}{name}", if p.lazy { "~" } else { "" })
                    })
                    .collect();
                format!("{} -> ", names.join(" "))
            }
        };
        let body: Vec<String> = l.body.iter().map(|s| self.stmt(s)).collect();
        format!("{{ {params}{} }}", body.join("; "))
    }
}
