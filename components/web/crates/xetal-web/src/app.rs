//! The editor's state and what changes it.

use xetal_layout::{Axis, divider, use_split};
use xetal_play::{Report, Run, is_library, statements};
use xetal_runner::{Mode, Runs, use_runs};
use yew::prelude::*;

use crate::keys::keys;
use crate::{DEMOS, choices, demos, open, panes, storage};
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
    let runs = use_runs();
    let lint = use_state(|| None::<Report>);
    let (result, running) = (runs.output.run.clone(), runs.output.running);
    let split = use_split();
    let (drawn, printed) = (use_node_ref(), use_node_ref());
    use_follow(&result, printed.clone());
    let buttons = run_buttons(&text, &files, &runs, &lint);
    let (z, h) = (zoom.clone(), help.clone());
    let toggle = Callback::from(move |_: ()| z.set(!*z));
    let show_help = Callback::from(move |open: bool| h.set(open));
    let page_keys = keys(buttons.run.clone(), toggle.clone(), show_help.clone());
    let c = current.clone();
    let focus = Callback::from(move |p: Pane| c.set(p));
    let in_source = *current == Pane::Source;
    let terminal = (&runs, in_source, focus.reform(|_| Pane::Output));
    let (typing, on_key) = xetal_typing::use_terminal(terminal, page_keys, printed.clone());
    let (edit, load) = editing(&text, &name, (&runs.clear, &lint), running);
    let save = saving(text.clone(), name.clone(), files.clone());
    let bar = chrome::Bar {
        load,
        save,
        runs: buttons,
        zoom: toggle,
        help: show_help.clone(),
        options: choices(&files),
        open,
        name: (*name).clone(),
        zoomed: *zoom,
    };
    html! {
        <div class="app" onkeydown={on_key}>
            { chrome::toolbar(bar) }
            <main class={classes!("panes", zoom.then_some("zoomed"), running.then_some("running"))} style={split.style()}>
                { panes::source(&text, *current, focus.clone(), edit, drawn.clone()) }
                { divider(Axis::Columns, &split) }
                { panes::rendered(&text, *current, focus.clone(), drawn) }
                { divider(Axis::Rows, &split) }
                { panes::output(&text, &runs, (*lint).as_ref(), *current, focus, printed, &typing) }
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

/// Editing the text (which clears a finished run's output and the lint
/// report; a running one goes on), and loading a file (its name and
/// text) into the editor.
fn editing(
    text: &UseStateHandle<String>,
    name: &UseStateHandle<String>,
    (clear, lint): (&Callback<()>, &UseStateHandle<Option<Report>>),
    running: bool,
) -> (Callback<String>, Callback<(String, String)>) {
    let (t, c, l) = (text.clone(), clear.clone(), lint.clone());
    let edit = Callback::from(move |v: String| {
        t.set(v);
        l.set(None);
        if !running {
            c.emit(());
        }
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

/// Lint: the text's findings in the output pane, any run's output
/// cleared; while they carry fixes the button is Fix, and a click makes
/// them in the text and lints again. Gives the callback and how many.
fn linting(
    text: &UseStateHandle<String>,
    runs: &Runs,
    lint: &UseStateHandle<Option<Report>>,
) -> (Callback<()>, usize) {
    let fixed = (**lint).as_ref().and_then(|r| r.fixed.clone());
    let fixable = (**lint).as_ref().map_or(0, |r| r.fixable);
    let (t, l, clear) = (text.clone(), lint.clone(), runs.clear.clone());
    let click = Callback::from(move |_: ()| {
        clear.emit(());
        let src = fixed.clone().unwrap_or_else(|| (*t).clone());
        if fixed.is_some() {
            t.set(src.clone());
        }
        l.set(Some(xetal_play::lint(&src)));
    });
    (click, fixable)
}

/// The run buttons: Run (or Stop), Lint (or Fix), Notebook, Step and
/// Reset, for the text and the saved files; off for a library.
fn run_buttons(
    text: &UseStateHandle<String>,
    files: &UseStateHandle<Vec<String>>,
    runs: &Runs,
    lint: &UseStateHandle<Option<Report>>,
) -> chrome::RunButtons {
    let (lint_click, fixable) = linting(text, runs, lint);
    let (library, running) = (is_library(text), runs.output.running);
    let (t, f, start, stop) = (
        text.clone(),
        files.clone(),
        runs.start.clone(),
        runs.stop.clone(),
    );
    let run = Callback::from(move |_: ()| match (running, library) {
        (true, _) => stop.emit(()),
        (false, true) => {}
        (false, false) => start.emit(demos::request(&t, &f, Mode::Run)),
    });
    let (t, f, start) = (text.clone(), files.clone(), runs.start.clone());
    let notebook =
        Callback::from(move |_: ()| start.emit(demos::request(&t, &f, Mode::Notebook(None))));
    let (t, f, step) = (text.clone(), files.clone(), runs.step.clone());
    let step = Callback::from(move |_: ()| step.emit(demos::request(&t, &f, Mode::Notebook(None))));
    let (c, l, toggle_boxed) = (runs.clear.clone(), lint.clone(), runs.toggle_boxed.clone());
    let clear = Callback::from(move |_: ()| {
        c.emit(());
        l.set(None);
    });
    let (boxed, stepped, statements) = (runs.boxed, runs.stepped, statements(text));
    chrome::RunButtons {
        run,
        lint: lint_click,
        fixable,
        notebook,
        step,
        clear,
        toggle_boxed,
        library,
        running,
        boxed,
        stepped,
        statements,
    }
}
