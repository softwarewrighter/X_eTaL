//! `launch`: load and check the program, and make the session the rest
//! of the requests drive. One launch a process: the program and its
//! sources live as long as the adapter (they are leaked), the output is
//! kept until it goes out as `output` events, and `[]R_EAD` reads the
//! lines the configuration gives (standard input is the protocol's).

use std::cell::RefCell;
use std::collections::VecDeque;
use std::io::Write;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use serde_json::Value;
use xetal_debug::Debugger;
use xetal_store::{Disk, Store};

/// A launched program and its debugger.
pub(crate) struct Session {
    pub(crate) dbg: Debugger<'static, 'static>,
    pub(crate) out: Rc<RefCell<Vec<u8>>>,
    pub(crate) stop_on_entry: bool,
}

/// The program's output, kept for the next `output` event.
struct Kept(Rc<RefCell<Vec<u8>>>);

impl Write for Kept {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.borrow_mut().extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// The disk, with the typed lines the launch configuration gives.
struct Typed(Mutex<VecDeque<String>>);

impl Store for Typed {
    fn get(&self, path: &str) -> Result<String, String> {
        Disk.get(path)
    }

    fn put(&self, path: &str, text: &str) -> Result<(), String> {
        Disk.put(path, text)
    }

    fn line(&self) -> Result<String, String> {
        let mut lines = self.0.lock().map_err(|e| e.to_string())?;
        lines.pop_front().ok_or_else(|| {
            "no more input: give the lines the program reads in the launch configuration's \"input\"".into()
        })
    }
}

/// The session for launch arguments `{program, seed?, stopOnEntry?,
/// input?}`, or why there is none.
pub(crate) fn launch(args: &Value) -> Result<Session, String> {
    let program = args["program"]
        .as_str()
        .ok_or("launch needs \"program\", a file")?;
    let text = std::fs::read_to_string(program).map_err(|e| format!("{program}: {e}"))?;
    let mut loaded = xetal_program::load(program, &text).map_err(|d| d.to_string())?;
    xetal_types::check_program(&mut loaded.program).map_err(|d| d.to_string())?;
    let loaded = Box::leak(Box::new(loaded));
    let lines = args["input"].as_array().into_iter().flatten();
    let lines = lines.filter_map(|l| l.as_str().map(String::from)).collect();
    xetal_store::install(Arc::new(Typed(Mutex::new(lines))));
    let out = Rc::new(RefCell::new(Vec::new()));
    let sink: &'static mut dyn Write = Box::leak(Box::new(Kept(out.clone())));
    let seed = args["seed"].as_u64().unwrap_or(0);
    let dbg = Debugger::new(&loaded.program, &loaded.sources, sink, seed);
    let stop_on_entry = args["stopOnEntry"].as_bool().unwrap_or(false);
    Ok(Session {
        dbg,
        out,
        stop_on_entry,
    })
}
