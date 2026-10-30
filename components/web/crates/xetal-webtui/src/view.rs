//! The component: a shell (xetal's commands at a prompt, the REPL, the
//! editor full screen) and an in-memory terminal, redrawn after every
//! key, shown as a focusable block of rows.

use ratatui::Terminal;
use ratatui::backend::TestBackend;
use web_sys::HtmlElement;
use xetal_shell::{Catalog, Flow, Shell};
use yew::prelude::*;

use crate::cells::{Run, rows};
use crate::keys::key_of;

/// The terminal's size in cells.
pub const COLS: u16 = 100;
pub const ROWS: u16 = 32;

/// The host's files: how a path is read and which paths `ls` lists
/// (the store, the demos, the standard libraries).
#[derive(Clone, Copy)]
pub struct Files {
    pub read: fn(&str) -> Option<String>,
    pub list: fn() -> Vec<String>,
}

impl PartialEq for Files {
    /// The host's files never change while the page is open.
    fn eq(&self, _: &Files) -> bool {
        true
    }
}

impl Catalog for Files {
    fn read(&self, path: &str) -> Option<String> {
        (self.read)(path)
    }

    fn list(&self) -> Vec<String> {
        (self.list)()
    }
}

/// The file's name and text to open, the host's files, and what takes
/// the file's name and text back when the shell ends (`exit`, Ctrl-D).
#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub text: String,
    pub files: Files,
    pub on_exit: Callback<(String, String)>,
    /// Counts the host's requests to leave: each change hands the file
    /// back as `exit` does.
    #[prop_or_default]
    pub leave: u32,
    /// Counts the host's requests to open a file (Open on the bar), and
    /// the file's name: each change runs `xetal edit NAME`.
    #[prop_or_default]
    pub open: (u32, String),
}

/// The shell and the terminal it draws into.
pub(crate) struct Session {
    shell: Shell,
    terminal: Terminal<TestBackend>,
}

impl Session {
    pub(crate) fn new(props: &Props) -> Session {
        let shell = Shell::new(&props.name, &props.text, Box::new(props.files));
        let terminal = Terminal::new(TestBackend::new(COLS, ROWS)).expect("an in-memory terminal");
        Session { shell, terminal }
    }

    /// Draw the shell and give the rows, the cursor marked when shown.
    pub(crate) fn draw(&mut self) -> Vec<Vec<Run>> {
        let Session { shell, terminal } = self;
        let _ = terminal.draw(|f| shell.draw(f));
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
    let session = use_mut_ref(|| Session::new(props));
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
            exit.emit(s.borrow_mut().shell.leave());
        }
    });
    let (s, r, b) = (session.clone(), redraw.clone(), block.clone());
    use_effect_with(props.open.clone(), move |(n, name)| {
        if *n > 0 {
            s.borrow_mut().shell.open(name);
            r.force_update();
            if let Some(el) = b.cast::<HtmlElement>() {
                let _ = el.focus();
            }
        }
    });
    let (s, exit) = (session.clone(), props.on_exit.clone());
    let on_key = Callback::from(move |e: KeyboardEvent| {
        let ctrl = e.ctrl_key() || e.meta_key();
        let Some(key) = key_of(&e.key(), ctrl, e.alt_key(), e.shift_key()) else {
            return;
        };
        e.prevent_default();
        let seed = (js_sys::Math::random() * 4_294_967_296.0) as u64;
        let flow = {
            let mut session = s.borrow_mut();
            session.shell.seed(seed);
            session.shell.handle(key)
        };
        match flow {
            Flow::Exit => exit.emit(s.borrow_mut().shell.leave()),
            Flow::Continue => redraw.force_update(),
        }
    });
    let drawn = session.borrow_mut().draw();
    let lines = drawn.into_iter().map(|row| {
        html! { <div class="row">{ for row.into_iter().map(|r| html! { <span class={r.class}>{ r.text }</span> }) }</div> }
    });
    html! {
        <pre class="tui" tabindex="0" ref={block} onkeydown={on_key}
            aria-label="xetal in a terminal: a shell, the REPL and the editor">{ for lines }</pre>
    }
}
