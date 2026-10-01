//! The editor's toolbar: the logo, Open, the file's name, Save, Save
//! as, Clear, Run (Stop while a run goes on), Zoom and Help.

use web_sys::HtmlSelectElement;
use yew::prelude::*;

/// What the toolbar's controls do.
pub struct Bar {
    pub load: Callback<(String, String)>,
    pub save: Callback<bool>,
    pub run: Callback<()>,
    pub zoom: Callback<()>,
    pub help: Callback<bool>,
    /// Stop any run and show the types again.
    pub clear: Callback<()>,
    /// A library is checked, not run: Run is off.
    pub library: bool,
    /// A run is going: Run is Stop.
    pub running: bool,
    /// What Open offers, as (group, value, label), and how a value opens.
    pub options: Vec<(&'static str, String, String)>,
    pub open: fn(&str) -> Option<(String, String)>,
    /// The file's name, and whether a pane is zoomed.
    pub name: String,
    pub zoomed: bool,
}

/// The logo, Open (demos, libraries, your files), the file's name,
/// Save, Save as, Clear, Run, Zoom and Help.
pub fn toolbar(bar: Bar) -> Html {
    let (load, open) = (bar.load.clone(), bar.open);
    let pick = Callback::from(move |e: Event| {
        let select: HtmlSelectElement = e.target_unchecked_into();
        if let Some(opened) = open(&select.value()) {
            load.emit(opened);
        }
    });
    let options = bar.options.iter().map(|(group, value, label)| {
        html! { <option value={value.clone()} data-group={*group} selected={value == "demo:0"}>{ format!("{group}: {label}") }</option> }
    });
    html! {
        <nav class="toolbar">
            <img class="logo" src="modern-xetal-logo.jpg" alt="X_eTaL"/>
            <select onchange={pick} title="Open a demo, a library or one of your files">{ for options }</select>
            <span class="name" title="The file being edited">{ &bar.name }</span>
            <button onclick={bar.save.reform(|_| false)} title="Save in this browser">{ "Save" }</button>
            <button onclick={bar.save.reform(|_| true)} title="Save under another name">{ "Save as" }</button>
            <button onclick={clearing(&bar)} title="Stop any run, and an empty editor">{ "Clear" }</button>
            <button class={classes!(bar.running.then_some("stop"))} onclick={bar.run.reform(|_| ())}
                disabled={bar.library} title={run_title(&bar)}>{ if bar.running { "Stop" } else { "Run" } }</button>
            <button onclick={bar.zoom.reform(|_| ())} title="Zoom the current pane (Ctrl-.)">
                { if bar.zoomed { "Unzoom" } else { "Zoom" } }
            </button>
            <button class="help" onclick={bar.help.reform(|_| true)} title="How it works">{ "Help" }</button>
        </nav>
    }
}

/// Clear: stop any run, then an empty editor.
fn clearing(bar: &Bar) -> Callback<MouseEvent> {
    let (clear, load) = (bar.clear.clone(), bar.load.clone());
    Callback::from(move |_| {
        clear.emit(());
        load.emit(("untitled.xtl".to_string(), String::new()));
    })
}

fn run_title(bar: &Bar) -> &'static str {
    match (bar.library, bar.running) {
        (true, _) => "A library is not run: its exports' types are below",
        (false, true) => "Stop the run (Ctrl-Enter)",
        (false, false) => "Run (Ctrl-Enter)",
    }
}
