//! Ticks for a program running on events (RS1): while it waits at
//! `[]E_VENT`, the page gives it `tick S` on the next animation frame,
//! S the seconds since the last tick, so a program that only ticks
//! runs at the display's rate and never floods the worker (one event
//! per wait). Pointer events are the Rosetta page's (its own pane).

use std::cell::Cell;
use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use yew::prelude::*;

/// When the program starts waiting for an event, send it a tick on
/// the next animation frame.
#[hook]
pub fn use_ticks(wants_event: bool, press: Callback<String>) {
    let last = use_mut_ref(|| Cell::new(0.0_f64));
    use_effect_with(wants_event, move |wants| {
        if *wants {
            let last = Rc::clone(&last);
            let at_frame = Closure::once_into_js(move |now: f64| {
                let before = last.borrow().replace(now);
                let seconds = if before == 0.0 {
                    0.0
                } else {
                    (now - before) / 1000.0
                };
                press.emit(format!("tick {seconds:.4}"));
            });
            if let Some(w) = web_sys::window() {
                let _ = w.request_animation_frame(at_frame.unchecked_ref());
            }
        }
    });
}
