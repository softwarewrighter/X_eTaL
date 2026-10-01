//! The run buttons: Run (Stop while a run goes on), Notebook (Run
//! shows each statement above its output), Step (the next statement,
//! as a notebook) and Reset (back to the types, the steps forgotten).

use yew::prelude::*;

/// What the run buttons do, and what they show.
pub struct RunButtons {
    pub run: Callback<()>,
    pub step: Callback<()>,
    /// Stop any run, reset the steps and show the types again.
    pub clear: Callback<()>,
    pub toggle: Callback<()>,
    /// A library is checked, not run: the run buttons are off.
    pub library: bool,
    /// A run is going: Run is Stop.
    pub running: bool,
    /// Run shows a notebook.
    pub notebook: bool,
    /// The statements Step has run, of how many.
    pub stepped: usize,
    pub statements: usize,
}

pub fn run_buttons(b: &RunButtons) -> Html {
    let done = b.stepped >= b.statements;
    let step_title = match done {
        true => "Every statement has run: Reset to start again",
        false => "Run the next statement (as a notebook)",
    };
    html! {
        <>
            <button class={classes!(b.running.then_some("stop"))} onclick={b.run.reform(|_| ())}
                disabled={b.library} title={run_title(b)}>{ if b.running { "Stop" } else { "Run" } }</button>
            <button class={classes!(b.notebook.then_some("on"))} onclick={b.toggle.reform(|_| ())}
                title="Run shows each statement above its output">{ "Notebook" }</button>
            <button onclick={b.step.reform(|_| ())} disabled={b.library || b.running || done}
                title={step_title}>{ format!("Step {}/{}", b.stepped, b.statements) }</button>
            if b.stepped > 0 {
                <button onclick={b.clear.reform(|_| ())} title="Back to the types; Step starts again">{ "Reset" }</button>
            }
        </>
    }
}

fn run_title(b: &RunButtons) -> &'static str {
    match (b.library, b.running) {
        (true, _) => "A library is not run: its exports' types are below",
        (false, true) => "Stop the run (Ctrl-Enter)",
        (false, false) => "Run (Ctrl-Enter)",
    }
}
