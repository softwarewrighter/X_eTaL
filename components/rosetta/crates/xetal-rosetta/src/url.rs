//! The address bar: `?idiom=rotate&top=xetal&bottom=k` read when the
//! page opens (sent to the program as choices) and written as the
//! program reports where it stands, so a link opens on a comparison.

use web_sys::UrlSearchParams;

/// The three keys named in the address, if any.
pub fn read() -> [Option<String>; 3] {
    let Some(search) = web_sys::window().and_then(|w| w.location().search().ok()) else {
        return [None, None, None];
    };
    let Ok(params) = UrlSearchParams::new_with_str(&search) else {
        return [None, None, None];
    };
    [params.get("idiom"), params.get("top"), params.get("bottom")]
}

/// The address made to say where the stone stands, without a reload.
pub fn write(at: &[String; 3]) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let query = format!("?idiom={}&top={}&bottom={}", at[0], at[1], at[2]);
    if let Ok(history) = window.history() {
        let _ = history.replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&query));
    }
}
