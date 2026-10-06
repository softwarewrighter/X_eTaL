//! The page's side: start a run in a worker, take its events as they
//! arrive, give a waiting program the lines typed, stop it.

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::{JsCast, JsValue, closure::Closure};
use web_sys::{ErrorEvent, MessageEvent, Worker};
use xetal_play::Run;
use yew::prelude::*;

use crate::{Action, Event, Mode, Output, Request};

/// The worker's files (trunk builds them beside the page, unhashed).
const WORKER: [&str; 2] = ["xetal-runner.js", "xetal-runner_bg.wasm"];

/// The build id the worker's URLs carry (build.rs), so a new page is
/// never run by a cached old worker.
const BUILD: &str = env!("XETAL_RUNNER_BUILD");

/// The worker's script as a blob URL: trunk's loader shim rewritten
/// with the page's address as the base and the build id on each file,
/// which is what makes the browser fetch this build's worker and not
/// the one it cached (the files' names never change).
fn worker_url() -> Option<String> {
    let base = web_sys::window()?.location().href().ok()?;
    let at = |file: &str| format!("new URL('{file}?v={BUILD}', '{base}').href");
    let script = format!(
        "importScripts({});wasm_bindgen({});",
        at(WORKER[0]),
        at(WORKER[1])
    );
    let parts = js_sys::Array::of1(&JsValue::from_str(&script));
    let options = web_sys::BlobPropertyBag::new();
    options.set_type("text/javascript");
    let blob = web_sys::Blob::new_with_str_sequence_and_options(&parts, &options).ok()?;
    web_sys::Url::create_object_url_with_blob(&blob).ok()
}

/// A handler the worker calls (a message, an error).
type Handler = Closure<dyn FnMut(JsValue)>;

/// The worker running now and its handlers (kept alive with it).
type Live = Rc<RefCell<Option<(Worker, [Handler; 2])>>>;

/// The runs of the page: what is shown, whether arrays print boxed, how
/// many statements Step has run, and how to start (plainly or as a
/// notebook, as the request says), step, stop, clear (which also resets
/// the steps) or toggle boxed printing.
#[derive(Clone, PartialEq)]
pub struct Runs {
    pub output: Output,
    pub boxed: bool,
    pub stepped: usize,
    pub start: Callback<Request>,
    pub step: Callback<Request>,
    pub stop: Callback<()>,
    pub clear: Callback<()>,
    pub toggle_boxed: Callback<()>,
    /// A line typed in the terminal for the waiting program.
    pub type_line: Callback<String>,
    /// A key pressed (by name) for a program waiting for one.
    pub press_key: Callback<String>,
}

#[hook]
pub fn use_runs() -> Runs {
    let state = use_reducer(Output::default);
    let live: Live = use_mut_ref(|| None);
    let (boxed, stepped) = (use_state(|| false), use_state(|| 0usize));
    let (start, step) = starting(&state.dispatcher(), &live, &stepped, *boxed);
    let (stop, clear) = ending(&state.dispatcher(), &live, &stepped);
    let b = boxed.clone();
    let toggle_boxed = Callback::from(move |_: ()| b.set(!*b));
    let (type_line, press_key) = typing(&state.dispatcher(), &live);
    let output = (*state).clone();
    let (stepped, boxed) = (*stepped, *boxed);
    Runs {
        output,
        boxed,
        stepped,
        start,
        step,
        stop,
        clear,
        toggle_boxed,
        type_line,
        press_key,
    }
}

/// A typed line (echoed in the output) or a key pressed (not echoed),
/// sent to the worker for the waiting program.
fn typing(
    state: &UseReducerDispatcher<Output>,
    live: &Live,
) -> (Callback<String>, Callback<String>) {
    let send = |s: UseReducerDispatcher<Output>, l: Live, echo: bool| {
        Callback::from(move |text: String| {
            if let Some((worker, _)) = l.borrow().as_ref() {
                let _ = worker.post_message(&crate::line_message(&text).as_str().into());
            }
            s.dispatch(if echo {
                Action::Typed(text)
            } else {
                Action::Pressed
            });
        })
    };
    let (s, l) = (state.clone(), live.clone());
    (send(s.clone(), l.clone(), true), send(s, l, false))
}

