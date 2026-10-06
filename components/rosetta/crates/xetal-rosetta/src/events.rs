//! The page's pointer and keys as the program's events (RS1): pointer
//! positions in the picture's own pixels (its viewBox), one line each,
//! and keys by the names `[]K_EY` uses. Moves are sent at most once a
//! frame, since the program draws no more often than that.

use std::cell::Cell;
use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{Element, KeyboardEvent, PointerEvent};
use yew::prelude::*;

/// The picture's size in its own pixels (Stone's size).
const SIZE: f64 = 420.0;

/// A pointer event's position in the picture's pixels: the element's
/// box scaled to the viewBox.
fn at(e: &PointerEvent) -> Option<(f64, f64)> {
    let target: Element = e.current_target()?.dyn_into().ok()?;
    let rect = target.get_bounding_client_rect();
    if rect.width() <= 0.0 || rect.height() <= 0.0 {
        return None;
    }
    let x = (f64::from(e.client_x()) - rect.left()) * SIZE / rect.width();
    let y = (f64::from(e.client_y()) - rect.top()) * SIZE / rect.height();
    Some((x, y))
}

/// The pointer handlers for the picture: down, move (throttled), up.
pub fn pointer(
    send: Callback<String>,
) -> (
    Callback<PointerEvent>,
    Callback<PointerEvent>,
    Callback<PointerEvent>,
) {
    let last_move = Rc::new(Cell::new(0.0_f64));
    let line = |kind: &'static str, send: Callback<String>| {
        Callback::from(move |e: PointerEvent| {
            if let Some((x, y)) = at(&e) {
                e.prevent_default();
                send.emit(format!("{kind} {x:.1} {y:.1}"));
            }
        })
    };
    let s = send.clone();
    let moved = Callback::from(move |e: PointerEvent| {
        if e.buttons() == 0 {
            return;
        }
        let now = js_sys::Date::now();
        if now - last_move.get() < 16.0 {
            return;
        }
        last_move.set(now);
        if let Some((x, y)) = at(&e) {
            e.prevent_default();
            s.emit(format!("move {x:.1} {y:.1}"));
        }
    });
    (line("down", send.clone()), moved, line("up", send))
}

/// The page's keys, heard on the whole window (no element needs the
/// focus first): arrows and letters go to the program as `key NAME`
/// while it runs on events; keys typed into a control (a list) and the
/// browser's own shortcuts are left alone.
#[hook]
pub fn use_keys(send: Callback<String>, listening: bool) {
    use_effect_with(listening, move |listening| {
        let handler = Closure::<dyn FnMut(KeyboardEvent)>::new(move |e: KeyboardEvent| {
            if in_control(&e) || e.ctrl_key() || e.meta_key() || e.alt_key() {
                return;
            }
            if let Some(name) = xetal_lineedit::key_name(&e.key()) {
                e.prevent_default();
                send.emit(format!("key {name}"));
            }
        });
        let window = web_sys::window().filter(|_| *listening);
        if let Some(w) = &window {
            let _ = w.add_event_listener_with_callback("keydown", handler.as_ref().unchecked_ref());
        }
        move || {
            if let Some(w) = window {
                let _ = w.remove_event_listener_with_callback(
                    "keydown",
                    handler.as_ref().unchecked_ref(),
                );
            }
        }
    });
}

/// Whether a key event comes from a control the key should operate
/// (a list, a field), not the stone.
fn in_control(e: &KeyboardEvent) -> bool {
    e.target()
        .and_then(|t| t.dyn_into::<Element>().ok())
        .is_some_and(|el| matches!(el.tag_name().as_str(), "SELECT" | "INPUT" | "TEXTAREA"))
}
