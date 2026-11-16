//! The requests that read the stopped run: breakpoints set by file,
//! the stack, the scopes and their variables (a value's children a page
//! at a time), and watches and hovers (`evaluate`).

use std::collections::HashMap;

use serde_json::{Value, json};
use xetal_debug::{Breakpoint, Debugger, Var};

/// Variable references from here up name a scope; below, a value the
/// debugger shows with children.
pub(crate) const SCOPES: i64 = 1_000_000;

/// `setBreakpoints`: the file's breakpoints, with conditions and hit
/// conditions; each is verified at its line.
pub(crate) fn breakpoints(dbg: &mut Debugger, args: &Value) -> Value {
    let file = args["source"]["path"].as_str().unwrap_or_default();
    let points: Vec<Breakpoint> = args["breakpoints"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|b| Breakpoint {
            line: b["line"].as_u64().unwrap_or(0) as usize,
            condition: b["condition"].as_str().map(String::from),
            hit: b["hitCondition"].as_str().map(String::from),
        })
        .collect();
    let shown: Vec<Value> = points
        .iter()
        .map(|b| json!({"verified": true, "line": b.line}))
        .collect();
    dbg.set_conditional(file, points);
    json!({"breakpoints": shown})
}

/// `stackTrace`: the frames, innermost first, each placed in its file.
pub(crate) fn stack(dbg: &Debugger) -> Value {
    let frames: Vec<Value> = dbg
        .stack()
        .into_iter()
        .enumerate()
        .map(|(i, f)| {
            let name = std::path::Path::new(&f.place.file)
                .file_name()
                .map_or(f.place.file.clone(), |n| n.to_string_lossy().into_owned());
            json!({
                "id": i + 1,
                "name": f.name,
                "source": {"name": name, "path": f.place.file},
                "line": f.place.line,
                "column": f.place.col,
            })
        })
        .collect();
    json!({"stackFrames": frames, "totalFrames": frames.len()})
}

/// `scopes`: Locals, Top level and Libraries at the stop; their
/// variables are kept for `variables`.
pub(crate) fn scopes(dbg: &mut Debugger, kept: &mut HashMap<i64, Vec<Var>>) -> Value {
    kept.clear();
    let scopes: Vec<Value> = dbg
        .scopes()
        .into_iter()
        .enumerate()
        .map(|(i, s)| {
            let reference = SCOPES + i as i64;
            let shown =
                json!({"name": s.name, "variablesReference": reference, "expensive": false});
            kept.insert(reference, s.vars);
            shown
        })
        .collect();
    json!({"scopes": scopes})
}

/// `variables`: a scope's variables, or a value's children (`start`
/// and `count` page them).
pub(crate) fn variables(dbg: &mut Debugger, kept: &HashMap<i64, Vec<Var>>, args: &Value) -> Value {
    let reference = args["variablesReference"].as_i64().unwrap_or(0);
    let vars = match kept.get(&reference) {
        Some(vars) => vars.clone(),
        None => {
            let start = args["start"].as_u64().unwrap_or(0) as usize;
            let count = args["count"].as_u64().map_or(usize::MAX, |c| c as usize);
            dbg.children(reference.max(0) as usize, start, count)
        }
    };
    json!({"variables": vars.iter().map(var).collect::<Vec<_>>()})
}

/// `evaluate`: a watch or a hover, evaluated where the run stopped.
pub(crate) fn evaluate(dbg: &mut Debugger, args: &Value) -> Result<Value, String> {
    let text = args["expression"].as_str().unwrap_or_default();
    let v = dbg.evaluate(text)?;
    let reference = if v.children > 0 { v.id } else { 0 };
    Ok(json!({"result": v.value, "type": v.type_name, "variablesReference": reference}))
}

/// A variable as DAP shows it.
fn var(v: &Var) -> Value {
    let reference = if v.children > 0 { v.id } else { 0 };
    json!({
        "name": v.name,
        "value": v.value,
        "type": v.type_name,
        "variablesReference": reference,
        "indexedVariables": v.children,
    })
}
