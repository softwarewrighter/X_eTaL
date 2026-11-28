//! The pane's drawing: the output (on the grid when the program placed
//! or styled its text), the line being typed, the errors and the
//! pictures, a click on a picture sent as `click X Y` while the program
//! waits for an event.

use wasm_bindgen::JsCast;
use web_sys::{HtmlImageElement, MouseEvent};
use xetal_runner::Runs;
use xetal_typing::{Typing, screen, typed_line};
use yew::prelude::*;

/// A picture a run showed (`[]S_HOW`), as an image: an SVG data URL,
/// so its animation plays and nothing in it runs as script. With
/// `click`, a click on it is sent as `click X Y` in the picture's own
/// pixels.
pub fn picture(svg: &str, click: Option<Callback<String>>) -> Html {
    let url = format!(
        "data:image/svg+xml;charset=utf-8,{}",
        js_sys::encode_uri_component(svg)
    );
    let onclick = click.map(|send| {
        Callback::from(move |e: MouseEvent| {
            if let Some((x, y)) = at(&e) {
                send.emit(format!("click {x:.1} {y:.1}"));
            }
        })
    });
    html! {
        <span class="picture">
            <img src={url} {onclick} alt="a picture the program showed"/>
        </span>
    }
}

/// Where a click fell, in the image's own pixels (its natural size;
/// an SVG's width and height) rather than the page's.
fn at(e: &MouseEvent) -> Option<(f64, f64)> {
    let img: HtmlImageElement = e.current_target()?.dyn_into().ok()?;
    let rect = img.get_bounding_client_rect();
    if rect.width() <= 0.0 || rect.height() <= 0.0 {
        return None;
    }
    let (w, h) = (
        f64::from(img.natural_width()),
        f64::from(img.natural_height()),
    );
    let (w, h) = if w > 0.0 && h > 0.0 {
        (w, h)
    } else {
        (rect.width(), rect.height())
    };
    let x = (f64::from(e.client_x()) - rect.left()) * w / rect.width();
    let y = (f64::from(e.client_y()) - rect.top()) * h / rect.height();
    Some((x, y))
}

/// What a plain run shows: its output (the grid when it placed or
/// styled text), the line being typed while it waits for one, its
/// errors and its pictures, clickable while it waits for an event.
pub fn body(runs: &Runs, typing: &Typing) -> Html {
    let o = &runs.output;
    let Some(r) = &o.run else {
        return html! {};
    };
    let input = (o.waiting && !o.wants_key).then(|| typed_line(typing));
    let click = o.wants_event.then(|| runs.press_key.clone());
    let out = screen(&r.out).unwrap_or_else(|| html! { { &r.out } });
    html! {
        <>
            { out }{ input }<span class="c-error">{ &r.err }</span>
            { for r.pictures.iter().map(|svg| picture(svg, click.clone())) }
        </>
    }
}
