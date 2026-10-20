//! A notation's LaTeX (MC32: what a `draws:` macro wrote) typeset to
//! HTML by KaTeX, which the page loads from a CDN (`katex.renderToString`,
//! inline mode, errors drawn in place rather than thrown); the LaTeX
//! itself, escaped, when there is no KaTeX (a page without it, or a
//! native build, where every call falls back) or it rejects the text.
//! Nothing of KaTeX is redistributed here.

/// The HTML KaTeX draws for `tex`; the LaTeX, escaped, without KaTeX.
pub fn html(tex: &str) -> String {
    typeset(tex).unwrap_or_else(|| escaped(tex))
}

/// `text` safe inside HTML text or a quoted attribute.
pub fn escaped(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = katex, js_name = renderToString, catch)]
    fn render_to_string(
        tex: &str,
        options: &wasm_bindgen::JsValue,
    ) -> Result<String, wasm_bindgen::JsValue>;
}

#[cfg(target_arch = "wasm32")]
fn typeset(tex: &str) -> Option<String> {
    let options = js_sys::Object::new();
    let _ = js_sys::Reflect::set(&options, &"throwOnError".into(), &false.into());
    render_to_string(tex, &options).ok()
}

#[cfg(not(target_arch = "wasm32"))]
fn typeset(_tex: &str) -> Option<String> {
    None
}
