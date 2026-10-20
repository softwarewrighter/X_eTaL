//! Reading the contract lines of a macro library: the `##` lines
//! directly above a definition `m:n_ame< := ...`; a blank line, a
//! comment or any other line ends the block.

use std::collections::HashMap;

use xetal_base::{Diagnostic, Span};

use crate::{Contract, Uses};

/// Per macro defined in `text`, by its name with the mark (`w_ith<`),
/// its contract; a malformed contract line is `bad-contract` at the line.
pub fn contracts_of(text: &str) -> Result<HashMap<String, Contract>, Diagnostic> {
    let (mut out, mut pending, mut at) = (HashMap::new(), Contract::default(), 0);
    let mut blocks: Vec<(String, Span)> = Vec::new();
    let mut block = Span::new(0, 0);
    for line in text.split_inclusive('\n') {
        let (start, line) = (at, line.trim_end_matches(['\n', '\r']));
        at += line.len() + 1;
        if let Some(doc) = line.strip_prefix("##") {
            let span = Span::new(start, start + line.len());
            if pending == Contract::default() {
                block = span;
            }
            read(&mut pending, doc.trim(), span)?;
            continue;
        }
        match defined(line) {
            Some(name) => {
                blocks.push((name.clone(), block));
                out.insert(name, std::mem::take(&mut pending))
            }
            None => Some(std::mem::take(&mut pending)),
        };
    }
    for (name, span) in blocks {
        drawn(&out, &name, span)?;
    }
    Ok(out)
}

/// A notation side has a `draws:` macro, and a `draws:` line a notation
/// side; the drawer is another macro of the library (MC32).
fn drawn(all: &HashMap<String, Contract>, name: &str, span: Span) -> Result<(), Diagnostic> {
    let c = &all[name];
    let notation = c.notation.0 || c.notation.1;
    let bad = |message: String| Err(Diagnostic::new("bad-contract", message).with_span(span));
    match &c.draws {
        None if notation => bad(format!(
            "{name} takes a notation: a draws: line names the macro of this library that draws it (## draws: t_ex<)"
        )),
        Some(d) if !notation => bad(format!(
            "{name} draws by {d} but takes no notation: write notation for the side it draws (## takes: text notation)"
        )),
        Some(d) if d == name => bad(format!(
            "{name} cannot draw itself: draws: names another macro of this library"
        )),
        Some(d) if !all.contains_key(d) => bad(format!(
            "{name} draws by {d}, which this library does not define"
        )),
        _ => Ok(()),
    }
}

/// `doc`, one line of the block with its `##` removed, into `c`.
fn read(c: &mut Contract, doc: &str, span: Span) -> Result<(), Diagnostic> {
    if let Some(names) = doc.strip_prefix("binds:") {
        c.binds.extend(names.split_whitespace().map(String::from));
    } else if let Some(sides) = doc.strip_prefix("takes:") {
        let (texts, code, notation) = takes(sides).ok_or_else(|| {
            let message = format!(
                "a takes: line is two of text, code, notation or @ (text text, @ code, text notation, @ @), not{sides}"
            );
            Diagnostic::new("bad-contract", message).with_span(span)
        })?;
        (c.takes, c.code, c.notation) = (Some(texts), code, notation);
    } else if let Some(drawer) = doc.strip_prefix("draws:") {
        let name = drawer.trim().rsplit(':').next().unwrap_or_default();
        if name.is_empty() || !name.ends_with('<') {
            let message = format!(
                "a draws: line names a macro of this library (## draws: t_ex<), not{drawer}"
            );
            return Err(Diagnostic::new("bad-contract", message).with_span(span));
        }
        c.draws = Some(name.to_string());
    } else if let Some(how) = doc.strip_prefix("uses:") {
        c.uses = match how.trim() {
            "copied" => Uses::Copied,
            "parsed" => Uses::Parsed,
            other => {
                let message = format!("a uses: line is copied or parsed, not {other}");
                return Err(Diagnostic::new("bad-contract", message).with_span(span));
            }
        };
    }
    Ok(())
}

/// The shape `sides` names: which sides are text, which of those are
/// code, and which a notation.
type Sides = ((bool, bool), (bool, bool), (bool, bool));
fn takes(sides: &str) -> Option<Sides> {
    let side = |w: &str| match w {
        "text" => Some((true, false, false)),
        "code" => Some((true, true, false)),
        "notation" => Some((true, false, true)),
        "@" => Some((false, false, false)),
        _ => None,
    };
    match sides.split_whitespace().collect::<Vec<_>>()[..] {
        [l, r] => {
            let (l, r) = (side(l)?, side(r)?);
            Some(((l.0, r.0), (l.1, r.1), (l.2, r.2)))
        }
        _ => None,
    }
}

/// The macro `line` defines (`m:w_ith< := ...` gives `w_ith<`).
fn defined(line: &str) -> Option<String> {
    let (head, _) = line.split_once(":=")?;
    let (_, name) = head.trim().split_once(':')?;
    name.ends_with('<').then(|| name.to_string())
}
