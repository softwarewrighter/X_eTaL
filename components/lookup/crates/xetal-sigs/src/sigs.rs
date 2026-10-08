//! Signature lines (T4, MC21): in System.xtlm, `s:u_se< :: Char -> Char
//! -> Unit` declares a built-in system macro, one with no definition in
//! X_eTaL, its type read as the catalog's signatures are and its `##`
//! block above it its documentation. The macro phase blanks the line
//! (keeping every offset) before reading the file; anywhere else `::`
//! is left to the lexer, which refuses it.

use xetal_base::{Diagnostic, Span};

/// A built-in system macro declared by a signature line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    /// The macro's name with its mark (`u_se<`).
    pub name: String,
    /// Its type, as written (`Char -> Char -> Unit`).
    pub ty: String,
    /// The `##` block directly above, without the `## `.
    pub doc: Vec<String>,
    /// Where the line is in the file.
    pub span: Span,
}

/// The signatures of the built-in System.xtlm.
pub fn system_signatures() -> Result<Vec<Signature>, Diagnostic> {
    signatures(xetal_libs::standard_macros("System").unwrap_or_default()).map(|(_, s)| s)
}

/// `text` (a system macro library) with its signature lines blanked,
/// and the signatures read from them.
pub fn signatures(text: &str) -> Result<(String, Vec<Signature>), Diagnostic> {
    let (mut blanked, mut sigs, mut doc, mut at) = (String::new(), Vec::new(), Vec::new(), 0);
    for line in text.split_inclusive('\n') {
        let body = line.trim_end_matches('\n');
        let span = Span::new(at, at + body.len());
        at += line.len();
        match signature(body, span, &mut doc)? {
            Some(sig) => {
                sigs.push(sig);
                blanked.push_str(&" ".repeat(body.len()));
                blanked.push_str(&line[body.len()..]);
            }
            None => blanked.push_str(line),
        }
    }
    Ok((blanked, sigs))
}

/// The signature `line` declares, if it is one; `doc` collects the
/// `##` block above it.
fn signature(
    line: &str,
    span: Span,
    doc: &mut Vec<String>,
) -> Result<Option<Signature>, Diagnostic> {
    if let Some(text) = line.strip_prefix("##") {
        doc.push(text.strip_prefix(' ').unwrap_or(text).to_string());
        return Ok(None);
    }
    let found = line
        .split_once("::")
        .filter(|_| !line.trim_start().starts_with('#'));
    let Some((name, ty)) = found else {
        doc.clear();
        return Ok(None);
    };
    let bad = |message: String| Diagnostic::new("bad-signature", message).with_span(span);
    let name = name.trim();
    let Some(macro_name) = name.strip_prefix("s:").filter(|n| n.ends_with('<')) else {
        return Err(bad(format!(
            "{name}: a signature line declares a system macro, s:n_ame< :: Type"
        )));
    };
    let mut u = xetal_ty::Unifier::default();
    xetal_prim_types::read(ty.trim(), &mut u).map_err(|d| bad(d.message))?;
    let words = ty.split(|c: char| !c.is_ascii_alphanumeric());
    if let Some(w) = words.filter(|w| !w.is_empty()).find(|w| !known(w)) {
        return Err(bad(format!("{w} is not a type")));
    }
    Ok(Some(Signature {
        name: macro_name.to_string(),
        ty: ty.trim().to_string(),
        doc: std::mem::take(doc),
        span,
    }))
}

/// A word a signature may hold: a type, a class, or a type variable.
fn known(word: &str) -> bool {
    const NAMES: [&str; 14] = [
        "Int", "Float", "Char", "Bool", "Unit", "Box", "Color", "Key", "Num", "Eq", "Ord",
        "Truthy", "Match", "Any",
    ];
    NAMES.contains(&word) || word.bytes().all(|b| b.is_ascii_lowercase())
}
