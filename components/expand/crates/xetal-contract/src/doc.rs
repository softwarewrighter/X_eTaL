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
    for line in text.split_inclusive('\n') {
        let (start, line) = (at, line.trim_end_matches(['\n', '\r']));
        at += line.len() + 1;
        if let Some(doc) = line.strip_prefix("##") {
            let span = Span::new(start, start + line.len());
            read(&mut pending, doc.trim(), span)?;
            continue;
        }
        match defined(line) {
            Some(name) => out.insert(name, std::mem::take(&mut pending)),
            None => Some(std::mem::take(&mut pending)),
        };
    }
    Ok(out)
}

/// `doc`, one line of the block with its `##` removed, into `c`.
fn read(c: &mut Contract, doc: &str, span: Span) -> Result<(), Diagnostic> {
    if let Some(names) = doc.strip_prefix("binds:") {
        c.binds.extend(names.split_whitespace().map(String::from));
    } else if let Some(sides) = doc.strip_prefix("takes:") {
        let (texts, code) = takes(sides).ok_or_else(|| {
            let message = format!(
                "a takes: line is two of text, code or @ (text text, @ code, text @, @ @), not{sides}"
            );
            Diagnostic::new("bad-contract", message).with_span(span)
        })?;
        (c.takes, c.code) = (Some(texts), code);
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

/// The shape `sides` names: which sides are text, and which of those
/// are code.
type Sides = ((bool, bool), (bool, bool));
fn takes(sides: &str) -> Option<Sides> {
    let side = |w: &str| match w {
        "text" => Some((true, false)),
        "code" => Some((true, true)),
        "@" => Some((false, false)),
        _ => None,
    };
    match sides.split_whitespace().collect::<Vec<_>>()[..] {
        [l, r] => {
            let (l, r) = (side(l)?, side(r)?);
            Some(((l.0, r.0), (l.1, r.1)))
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
