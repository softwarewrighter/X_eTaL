//! The hook: the list's state, opened after a pause, and its keys.

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;
use web_sys::HtmlTextAreaElement;
use xetal_complete::{Completion, Context, complete};
use yew::prelude::*;

use crate::list::{accept, byte_offset, draw};

/// What the pane gets: the list to draw after the text area, the key
/// handler for the text area, and what to call after each input.
pub struct Suggest {
    pub list: Html,
    pub on_key: Callback<KeyboardEvent>,
    pub on_input: Callback<()>,
}

/// The list open (the completion and the chosen candidate), or closed.
pub(crate) type Open = UseStateHandle<Option<(Completion, usize)>>;

/// How long typing pauses before the list opens, in milliseconds.
const PAUSE: i32 = 300;

/// Completion for the text area `area`; `files` are the saved files
/// (library names), `edit` takes the text after a candidate is written.
#[hook]
pub fn use_suggest(files: Vec<String>, area: NodeRef, edit: Callback<String>) -> Suggest {
    let open: Open = use_state(|| None);
    let timer = use_mut_ref(|| None::<i32>);
    let (o, a) = (open.clone(), area.clone());
    let on_input = Callback::from(move |_: ()| schedule(&o, &timer, &a, &files));
    let on_key = keys(open.clone(), area.clone(), edit.clone());
    let list = draw(&open, &area, &edit);
    Suggest {
        list,
        on_key,
        on_input,
    }
}

/// After the pause without another input, open the list for the word
/// at the text area's cursor (or close it when nothing is offered).
fn schedule(open: &Open, timer: &Rc<RefCell<Option<i32>>>, area: &NodeRef, files: &[String]) {
    let Some(window) = web_sys::window() else {
        return;
    };
    if let Some(id) = timer.borrow_mut().take() {
        window.clear_timeout_with_handle(id);
    }
    let (open, area, files) = (open.clone(), area.clone(), files.to_vec());
    let later = Closure::once_into_js(move || open.set(found(&area, &files)));
    let id =
        window.set_timeout_with_callback_and_timeout_and_arguments_0(later.unchecked_ref(), PAUSE);
    *timer.borrow_mut() = id.ok();
}

/// The completion at the text area's cursor; libraries are the
/// standard ones and the saved files.
fn found(area: &NodeRef, files: &[String]) -> Option<(Completion, usize)> {
    let area = area.cast::<HtmlTextAreaElement>()?;
    let text = area.value();
    let at = area.selection_start().ok().flatten().unwrap_or(0) as usize;
    let library = |name: &str| {
        let standard = xetal_libs::standard(name).map(String::from);
        standard.or_else(|| xetal_store::read(&format!("{name}.xtl")).ok())
    };
    let ctx = Context {
        files,
        library: &library,
    };
    complete(&text, byte_offset(&text, at), &ctx).map(|c| (c, 0))
}

/// While the list is open: Tab or Enter accepts the chosen candidate,
/// Up and Down choose, Escape closes; any other key is the text area's.
fn keys(open: Open, area: NodeRef, edit: Callback<String>) -> Callback<KeyboardEvent> {
    Callback::from(move |e: KeyboardEvent| {
        let Some((c, i)) = (*open).clone() else {
            return;
        };
        let n = c.candidates.len();
        match e.key().as_str() {
            "Tab" | "Enter" => {
                accept(&area, &c, i, &edit);
                open.set(None);
            }
            "ArrowDown" => open.set(Some((c, (i + 1) % n))),
            "ArrowUp" => open.set(Some((c, (i + n - 1) % n))),
            "Escape" => open.set(None),
            _ => return,
        }
        e.prevent_default();
    })
}
