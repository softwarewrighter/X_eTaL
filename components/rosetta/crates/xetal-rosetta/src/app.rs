//! The page: the stone's latest frame, the controls beneath it, and
//! the plumbing between the worker and the page's events.

use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use xetal_runner::{Runs, use_runs};
use yew::prelude::*;

use crate::events::{keys, pointer};
use crate::program::{Named, lists, position, request};
use crate::url;

/// The idiom and language lists, read once.
type Lists = (Named, Named);

#[function_component(App)]
pub fn app() -> Html {
    let runs = use_runs();
    let lists: UseStateHandle<Lists> = use_state(lists);
    let send = runs.press_key.clone();
    use_start(&runs, &lists);
    xetal_typing::use_ticks(runs.output.wants_event, send.clone());
    let out = runs
        .output
        .run
        .as_ref()
        .map(|r| r.out.clone())
        .unwrap_or_default();
    let at = position(&out);
    if let Some(at) = &at {
        url::write(at);
    }
    let frame = runs.output.run.as_ref().and_then(|r| r.frame.clone());
    let (down, moved, up) = pointer(send.clone());
    let on_key = keys(send.clone(), runs.output.running);
    let picture = Html::from_html_unchecked(AttrValue::from(frame.unwrap_or_default()));
    let err = runs
        .output
        .run
        .as_ref()
        .map(|r| r.err.clone())
        .unwrap_or_default();
    html! {
        <main class="rosetta" tabindex="0" onkeydown={on_key}>
            <h1>{ "The Rosetta stone" }</h1>
            <p class="lead">{ "How languages write the same idiom. Drag the top half or the bottom half sideways to turn it; drag up or down to roll to another idiom; click a half to pause it. Left alone, it tours." }</p>
            <div class="stone" onpointerdown={down} onpointermove={moved} onpointerup={up}>
                { picture }
            </div>
            { controls(&lists, at.as_ref(), send) }
            if !err.is_empty() { <pre class="err">{ err }</pre> }
            <p class="foot">{ "An X_eTaL program (demos/rosetta/rosetta.xtl) drawing SVG; Rust and Yew only carry the events. " }<a href="../">{ "The live editor" }</a>{ " · " }<a href="https://github.com/softwarewrighter/X_eTaL/blob/main/docs/rosetta.md">{ "How it works" }</a></p>
        </main>
    }
}

/// Start the program once, then send the address bar's choices and,
/// with reduced motion asked for, pause the three axes.
#[hook]
fn use_start(runs: &Runs, lists: &UseStateHandle<Lists>) {
    let started = use_mut_ref(|| false);
    let (start, send, lists) = (runs.start.clone(), runs.press_key.clone(), lists.clone());
    use_effect(move || {
        if !*started.borrow() {
            *started.borrow_mut() = true;
            start.emit(request());
            for line in opening(&lists) {
                send.emit(line);
            }
        }
    });
}

/// The lines sent as the page opens: the address bar's choices, and
/// the pauses when the viewer prefers reduced motion.
fn opening(lists: &Lists) -> Vec<String> {
    let mut lines = Vec::new();
    let [idiom, top, bottom] = url::read();
    let index = |list: &[(String, String)], key: &Option<String>| {
        key.as_ref()
            .and_then(|k| list.iter().position(|(key, _)| key == k))
            .map(|i| i + 1)
    };
    for (axis, item) in [
        (1, index(&lists.0, &idiom)),
        (2, index(&lists.1, &top)),
        (3, index(&lists.1, &bottom)),
    ] {
        if let Some(item) = item {
            lines.push(format!("choose {axis} {item}"));
        }
    }
    let reduced = web_sys::window()
        .and_then(|w| {
            w.match_media("(prefers-reduced-motion: reduce)")
                .ok()
                .flatten()
        })
        .is_some_and(|m| m.matches());
    if reduced {
        lines.extend(["key w", "key  ", "key s"].map(String::from));
    }
    lines
}

/// The controls: the idiom, the top language and the bottom language,
/// each a list; a choice is sent to the program as `choose AXIS ITEM`.
fn controls(lists: &Lists, at: Option<&[String; 3]>, send: Callback<String>) -> Html {
    let select = |axis: usize,
                  label: &str,
                  items: &Named,
                  current: Option<&String>,
                  disabled: Option<&String>| {
        let s = send.clone();
        let onchange = Callback::from(move |e: Event| {
            let Some(target) = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlSelectElement>().ok())
            else {
                return;
            };
            s.emit(format!("choose {axis} {}", target.selected_index() + 1));
        });
        html! {
            <label>{ label }
                <select {onchange}>
                    { for items.iter().map(|(key, name)| html! {
                        <option value={key.clone()} selected={Some(key) == current} disabled={Some(key) == disabled}>{ name }</option>
                    }) }
                </select>
            </label>
        }
    };
    let (idiom, top, bottom) = match at {
        Some([i, t, b]) => (Some(i), Some(t), Some(b)),
        None => (None, None, None),
    };
    html! {
        <div class="controls">
            { select(1, "Idiom", &lists.0, idiom, None) }
            { select(2, "Top", &lists.1, top, bottom) }
            { select(3, "vs.", &lists.1, bottom, top) }
            <span class="keys">{ "Keys: arrows roll and turn the top; a/d turn the bottom; space, s, w pause." }</span>
        </div>
    }
}
