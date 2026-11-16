//! The adapter: each request answered with its response, then the
//! program's new output and the events the request caused (`stopped`,
//! `terminated`), numbered in one sequence.

use std::collections::HashMap;

use serde_json::{Value, json};
use xetal_debug::Var;

use crate::inspect;
use crate::launch::{Session, launch};
use crate::motion;

/// What the adapter keeps between requests.
#[derive(Default)]
pub struct Server {
    seq: i64,
    session: Option<Session>,
    /// The variables of the scopes shown at this stop, by reference.
    kept: HashMap<i64, Vec<Var>>,
}

type Answer = (Result<Value, String>, Vec<(&'static str, Value)>);

impl Server {
    /// The messages answering `request`, and whether the client has
    /// disconnected.
    pub fn handle(&mut self, request: &Value) -> (Vec<Value>, bool) {
        let command = request["command"].as_str().unwrap_or_default();
        let (result, events) = self.dispatch(command, &request["arguments"]);
        let mut out = vec![self.response(request, result)];
        let output = self
            .session
            .as_ref()
            .map(|s| std::mem::take(&mut *s.out.borrow_mut()));
        if let Some(text) = output.filter(|o| !o.is_empty()) {
            let text = String::from_utf8_lossy(&text).into_owned();
            out.push(self.event("output", json!({"category": "stdout", "output": text})));
        }
        for (name, body) in events {
            out.push(self.event(name, body));
        }
        (out, command == "disconnect")
    }

    fn dispatch(&mut self, command: &str, args: &Value) -> Answer {
        let none = Vec::new();
        if command == "initialize" {
            return (Ok(capabilities()), none);
        }
        if command == "launch" {
            return match launch(args) {
                Ok(session) => {
                    self.session = Some(session);
                    (Ok(json!({})), vec![("initialized", json!({}))])
                }
                Err(e) => (Err(e), none),
            };
        }
        if matches!(command, "disconnect" | "pause") {
            return (Ok(json!({})), none);
        }
        let Some(session) = self.session.as_mut() else {
            return (Err(format!("{command} before launch")), none);
        };
        if command == "configurationDone" {
            let stop = motion::begin(session);
            return (Ok(json!({})), motion::events(&stop));
        }
        if let Some(stop) = motion::step(session, command, args) {
            return (
                Ok(json!({"allThreadsContinued": true})),
                motion::events(&stop),
            );
        }
        let dbg = &mut session.dbg;
        let result = match command {
            "setBreakpoints" => Ok(inspect::breakpoints(dbg, args)),
            "threads" => Ok(json!({"threads": [{"id": 1, "name": "main"}]})),
            "stackTrace" => Ok(inspect::stack(dbg)),
            "scopes" => Ok(inspect::scopes(dbg, &mut self.kept)),
            "variables" => Ok(inspect::variables(dbg, &self.kept, args)),
            "evaluate" => inspect::evaluate(dbg, args),
            _ => Err(format!("{command} is not supported")),
        };
        (result, none)
    }

    fn response(&mut self, request: &Value, result: Result<Value, String>) -> Value {
        self.seq += 1;
        let mut out = json!({
            "seq": self.seq,
            "type": "response",
            "request_seq": request["seq"],
            "command": request["command"],
            "success": result.is_ok(),
        });
        match result {
            Ok(body) => out["body"] = body,
            Err(message) => out["message"] = json!(message),
        }
        out
    }

    fn event(&mut self, name: &str, body: Value) -> Value {
        self.seq += 1;
        json!({"seq": self.seq, "type": "event", "event": name, "body": body})
    }
}

/// What the adapter can do (`initialize`'s answer).
fn capabilities() -> Value {
    json!({
        "supportsConfigurationDoneRequest": true,
        "supportsStepBack": true,
        "supportsConditionalBreakpoints": true,
        "supportsHitConditionalBreakpoints": true,
        "supportsEvaluateForHovers": true,
    })
}
