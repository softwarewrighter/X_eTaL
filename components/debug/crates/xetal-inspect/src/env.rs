//! A frame's own bindings: an environment walked above the top level,
//! innermost first, for a debugger's Locals and a watch's frame.

use std::rc::Rc;

use xetal_value::{Env, Frame as EnvFrame, Slot, Value};

/// The frames of `env` above the top level `top` (a closure's env ends
/// in the top level as it was when the closure was made, so the walk
/// stops at any frame of `top`'s chain), innermost first.
pub fn own<'e, 'a>(env: &'e Env<'a>, top: &Env<'a>) -> Vec<&'e Rc<EnvFrame<'a>>> {
    let mut tops: Vec<*const EnvFrame<'a>> = Vec::new();
    let mut t = top.as_ref();
    while let Some(f) = t {
        tops.push(Rc::as_ptr(f));
        t = f.next.as_ref();
    }
    let mut out = Vec::new();
    let mut frame = env.as_ref();
    while let Some(f) = frame.filter(|f| !tops.contains(&Rc::as_ptr(f))) {
        out.push(f);
        frame = f.next.as_ref();
    }
    out
}

/// The bindings of `env` above `stop`, innermost first; a name hidden
/// by an inner one is left out, and so are the names the compiler made
/// (`#n1`). A lazy argument not yet needed is a note.
pub fn bindings<'a>(env: &Env<'a>, stop: &Env<'a>) -> Vec<(String, Result<Value<'a>, String>)> {
    let mut out: Vec<(String, Result<Value<'a>, String>)> = Vec::new();
    for f in own(env, stop) {
        if !f.name.starts_with('#') && !out.iter().any(|(n, _)| n == f.name) {
            let value = match &*f.slot.borrow() {
                Slot::Value(v) => Ok(v.clone()),
                Slot::Thunk(..) => Err("(lazy: not evaluated yet)".to_string()),
                _ => Err("(no value yet)".to_string()),
            };
            out.push((f.name.to_string(), value));
        }
    }
    out
}
