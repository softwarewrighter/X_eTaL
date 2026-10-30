//! The editor's state and what changes it.

use xetal_layout::{Axis, divider, use_split};
use xetal_play::{Report, is_library, statements};
use xetal_runner::{Mode, Runs, use_runs};
use yew::prelude::*;

use crate::demos::{saving, use_follow};
use crate::keys::{keys, wants_terminal};
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
    let (files, current) = (use_state(storage::saved), use_state(|| Pane::Source));
    let (zoom, help) = (use_state(|| false), use_state(|| false));
    let (tui, leave, opened) = (
        use_state(wants_terminal),
        use_state(|| 0u32),
        use_state(|| (0u32, String::new())),
    );
    let (runs, lint) = (use_runs(), use_state(|| None::<Report>));
    let running = runs.output.running;
    let split = use_split();
    let (drawn, printed, area) = (use_node_ref(), use_node_ref(), use_node_ref());
    use_follow(&runs.output.run, printed.clone());
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
    let suggest = xetal_suggest::use_suggest((*files).clone(), area.clone(), edit.clone());
    if *tui {
        return terminal_view(
            (&text, &name, &files),
            (&tui, &leave, &opened),
            (show_help, *help),
        );
    }
    let bar = bar(
        (load, save, buttons),
        (toggle, show_help.clone(), &tui),
        (&files, &name, *zoom),
    );
    html! {
        <div class="app" onkeydown={on_key}>
            { chrome::toolbar(bar) }
            <main class={classes!("panes", zoom.then_some("zoomed"), running.then_some("running"))} style={split.style()}>
                { panes::source(&text, *current, focus.clone(), edit, drawn.clone(), (area, &suggest)) }
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

/// The terminal page (`#tui`): a shell where xetal's commands run,
/// opened on the file in xetal edit, drawn in the page; `exit`, Ctrl-D
/// at its prompt, or the bar's button returns to the three panes with
/// the file last edited and its text. Open on the bar edits a file.
fn terminal_view(
    (text, name, files): (
        &UseStateHandle<String>,
        &UseStateHandle<String>,
        &UseStateHandle<Vec<String>>,
    ),
    (tui, leave, opened): (
        &UseStateHandle<bool>,
        &UseStateHandle<u32>,
        &UseStateHandle<(u32, String)>,
    ),
    (help, shown): (Callback<bool>, bool),
) -> Html {
    let (t, n, f, tu) = (text.clone(), name.clone(), files.clone(), tui.clone());
    let on_exit = Callback::from(move |(file, edited): (String, String)| {
        n.set(file);
        t.set(edited);
        f.set(storage::saved());
        tu.set(false);
    });
    let l = leave.clone();
    let back = Callback::from(move |_: ()| l.set(*l + 1));
    let (o, n) = (opened.clone(), name.clone());
    let pick = Callback::from(move |value: String| {
        if let Some((file, _)) = open(&value) {
            n.set(file.clone());
            o.set((o.0 + 1, file));
        }
    });
    let menu = (choices(files), pick);
    html! {
        <div class="app">
            { chrome::terminal_bar(name, back, help.clone(), menu) }
            <main class="tui-main">
                <xetal_webtui::Tui name={(**name).clone()} text={(**text).clone()} files={crate::files::FILES}
                    {on_exit} leave={**leave} open={(**opened).clone()}/>
            </main>
            { chrome::footer() }
            { if shown { chrome::help(help.reform(|_| false)) } else { html! {} } }
        </div>
    }
}

/// The toolbar's controls: what Open, Save, the run buttons, Zoom,
/// Terminal and Help do, and what it shows.
fn bar(
    (load, save, runs): (
        Callback<(String, String)>,
        Callback<bool>,
        chrome::RunButtons,
    ),
    (zoom, help, tui): (Callback<()>, Callback<bool>, &UseStateHandle<bool>),
    (files, name, zoomed): (&UseStateHandle<Vec<String>>, &UseStateHandle<String>, bool),
) -> chrome::Bar {
    let t = tui.clone();
    chrome::Bar {
        load,
        save,
        runs,
        zoom,
        terminal: Callback::from(move |_: ()| t.set(true)),
        help,
        options: choices(files),
        open,
        name: (**name).clone(),
        zoomed,
    }
}
