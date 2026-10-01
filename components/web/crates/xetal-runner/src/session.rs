//! The page's side: start a run (in a worker, or on the page when it
//! reads the keyboard), take its events as they arrive, stop it.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use wasm_bindgen::{JsCast, JsValue, closure::Closure};
use web_sys::{ErrorEvent, MessageEvent, Worker};
use xetal_play::Run;
use yew::prelude::*;

use crate::{Action, Event, Output, Request};

/// The worker's script (trunk builds it beside the page).
const WORKER: &str = "./xetal-runner_loader.js";

/// A handler the worker calls (a message, an error).
type Handler = Closure<dyn FnMut(JsValue)>;

/// The worker running now and its handlers (kept alive with it).
type Live = Rc<RefCell<Option<(Worker, [Handler; 2])>>>;

/// The last lines a run on the page printed, for the keyboard prompt.
static RECENT: Mutex<VecDeque<String>> = Mutex::new(VecDeque::new());

/// The last lines printed by a run on the page, shown when it asks for
/// a line ([]R_EAD), since the page cannot repaint while it runs.
pub fn recent() -> String {
    RECENT
        .lock()
        .map(|r| r.iter().cloned().collect::<Vec<_>>().join("\n"))
        .unwrap_or_default()
}

/// The runs of the page: what is shown, and how to start, stop or clear.
#[derive(Clone, PartialEq)]
pub struct Runs {
    pub output: Output,
    pub start: Callback<Request>,
    pub stop: Callback<()>,
    pub clear: Callback<()>,
}

#[hook]
pub fn use_runs() -> Runs {
    let state = use_reducer(Output::default);
    let live: Live = use_mut_ref(|| None);
    let (s, l) = (state.dispatcher(), live.clone());
    let start = Callback::from(move |req: Request| begin(req, &s, &l));
    let (s, l) = (state.dispatcher(), live.clone());
    let stop = Callback::from(move |_: ()| {
        if end(&l) {
            s.dispatch(Action::Stop);
        }
    });
    let (s, l) = (state.dispatcher(), live);
    let clear = Callback::from(move |_: ()| {
        end(&l);
        s.dispatch(Action::Clear);
    });
    Runs {
        output: (*state).clone(),
        start,
        stop,
        clear,
    }
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
    if req.src.contains("[]R_EAD") {
        state.dispatch(Action::Finished(on_page(&req)));
        return;
    }
    let Ok(worker) = Worker::new(WORKER) else {
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

/// Run on the page (a program reading the keyboard), keeping the last
/// lines printed for the prompt.
fn on_page(req: &Request) -> Run {
    let printed = Arc::new(Mutex::new(String::new()));
    let p = printed.clone();
    if let Ok(mut recent) = RECENT.lock() {
        recent.clear();
    }
    let mut out = xetal_play::Lines::new(move |line: &str| {
        if let Ok(mut s) = p.lock() {
            s.push_str(&format!("{line}\n"));
        }
        if let Ok(mut recent) = RECENT.lock() {
            recent.push_back(line.into());
            if recent.len() > 12 {
                recent.pop_front();
            }
        }
    });
    let run = xetal_play::run_to(&req.src, req.seed, &mut out);
    drop(out);
    let out = printed.lock().map(|s| s.clone()).unwrap_or_default() + &run.out;
    Run { out, ..run }
}
