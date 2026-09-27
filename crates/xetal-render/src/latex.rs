//! Raw ASCII -> LaTeX math, one way, for post-processing (KaTeX,
//! MathJax, pdflatex). Every token is braced so TeX adds no operator
//! spacing; source spacing is explicit (`\ `, `\\` per newline).

use xetal_base::Diagnostic;
use xetal_lex::{Name, Side, Sub, TokenKind, lex};

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

fn spacing(gap: &str) -> String {
    gap.chars().filter(|c| *c != '\r').map(|_| "\\ ").collect()
}

fn token_tex(kind: &TokenKind, raw: &str) -> String {
    match kind {
        TokenKind::Name(name) => name_tex(name),
        TokenKind::LamArg(Side::Left) => r"\_\mathrm{l}".into(),
        TokenKind::LamArg(Side::Right) => r"\_\mathrm{r}".into(),
        TokenKind::LBrace => r"\{".into(),
        TokenKind::RBrace => r"\}".into(),
        _ => raw.into(),
    }
}

fn stem_tex(name: &Name) -> String {
    match name.stem.as_str() {
        "*" => r"\ast".into(),
        "|" => r"\mid".into(),
        s if name.symbol => s.into(),
        s => format!(r"\mathrm{{{s}}}"),
    }
}

fn name_tex(name: &Name) -> String {
    let mut out = name
        .ns
        .as_ref()
        .map_or(String::new(), |ns| format!(r"\mathrm{{{ns}}}."));
    let stem = stem_tex(name);
    let underlined = name.deriv.is_none()
        && match name.sub {
            Some(Sub::Niladic) => true,
            Some(_) => !name.symbol,
            None => false,
        };
    if underlined {
        out.push_str(&format!(r"\underline{{{stem}}}"));
    } else {
        out.push_str(&stem);
    }
    if let Some(word) = &name.deriv {
        out.push_str(&format!(r"^{{\mathrm{{{word}}}}}"));
    }
    match &name.sub {
        Some(Sub::Axes(axes)) => {
            let digits: String = axes.iter().map(u8::to_string).collect();
            out.push_str(&format!("_{{{digits}}}"));
        }
        Some(Sub::Niladic) => out.push_str("_{@}"),
        Some(Sub::Bare) | None => {}
    }
    out
}
