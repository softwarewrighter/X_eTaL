//! Names and binding targets as written.

use xetal_lex::{FuncName, Var};
use xetal_syntax::Target;

pub(crate) fn func_text(n: &FuncName) -> String {
    let ns = xetal_lex::ns_text(&n.ns);
    let axes: String = n.axes.iter().map(u8::to_string).collect();
    let sub = if axes.is_empty() {
        String::new()
    } else {
        format!("_{axes}")
    };
    format!("{ns}{}{sub}", n.spelled())
}

pub(crate) fn var_text(v: &Var) -> String {
    let ns = xetal_lex::ns_text(&v.ns);
    format!("{ns}{}{}", v.name, if v.mutable { "!" } else { "" })
}

/// A bound name or tuple pattern as written: `x`, `f_`, `(a, (b, _))`.
pub(crate) fn target_text(t: &Target) -> String {
    match t {
        Target::Var(v) => var_text(v),
        Target::Func(f) => func_text(f),
        Target::Tuple(parts) => {
            let parts: Vec<String> = parts.iter().map(target_text).collect();
            format!("({})", parts.join(", "))
        }
        Target::Wild => "_".into(),
    }
}
