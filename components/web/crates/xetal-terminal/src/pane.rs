//! The `Terminal` component: a program, its files and a seed, run in a
//! worker as soon as it is shown and again whenever any of them or the
//! restart count changes.

use xetal_runner::{Mode, Request, use_runs};
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct TerminalProps {
    /// The program's source.
    pub src: AttrValue,
    /// Its libraries and data files, by name, as `xetal-store` holds
    /// them (`"Rules.xtl"`, `"data/words.txt"`).
    #[prop_or_default]
    pub files: Vec<(String, String)>,
    /// The seed for `r_oll!` and the like.
    #[prop_or_default]
    pub seed: u64,
    /// Change it to run the program again from the start.
    #[prop_or_default]
    pub restart: u32,
}

/// A terminal running `src`: it takes the keyboard while the program
/// waits (a line, a key or an event), ticks while it runs on events,
/// and Ctrl-C stops it.
#[function_component(Terminal)]
pub fn terminal(props: &TerminalProps) -> Html {
    let runs = use_runs();
    let printed = use_node_ref();
    let terminal = (&runs, false, Callback::noop());
    let (typing, keys) = xetal_typing::use_terminal(terminal, Callback::noop(), printed.clone());
    let start = runs.start.clone();
    let run = (
        props.src.clone(),
        props.files.clone(),
        props.seed,
        props.restart,
    );
    use_effect_with(run, move |(src, files, seed, _)| {
        start.emit(Request {
            src: src.to_string(),
            seed: *seed,
            mode: Mode::Run,
            boxed: false,
            frames: false,
            files: files.clone(),
        });
    });
    html! {
        <pre class="xetal-terminal" tabindex="0" ref={printed} onkeydown={keys}>
            { crate::body(&runs, &typing) }
        </pre>
    }
}
