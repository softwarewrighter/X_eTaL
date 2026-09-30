//! The editor's state and what changes it.

use web_sys::HtmlSelectElement;
use xetal_play::{Run, run};
use yew::prelude::*;

use crate::keys::{Action, action};
use crate::{DEMOS, panes};

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
    let zoom = use_state(|| false);
    let result = use_state(|| None::<Run>);
    let (t, r) = (text.clone(), result.clone());
    let run_now = Callback::from(move |_: ()| r.set(Some(run(&t, seed()))));
    let z = zoom.clone();
    let toggle = Callback::from(move |_: ()| z.set(!*z));
    let (rn, tg) = (run_now.clone(), toggle.clone());
    let on_key = Callback::from(move |e: KeyboardEvent| {
        let pressed = action(&e.key(), e.ctrl_key() || e.meta_key());
        if let Some(act) = pressed {
            e.prevent_default();
            match act {
                Action::Run => rn.emit(()),
                Action::Zoom => tg.emit(()),
            }
        }
    });
    let (t, r) = (text.clone(), result.clone());
    let edit = Callback::from(move |v: String| {
        t.set(v);
        r.set(None);
    });
    let c = current.clone();
    let focus = Callback::from(move |p: Pane| c.set(p));
    let drawn = use_node_ref();
    html! {
        <div class="app" onkeydown={on_key}>
            { toolbar(edit.clone(), run_now, toggle, *zoom) }
            <main class={classes!("panes", zoom.then_some("zoomed"))}>
                { panes::source(&text, *current, focus.clone(), edit, drawn.clone()) }
                { panes::rendered(&text, *current, focus.clone(), drawn) }
                { panes::output(&text, &result, *current, focus) }
            </main>
        </div>
    }
}

/// The drop-down of programs, Clear, Run and Zoom.
fn toolbar(load: Callback<String>, run: Callback<()>, zoom: Callback<()>, zoomed: bool) -> Html {
    let clear = load.reform(|_: MouseEvent| String::new());
    let pick = Callback::from(move |e: Event| {
        let select: HtmlSelectElement = e.target_unchecked_into();
        let i = usize::try_from(select.selected_index()).unwrap_or(0);
        load.emit(DEMOS.get(i).unwrap_or(&DEMOS[0]).text.to_string());
    });
    html! {
        <nav class="toolbar">
            <span class="brand">{ "X_eTaL" }</span>
            <select onchange={pick} title="Load a program">
                { for DEMOS.iter().enumerate().map(|(i, d)| html! {
                    <option selected={i == 0}>{ d.name }</option>
                }) }
            </select>
            <button onclick={clear} title="Clear the editor and its output">{ "Clear" }</button>
            <button onclick={run.reform(|_| ())} title="Run (Ctrl-Enter)">{ "Run" }</button>
            <button onclick={zoom.reform(|_| ())} title="Zoom the current pane (Ctrl-.)">
                { if zoomed { "Unzoom" } else { "Zoom" } }
            </button>
        </nav>
    }
}

/// A seed for `r_oll!`, different on every run.
fn seed() -> u64 {
    (js_sys::Math::random() * 4_294_967_296.0) as u64
}
