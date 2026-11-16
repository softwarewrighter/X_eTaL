//! Moving through the run: the first stop after `configurationDone`,
//! the step requests forward and back, and the events a stop gives.

use serde_json::{Value, json};
use xetal_debug::{Reason, Stop};

use crate::launch::Session;

/// The first stop: the entry when asked for, else the first breakpoint
/// (or the end).
pub(crate) fn begin(session: &mut Session) -> Stop {
    let entry = session.dbg.start();
    match session.stop_on_entry || entry.reason != Reason::Entry {
        true => entry,
        false => session.dbg.resume(),
    }
}

/// The stop a step request `command` reaches; `None` when it is not one.
pub(crate) fn step(session: &mut Session, command: &str, args: &Value) -> Option<Stop> {
    let dbg = &mut session.dbg;
    Some(match command {
        "next" => dbg.step_over(),
        "stepIn" => dbg.step_in(),
        "stepOut" => dbg.step_out(),
        "continue" => dbg.resume(),
        "stepBack" => dbg.step_back(),
        "reverseContinue" => dbg.reverse_continue(),
        "xetal/runTo" => dbg.run_to(args["step"].as_u64().unwrap_or(0) as usize),
        _ => return None,
    })
}

/// The events a stop gives, in order: a note for each file written
/// after it (when it was reached going back), then `stopped`, or at
/// the end `exited` and `terminated`.
pub(crate) fn events(stop: &Stop) -> Vec<(&'static str, Value)> {
    let mut out: Vec<(&'static str, Value)> = stop
        .rewound
        .iter()
        .map(|path| {
            let note = format!("stepped back over a write to {path}; the file stays as written\n");
            ("output", json!({"category": "console", "output": note}))
        })
        .collect();
    let stopped =
        |reason: &str| json!({"reason": reason, "threadId": 1, "allThreadsStopped": true});
    match &stop.reason {
        Reason::Entry => out.push(("stopped", stopped("entry"))),
        Reason::Step => out.push(("stopped", stopped("step"))),
        Reason::Breakpoint => out.push(("stopped", stopped("breakpoint"))),
        Reason::Error(d) => {
            let mut body = stopped("exception");
            body["text"] = json!(d.message);
            out.push(("stopped", body));
        }
        Reason::Waiting | Reason::End => {
            out.push(("exited", json!({"exitCode": 0})));
            out.push(("terminated", json!({})));
        }
    }
    out
}
