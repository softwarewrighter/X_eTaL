//! The editor's state and what changes it.

use web_sys::HtmlSelectElement;
use xetal_layout::{Axis, divider, use_split};
use xetal_play::{Run, is_library, run};
use yew::prelude::*;

use crate::keys::keys;
use crate::{DEMOS, choices, open, panes, storage};
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
    let name = use_state(|| DEMOS[0].name.to_string());
    let files = use_state(storage::saved);
    let current = use_state(|| Pane::Source);
    let (zoom, help) = (use_state(|| false), use_state(|| false));
    let result = use_state(|| None::<Run>);
    let split = use_split();
    let (drawn, printed) = (use_node_ref(), use_node_ref());
    use_follow(&result, printed.clone());
    let library = is_library(&text);
    let (t, r) = (text.clone(), result.clone());
    let run_now = Callback::from(move |_: ()| {
        if !library {
            r.set(Some(run(&t, seed())));
        }
    });
    let z = zoom.clone();
    let toggle = Callback::from(move |_: ()| z.set(!*z));
    let h = help.clone();
    let show_help = Callback::from(move |open: bool| h.set(open));
    let on_key = keys(run_now.clone(), toggle.clone(), show_help.clone());
    let (edit, load) = editing(&text, &name, &result);
    let c = current.clone();
    let focus = Callback::from(move |p: Pane| c.set(p));
    let save = saving(text.clone(), name.clone(), files.clone());
    let bar = Bar {
        load,
        save,
        run: run_now,
        zoom: toggle,
        help: show_help.clone(),
        library,
    };
    html! {
        <div class="app" onkeydown={on_key}>
            { toolbar(bar, &files, &name, *zoom) }
            <main class={classes!("panes", zoom.then_some("zoomed"))} style={split.style()}>
                { panes::source(&text, *current, focus.clone(), edit, drawn.clone()) }
                { divider(Axis::Columns, &split) }
                { panes::rendered(&text, *current, focus.clone(), drawn) }
                { divider(Axis::Rows, &split) }
                { panes::output(&text, &result, *current, focus, printed) }
            </main>
            { chrome::footer() }
            { if *help { chrome::help(show_help.reform(|_| false)) } else { html! {} } }
        </div>
    }
}

/// Keep the output pane scrolled to its end as output arrives.
#[hook]
fn use_follow(result: &Option<Run>, pane: NodeRef) {
    use_effect_with(result.clone(), move |_| {
        if let Some(pane) = pane.cast::<web_sys::Element>() {
            pane.set_scroll_top(pane.scroll_height());
        }
    });
}

/// Editing the text (which clears a run's output), and loading a file
/// (its name and text) into the editor.
fn editing(
    text: &UseStateHandle<String>,
    name: &UseStateHandle<String>,
    result: &UseStateHandle<Option<Run>>,
) -> (Callback<String>, Callback<(String, String)>) {
    let (t, r) = (text.clone(), result.clone());
    let edit = Callback::from(move |v: String| {
        t.set(v);
        r.set(None);
    });
    let (e, n) = (edit.clone(), name.clone());
    let load = Callback::from(move |(file, body): (String, String)| {
        n.set(file);
        e.emit(body);
    });
    (edit, load)
}

/// Save the text under its name, or (`true`) under a name asked for.
fn saving(
    text: UseStateHandle<String>,
    name: UseStateHandle<String>,
    files: UseStateHandle<Vec<String>>,
) -> Callback<bool> {
    Callback::from(move |ask: bool| {
        let chosen = match ask {
            true => web_sys::window().and_then(|w| {
                w.prompt_with_message_and_default("Save as", &name)
                    .ok()
                    .flatten()
            }),
            false => Some((*name).clone()),
        };
        let Some(path) = chosen.filter(|p| !p.trim().is_empty()) else {
            return;
        };
        if xetal_store::write(path.trim(), &text).is_ok() {
            name.set(path.trim().to_string());
            files.set(storage::saved());
        }
    })
}

/// What the toolbar's controls do.
struct Bar {
    load: Callback<(String, String)>,
    save: Callback<bool>,
    run: Callback<()>,
    zoom: Callback<()>,
    help: Callback<bool>,
    /// A library is checked, not run: Run is off.
    library: bool,
}

/// The logo, Open (demos, libraries, your files), the file's name,
/// Save, Save as, Clear, Run, Zoom and Help.
fn toolbar(bar: Bar, files: &[String], name: &str, zoomed: bool) -> Html {
    let load = bar.load.clone();
    let pick = Callback::from(move |e: Event| {
        let select: HtmlSelectElement = e.target_unchecked_into();
        if let Some(opened) = open(&select.value()) {
            load.emit(opened);
        }
    });
    let options = choices(files).into_iter().map(|(group, value, label)| {
        html! { <option value={value.clone()} data-group={group} selected={value == "demo:0"}>{ format!("{group}: {label}") }</option> }
    });
    html! {
        <nav class="toolbar">
            <img class="logo" src="modern-xetal-logo.jpg" alt="X_eTaL"/>
            <select onchange={pick} title="Open a demo, a library or one of your files">{ for options }</select>
            <span class="name" title="The file being edited">{ name }</span>
            <button onclick={bar.save.reform(|_| false)} title="Save in this browser">{ "Save" }</button>
            <button onclick={bar.save.reform(|_| true)} title="Save under another name">{ "Save as" }</button>
            <button onclick={bar.load.reform(|_| ("untitled.xtl".to_string(), String::new()))} title="An empty editor">{ "Clear" }</button>
            <button onclick={bar.run.reform(|_| ())} disabled={bar.library}
                title={if bar.library { "A library is not run: its exports' types are below" } else { "Run (Ctrl-Enter)" }}>{ "Run" }</button>
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
