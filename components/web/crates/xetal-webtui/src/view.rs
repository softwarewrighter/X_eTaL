//! The component: an editor and an in-memory terminal, redrawn after
//! every key, shown as a focusable block of rows.

use std::path::Path;

use ratatui::Terminal;
use ratatui::backend::TestBackend;
use web_sys::HtmlElement;
use xetal_edit::{Editor, Flow, StoreLibraries};
use yew::prelude::*;

use crate::cells::{Run, rows};
use crate::keys::key_of;

/// The terminal's size in cells.
pub const COLS: u16 = 100;
pub const ROWS: u16 = 32;

/// The file's name and text to edit, and what takes the text back
/// when the editor quits (Ctrl-Q).
#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub text: String,
    pub on_exit: Callback<String>,
    /// Counts the host's requests to leave: each change hands the text
    /// back as Ctrl-Q does.
    #[prop_or_default]
    pub leave: u32,
}

/// The editor and the terminal it draws into.
pub(crate) struct Session {
    editor: Editor,
    terminal: Terminal<TestBackend>,
}

impl Session {
    pub(crate) fn new(name: &str, text: &str) -> Session {
        let editor = Editor::with_text(Path::new(name), text, Box::new(StoreLibraries));
        let terminal = Terminal::new(TestBackend::new(COLS, ROWS)).expect("an in-memory terminal");
        Session { editor, terminal }
    }

    /// Draw the editor and give the rows, the cursor marked when shown.
    pub(crate) fn draw(&mut self) -> Vec<Vec<Run>> {
        let Session { editor, terminal } = self;
        let _ = terminal.draw(|f| editor.draw(f));
        let backend = terminal.backend();
        let cursor = backend.cursor_visible().then(|| {
            let at = backend.cursor_position();
            (at.x, at.y)
        });
        rows(backend.buffer(), cursor)
    }
}

#[function_component(Tui)]
pub fn tui(props: &Props) -> Html {
    let session = use_mut_ref(|| Session::new(&props.name, &props.text));
    let redraw = use_force_update();
    let block = use_node_ref();
    let b = block.clone();
    use_effect_with((), move |_| {
        if let Some(el) = b.cast::<HtmlElement>() {
            let _ = el.focus();
        }
    });
    let (s, exit) = (session.clone(), props.on_exit.clone());
    use_effect_with(props.leave, move |n| {
        if *n > 0 {
            exit.emit(s.borrow().editor.text());
        }
    });
    let (s, exit) = (session.clone(), props.on_exit.clone());
    let on_key = Callback::from(move |e: KeyboardEvent| {
        let ctrl = e.ctrl_key() || e.meta_key();
        let Some(key) = key_of(&e.key(), ctrl, e.alt_key(), e.shift_key()) else {
            return;
        };
        e.prevent_default();
        let flow = s.borrow_mut().editor.handle(key);
        match flow {
            Flow::Quit => exit.emit(s.borrow().editor.text()),
            Flow::Continue => redraw.force_update(),
        }
    });
    let drawn = session.borrow_mut().draw();
    let lines = drawn.into_iter().map(|row| {
        html! { <div class="row">{ for row.into_iter().map(|r| html! { <span class={r.class}>{ r.text }</span> }) }</div> }
    });
    html! {
        <pre class="tui" tabindex="0" ref={block} onkeydown={on_key}
            aria-label="xetal edit, as in the terminal">{ for lines }</pre>
    }
}
