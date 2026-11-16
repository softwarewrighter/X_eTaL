//! Messages on the wire: DAP frames each JSON message with a
//! `Content-Length` header; `--lines` writes one message a line (for
//! transcripts and tests).

use std::io::{BufRead, Result, Write};

use serde_json::Value;

use crate::server::Server;

/// How messages are framed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Framing {
    /// `Content-Length: N`, a blank line, then N bytes of JSON (DAP).
    Headers,
    /// One JSON message a line.
    Lines,
}

/// Answer the requests read from `input` on `output` until the client
/// disconnects or the input ends.
pub fn serve(input: &mut dyn BufRead, output: &mut dyn Write, framing: Framing) -> Result<()> {
    let mut server = Server::default();
    while let Some(message) = read(input, framing)? {
        let (replies, done) = server.handle(&message);
        for reply in replies {
            write(output, framing, &reply)?;
        }
        if done {
            break;
        }
    }
    Ok(())
}

/// The next message, or `None` at the end of the input. A line or a
/// body that is not JSON is skipped as a JSON null.
pub fn read(input: &mut dyn BufRead, framing: Framing) -> Result<Option<Value>> {
    let mut line = String::new();
    if framing == Framing::Lines {
        loop {
            line.clear();
            if input.read_line(&mut line)? == 0 {
                return Ok(None);
            }
            if !line.trim().is_empty() {
                return Ok(Some(serde_json::from_str(&line).unwrap_or(Value::Null)));
            }
        }
    }
    let mut length = None;
    loop {
        line.clear();
        if input.read_line(&mut line)? == 0 {
            return Ok(None);
        }
        let header = line.trim();
        if header.is_empty() && length.is_some() {
            break;
        }
        if let Some(n) = header.strip_prefix("Content-Length:") {
            length = n.trim().parse::<usize>().ok();
        }
    }
    let mut body = vec![0; length.unwrap_or(0)];
    input.read_exact(&mut body)?;
    Ok(Some(serde_json::from_slice(&body).unwrap_or(Value::Null)))
}

/// Write `message`, framed.
pub fn write(output: &mut dyn Write, framing: Framing, message: &Value) -> Result<()> {
    let body = message.to_string();
    match framing {
        Framing::Lines => writeln!(output, "{body}")?,
        Framing::Headers => write!(output, "Content-Length: {}\r\n\r\n{body}", body.len())?,
    }
    output.flush()
}
