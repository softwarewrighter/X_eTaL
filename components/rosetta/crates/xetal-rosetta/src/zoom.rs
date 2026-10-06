//! How big the stone is on the screen: a scale the viewer sets with
//! two buttons (the picture is vector, so the faces grow with it), and
//! full screen for the stone alone. The picture's own size never
//! changes (420 by 420 of its own pixels; the pointer is scaled back
//! to them), so the program knows nothing of this.

use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;

/// The scales offered, smallest to largest; the page opens at 1.
const STEPS: [f64; 7] = [0.5, 0.75, 1.0, 1.25, 1.5, 2.0, 2.5];

/// The next scale up or down from `scale` (the ends stay put).
fn step(scale: f64, up: bool) -> f64 {
    let i = STEPS
        .iter()
        .position(|s| (s - scale).abs() < 1e-9)
        .unwrap_or(2);
    let j = if up { i + 1 } else { i.saturating_sub(1) };
    STEPS[j.min(STEPS.len() - 1)]
}

/// The zoom buttons and the full-screen button; `scale` is the page's.
pub fn zoom(scale: UseStateHandle<f64>) -> Html {
    let (out, back) = (scale.clone(), scale.clone());
    let smaller = Callback::from(move |_| out.set(step(*out, false)));
    let larger = Callback::from(move |_| back.set(step(*back, true)));
    let full = Callback::from(|_| fullscreen());
    html! {
        <span class="zoom">
            <button type="button" onclick={smaller} title="Smaller" aria-label="Smaller">{ "\u{2212}" }</button>
            <span class="scale">{ format!("{:.0}%", *scale * 100.0) }</span>
            <button type="button" onclick={larger} title="Larger" aria-label="Larger">{ "+" }</button>
            <button type="button" onclick={full} title="The stone alone, full screen">{ "Full screen" }</button>
        </span>
    }
}

/// The stone's pane full screen (the browser's own key leaves it).
fn fullscreen() {
    let Some(stone) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.query_selector(".stone").ok().flatten())
        .and_then(|e| e.dyn_into::<HtmlElement>().ok())
    else {
        return;
    };
    let _ = stone.request_fullscreen();
}
