//! The page: the stone's latest frame, the controls beneath it, and
//! the plumbing between the worker and the page's events.

use xetal_runner::{Runs, use_runs};
use yew::prelude::*;

use crate::events::{pointer, use_keys};
use crate::follow::{Selects, chosen, read_address, use_follow, write_address};
use crate::program::{Named, lists, mode, position, request};
use crate::zoom::zoom;

/// The idiom and language lists, read once.
type Lists = (Named, Named);

#[function_component(App)]
pub fn app() -> Html {
    let runs = use_runs();
    let lists: UseStateHandle<Lists> = use_state(lists);
    let scale = use_state(|| 1.0_f64);
    let selects: Selects = [NodeRef::default(), NodeRef::default(), NodeRef::default()];
    let send = runs.press_key.clone();
    use_start(&runs, &lists);
    use_keys(send.clone(), runs.output.running);
    xetal_typing::use_ticks(runs.output.wants_event, send.clone());
    let out = runs
        .output
        .run
        .as_ref()
        .map(|r| r.out.clone())
        .unwrap_or_default();
    let at = position(&out);
    let tour = mode(&out);
    if let Some(at) = &at {
        write_address(at);
    }
    use_follow(at.clone(), selects.clone());
    let frame = runs.output.run.as_ref().and_then(|r| r.frame.clone());
    let (down, moved, up) = pointer(send.clone());
    let picture = Html::from_html_unchecked(AttrValue::from(frame.unwrap_or_default()));
    let err = runs
        .output
        .run
        .as_ref()
        .map(|r| r.err.clone())
        .unwrap_or_default();
    html! {
        <>
        <main class="rosetta">
            <h1>{ "The Rosetta stone" }</h1>
            <p class="lead">{ "How languages write the same idiom. Drag the top half or the bottom half sideways to turn it; drag up or down to roll to another idiom; click a half to pause it. Left alone, it tours." }</p>
            <div class="stone" style={format!("--scale: {}", *scale)} onpointerdown={down} onpointermove={moved} onpointerup={up}>
                { picture }
            </div>
            { badge(tour.as_deref()) }
            { controls(&lists, at.as_ref(), &selects, send) }
            { zoom(scale) }
            if !err.is_empty() { <pre class="err">{ err }</pre> }
            <p class="foot">{ "An X_eTaL program (demos/rosetta/rosetta.xtl) drawing SVG; Rust and Yew only carry the events. " }<a href="https://github.com/softwarewrighter/X_eTaL/blob/main/docs/rosetta.md">{ "How it works" }</a></p>
        </main>
        { xetal_chrome::footer_at("../") }
        </>
    }
}

/// Start the program once; the address bar's choices (and, with
/// reduced motion asked for, the space bar, which pauses the tour) go to it the first time
/// it waits for an event, since a line sent before the worker has the
/// program is lost. The address is read at once, before the program's
/// first position replaces it.
#[hook]
fn use_start(runs: &Runs, lists: &UseStateHandle<Lists>) {
    let started = use_mut_ref(|| false);
    let pending = use_mut_ref(|| opening(lists));
    let (start, send) = (runs.start.clone(), runs.press_key.clone());
    use_effect(move || {
        if !*started.borrow() {
            *started.borrow_mut() = true;
            start.emit(request());
        }
    });
    use_effect_with(runs.output.wants_event, move |wants| {
        if *wants {
            for line in pending.borrow_mut().drain(..) {
                send.emit(line);
            }
        }
    });
}

/// The lines sent as the page opens: the address bar's choices, and
/// the pauses when the viewer prefers reduced motion.
fn opening(lists: &Lists) -> Vec<String> {
    let mut lines = Vec::new();
    let [idiom, top, bottom] = read_address();
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
    // A choice pauses its axis (choosing is navigating); the tour goes on.
    lines.push("key r".to_string());
    let reduced = web_sys::window()
        .and_then(|w| {
            w.match_media("(prefers-reduced-motion: reduce)")
                .ok()
                .flatten()
        })
        .is_some_and(|m| m.matches());
    if reduced {
        lines.push("key  ".to_string());
    }
    lines
}

/// The tour's mode under the stone, in the program's words: touring,
/// holding (after a touch), or paused with the axes paused; and what
/// to press.
fn badge(mode: Option<&str>) -> Html {
    let Some(mode) = mode else {
        return html! { <p class="mode">{ "Starting" }</p> };
    };
    let hint = if mode.starts_with("paused") {
        " (space resumes)"
    } else if mode == "holding" {
        " after your move; the tour goes on in a moment (space pauses)"
    } else {
        " (space pauses)"
    };
    let class = if mode.starts_with("paused") {
        "mode paused"
    } else {
        "mode"
    };
    html! { <p class={class}><span class="dot"></span>{ format!("{}{hint}", capitalized(mode)) }</p> }
}

/// A word with its first letter capitalized.
fn capitalized(s: &str) -> String {
    let mut c = s.chars();
    c.next()
        .map(|f| f.to_uppercase().collect::<String>() + c.as_str())
        .unwrap_or_default()
}

/// The controls: the idiom, the top language and the bottom language,
/// each a list; a choice is sent to the program as `choose AXIS ITEM`,
/// and the lists follow the program's position (`use_follow`). The
/// language the other half shows is offered disabled.
fn controls(
    lists: &Lists,
    at: Option<&[String; 3]>,
    selects: &Selects,
    send: Callback<String>,
) -> Html {
    let select = |axis: usize, label: &str, items: &Named, disabled: Option<&String>| {
        html! {
            <label>{ label }
                <select ref={selects[axis - 1].clone()} onchange={chosen(axis, send.clone())}>
                    { for items.iter().map(|(key, name)| html! {
                        <option value={key.clone()} disabled={Some(key) == disabled}>{ name }</option>
                    }) }
                </select>
            </label>
        }
    };
    let (top, bottom) = match at {
        Some([_, t, b]) => (Some(t), Some(b)),
        None => (None, None),
    };
    html! {
        <div class="controls">
            { select(1, "Idiom", &lists.0, None) }
            { select(2, "Top", &lists.1, bottom) }
            { select(3, "vs.", &lists.1, top) }
            <span class="keys">{ "Keys: arrows roll and turn the top; a/d turn the bottom; space pauses and resumes the tour, r resumes it (s the bottom, w the roll)." }</span>
        </div>
    }
}
