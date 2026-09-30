//! Raw ASCII -> LaTeX math, one way, for post-processing (KaTeX,
//! MathJax, pdflatex). Every token is braced so TeX adds no operator
//! spacing; source spacing is explicit (`\ `, `\\` per newline). An
//! exponent is left unbraced so it is attached to the token it touches,
//! and axes subscript a function's whole name.

use xetal_base::Diagnostic;
use xetal_lex::{FuncName, Side, Symbol, TokenKind, lex};

/// Render lexable raw source as the body of a LaTeX math environment.
pub fn latex(src: &str) -> Result<String, Diagnostic> {
    let tokens = lex(src)?;
    let mut out = String::new();
    let mut pos = 0;
    for token in &tokens {
        out.push_str(&spacing(&src[pos..token.span.start]));
        let raw = &src[token.span.start..token.span.end];
        if token.kind == TokenKind::Newline {
            out.push_str("\\\\\n");
        } else if let TokenKind::Exp(_) = token.kind {
            // Unbraced, so it is the exponent of the token it touches.
            out.push_str(&token_tex(&token.kind, raw));
        } else {
            out.push('{');
            out.push_str(&token_tex(&token.kind, raw));
            out.push('}');
        }
        pos = token.span.end;
    }
    out.push_str(&spacing(&src[pos..]));
    Ok(out)
}

/// Whitespace becomes explicit spaces; comment text is dropped.
fn spacing(gap: &str) -> String {
    gap.split('#')
        .next()
        .unwrap_or("")
        .chars()
        .filter(|c| *c != '\r')
        .map(|_| "\\ ")
        .collect()
}

fn token_tex(kind: &TokenKind, raw: &str) -> String {
    match kind {
        TokenKind::Var(v) => {
            let ns = ns_tex(&v.ns);
            let bang = if v.mutable { "!" } else { "" };
            format!(r"{ns}\mathrm{{{}}}{bang}", v.name)
        }
        TokenKind::Func(name) => func_tex(name),
        TokenKind::LamArg { side, applied } => {
            let s = if *side == Side::Left { "l" } else { "r" };
            let a = if *applied { r"\_" } else { "" };
            format!(r"\_\mathrm{{{s}}}{a}")
        }
        TokenKind::Exp(_) => format!("^{{{}}}", &raw[1..]),
        TokenKind::Sym(sym) => symbol_tex(*sym).into(),
        TokenKind::Str(_) => format!(r"\text{{{}}}", raw.replace('\\', r"\textbackslash{}")),
        TokenKind::Assign => r"\leftarrow".into(),
        TokenKind::Semi => r"\diamond".into(),
        TokenKind::Arrow => r"\to".into(),
        TokenKind::Lazy => r"\sim".into(),
        TokenKind::Quote => r"\text{'}".into(),
        TokenKind::Apply => r"\_".into(),
        TokenKind::LBrace => r"\{".into(),
        TokenKind::RBrace => r"\}".into(),
        _ => raw.into(),
    }
}

fn func_tex(name: &FuncName) -> String {
    let mut out = ns_tex(&name.ns);
    let (before, rest) = name.stem.split_at(name.underline);
    let (letter, after) = rest.split_at(1);
    out.push_str(&format!(
        r"\mathrm{{{before}\underline{{{letter}}}{after}}}"
    ));
    if let Some(mark) = name.mark {
        out.push_str(&format!("{{{}}}", mark_tex(mark)));
    }
    if name.axes.is_empty() {
        return out;
    }
    // The whole name carries the axes, never a bare mark.
    let digits: String = name.axes.iter().map(u8::to_string).collect();
    format!("{{{out}}}_{{{digits}}}")
}

/// A function name's trailing mark, braced by the caller so TeX sets it
/// as an ordinary symbol.
fn mark_tex(mark: char) -> String {
    match mark {
        '\\' => r"\backslash".into(),
        '|' => r"\mid".into(),
        '~' => r"\sim".into(),
        '%' => r"\%".into(),
        '$' => r"\$".into(),
        '&' => r"\&".into(),
        '*' => r"\ast".into(),
        other => other.to_string(),
    }
}

/// A namespace as a leading superscript.
fn ns_tex(ns: &Option<String>) -> String {
    ns.as_ref().map_or(String::new(), |ns| match ns.as_str() {
        xetal_lex::SYSTEM => r"\square ".to_string(),
        ns => format!(r"{{}}^{{\mathrm{{{ns}}}}}"),
    })
}

fn symbol_tex(sym: Symbol) -> &'static str {
    match sym {
        Symbol::Times => r"\times",
        Symbol::Divide => r"\div",
        Symbol::Or => r"\vee",
        Symbol::And => r"\wedge",
        Symbol::Ne => r"\neq",
        Symbol::Le => r"\leq",
        Symbol::Ge => r"\geq",
        Symbol::Power => r"\mathbin{\hat{}}",
        other => other.text(),
    }
}