/// Start: run (which also resets the steps); Step: run as a notebook up
/// to the next statement.
fn starting(
    state: &UseReducerDispatcher<Output>,
    live: &Live,
    stepped: &UseStateHandle<usize>,
    boxed: bool,
) -> (Callback<Request>, Callback<Request>) {
    let (s, l, k) = (state.clone(), live.clone(), stepped.clone());
    let start = Callback::from(move |req: Request| {
        k.set(0);
        begin(Request { boxed, ..req }, &s, &l)
    });
    let (s, l, k) = (state.clone(), live.clone(), stepped.clone());
    let step = Callback::from(move |req: Request| {
        k.set(*k + 1);
        let mode = Mode::Notebook(Some(*k + 1));
        begin(Request { mode, boxed, ..req }, &s, &l)
    });
    (start, step)
}

/// Stop: end the run, keeping what it showed; Clear: end it, reset the
/// steps and show the types again.
fn ending(
    state: &UseReducerDispatcher<Output>,
    live: &Live,
    stepped: &UseStateHandle<usize>,
) -> (Callback<()>, Callback<()>) {
    let (s, l) = (state.clone(), live.clone());
    let stop = Callback::from(move |_: ()| {
        if end(&l) {
            s.dispatch(Action::Stop);
        }
    });
    let (s, l, k) = (state.clone(), live.clone(), stepped.clone());
    let clear = Callback::from(move |_: ()| {
        end(&l);
        k.set(0);
        s.dispatch(Action::Clear);
    });
    (stop, clear)
}

/// End the worker running now, if any; true when there was one.
fn end(live: &Live) -> bool {
    live.borrow_mut()
        .take()
        .map(|(w, _)| w.terminate())
        .is_some()
}

fn begin(req: Request, state: &UseReducerDispatcher<Output>, live: &Live) {
    end(live);
    state.dispatch(Action::Start);
    let Ok(worker) = worker_url()
        .ok_or(())
        .and_then(|url| Worker::new(&url).map_err(drop))
    else {
        let err = "the worker that runs programs could not start\n".into();
        return state.dispatch(Action::Finished(Run {
            err,
            ..Run::default()
        }));
    };
    let (s, w, request) = (state.clone(), worker.clone(), req.encode());
    let on_message = Handler::new(move |m: JsValue| {
        let m: MessageEvent = m.unchecked_into();
        if let Some(event) = m.data().as_string().and_then(|t| Event::decode(&t)) {
            match &event {
                Event::Ready => drop(w.post_message(&request.as_str().into())),
                Event::Wrote(path, text) => drop(xetal_store::write(path, text)),
                _ => {}
            }
            s.dispatch(Action::Event(event));
        }
    });
    let on_error = failing(state, live);
    worker.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
    worker.set_onerror(Some(on_error.as_ref().unchecked_ref()));
    *live.borrow_mut() = Some((worker, [on_message, on_error]));
}

/// When the worker fails (a stack overflow in deep recursion, say), the
/// run ends with the error shown, instead of seeming to run on.
fn failing(state: &UseReducerDispatcher<Output>, live: &Live) -> Handler {
    let (s, l) = (state.clone(), live.clone());
    Handler::new(move |e: JsValue| {
        let e: ErrorEvent = e.unchecked_into();
        let why = format!(
            "error[stopped]: the run failed in the browser: {}",
            e.message()
        );
        s.dispatch(Action::Event(Event::Err(why)));
        s.dispatch(Action::Event(Event::Done));
        if let Some((worker, _)) = l.borrow().as_ref() {
            worker.terminate();
        }
    })
}
