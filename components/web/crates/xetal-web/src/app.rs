//! The editor's state and what changes it.

use web_sys::HtmlSelectElement;
use xetal_play::{Run, run};
use yew::prelude::*;

use crate::keys::{Action, action};
use crate::{DEMOS, panes};
use xetal_chrome as chrome;

/// A pane of the editor, as in `xetal edit`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pane {
    Source,
    Rendered,
    Output,
}

impl Pane {
    pub(crate) fn css(self) -> &'static str {
        match self {
            Pane::Source => "source",
            Pane::Rendered => "rendered",
            Pane::Output => "output",
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let text = use_state(|| DEMOS[0].text.to_string());
    let current = use_state(|| Pane::Source);
    let (zoom, help) = (use_state(|| false), use_state(|| false));
    let result = use_state(|| None::<Run>);
    let (drawn, printed) = (use_node_ref(), use_node_ref());
    // After a run, show the end of the output, where the newest is.
    let end = printed.clone();
    use_effect_with((*result).clone(), move |_| {
        if let Some(pane) = end.cast::<web_sys::Element>() {
            pane.set_scroll_top(pane.scroll_height());
        }
    });
    let (t, r) = (text.clone(), result.clone());
    let run_now = Callback::from(move |_: ()| r.set(Some(run(&t, seed()))));
    let z = zoom.clone();
    let toggle = Callback::from(move |_: ()| z.set(!*z));
    let h = help.clone();
    let show_help = Callback::from(move |open: bool| h.set(open));
    let on_key = keys(run_now.clone(), toggle.clone(), show_help.clone());
    let (t, r) = (text.clone(), result.clone());
    let edit = Callback::from(move |v: String| {
        t.set(v);
        r.set(None);
    });
    let c = current.clone();
    let focus = Callback::from(move |p: Pane| c.set(p));
    let bar = Bar {
        load: edit.clone(),
        run: run_now,
        zoom: toggle,
        help: show_help.clone(),
    };
    html! {
        <div class="app" onkeydown={on_key}>
            { toolbar(bar, *zoom) }
            <main class={classes!("panes", zoom.then_some("zoomed"))}>
                { panes::source(&text, *current, focus.clone(), edit, drawn.clone()) }
                { panes::rendered(&text, *current, focus.clone(), drawn) }
                { panes::output(&text, &result, *current, focus, printed) }
            </main>
            { chrome::footer() }
            { if *help { chrome::help(show_help.reform(|_| false)) } else { html! {} } }
        </div>
    }
}

/// Keys for the whole page: Run, Zoom, and Escape to close Help.
fn keys(run: Callback<()>, zoom: Callback<()>, help: Callback<bool>) -> Callback<KeyboardEvent> {
    Callback::from(move |e: KeyboardEvent| {
        if e.key() == "Escape" {
            help.emit(false);
        }
        if let Some(act) = action(&e.key(), e.ctrl_key() || e.meta_key()) {
            e.prevent_default();
            match act {
                Action::Run => run.emit(()),
                Action::Zoom => zoom.emit(()),
            }
        }
    })
}

/// What the toolbar's controls do.
struct Bar {
    load: Callback<String>,
    run: Callback<()>,
    zoom: Callback<()>,
    help: Callback<bool>,
}

/// The logo, the drop-down of programs, Clear, Run, Zoom and Help.
fn toolbar(bar: Bar, zoomed: bool) -> Html {
    let load = bar.load.clone();
    let pick = Callback::from(move |e: Event| {
        let select: HtmlSelectElement = e.target_unchecked_into();
        let i = usize::try_from(select.selected_index()).unwrap_or(0);
        load.emit(DEMOS.get(i).unwrap_or(&DEMOS[0]).text.to_string());
    });
    html! {
        <nav class="toolbar">
            <img class="logo" src="modern-xetal-logo.jpg" alt="X_eTaL"/>
            <select onchange={pick} title="Load a program">
                { for DEMOS.iter().enumerate().map(|(i, d)| html! {
                    <option selected={i == 0}>{ d.name }</option>
                }) }
            </select>
            <button onclick={bar.load.reform(|_| String::new())} title="Clear the editor">{ "Clear" }</button>
            <button onclick={bar.run.reform(|_| ())} title="Run (Ctrl-Enter)">{ "Run" }</button>
            <button onclick={bar.zoom.reform(|_| ())} title="Zoom the current pane (Ctrl-.)">
                { if zoomed { "Unzoom" } else { "Zoom" } }
            </button>
            <button class="help" onclick={bar.help.reform(|_| true)} title="How it works">{ "Help" }</button>
        </nav>
    }
}

/// A seed for `r_oll!`, different on every run.
fn seed() -> u64 {
    (js_sys::Math::random() * 4_294_967_296.0) as u64
}
