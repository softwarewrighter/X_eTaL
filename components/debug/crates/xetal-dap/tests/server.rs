//! The adapter driven as a client drives it: launch, a breakpoint, the
//! first stop, the stack, the variables, a watch, steps forward and
//! back, the end; each request answered, then its output and events.

use serde_json::{Value, json};
use xetal_dap::{Framing, Server};

fn program() -> String {
    let path = std::env::temp_dir().join(format!("xetal-dap-{}.xtl", std::process::id()));
    let text = "total! := 0\nu:a_dd := { n ->\n  total! := total! + n\n  total!\n}\nu:a_dd 1\nx := u:a_dd 2\nx * 10\n";
    std::fs::write(&path, text).unwrap();
    path.display().to_string()
}

/// The messages answering each request, in order.
fn session(requests: &[Value]) -> Vec<Value> {
    let mut server = Server::default();
    let mut out = Vec::new();
    for (i, r) in requests.iter().enumerate() {
        let mut r = r.clone();
        r["seq"] = json!(i + 1);
        r["type"] = json!("request");
        out.extend(server.handle(&r).0);
    }
    out
}

fn named<'v>(all: &'v [Value], event: &str) -> Vec<&'v Value> {
    all.iter().filter(|m| m["event"] == event).collect()
}

fn answer<'v>(all: &'v [Value], command: &str) -> Vec<&'v Value> {
    all.iter()
        .filter(|m| m["type"] == "response" && m["command"] == command)
        .collect()
}

#[test]
fn a_session_steps_forward_and_back() {
    let path = program();
    let all = session(&[
        json!({"command": "initialize", "arguments": {"adapterID": "xetal"}}),
        json!({"command": "launch", "arguments": {"program": path, "seed": 1}}),
        json!({"command": "setBreakpoints", "arguments": {"source": {"path": path}, "breakpoints": [{"line": 4}]}}),
        json!({"command": "configurationDone"}),
        json!({"command": "stackTrace", "arguments": {"threadId": 1}}),
        json!({"command": "scopes", "arguments": {"frameId": 1}}),
        json!({"command": "variables", "arguments": {"variablesReference": 1_000_000}}),
        json!({"command": "evaluate", "arguments": {"expression": "n * 100"}}),
        json!({"command": "continue", "arguments": {"threadId": 1}}),
        json!({"command": "reverseContinue", "arguments": {"threadId": 1}}),
        json!({"command": "continue", "arguments": {"threadId": 1}}),
        json!({"command": "continue", "arguments": {"threadId": 1}}),
        json!({"command": "disconnect"}),
    ]);
    assert!(
        all.iter()
            .filter(|m| m["type"] == "response")
            .all(|m| m["success"] == true),
        "{all:#?}"
    );
    assert_eq!(
        answer(&all, "initialize")[0]["body"]["supportsStepBack"],
        true
    );
    let reasons: Vec<&Value> = named(&all, "stopped")
        .iter()
        .map(|e| &e["body"]["reason"])
        .collect();
    assert_eq!(
        reasons,
        ["breakpoint", "breakpoint", "breakpoint", "breakpoint"]
    );
    let frames = &answer(&all, "stackTrace")[0]["body"]["stackFrames"];
    assert_eq!(frames[0]["line"], 4);
    assert_eq!(frames[0]["source"]["path"], path.as_str());
    let locals = &answer(&all, "variables")[0]["body"]["variables"];
    assert_eq!(locals[0]["name"], "n");
    assert_eq!(locals[0]["value"], "1");
    assert_eq!(answer(&all, "evaluate")[0]["body"]["result"], "100");
    assert_eq!(named(&all, "terminated").len(), 1);
    let printed: String = named(&all, "output")
        .iter()
        .map(|e| e["body"]["output"].as_str().unwrap())
        .collect();
    assert_eq!(printed, "1\n30\n");
    let _ = std::fs::remove_file(&path);
}

#[test]
fn requests_before_launch_and_unknown_ones_are_refused() {
    let all = session(&[
        json!({"command": "stackTrace"}),
        json!({"command": "launch", "arguments": {}}),
        json!({"command": "frobnicate"}),
    ]);
    assert!(all.iter().all(|m| m["success"] == false), "{all:#?}");
}

#[test]
fn framing_reads_and_writes_content_length_headers() {
    let body = r#"{"seq":1,"type":"request","command":"initialize"}"#;
    let input = format!("Content-Length: {}\r\n\r\n{body}", body.len());
    let mut out = Vec::new();
    xetal_dap::serve(&mut input.as_bytes(), &mut out, Framing::Headers).unwrap();
    let text = String::from_utf8(out).unwrap();
    assert!(text.starts_with("Content-Length: "), "{text}");
    assert!(text.contains("\"supportsStepBack\":true"), "{text}");
}
