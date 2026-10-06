//! The page follows the stone: when the program says where it stands
//! (`at IDIOM TOP BOTTOM`, which it prints as a turn lands), each list
//! is set to that item and the address bar is made to say it
//! (`?idiom=rotate&top=xetal&bottom=k`, read when the page opens and
//! sent to the program as choices, so a link opens on a comparison).
//! The lists are not bound to the position on every render, since a
//! choice made in a list would then snap back to the old item until
//! the turn landed half a second later.

use wasm_bindgen::JsCast;
use web_sys::{HtmlSelectElement, UrlSearchParams};
use yew::prelude::*;

/// The three lists' nodes: idiom, top, bottom.
pub type Selects = [NodeRef; 3];

/// Set each list to the position's item whenever the position changes.
#[hook]
pub fn use_follow(at: Option<[String; 3]>, selects: Selects) {
    use_effect_with(at, move |at| {
        if let Some(at) = at {
            for (node, key) in selects.iter().zip(at) {
                if let Some(select) = node.cast::<HtmlSelectElement>() {
                    select.set_value(key);
                }
            }
        }
    });
}

/// One list's choice as the program's event, `choose AXIS ITEM`.
pub fn chosen(axis: usize, send: Callback<String>) -> Callback<Event> {
    Callback::from(move |e: Event| {
        let Some(target) = e
            .target()
            .and_then(|t| t.dyn_into::<HtmlSelectElement>().ok())
        else {
            return;
        };
        send.emit(format!("choose {axis} {}", target.selected_index() + 1));
    })
}

/// The three keys named in the address bar, if any.
pub fn read_address() -> [Option<String>; 3] {
    let Some(search) = web_sys::window().and_then(|w| w.location().search().ok()) else {
        return [None, None, None];
    };
    let Ok(params) = UrlSearchParams::new_with_str(&search) else {
        return [None, None, None];
    };
    [params.get("idiom"), params.get("top"), params.get("bottom")]
}

/// The address bar made to say where the stone stands, without a reload.
pub fn write_address(at: &[String; 3]) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let query = format!("?idiom={}&top={}&bottom={}", at[0], at[1], at[2]);
    if let Ok(history) = window.history() {
        let _ = history.replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&query));
    }
}
